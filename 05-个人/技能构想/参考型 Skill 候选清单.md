---
title: 参考型 Skill 候选清单
date: 2026-06-21
tags:
  - skills
  - claude-code
  - 构想
aliases:
  - reference-skills
status: 🌱 构想中
lark_doc_url: https://my.feishu.cn/docx/Qm2qdF2nUoRajGxDpeVc9CfcnCd
lark_doc_token: Qm2qdF2nUoRajGxDpeVc9CfcnCd
---

## 候选 Skill 一览

| # | Skill 名称 | 类型 | 一句话 | Claude 自己能做吗 |
|---|-----------|------|--------|-----------------|
| 1 | 数据库死锁日志解读 | 参考型 | 教 Claude 读懂本项目的死锁日志格式和常见模式 | 半会（懂死锁，不懂你的日志格式） |
| 2 | 后端代码评审模板 | 参考型 | 项目专属的 Code Review checklist + 常见问题清单 | 会（懂代码，但需要你的标准） |
| 3 | 项目任务拆分指导 | 参考型 | 把大需求拆成可执行子任务的项目约定 | 半会（懂拆分，不懂你的项目结构） |
| 4 | 项目 CRUD 代码生成 | 参考型 | 按项目统一模式生成 CRUD 代码 | 会（但是按它自己的风格，不是你的） |
| 5 | 特定代码开发场景 | 参考型 | 某类特定场景（如消息队列消费、定时任务）的标准化写法 | 半会（懂模式，不懂你的约定） |
| 6 | 反直觉知识（骚操作） | 参考型 | 项目里的奇怪配置、奇怪方案、奇怪定义——所有反常识的东西 | 不会（这些是项目特有的"暗知识"） |

---

## 逐个展开

### 1. 数据库死锁日志解读

```yaml
name: deadlock-analysis
description: Parse and diagnose database deadlock logs for this project. Understands PostgreSQL deadlock report format, identifies the involved transactions and resources, and maps them to project code. Use when the user shares a deadlock log, asks about database lock issues, or troubleshoots concurrent transaction failures.
```

**Claude 缺什么**：
- 死锁的原理和排查方法论 → Claude 有
- 本项目日志的**具体格式**（JSON？纯文本？字段含义？）→ Claude 没有
- 日志中的表名/索引名到**项目代码的映射** → Claude 没有

**reference/ 放什么**：
- 死锁日志样例 + 字段说明
- 常见死锁模式与根因对照表（如"订单表行锁 + 库存表行锁的循环等待"）
- 历史上的死锁案例和修复方案

### 2. 后端代码评审模板

```yaml
name: code-review-backend
description: Review backend code following this project's standards and common pitfalls. Checks for proper error handling patterns, transaction boundaries, API response formats, and project-specific conventions. Use when the user asks for code review, wants to check a PR, or mentions reviewing backend changes.
```

**Claude 缺什么**：
- 通用代码质量检查 → Claude 有
- **本项目**认为"好的错误处理"长什么样 → Claude 没有
- **本项目**历史上高频出现的 bug 类型 → Claude 没有

**reference/ 放什么**：
- 项目代码规范（FastAPI 的路由写法、异常处理模式）
- 常见反模式清单（如"在循环里查数据库""事务范围过大"）
- 历史 bug 回顾（过去 6 个月最常出现的 5 类问题）

### 3. 项目任务拆分指导

```yaml
name: task-breakdown
description: Break down feature requirements into implementable subtasks following this project's conventions. Understands the project's module structure, dependency order, and typical task granularity. Use when planning a new feature, creating development tasks, or estimating work effort.
```

**Claude 缺什么**：
- 通用任务拆分方法论 → Claude 有
- **本项目**的模块划分、依赖关系 → Claude 没有
- **本项目**认为"一个合理的 task 粒度"是多大 → Claude 没有

**reference/ 放什么**：
- 项目模块结构图（哪些模块、各自职责）
- 历史 feature 的拆分案例（一个完整的 feature → 拆成了 8 个 task）
- 拆分粒度标准（一个 task 不超过 2 天、必须可独立测试）

### 4. 项目 CRUD 代码生成

