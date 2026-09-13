---
lark_doc_url: https://my.feishu.cn/docx/CG72dNDOKoofMRx2NjPcnkD0nrc
---
# 贡献指南

感谢你为 Go + Docker 综合项目贡献代码！

## 开发环境搭建

```bash
# 克隆仓库
git clone https://github.com/summer/go-todo-api.git
cd go-todo-api

# 安装依赖
go mod download

# 运行测试
go test ./...

# 代码检查
go vet ./...
golangci-lint run
```

## 本地开发（使用 Docker Compose）

```bash
# 开发环境（热更新）
docker compose -f compose.yaml -f compose.dev.yaml up -d --build

# 含监控的完整环境
docker compose -f compose.yaml -f compose.monitoring.yaml up -d --build

# 开发 + 监控
docker compose -f compose.yaml -f compose.dev.yaml -f compose.monitoring.yaml up -d --build
```

## Commit 规范

本项目遵循 Conventional Commits 规范：

| 前缀 | 用途 |
|------|------|
| `feat:` | 新功能 |
| `fix:` | Bug 修复 |
| `docs:` | 文档变更 |
| `refactor:` | 重构 |
| `test:` | 测试 |
| `chore:` | 构建/工具变更 |
| `security:` | 安全修复 |

## 分支策略

- `main` — 稳定分支，发布 tag
- `feat/xxx` — 功能分支
- `fix/xxx` — 修复分支

## 提交 PR 前检查

- [ ] 代码通过 `go vet` 和 `golangci-lint`
- [ ] 已添加/更新测试
- [ ] 本地 Docker Compose 测试通过
- [ ] commit message 遵循 Conventional Commits

## 代码风格

- 遵循 Go 官方代码规范（`gofmt`）
- 函数名使用驼峰命名
- 错误处理：不忽略 error，不用 panic 处理业务逻辑
- 路由组合：使用 Gin Group 组织 API 路由
