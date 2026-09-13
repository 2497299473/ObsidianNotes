// v1-基础语法迁移版
// 从 Python 任务调度器迁移为 Go，只改语法不改架构
// 对比 Python: python v1_original.py
package main

import (
	"fmt"
	"math/rand"
	"time"
)

// taskDef 任务定义（v1 用 struct 替代 Python 的 tuple）
type taskDef struct {
	name        string
	duration    float64
	successRate float64
}

// Result 结果用 struct 替代 Python 的 dict
type Result struct {
	Name      string
	Success   bool
	Duration  float64
	Timestamp string
}

// runTask 运行单个任务（等价 Python 的 def run_task）
func runTask(name string, duration, successRate float64) Result {
	fmt.Printf("[%s] 开始: %s (预计 %.1fs)\n",
		time.Now().Format("15:04:05"), name, duration)
	time.Sleep(time.Duration(duration * float64(time.Second)))
	return Result{
		Name:      name,
		Success:   rand.Float64() < successRate,
		Duration:  duration,
		Timestamp: time.Now().Format("15:04:05"),
	}
}

func main() {
	fmt.Println("=== 任务调度器 v1 ===")
	tasks := []taskDef{
		{"发送邮件", 2, 0.9},
		{"生成报告", 3, 0.8},
		{"数据备份", 5, 0.95},
		{"日志清理", 1, 1.0},
	}

	var results []Result
	// range 返回 (索引, 值)，用 _ 忽略索引
	for _, t := range tasks {
		result := runTask(t.name, t.duration, t.successRate)
		status := "❌"
		if result.Success {
			status = "✅"
		}
		fmt.Printf("[%s] %s %s (%.1fs)\n",
			result.Timestamp, status, result.Name, result.Duration)
		results = append(results, result)
	}

	succeeded := 0
	for _, r := range results {
		if r.Success {
			succeeded++
		}
	}
	fmt.Printf("\n=== 完成: %d/%d 成功 ===\n", succeeded, len(results))
}
