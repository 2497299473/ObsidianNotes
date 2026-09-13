// v5-测试与部署版 - 测试文件
package scheduler

import (
	"context"
	"fmt"
	"sync/atomic"
	"testing"
	"time"
)

// MockTask 测试用的 mock 任务（实现 Runnable 接口）
type MockTask struct {
	name     string
	success  bool
	duration time.Duration
	runCount int32 // 原子计数器，记录被调用次数
}

func (m *MockTask) Name() string { return m.name }

func (m *MockTask) Run(ctx context.Context) (*Result, error) {
	atomic.AddInt32(&m.runCount, 1)
	select {
	case <-time.After(m.duration):
		if !m.success {
			return nil, fmt.Errorf("task %q: %w", m.name, ErrTaskFailed)
		}
		return &Result{TaskName: m.name, Success: true,
			Duration: m.duration.Seconds(), Timestamp: time.Now()}, nil
	case <-ctx.Done():
		return nil, ctx.Err()
	}
}

func (m *MockTask) RunCount() int32 {
	return atomic.LoadInt32(&m.runCount)
}

// 表格驱动测试（Go 社区推荐模式）
func TestScheduler_RunAll(t *testing.T) {
	tests := []struct {
		name     string
		tasks    []*MockTask
		wantSucc int
		wantFail int
	}{
		{
			name: "全部成功",
			tasks: []*MockTask{
				{name: "t1", success: true, duration: 10 * time.Millisecond},
				{name: "t2", success: true, duration: 10 * time.Millisecond},
			},
			wantSucc: 2, wantFail: 0,
		},
		{
			name: "部分失败",
			tasks: []*MockTask{
				{name: "ok", success: true, duration: 10 * time.Millisecond},
				{name: "fail", success: false, duration: 10 * time.Millisecond},
			},
			wantSucc: 1, wantFail: 1,
		},
		{
			name:     "空任务列表",
			tasks:    []*MockTask{},
			wantSucc: 0, wantFail: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			sched := NewScheduler(3)
			for _, task := range tt.tasks {
				sched.AddTask(task)
			}
			ctx := context.Background()
			results := sched.RunAll(ctx)

			succ, fail := 0, 0
			for _, r := range results {
				if r.Success {
					succ++
				} else {
					fail++
				}
			}
			if succ != tt.wantSucc {
				t.Errorf("成功数 = %d, want %d", succ, tt.wantSucc)
			}
			if fail != tt.wantFail {
				t.Errorf("失败数 = %d, want %d", fail, tt.wantFail)
			}
		})
	}
}

// 基准测试
func BenchmarkScheduler_RunAll(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sched := NewScheduler(3)
		for j := 0; j < 10; j++ {
			sched.AddTask(&MockTask{
				name:     fmt.Sprintf("task-%d", j),
				success:  true,
				duration: 1 * time.Millisecond,
			})
		}
		sched.RunAll(context.Background())
	}
}

// 竞争检测：go test -race
func TestScheduler_Concurrent(t *testing.T) {
	sched := NewScheduler(3)
	// 并发添加任务，检测数据竞争
	for i := 0; i < 100; i++ {
		go func(n int) {
			sched.AddTask(&MockTask{
				name:     fmt.Sprintf("task-%d", n),
				success:  true,
				duration: 1 * time.Millisecond,
			})
		}(i)
	}
	time.Sleep(100 * time.Millisecond)
	if sched.TaskCount() != 100 {
		t.Errorf("TaskCount = %d, want 100", sched.TaskCount())
	}
}