```yaml
name: crud-generator
description: Generate CRUD endpoints following this project's standard patterns. Produces FastAPI routes, Pydantic schemas, SQLAlchemy queries, and service-layer logic in the project's consistent style. Use when creating new API resources, adding data models, or implementing standard CRUD operations.
```

**Claude 缺什么**：
- CRUD 代码本身 → Claude 会写
- **本项目**的目录结构、命名约定、BaseModel 继承链 → Claude 没有
- **本项目**的分页格式、错误响应格式 → Claude 没有

**reference/ 放什么**：
- 一个"标准 CRUD 模块"的完整样例（如已有的 `users` 模块）
- 项目代码生成模板（路由模板、Schema 模板、Service 模板）
- 命名约定速查（文件名、类名、函数名、URL 路径）

### 5. 特定代码开发场景

```yaml
name: dev-patterns
description: Implement common backend patterns following this project's conventions. Covers message queue consumers, scheduled tasks, file upload processing, and event-driven handlers. Use when implementing infrastructure-level features or recurring integration patterns.
```

**Claude 缺什么**：
- 消费者/定时任务/Celery 的通用写法 → Claude 会
- **本项目**用哪个消息队列、Celery 怎么配、文件存在哪 → Claude 没有
- **本项目**这些场景的统一封装（BaseConsumer、BaseTask 等抽象类）→ Claude 没有

**reference/ 放什么**：
- 每种场景的标准代码模板
- 基础设施配置说明（MQ broker 地址、存储桶名、Celery backend）
- 已有的消费者/任务代码作为参考样例

### 6. 反直觉知识（骚操作）

```yaml
name: project-dark-knowledge
description: Documents counterintuitive patterns, unusual configurations, and non-obvious design decisions in this project. Explains why things are done in unexpected ways. Use when the user encounters something that seems wrong or asks "why is this done this way" — it probably has a reason documented here.
```

**Claude 缺什么**：
- Claude 看到代码会用通用最佳实践去理解 → 但对"骚操作"会误判为错误
- 举例：`timeout=300` 看起来不合理 → 但 reference/ 里记录了"第三方 API P95 响应 280s，改了会雪崩"

**reference/ 放什么**：
- 反直觉配置清单（每一项附带历史原因）
- 奇怪的架构决策记录（"为什么订单表没有外键""为什么用了双写而不是 CDC"）
- "看起来像 bug 但不是 bug"的代码位置清单
- 每个条目格式：`现象 | 直觉反应 | 实际原因 | 改了会怎样`

**为什么这个 Skill 价值极高**：
- 这类知识在团队里通常是口口相传的——老人知道，新人踩坑才知道
- Claude 更危险：它不仅不知道，还会在"优化代码"时**主动把骚操作改掉**
- 这个 Skill 本质是**保护层**——在 Claude 冲动之前拦住它

---

## 设计原则总结

### Skills 的本质：填补 Claude 的"已知"和"会做"之间的鸿沟

```
Claude 的知识（通用）          Skill 补充（项目特定）
─────────────────────         ─────────────────────
SQL 怎么写               →    这个表叫什么名字
索引怎么建               →    这个表适不适合建索引（写入量大不大）
代码怎么 review          →    这个项目最常出哪几类 bug
任务怎么拆               →    这个项目的模块边界在哪条线
CRUD 怎么写              →    这个项目的 BaseModel 继承链和分页格式
```

**一个验证方法**：问自己"如果给一个不懂这个项目但很厉害的工程师看，他还缺什么信息？"——缺的那部分，就是 reference/ 该放的内容。

### Skills = 提示词工程 + RAG + MCP 的整合

你这个洞察很干净：

| 组件 | 在 Skill 中的对应 | 作用 |
|------|-----------------|------|
| 提示词工程 | `SKILL.md` 的 description + 工作流指令 | 触发 + 执行路径 |
| RAG（知识库） | `reference/` 目录 | 项目特定知识，按需检索 |
| MCP / Tool | `scripts/` + `allowed-tools` | 可执行脚本 + 工具权限约束 |

