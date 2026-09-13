// v4-并发与Channel版
// 引入 goroutine 并发执行任务，用 channel 收集结果
// 用 context 实现超时控制，用信号量限流
package main

import (
	"context"
	"errors"
	"fmt"
	"math/rand"
	"sync"
	"time"
)

var ErrTaskFailed = errors.New("task execution failed")

type Logger interface {
	Log(level, msg string)
}

type ConsoleLogger struct{}

func (l ConsoleLogger) Log(level, msg string) {
	fmt.Printf("[%s][%s] %s\n", time.Now().Format("15:04:05"), level, msg)
}

// Runnable 接口（增加 context 参数支持超时）
type Runnable interface {
	Run(ctx context.Context) (*Result, error)
	Name() string
}

type Task struct {
	name        string
	duration    float64
	successRate float64
}

func NewTask(name string, duration, successRate float64) *Task {
	return &Task{name: name, duration: duration, successRate: successRate}
}

func (t *Task) Name() string { return t.name }

// Run 接受 context，支持超时控制
func (t *Task) Run(ctx context.Context) (*Result, error) {
	// select 实现超时检测（等价 Python 的 asyncio.wait_for）
	select {
	case <-time.After(time.Duration(t.duration * float64(time.Second))):
		success := rand.Float64() < t.successRate
		if !success {
			return nil, fmt.Errorf("task %q: %w", t.name, ErrTaskFailed)
		}
		return &Result{TaskName: t.name, Success: true,
			Duration: t.duration, Timestamp: time.Now()}, nil
	case <-ctx.Done():
		// context 被取消（超时或手动取消）
		return nil, fmt.Errorf("task %q cancelled: %w", t.name, ctx.Err())
	}
}

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

// Scheduler 并发版
type Scheduler struct {
	tasks      []Runnable
	logger     Logger
	maxWorkers int // 最大并发数
}

func NewScheduler(logger Logger, maxWorkers int) *Scheduler {
	if maxWorkers <= 0 {
		maxWorkers = 3
	}
	return &Scheduler{
		tasks: make([]Runnable, 0), logger: logger, maxWorkers: maxWorkers,
	}
}

func (s *Scheduler) AddTask(t Runnable) {
	s.tasks = append(s.tasks, t)
	s.logger.Log("INFO", fmt.Sprintf("添加任务: %s", t.Name()))
}

// RunAll 并发运行所有任务
// 对比 Python: asyncio.gather(*[task.run() for task in tasks])
func (s *Scheduler) RunAll(timeout time.Duration) []*Result {
	// 超时 context（等价 asyncio.wait_for(..., timeout)）
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel() // 必须释放资源

	// 信号量：控制并发数（等价 asyncio.Semaphore）
	sem := make(chan struct{}, s.maxWorkers)
	// 结果 channel（缓冲大小 = 任务数，避免阻塞）
	results := make(chan *Result, len(s.tasks))
	var wg sync.WaitGroup

	for _, task := range s.tasks {
		wg.Add(1)
		go func(t Runnable) {
			defer wg.Done()
			// 获取信号量（限流）
			sem <- struct{}{}
			defer func() { <-sem }() // defer 释放

			result, err := t.Run(ctx)
			if err != nil {
				if errors.Is(err, ErrTaskFailed) {
					s.logger.Log("ERROR", fmt.Sprintf("失败: %v", err))
					results <- &Result{TaskName: t.Name(),
						Success: false, Timestamp: time.Now()}
					return
				}
				s.logger.Log("ERROR", fmt.Sprintf("错误: %v", err))
				results <- &Result{TaskName: t.Name(),
					Success: false, Timestamp: time.Now()}
				return
			}
			s.logger.Log("INFO", fmt.Sprintf("完成: %s", result))
			results <- result
		}(task)
	}

	// 等待所有 goroutine 完成，然后关闭 channel
	go func() {
		wg.Wait()
		close(results) // 关闭 channel 通知 range 结束
	}()

	// 收集结果
	var allResults []*Result
	for r := range results {
		allResults = append(allResults, r)
	}
	return allResults
}

func main() {
	fmt.Println("=== 任务调度器 v4（并发版）===")
	scheduler := NewScheduler(ConsoleLogger{}, 3)
	scheduler.AddTask(NewTask("发送邮件", 2, 0.9))
	scheduler.AddTask(NewTask("生成报告", 3, 0.8))
	scheduler.AddTask(NewTask("数据备份", 5, 0.95))
	scheduler.AddTask(NewTask("日志清理", 1, 1.0))

	start := time.Now()
	// 10 秒超时
	results := scheduler.RunAll(10 * time.Second)
	elapsed := time.Since(start)

	succ, fail := 0, 0
	for _, r := range results {
		if r.Success {
			succ++
		} else {
			fail++
		}
		fmt.Println(r)
	}
	fmt.Printf("\n=== 完成: %d 成功, %d 失败 (耗时 %.1fs) ===\n",
		succ, fail, elapsed.Seconds())
	fmt.Println("对比 v3 顺序执行：v4 总耗时 ≈ 最长任务耗时")
}
