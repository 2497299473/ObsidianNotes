// v5-测试与部署版 - 入口
package main

import (
	"context"
	"fmt"
	"math/rand"
	"time"

	"github.com/yourname/scheduler/v5/scheduler"
)

// RealTask 实现 scheduler.Runnable 接口
type RealTask struct {
	name        string
	duration    float64
	successRate float64
}

func NewTask(name string, duration, successRate float64) *RealTask {
	return &RealTask{name: name, duration: duration, successRate: successRate}
}

func (t *RealTask) Name() string { return t.name }

func (t *RealTask) Run(ctx context.Context) (*scheduler.Result, error) {
	select {
	case <-time.After(time.Duration(t.duration * float64(time.Second))):
		success := rand.Float64() < t.successRate
		if !success {
			return nil, fmt.Errorf("task %q: %w", t.name, scheduler.ErrTaskFailed)
		}
		return &scheduler.Result{TaskName: t.name, Success: true,
			Duration: t.duration, Timestamp: time.Now()}, nil
	case <-ctx.Done():
		return nil, ctx.Err()
	}
}

func main() {
	fmt.Println("=== 任务调度器 v5（测试与部署版）===")
	sched := scheduler.NewScheduler(3)
	sched.AddTask(NewTask("发送邮件", 2, 0.9))
	sched.AddTask(NewTask("生成报告", 3, 0.8))
	sched.AddTask(NewTask("数据备份", 5, 0.95))
	sched.AddTask(NewTask("日志清理", 1, 1.0))

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	start := time.Now()
	results := sched.RunAll(ctx)
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
}
