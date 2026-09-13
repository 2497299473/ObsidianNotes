---
lark_doc_url: https://my.feishu.cn/docx/CVv4dqNdzoIkEGxVswQcVcPbnfc
lark_doc_token: CVv4dqNdzoIkEGxVswQcVcPbnfc
---
# My-First-Obsidian Vault

> [!tip] 使用指南
> 这份文件是给 Claude 读的「入职手册」。想知道具体怎么用、在哪些场景下能发挥作用，见 [[03-AI工具/Claude的使用/CLAUDE.md 使用指南：八个场景与实操]]。

## 关于我

- 刘方轩，4 年 Python 后端开发经验
- 技术栈：Python / FastAPI → React / TypeScript → PostgreSQL / Redis → TensorFlow / 机器学习
- 正在系统学习算法 + 准备面试

## Vault 组织规则

按 **DAT 体系**（Directory > Atlas > Tags）组织，根目录只有六大类：

```
🏠 知识库总览.md   ← Atlas 总入口
📁 01-学习/        ← 结构化学习路径（从零学 + 迁移式学）
📁 02-笔记与项目/  ← 技术知识沉淀 + 项目笔记
📁 03-AI工具/      ← Claude / Prompt / AI 方法论
📁 04-GitHub研究/  ← GitHub 开源项目研究
📁 05-个人/        ← 面试 / 兴趣 / 配置 / 模板
```

各领域的总览索引：
```
01-学习/
  算法/              → [[01-学习/算法/基础概念/算法总览索引]]
  Docker/            → [[01-学习/Docker/Docker 总览索引]]
  React学习路径/     → [[01-学习/React学习路径/00-React学习路径总索引]]
  TypeScript学习路径/ → [[01-学习/TypeScript学习路径/00-TypeScript学习路径总索引]]
  Redis学习路径/     → [[01-学习/Redis学习路径/00-Redis学习路径总索引]]
  机器学习/          → 课程笔记 + 实践
  深度学习/          → CNN + RNN 实战
  迁移式学习/         → Go(Python→), TS(Python→), K8S(Docker→), PG(MySQL→), TF(Sklearn→)
02-笔记与项目/
  Redis/             → [[02-笔记与项目/Redis/Redis 总览索引]]
  全栈项目/          → [[02-笔记与项目/全栈项目/全栈项目总览索引]]
03-AI工具/
  Claude的使用/      → 极客时间课程阅读笔记 + 个人实践
  AI协作方法论/      → 学习路径构建方法论
  Prompt模板/        → [[03-AI工具/Prompt模板/Prompt 模板集 MOC]]
```

**创建新笔记**：只需 5 选 1（学习 / 笔记项目 / AI工具 / GitHub / 个人），丢进对应目录，不需要纠结具体子目录。

## 写作规范

### 语言

- 正文用**中文**
- 代码注释用英文
- 解释简洁直接，不铺陈

### Frontmatter

**学习笔记/技术文章**（算法、Redis、TF 等）：
```yaml
---
title: "标题"
created: YYYY-MM-DD
tags:
  - tag1
  - tag2
description: "一句话描述"
---
```

**个人笔记/杂项**（面试、Claude 使用、配置等）：
```yaml
---
title: 标题
date: "YYYY-MM-DD"
tags:
  - tag1
aliases:
  - 别名1
status: 📝 待处理
---
```

模板文件在：[[05-个人/Templates/通用模板]]

### 链接

- 内部链接用标准 Wiki-link：`[[note-name]]` 或 `[[folder/note-name]]`
- 不要用 Markdown 链接指向 vault 内部文件

### 图表

- 流程图 / 学习路线用 **Mermaid**
- 架构图优先 Mermaid，截图/手绘图放根目录 media 文件夹（目前没有统一 media 文件夹，如有图片直接用相对路径嵌入）

### 代码块

- 必须标注语言：```` ```python ````、```` ```typescript ````、```` ```bash ```` 等
- 不要用无标注的代码块

## 常用操作

- 创建笔记：根据内容确定文件夹 → 用对应 frontmatter 模板 → 写正文
- 更新索引：如果在一个领域新增了笔记，同步更新该领域的 `*总览索引.md` 中的表格
- 跨领域引用：用 Wiki-link 直接链接，不要复制内容

## 不要做的事

- 不要在 vault 根目录随意创建散落的 .md 文件（除非是临时草稿）
- 不要修改 `.obsidian/` 下的配置，除非明确要求
- 不要在正文里重复描述 Claude 已经知道的知识（如"写高质量代码""遵循最佳实践"）

## Explore Subagent 配置

- 使用 Explore subagent 时:
  - 涉及调用链分析、架构梳理、跨模块搜索 → 使用 "very thorough"
  - 涉及查找特定文件或类 → 使用 "quick"
  - 其他情况 → 使用 "medium"
