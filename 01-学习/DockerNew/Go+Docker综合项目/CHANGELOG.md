---
lark_doc_url: https://my.feishu.cn/docx/UazIdAeJSo0rnxxPUxgcyCkinyb
---
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Prometheus 指标暴露（`/metrics` 端点）：HTTP 请求速率、延迟分布、数据库操作计数
- Nginx 反向代理：限流、安全头、Gzip 压缩、健康检查、端口收敛
- Grafana 仪表盘：5 个面板（请求速率、P95 延迟、数据库操作、总量统计）
- GitHub Actions CI/CD：lint → test → build → push → Trivy 安全扫描
- 自动 Release 工作流（tag 触发，含 Changelog 自动生成）
- 多架构镜像构建（linux/amd64 + linux/arm64）
- GitHub 社区文件（Issue/PR 模板、LICENSE、CONTRIBUTING）
- `.env.example` 环境变量模板
- `.dockerignore` 排除监控配置和 CI 文件

### Changed
- `main.go` 添加 Prometheus 指标中间件和 `/metrics` 端点
- Todo 模型添加 `CreatedAt` / `UpdatedAt` 时间戳字段
- Dockerfile 构建步骤优化（go.mod 先行 COPY，缓存友好）

## [1.0.0] - 2026-07-28

### Added
- 初始版本：Go TODO REST API
- Gin + GORM + PostgreSQL + Redis 技术栈
- 多阶段 Dockerfile（生产镜像 ~15MB）
- Docker Compose 编排（3 服务 + 健康检查 + Volume）
- 多环境配置（dev/prod 覆盖）
- 安全加固（非 root 用户 + 资源限制 + 只读文件系统）
- 优雅关闭（SIGTERM 处理）
- 跨路径学习文档（Go 学习路径 + DockerNew 学习路径知识点映射）
