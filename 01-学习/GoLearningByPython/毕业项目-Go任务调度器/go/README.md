---
lark_doc_url: https://my.feishu.cn/docx/MpQvdDTxDoZSOXxec8BcecCunid
---
# 毕业项目 - Go 源码

本目录包含毕业项目 v1→v5 的完整可编译 Go 源码。

## 目录结构

```
go/
├── v1/                    # 基础语法迁移版
│   ├── go.mod
│   └── main.go
├── v2/                    # struct 与方法版
│   ├── go.mod
│   └── main.go
├── v3/                    # 接口与错误处理版
│   ├── go.mod
│   └── main.go
├── v4/                    # 并发与 Channel 版
│   ├── go.mod
│   └── main.go
├── v5/                    # 测试与部署版
│   ├── go.mod
│   ├── Makefile
│   ├── main.go
│   └── scheduler/
│       ├── scheduler.go
│       └── scheduler_test.go
└── README.md             # 本文件
```

## 运行方式

```bash
# v1-v4：进入对应目录直接运行
cd v1 && go run main.go
cd v2 && go run main.go
cd v3 && go run main.go
cd v4 && go run main.go

# v5：运行测试 + 编译
cd v5
go test -v -race -cover ./...     # 运行所有测试
go test -bench=. -benchmem        # 基准测试
make build-all                    # 交叉编译
```

## 版本递进说明

| 版本 | 改造维度 | 关键 Go 概念 | Python 对比 |
|------|---------|-------------|-------------|
| v1 | 基础语法 | 变量声明、for range、struct | Python 脚本 → Go 语法 |
| v2 | struct+method | 构造函数、Stringer 接口、指针接收者 | Python class → Go struct |
| v3 | 接口+错误 | Runnable 接口、errors.Is/As、%w 包装 | try/except → error + interface |
| v4 | 并发+channel | goroutine、channel、context、信号量 | asyncio → goroutine |
| v5 | 测试+部署 | go test、表格驱动测试、交叉编译 | pytest → go test |

---

## 🎯 源码自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 依次 `cd v1 && go run main.go` 到 `v4`，四个版本都能跑完并打印 `=== 完成: ... ===` 统计行（成功数随 `rand` 浮动属正常；v4 还应输出「总耗时 ≈ 最长任务耗时」） |
| 🟢 基础 | `cd v5 && go build .` 通过，且 `go test ./... -cover` 中 scheduler 包覆盖率 ≥ 80%（本机实测 87.2%） |
| 🟡 进阶 | 能说出 v5 为什么拆成 `main` 包 + `scheduler` 子包（跨包实现接口、可单测），并指出 `RealTask` 在 `v5/main.go` 里 |
| 🟡 进阶 | 能解释 `go test -race` 在本机为何可能失败（无 gcc → cgo 不可用，`-race requires cgo`），并给出替代验证（`-count=10` 重复跑 + 互斥量自检） |
| 🔴 挑战 | 不看代码，从零写出 `Scheduler.RunAll` 的「信号量限流 + WaitGroup + 结果 channel + close 收尾」四件套，`go vet` 零告警 |

---

*最后更新：2026-11-14*
