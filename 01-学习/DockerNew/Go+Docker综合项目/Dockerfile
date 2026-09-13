# syntax=docker/dockerfile:1.7

# ===== 构建阶段 =====
# 对应 DockerNew 第 8 章：多阶段构建
FROM golang:1.22-alpine AS builder

WORKDIR /app

# 缓存友好：先 COPY 依赖文件（对应 DockerNew 04 章）
COPY go.mod go.sum ./
RUN go mod download

COPY . .

# 静态编译 + 去掉调试信息（对应 DockerNew 08 章：-ldflags=-s -w）
RUN CGO_ENABLED=0 GOOS=linux go build \
    -ldflags="-s -w" \
    -o /app/server .

# ===== 运行阶段 =====
# 对应 DockerNew 09 章：生产安全加固
FROM alpine:3.20

# 安装 CA 证书（HTTPS 需要，OTel exporter 也需要）+ tzdata
RUN apk add --no-cache ca-certificates tzdata && \
    update-ca-certificates

# 创建非 root 用户（对应 DockerNew 09 章）
RUN addgroup -S app && adduser -S app -G app -u 1000

WORKDIR /app

# 只复制编译产物（不包含编译器、源码）
COPY --from=builder /app/server /app/server

# 切换到非 root 用户
USER 1000:1000

EXPOSE 8080

# exec 格式（PID 1 是应用本身，能接收 SIGTERM 优雅停止）
ENTRYPOINT ["/app/server"]
