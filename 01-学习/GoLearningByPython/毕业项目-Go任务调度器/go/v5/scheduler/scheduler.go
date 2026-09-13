// v5-测试与部署版 - scheduler 包
// 从 v4 提取核心代码，添加 sync.Mutex 保护并发访问
package scheduler

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"time"
)

var ErrTaskFailed = errors.New("task execution failed")

type Runnable interface {
	Run(ctx context.Context) (*Result, error)
	Name() string
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

type Scheduler struct {
	tasks      []Runnable
	maxWorkers int
	mu         sync.Mutex // 保护 tasks 的并发访问
}

func NewScheduler(maxWorkers int) *Scheduler {
	if maxWorkers <= 0 {
		maxWorkers = 3
	}
	return &Scheduler{tasks: make([]Runnable, 0), maxWorkers: maxWorkers}
}

func (s *Scheduler) AddTask(t Runnable) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.tasks = append(s.tasks, t)
}

func (s *Scheduler) TaskCount() int {
	s.mu.Lock()
	defer s.mu.Unlock()
	return len(s.tasks)
}

// RunAll 并发执行所有任务
func (s *Scheduler) RunAll(ctx context.Context) []*Result {
	s.mu.Lock()
	tasks := make([]Runnable, len(s.tasks))
	copy(tasks, s.tasks)
	s.mu.Unlock()

	sem := make(chan struct{}, s.maxWorkers)
	results := make(chan *Result, len(tasks))
	var wg sync.WaitGroup

	for _, task := range tasks {
		wg.Add(1)
		go func(t Runnable) {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()
			result, err := t.Run(ctx)
			if err != nil {
				results <- &Result{TaskName: t.Name(),
					Success: false, Timestamp: time.Now()}
				return
			}
			results <- result
		}(task)
	}

	go func() { wg.Wait(); close(results) }()

	var allResults []*Result
	for r := range results {
		allResults = append(allResults, r)
	}
	return allResults
}
