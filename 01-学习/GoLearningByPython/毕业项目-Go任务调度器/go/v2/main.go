// v2-struct与方法版
// 把 v1 的独立函数重构为 struct + method
// 引入 NewXxx() 构造函数和 String() Stringer 接口
package main

import (
	"fmt"
	"math/rand"
	"time"
)

// Task 任务结构体（等价 Python 的 @dataclass）
type Task struct {
	Name        string
	Duration    float64
	SuccessRate float64
}

// NewTask 构造函数约定（Go 无 __init__）
func NewTask(name string, duration, successRate float64) *Task {
	return &Task{Name: name, Duration: duration, SuccessRate: successRate}
}

// Run 运行任务（指针接收者，可能修改状态）
func (t *Task) Run() *Result {
	fmt.Printf("[%s] 开始: %s (预计 %.1fs)\n",
		time.Now().Format("15:04:05"), t.Name, t.Duration)
	time.Sleep(time.Duration(t.Duration * float64(time.Second)))
	return &Result{
		TaskName:  t.Name,
		Success:   rand.Float64() < t.SuccessRate,
		Duration:  t.Duration,
		Timestamp: time.Now(),
	}
}

// String 实现 fmt.Stringer 接口（等价 Python 的 __str__）
func (t Task) String() string {
	return fmt.Sprintf("Task{%s, %.1fs, %.0f%%}",
		t.Name, t.Duration, t.SuccessRate*100)
}

// Result 任务结果
type Result struct {
	TaskName  string
	Success   bool
	Duration  float64
	Timestamp time.Time
}

// String 实现 Stringer
func (r Result) String() string {
	status := "❌"
	if r.Success {
		status = "✅"
	}
	return fmt.Sprintf("[%s] %s %s (%.1fs)",
		r.Timestamp.Format("15:04:05"), status, r.TaskName, r.Duration)
}

// Scheduler 任务调度器（等价 Python 的 class Scheduler）
type Scheduler struct {
	tasks   []*Task
	results []*Result
}

// NewScheduler 构造函数
func NewScheduler() *Scheduler {
	return &Scheduler{
		tasks:   make([]*Task, 0),
		results: make([]*Result, 0),
	}
}

// AddTask 添加任务（指针接收者，修改 tasks）
func (s *Scheduler) AddTask(t *Task) {
	s.tasks = append(s.tasks, t)
}

// RunAll 运行所有任务
func (s *Scheduler) RunAll() {
	for _, task := range s.tasks {
		result := task.Run()
		s.results = append(s.results, result)
	}
}

// Stats 返回统计数据（值接收者，只读）
// 命名返回值，裸 return
func (s Scheduler) Stats() (succeeded, total int) {
	total = len(s.results)
	for _, r := range s.results {
		if r.Success {
			succeeded++
		}
	}
	return
}

func main() {
	fmt.Println("=== 任务调度器 v2 ===")
	scheduler := NewScheduler()
	scheduler.AddTask(NewTask("发送邮件", 2, 0.9))
	scheduler.AddTask(NewTask("生成报告", 3, 0.8))
	scheduler.AddTask(NewTask("数据备份", 5, 0.95))
	scheduler.AddTask(NewTask("日志清理", 1, 1.0))

	scheduler.RunAll()

	for _, r := range scheduler.results {
		fmt.Println(r) // 自动调用 Result.String()
	}
	succ, total := scheduler.Stats()
	fmt.Printf("\n=== 完成: %d/%d 成功 ===\n", succ, total)
}
