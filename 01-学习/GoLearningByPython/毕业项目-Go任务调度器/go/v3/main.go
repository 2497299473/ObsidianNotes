// v3-接口与错误处理版
// 引入 Runnable 接口和显式错误处理
// 用自定义错误类型和 errors.Is/errors.As 替代 Python 的 try/except
package main

import (
	"errors"
	"fmt"
	"math/rand"
	"time"
)

// ========== 错误定义 ==========

// 哨兵错误（Sentinel Error）
var ErrTaskFailed = errors.New("task execution failed")

// TaskError 自定义错误类型（等价 Python 的 class TaskError(Exception)）
type TaskError struct {
	TaskName string
	Message  string
}

// Error 实现 error 接口
func (e *TaskError) Error() string {
	return fmt.Sprintf("TaskError{%s}: %s", e.TaskName, e.Message)
}

// ========== Logger 接口 ==========

type Logger interface {
	Log(level, msg string)
}

type ConsoleLogger struct{}

func (l ConsoleLogger) Log(level, msg string) {
	fmt.Printf("[%s][%s] %s\n", time.Now().Format("15:04:05"), level, msg)
}

// ========== Runnable 接口 ==========

// Runnable 可运行接口（替代 Python 的 Protocol）
type Runnable interface {
	Run() (*Result, error)
	Name() string
}

// ========== Task 实现 Runnable ==========

type Task struct {
	name        string
	duration    float64
	successRate float64
}

func NewTask(name string, duration, successRate float64) *Task {
	return &Task{name: name, duration: duration, successRate: successRate}
}

func (t *Task) Name() string { return t.name }

// Run 返回 (*Result, error)——Go 错误处理核心模式
func (t *Task) Run() (*Result, error) {
	fmt.Printf("[INFO] 开始: %s (预计 %.1fs)\n", t.name, t.duration)
	time.Sleep(time.Duration(t.duration * float64(time.Second)))
	success := rand.Float64() < t.successRate
	if !success {
		// 包装错误，保留错误链（等价 Python 的 raise ... from ...）
		return nil, fmt.Errorf("task %q: %w", t.name, ErrTaskFailed)
	}
	return &Result{TaskName: t.name, Success: true,
		Duration: t.duration, Timestamp: time.Now()}, nil
}

// ========== DelayedTask（另一种 Runnable 实现） ==========

// DelayedTask 嵌入 Task（组合替代继承）
type DelayedTask struct {
	Task  // 匿名嵌入
	Delay float64
}

func NewDelayedTask(name string, duration, successRate, delay float64) *DelayedTask {
	return &DelayedTask{
		Task:  Task{name: name, duration: duration, successRate: successRate},
		Delay: delay,
	}
}

// Run 重写 Task.Run
func (dt *DelayedTask) Run() (*Result, error) {
	fmt.Printf("[INFO] 等待 %.1fs 后启动: %s\n", dt.Delay, dt.name)
	time.Sleep(time.Duration(dt.Delay * float64(time.Second)))
	return dt.Task.Run() // 调用嵌入的 Task.Run
}

// ========== Result ==========

type Result struct {
	TaskName  string
	Success   bool
	Duration  float64
	Timestamp time.Time
}

func (r Result) String() string {
	status := "❌"
	if r.Success {
		status = "✅"
	}
	return fmt.Sprintf("[%s] %s %s (%.1fs)",
		r.Timestamp.Format("15:04:05"), status, r.TaskName, r.Duration)
}

// ========== Scheduler ==========

type Scheduler struct {
	tasks   []Runnable // 接口 slice，可容纳任何 Runnable 实现
	results []*Result
	logger  Logger // 依赖注入
}

func NewScheduler(logger Logger) *Scheduler {
	return &Scheduler{
		tasks:   make([]Runnable, 0),
		results: make([]*Result, 0),
		logger:  logger,
	}
}

func (s *Scheduler) AddTask(t Runnable) {
	s.tasks = append(s.tasks, t)
	s.logger.Log("INFO", fmt.Sprintf("添加任务: %s", t.Name()))
}

// RunAll 运行所有任务，处理错误（对比 Python 的 try/except）
func (s *Scheduler) RunAll() {
	for _, task := range s.tasks {
		result, err := task.Run()
		if err != nil {
			// errors.Is 检查错误链（等价 Python 的 except SpecificError）
			if errors.Is(err, ErrTaskFailed) {
				s.logger.Log("ERROR", fmt.Sprintf("任务失败: %v", err))
				s.results = append(s.results, &Result{
					TaskName: task.Name(), Success: false,
					Timestamp: time.Now(),
				})
				continue
			}
			// errors.As 提取自定义错误类型
			var te *TaskError
			if errors.As(err, &te) {
				s.logger.Log("ERROR", fmt.Sprintf("TaskError: %s - %s",
					te.TaskName, te.Message))
			}
			continue
		}
		s.results = append(s.results, result)
		s.logger.Log("INFO", fmt.Sprintf("任务完成: %s", result))
	}
}

func (s Scheduler) Stats() (succeeded, failed int) {
	for _, r := range s.results {
		if r.Success {
			succeeded++
		} else {
			failed++
		}
	}
	return
}

func main() {
	fmt.Println("=== 任务调度器 v3 ===")
	// 依赖注入：传入 Logger 实现
	scheduler := NewScheduler(ConsoleLogger{})
	scheduler.AddTask(NewTask("发送邮件", 2, 0.9))
	scheduler.AddTask(NewTask("生成报告", 3, 0.5)) // 低成功率
	scheduler.AddTask(NewDelayedTask("数据备份", 2, 0.95, 1.0))
	scheduler.AddTask(NewTask("日志清理", 1, 1.0))

	scheduler.RunAll()
	succ, fail := scheduler.Stats()
	fmt.Printf("\n=== 完成: %d 成功, %d 失败 ===\n", succ, fail)
}
