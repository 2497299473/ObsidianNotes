package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"syscall"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/redis/go-redis/v9"
	"go.opentelemetry.io/contrib/instrumentation/github.com/gin-gonic/gin/otelgin"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracehttp"
	"go.opentelemetry.io/otel/propagation"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.24.0"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// Todo 模型
type Todo struct {
	ID        uint      `json:"id" gorm:"primaryKey"`
	Title     string    `json:"title" gorm:"not null"`
	Done      bool      `json:"done" gorm:"default:false"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

var (
	db    *gorm.DB
	cache *redis.Client

	// Prometheus 指标定义（对应 DockerNew 第 10 章：监控）
	requestDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "http_request_duration_seconds",
			Help:    "HTTP 请求耗时分布",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"method", "path", "status"},
	)
	requestTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "http_requests_total",
			Help: "HTTP 请求总数",
		},
		[]string{"method", "path", "status"},
	)
	dbOperations = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "db_operations_total",
			Help: "数据库操作总数",
		},
		[]string{"operation", "table"},
	)
)

func init() {
	prometheus.MustRegister(requestDuration)
	prometheus.MustRegister(requestTotal)
	prometheus.MustRegister(dbOperations)
}

// initTracer 初始化 OpenTelemetry TracerProvider
// 对应 DockerNew 第 10 章：Jaeger + OpenTelemetry 链路追踪
// 通过 OTLP HTTP 协议将 trace 发送到 Jaeger（默认 jaeger:4318）
// 如果 OTEL_EXPORTER_OTLP_ENDPOINT 未设置，返回 nil（tracing 禁用，不影响应用启动）
func initTracer() func() {
	endpoint := os.Getenv("OTEL_EXPORTER_OTLP_ENDPOINT")
	if endpoint == "" {
		log.Println("OTEL_EXPORTER_OTLP_ENDPOINT not set, tracing disabled")
		return func() {}
	}

	ctx := context.Background()

	exporter, err := otlptracehttp.New(ctx,
		otlptracehttp.WithEndpoint(endpoint),
		otlptracehttp.WithInsecure(),
	)
	if err != nil {
		log.Printf("Warning: OTel exporter init failed (Jaeger not running?): %v", err)
		return func() {}
	}

	res, err := resource.New(ctx,
		resource.WithAttributes(
			semconv.ServiceName("go-todo-api"),
			semconv.ServiceVersion("1.0.0"),
		),
	)
	if err != nil {
		log.Printf("Warning: OTel resource init failed: %v", err)
		return func() {}
	}

	tp := sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(exporter),
		sdktrace.WithResource(res),
		sdktrace.WithSampler(sdktrace.AlwaysSample()),
	)
	otel.SetTracerProvider(tp)
	otel.SetTextMapPropagator(propagation.TraceContext{})

	log.Printf("OpenTelemetry tracer initialized, exporting to %s", endpoint)
	return func() {
		shutdownCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_ = tp.Shutdown(shutdownCtx)
	}
}

func main() {
	// 初始化链路追踪（对应 DockerNew 10 章：Jaeger + OpenTelemetry）
	shutdownTracer := initTracer()
	defer shutdownTracer()

	// 初始化数据库
	dsn := os.Getenv("DB_DSN")
	if dsn == "" {
		dsn = "host=db user=app password=devpass dbname=appdb port=5432 sslmode=disable"
	}

	var err error
	db, err = gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatalf("Failed to connect database: %v", err)
	}

	// 自动迁移
	if err := db.AutoMigrate(&Todo{}); err != nil {
		log.Fatalf("Failed to migrate: %v", err)
	}
	log.Println("Database connected & migrated")

	// 初始化 Redis
	redisURL := os.Getenv("REDIS_URL")
	if redisURL == "" {
		redisURL = "redis://cache:6379"
	}
	opts, _ := redis.ParseURL(redisURL)
	cache = redis.NewClient(opts)
	if err := cache.Ping(context.Background()).Err(); err != nil {
		log.Fatalf("Failed to connect Redis: %v", err)
	}
	log.Println("Redis connected")

	// Gin 路由
	r := gin.New()
	r.Use(gin.Logger(), gin.Recovery())

	// OpenTelemetry 链路追踪中间件（对应 DockerNew 10 章）
	// 每个 HTTP 请求自动创建一个 span，记录方法、路径、状态码
	r.Use(otelgin.Middleware("go-todo-api"))

	// Prometheus 指标中间件
	r.Use(prometheusMiddleware())

	r.GET("/health", healthCheck)
	r.GET("/metrics", gin.WrapH(promhttp.Handler()))

	api := r.Group("/api")
	{
		api.GET("/todos", getTodos)
		api.POST("/todos", createTodo)
		api.GET("/todos/:id", getTodo)
		api.PUT("/todos/:id", updateTodo)
		api.DELETE("/todos/:id", deleteTodo)
	}

	// 优雅关闭
	// 读取 PORT 环境变量（默认 8080），拼出监听地址，避免硬编码导致改 PORT 不生效
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	srv := &http.Server{
		Addr:         fmt.Sprintf(":%s", port),
		Handler:      r,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	go func() {
		log.Printf("Server starting on :%s", port)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	// 等待中断信号
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Println("Shutting down server...")

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	if err := srv.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced shutdown: %v", err)
	}
	log.Println("Server stopped gracefully")
}

// prometheusMiddleware 记录 HTTP 请求指标
func prometheusMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		if c.Request.URL.Path == "/metrics" {
			c.Next()
			return
		}

		start := time.Now()
		path := c.Request.URL.Path
		method := c.Request.Method

		c.Next()

		status := strconv.Itoa(c.Writer.Status())
		duration := time.Since(start).Seconds()

		requestDuration.WithLabelValues(method, path, status).Observe(duration)
		requestTotal.WithLabelValues(method, path, status).Inc()
	}
}

func healthCheck(c *gin.Context) {
	sqlDB, err := db.DB()
	if err != nil {
		c.JSON(http.StatusServiceUnavailable, gin.H{"status": "error", "detail": "db error"})
		return
	}
	if err := sqlDB.Ping(); err != nil {
		c.JSON(http.StatusServiceUnavailable, gin.H{"status": "error", "detail": "db unreachable"})
		return
	}
	if err := cache.Ping(c.Request.Context()).Err(); err != nil {
		c.JSON(http.StatusServiceUnavailable, gin.H{"status": "error", "detail": "redis unreachable"})
		return
	}
	c.JSON(http.StatusOK, gin.H{"status": "ok"})
}

func getTodos(c *gin.Context) {
	var todos []Todo
	if err := db.Find(&todos).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	dbOperations.WithLabelValues("select", "todos").Inc()
	c.JSON(http.StatusOK, todos)
}

func createTodo(c *gin.Context) {
	var todo Todo
	if err := c.ShouldBindJSON(&todo); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	if err := db.Create(&todo).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	dbOperations.WithLabelValues("insert", "todos").Inc()
	c.JSON(http.StatusCreated, todo)
}

func getTodo(c *gin.Context) {
	var todo Todo
	if err := db.First(&todo, c.Param("id")).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "not found"})
		return
	}
	dbOperations.WithLabelValues("select", "todos").Inc()
	c.JSON(http.StatusOK, todo)
}

func updateTodo(c *gin.Context) {
	var todo Todo
	if err := db.First(&todo, c.Param("id")).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "not found"})
		return
	}
	if err := c.ShouldBindJSON(&todo); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	if err := db.Save(&todo).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	dbOperations.WithLabelValues("update", "todos").Inc()
	c.JSON(http.StatusOK, todo)
}

func deleteTodo(c *gin.Context) {
	if err := db.Delete(&Todo{}, c.Param("id")).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	dbOperations.WithLabelValues("delete", "todos").Inc()
	c.JSON(http.StatusNoContent, nil)
}