这解释了为什么以前用纯提示词搭建的 AI 应用可以用 Skill 重写——Skill 给了它们**目录结构**（reference/ 解决了 prompt 塞不下的知识）和**触发机制**（description 让知识在正确的时刻自动加载）。

### 重叠 Skill 的 description 设计

你举的 nanobanana 2.0 vs nanobanana pro 的例子很好。核心不是"怎么写得不一样"，而是**让 AI 能在看到用户输入的第一秒就做出正确路由**。

```yaml
# ❌ 模糊——AI 无法判断
name: nanobanana-v2
description: Generate images using nanobanana 2.0 model.

name: nanobanana-pro
description: Generate images using nanobanana pro model.

# ✅ 明确路由
name: nanobanana-v2
description: Generate simple, quick illustrations and icons. Best for flat design, minimal shading, and simple compositions. Use when the user wants fast iteration, simple icons, placeholder images, or low-complexity illustrations. Not suitable for photorealistic images or detailed scenes.

name: nanobanana-pro
description: Generate high-quality, detailed images with complex composition, realistic lighting, and fine textures. Best for product shots, photorealistic scenes, and detailed artwork. Use when the user wants production-quality images, realistic rendering, or needs complex scene composition. For simple icons or quick drafts, use nanobanana-v2 instead.
```

关键手法：
1. **description 第一句就立边界**：简单 vs 高质量
2. **用"Use when"列举用户可能说的话**：`fast iteration` vs `production-quality`
3. **互相引用**：每个 description 指向另一个的适用场景（"if X, use the other one"）
4. **否定句也有用**："Not suitable for photorealism" 帮 AI 排除选项

这本质上是**把路由逻辑写进 description 里**——让 AI 的语义匹配阶段就能做出正确判断，而不需要加载整个 Skill 之后才发现选错了。

### 重叠 Skill 的另一种解法：抽取公共依赖

除了 description 路由，还有一种更干净的架构解法——把重叠部分抽成独立 Skill：

```
❌ 重叠——容易冲突
skill-a (包含数据库规范)     skill-b (也包含数据库规范)
        ↓ 用户问数据库问题         ↓
        AI 不确定选哪个

✅ 抽取公共依赖
skill-a ──→ skill-c (数据库规范) ←── skill-b
              ↑
         description 区分清晰：
         skill-c: 数据库规范本身
         skill-a: 功能 A + 引用 skill-c
         skill-b: 功能 B + 引用 skill-c
```

**什么时候用路由（description 互斥），什么时候用抽取（公共依赖）？**

| 场景 | 用哪种 | 例子 |
|------|-------|------|
| 两个 Skill 做同一件事的不同版本 | 路由 | nanobanana v2 vs pro |
| 两个 Skill 都需要同一块知识 | 抽取 | 订单 Skill 和库存 Skill 都需要数据库规范 |
| 一个 Skill 是另一个的子集 | 合并 | 如果 B 完全是 A 的子集，合并进 A |

### Skills vs Tools：同一个发现机制，不同的作用方式

```
发现机制：description 语义匹配 → 模型决定何时加载
                    ↓
    ┌───────────────┴───────────────┐
    ↓                               ↓
  Tool                            Skill
  给能力（代码执行）               给知识（上下文注入）
  操作层                          认知层
  "能做这个操作"                  "在这个项目里怎么做"
```

Skills 本质上是对 Tool 机制的一层语义封装——把"非标准化的、项目特定的、用户自定义的"那部分，用同一套 description 发现机制暴露给模型。名称从 Tool 改为 Skill，反映的是从"原子操作"到"能力包"的粒度跃迁。

### Skills ≈ 更粗粒度的 RAG

| | 传统 RAG | Skills |
|---|---------|--------|
| 检索单元 | 文档块（chunk） | 能力包（SKILL.md + reference/） |
| 触发方式 | 向量相似度检索 | description 语义匹配 |
| 返回内容 | 相关文本片段 | 结构化工作流 + 领域知识 |
| 组织方式 | 按文档 | 按任务/领域 |

Skills 把 RAG 的检索粒度从"段落级"提升到了"能力级"——不是"给你看这段文档"，而是"现在你是这个领域的专家，按这个流程做"。
