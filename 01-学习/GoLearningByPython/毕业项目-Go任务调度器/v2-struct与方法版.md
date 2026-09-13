---
title: v2-struct与方法版
created: 2026-07-24
stage: 4
order: 2
difficulty: ⭐⭐⭐
estimated_hours: 3
tags:
  - Go
  - 毕业项目
  - struct
  - OOP
description: 毕业项目 v2：将 v1 的独立函数重构为 struct + method，引入构造函数和 Stringer 接口，用 Go 的 OOP 模式重写任务调度器。
lark_doc_url: https://my.feishu.cn/docx/YRu2d83knowOZSxLbQ4cKykenud
---

## 项目目标

**在 v1 基础上重构**：把独立函数替换为 method，引入 `NewXxx()` 构造函数和 `String()` Stringer 接口。

---

## 改造要点

| v1（函数式） | v2（OOP 风格） | 改造维度 |
|-------------|---------------|---------|
| 独立函数 `runTask()` | `Task.Run()` 方法 | OOP 封装 |
| 无构造函数 | `NewTask()` / `NewScheduler()` | Go 约定 |
| `fmt.Printf` 手动拼接 | `String()` 实现 Stringer | 接口实现 |
| `[]taskDef` 裸 slice | `Scheduler` struct 持有 tasks | 数据封装 |

---

## Go 代码 v2

```go
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
        fmt.Println(r)  // 自动调用 Result.String()
    }
    succ, total := scheduler.Stats()
    fmt.Printf("\n=== 完成: %d/%d 成功 ===\n", succ, total)
}
```

---

## Python vs Go 对照

| Python（v1 风格） | Go（v2 OOP 风格） |
|------------------|-------------------|
| `class Task:` + `@dataclass` | `type Task struct {}` |
| `def __init__(self, ...)` | `func NewTask(...) *Task` |
| `def __str__(self):` | `func (t Task) String() string` |
| `class Scheduler:` | `type Scheduler struct {}` |
| `self.tasks = []` | `s.tasks = make([]*Task, 0)` |
| `self.tasks.append(task)` | `s.tasks = append(s.tasks, task)` |

---

## 🎯 本版本自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能用 struct 替代 dict，写 `NewXxx()` 构造函数 |
| 🟡 进阶 | 能区分值接收者和指针接收者，实现 `String()` 方法 |
| 🔴 挑战 | 能对比 Python class 和 Go struct+method 的设计哲学 |

---

## 相关笔记

- ⬅️ 前置：[[v1-基础语法迁移版]]
- ⬅️ 前置：[[03-函数与结构体-从Python class到Go]]
- ➡️ 下一步：[[v3-接口与错误处理版]]

---

*最后更新：2026-07-24*