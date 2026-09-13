---
title: 代码项目 CLAUDE.md 模板
date: 2026-06-16
tags:
  - Claude
  - CLAUDE.md
  - 模板
  - 工程
aliases:
  - 项目级 CLAUDE.md
  - 代码项目记忆文件
status: ✅ 已完成
related:
  - "[[CLAUDE.md]]"
  - "[[Claude的使用/02｜过目不忘：Claude Code 记忆系统与 CLAUDE.md]]"
lark_doc_url: https://my.feishu.cn/docx/VaX8dVCWdoUCzaxsbXscuIUinDc
---

> [!tip] 使用说明
> 这是一份**面向代码项目的 CLAUDE.md 模板**。复制到你的代码项目根目录，按实际情况修改 `[...]` 占位内容。提取自 vault 级 CLAUDE.md 的通用部分已自动内嵌。

---

# [项目名称]

## 关于我

- 刘方轩，4 年 Python 后端开发经验
- 技术栈：Python / FastAPI → React / TypeScript → PostgreSQL / Redis → TensorFlow / 机器学习
- 沟通：正文中文，代码注释用英文，解释简洁直接

## 技术栈

- 后端：[Python 3.12 / FastAPI / ...]
- 前端：[React 18 / TypeScript / ...]
- 数据库：[PostgreSQL / Redis / ...]
- 部署：[Docker / ...]

## 目录结构

```
[src/]
[├── routes/        # 路由定义]
[├── services/      # 业务逻辑]
[├── repositories/  # 数据访问]
[├── models/        # 数据模型]
[└── utils/         # 工具函数]
```

## 编码规范

### 语言

- 正文用中文，代码注释用英文
- 解释简洁直接，不铺陈

### 代码风格

- [Python: 遵循 PEP 8，使用 ruff 格式化]
- [TypeScript: 使用 interface 定义对象，type 用于联合类型]
- [禁用 any，使用 unknown + 类型守卫]
- [函数参数 > 3 个时使用对象参数]
- 代码块必须标注语言（```python、```typescript 等）

### 错误处理

- [业务错误使用自定义 Error 类，避免裸字符串]
- [controller 层不要 try-catch，由全局错误中间件统一处理]

## 常用命令

```bash
# 开发
[pnpm dev]          # 启动开发服务器
[pnpm build]        # 构建

# 测试
[pnpm test]         # 运行全部测试
[pnpm test:watch]   # 监听模式

# 数据库
[prisma migrate dev # 运行迁移]
[prisma studio      # 数据库管理界面]
```

## 架构决策

- [为什么选 FastAPI 而非 Django：异步性能、类型安全、生态轻量]
- [为什么用 Prisma 而非原生 SQL：类型安全迁移、团队上手成本低]
- [如有详细设计文档，在此引用：见 docs/architecture.md]

## Explore Subagent 配置

- 使用 Explore subagent 时:
  - 涉及调用链分析、架构梳理、跨模块搜索 → 使用 "very thorough"
  - 涉及查找特定文件或类 → 使用 "quick"
  - 其他情况 → 使用 "medium"

## 不要做的事

- 不要在代码中硬编码密钥或敏感信息
- 不要跳过类型检查（no `# type: ignore` without comment）
- 不要引入未经团队评审的第三方依赖
- 不要在正文里重复描述 Claude 已经知道的知识（如"写高质量代码""遵循最佳实践"）

## 详细参考

- API 规范：[见 docs/api-spec.md]
- 数据库设计：[见 docs/database.md]
- 部署流程：[见 docs/deployment.md]
