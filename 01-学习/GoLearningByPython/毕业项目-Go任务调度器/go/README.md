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
