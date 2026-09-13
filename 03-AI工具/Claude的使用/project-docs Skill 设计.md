---
title: /project-docs Skill 设计：RAG 驱动的项目知识库
date: 2026-06-21
tags:
  - claude-code
  - skills
  - sub-agents
  - rag
  - project-docs
aliases:
  - project-docs 设计文档
  - RAG 项目知识库
status: 📝 待处理
lark_doc_url: https://my.feishu.cn/docx/RtrWde0Mvoxx8rxRrtFcRzG3nxb
---

> [!ABSTRACT] 快速概览
> 通过任务型 Skill + Sub-Agent 并行分析项目，生成 5 份结构化文档作为 RAG 向量库的知识来源，让 Agent 检索项目信息时无需全量加载上下文。

---

## 💭 想法 & 灵感

核心痛点：Agent 进入一个新项目时，缺少结构化的项目认知。每次都要花大量 token 重新探索目录结构、理解架构、找到关键文件。

解决思路：
- **Skill 做入口**：`/project-docs` 一键触发生成
- **Sub-Agent 做提取**：5 个代理并行分析，`context: fork` 隔离上下文
- **文档做知识源**：输出到 `docs/rag/`，作为 RAG 向量库的原材料
- **通用可复用**：用户级 Skill（`~/.claude/skills/`），跨项目可用

来自 [[Claude的使用/10｜令行禁止：任务型 Skills （斜杠命令 Command）实战]] 的核心启发：
> 任务型 Skill = `disable-model-invocation: true`，命令的价值在于积累——团队标准化流程就是把最佳实践固化为任务型 Skill。

---

## 📝 正文内容

### 架构总览

```
/project-docs 触发
  │
  ├── Step 1: 项目快扫（主上下文）
  │   收集：技术栈、目录结构、包管理器、git 信息
  │
  ├── Step 2: 并行 Sub-Agent（context: fork）
  │   ├── Agent A → docs/rag/01-architecture.md
  │   ├── Agent B → docs/rag/02-workflows.md
  │   ├── Agent C → docs/rag/03-database.md
  │   ├── Agent D → docs/rag/04-external-apis.md
  │   └── Agent E → docs/rag/05-cicd.md
  │
  └── Step 3: 汇总报告
```

### 五大文档维度

| # | 文档 | 核心内容 | Agent 角色 |
|---|------|---------|-----------|
| 1 | **架构** | 技术栈、目录结构、模块边界、设计模式、数据流 | 软件架构师 |
| 2 | **工作流** | API 路由、业务流程、状态机、后台任务、错误处理 | 业务分析师 |
| 3 | **数据库** | Schema、表关系、索引、查询模式、迁移策略 | 数据库工程师 |
| 4 | **外部调用** | 第三方服务、SDK、Webhook、环境变量 | 集成工程师 |
| 5 | **CI/CD** | 构建、测试、容器化、部署、监控 | DevOps 工程师 |

### 设计原则

1. **上下文隔离**：每个 Sub-Agent 在 `context: fork` 中运行，互不污染
2. **自包含**：每份文档可独立理解，不依赖对话上下文
3. **RAG 优化**：清晰标题、简洁段落、显式的文件路径和行号引用
4. **幂等**：多次运行生成一致的、最新的文档

### 使用方式

```bash
# 全项目分析
/project-docs

# 限定子目录（适合 monorepo 或大型项目）
/project-docs src/api
/project-docs packages/backend
```

### 与 RAG 集成

生成的文档位于 `docs/rag/`，可直接作为向量数据库的文档源：

```mermaid
flowchart LR
    A[📄 docs/rag/*.md] --> B[🔢 Embedding]
    B --> C[🗄️ 向量数据库]
    C --> D[🔍 Agent 检索]
    D --> E[💬 精准回答]

    style A fill:#e1f5fe
    style C fill:#fff3e0
    style E fill:#e8f5e9
```

> [!TIP] 扩展思路
> 可以增加一个参考型 Skill——Agent 在需要项目信息时自动触发 RAG 检索，而不是每次都重新读取文档。参考型 Skill 的 `description` 让 Claude 自动判断何时需要检索。

### Skill 文件位置

- **实际 Skill**：`~/.claude/skills/project-docs/SKILL.md`（用户级，跨项目可用）
- **本文档**：`Claude的使用/project-docs Skill 设计.md`（vault 设计笔记）

---

## 🔗 关联笔记

- [[Claude的使用/10｜令行禁止：任务型 Skills （斜杠命令 Command）实战]]
- [[Claude的使用/09｜触类旁通：SKILL.md 结构与触发机制]]

---

## 📊 元数据

| 字段 | 值 |
|------|-----|
| 创建日期 | 2026-06-21 |
| 更新时间 | 2026-06-21 |
| 类型 | `笔记` |
| 状态 | 📝 待处理 |
