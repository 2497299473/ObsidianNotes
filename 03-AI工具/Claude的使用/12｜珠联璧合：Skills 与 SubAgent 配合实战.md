---
title: 12｜珠联璧合：Skills 与 SubAgent 配合实战
source: https://time.geekbang.org/column/article/947718
author: 黄佳
published: 2026-03-06
created: 2026-06-21
tags:
  - clippings
  - Claude Code
  - Skills
  - SubAgent
description: Skills 与 SubAgent 两大关键技术栈的组合实战：两种组合方向（SubAgent 包含 Skill / Skill 包含 SubAgent）、三种实战模式、生产级 Skill 构建与流水线编排。
lark_doc_url: https://my.feishu.cn/docx/SEWCdv94qoMGkcx1RKdc0WBKnqf
---

> 黄佳 · Claude Code 工程化实战

释题：珠联璧合。在这一课中，我们将通过三个项目和一条进化线：从构建生产级 Skill 到确保 SubAgent 真正执行它，把 Skills 与 SubAgent 两大关键技术栈融为一体。

你好，我是黄佳。前面我们学习了 Skills 的基础结构（ 第 9 讲 ）、任务型 Skills 实战（ 第 10 讲 ）和渐进式披露架构（ 第 11 讲 ）。很明显，Skills 单独使用已经很强，但它真正的威力在于 与 SubAgent 组合 ——把一个通用的 Agent 打造成领域专家。

在这一讲中，我们先建立起全局视角，再 设计一个好的 Skill ，再来看 怎么把 Skills 和 SubAgents 配合起来用 。

那么这节课正好可以针对 Jxin 在子代理第一讲留言中提出的问题，给出答案。

![](https://static001.geekbang.org/resource/image/79/3e/79e5af0daf62e090yy11246597db9d3e.png?wh=865x316)

Skills 和 SubAgents 可以相互嵌套

## 两个组合方向：谁包含谁

之前我们讲过了，Skills 解决的是“怎么做”的问题，本质是知识注入——它让同一个 Agent 学会新的能力，就像给一个员工发了一本操作手册，员工还是那个人，只是掌握了更多方法与规范。SubAgents 解决的是“谁来做”的问题，本质是任务委托——它创建一个独立的执行者去完成某件事，就像把任务交给另一位同事，对方拥有自己的上下文、职责边界和决策空间。

![](https://static001.geekbang.org/resource/image/eb/f0/ebf3b49af650a4a8f46e62dfdb4127f0.jpg?wh=1962x1173)

当你犹豫到底是用 Skill 还是用 SubAgent 的时候，做出选择的核心判断标准在于：这件事到底需要“另一个人”来承担，还是只需要“多一本手册”来指导？

而更常见的情况是：结合起来使用。当你把两者组合时，本质上只有两个原子方向—— 是 SubAgent 内部加载 Skills，还是由主 Agent 通过 Skills 去编排和调用 SubAgents。

### 方向 A：SubAgent 包含 Skill（skills 字段）

此时，你定义子代理的角色，通过 skills 字段给它预加载领域知识。 SubAgent 是老板，Skill 是工具书。

```yaml
# .claude/agents/api-doc-generator.md
---
name: api-doc-generator
description: Generate API documentation by scanning Express route files.
tools: [Read, Grep, Glob, Write, Bash]
skills:
  - api-generating  # ← 关键：预加载 Skill 作为领域知识
---
```

```markdown
You are an API documentation specialist.

## Your Mission

Generate or update API documentation for Express.js routes.
```

这种情况下，Claude Code 的执行流程如下：

Claude 主对话: "用 api-doc-generator 为 src/ 生成 API 文档"

主对话 SubAgent (api-doc-generator)

│ │

├─ 创建子代理 + 注入 Skill ──────→ │

│ ├─ 上下文中已有：角色定义 + SKILL.md 全文

│ ├─ 按 SKILL.md 步骤执行任务

│ ├─ 使用 Skill 提供的脚本和模板

│ ├─ 生成文档

│ ←──── 返回结果摘要 ──────────── │

├─ 继续对话 （子代理结束）

SubAgent 包含 Skill 是最常见的情况，适用场景包括子代理需要特定领域的专业知识来完成任务，同一个 Skill 可以被不同角色的 SubAgent 复用，以及需要长期维护的专家型 Agent。

> [!warning] 全量加载 vs 渐进式披露
> **主 Agent** 中的 Skill 是渐进式披露——只有用户输入匹配 `description` 时才按需加载 SKILL.md。
> **SubAgent** 中通过 `skills:` 字段预加载是**全量加载**——SKILL.md 的完整内容在 SubAgent 创建时一次性注入到它的上下文中。因为 SubAgent 生命周期短、任务单一，全量加载是合理的：这个 SubAgent 就是为了执行这个特定领域任务而生的。
>
> 参考：[Preload skills into subagents](https://code.claude.com/docs/en/sub-agents#preload-skills-into-subagents)

### 方向 B：Skill 包含 SubAgent（context: fork）

在这种情况下，Skill 自带任务指令，通过 context: fork 配置自动”派遣”一个子代理去执行。 Skill 是老板，SubAgent 是执行者。

```yaml
---
name: deep-research
description: Research a topic thoroughly in the codebase
context: fork  # ← 关键：让 Skill 在独立子代理中执行
agent: Explore  # ← 子代理类型
---
```

```
Research $ARGUMENTS thoroughly:
1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

此时的 Claude Code 执行流程如下：

子代理 看不到 你之前的对话历史，因此 SKILL.md 的内容成为子代理的任务指令 。 agent 字段决定子代理类型： Explore （只读探索）、 Plan （规划）、 general-purpose （通用，省略 agent 时默认使用)

这种应用方式的适用场景是 研究型任务（深度探索代码库，不污染主对话），重型生成（批量生成文档，中间过程不需要用户看到），以及安全隔离（Skill 的操作不应影响主对话状态）。

![](https://static001.geekbang.org/resource/image/85/51/85ff4fee38a113b3f2621ff70ddb2451.jpg?wh=1947x707)

下面是两个方向的对照说明表。

![](https://static001.geekbang.org/resource/image/27/6e/27446fyy953bb69c9f9bc883ba70276e.jpg?wh=2947x1410) ![](https://static001.geekbang.org/resource/image/7d/e0/7d9623d0e6c61025ac51f63842ab7ee0.jpg?wh=1971x583)

## 构建 Skill：参照着用

下面我们要开始通过示例来讲解二者的组合使用。不过在这之前，先来构建出一个 skill，也算是之前内容的又一次复习。

这个配套项目位于： 04-Skills/projects/05-api-generator/

一个生产级的 Skill 的完整架构应该包含这些组件：

.claude/skills/api-generating/ # 标准 Skill 目录

├── SKILL.md # 入口：路由 + 核心逻辑

├── PATTERNS.md # 知识：框架识别模式

├── STANDARDS.md # 规范：文档编写标准

├── EXAMPLES.md # 示例：输入输出案例

├── templates/

│ ├── index.md # 模板：API 索引页

│ ├── endpoint.md # 模板：端点文档

│ └── openapi.yaml # 模板：OpenAPI 规范

└── scripts/

├── detect_routes.py # 脚本：路由检测

└── validate_openapi.sh # 脚本：规范验证

每个组件都有明确的职责。

![](https://static001.geekbang.org/resource/image/92/05/92cc14500687eb22fcf5f2763ec8da05.jpg?wh=2259x1125)

也许你觉得只不过设计个 API 而已，为什么需要这么多组件？

但其实一位 API 文档专家，当有人请你写文档时，你需要 识别技术栈 （Express? FastAPI? Spring?）→ PATTERNS.md、 遵循规范 （字段命名、格式要求）→ STANDARDS.md、 参考案例 （不确定时看例子）→ EXAMPLES.md、 使用模板 （保证一致性）→ templates// 批量处理 （几十个端点不可能手写）→ scripts/。

这些都是长期的经验积累之后内化的结果，而一个生产级 Skill 所希望做到的， 就是把这些专家经验形式化。

主文件 SKILL.md

主文件不是堆砌所有内容，而是路由器——根据任务需求指向正确的资源。设计要点包括：使用 Quick Reference 表格作为一目了然的资源索引，用清晰的步骤告诉 Claude 标准流程，以及按需指引（只在需要时才去读详细文档）。这些都是我们已经熟悉的内容。

```yaml
---
name: api-documenting
description: Generate API documentation from code. Use when the user wants to document APIs, create API reference, generate endpoint documentation, or needs help with OpenAPI/Swagger specs.
allowed-tools:
  - Read
  - Grep
  - Glob
  - Write
  - Bash(python:*)
  - Bash(./scripts/*:*)
---
```

~~~markdown
# API Documentation Generator

Generate comprehensive API documentation from source code.

## Quick Reference

| Task | Resource |
|------|----------|
| Identify framework | See `PATTERNS.md` |
| Documentation standards | See `STANDARDS.md` |
| Example outputs | See `EXAMPLES.md` |

## Process

### Step 1: Identify API Endpoints

Look for route definitions. For framework-specific patterns, see `PATTERNS.md`.

### Step 2: Extract Information

For each endpoint, extract:
- HTTP method (GET, POST, PUT, DELETE, etc.)
- Path/route
- Parameters (path, query, body)
- Request/response schemas
- Authentication requirements

### Step 3: Generate Documentation

Use the template in `templates/endpoint.md` for each endpoint.

### Step 4: Create Overview

Generate an index using `templates/index.md`.

## Output Formats

### Markdown (Default)

Generate markdown suitable for README or docs site.

### OpenAPI/Swagger

If requested, generate OpenAPI 3.0 spec. See `templates/openapi.yaml`.

## Automation

To auto-detect routes:
```bash
python scripts/detect_routes.py <source_directory>
```

To validate OpenAPI spec:
```bash
./scripts/validate_openapi.sh <spec_file>
```
~~~

剩下的各个文件，这里就不再赘述，大家直接去 Repo 阅读具体的文件和 Readme 说明就好啦。

可以在 Claude Code 中先对这个 Skill 的能力做一个测试。他会生成 10 个 API 端点的完整文档。

![](https://static001.geekbang.org/resource/image/17/9b/17c79a70731699cbce4d391437b0279b.png?wh=1033x519) ![](https://static001.geekbang.org/resource/image/98/77/9822fa90548360ec04378b2126673977.png?wh=1033x519)

## 三种组合模式：具体咋用

第一部分介绍的两个组合方向是基础构件。实战中，它们还衍生出三种组合模式。

## 模式一：SubAgent 预加载 Skills（方向 A 的单次应用）

一个子代理预加载一个或多个 Skill，用领域知识增强自己的能力。这是最常见的模式，也是本讲的实战重点。

```yaml
# 子代理在创建时预加载 Skill 作为领域知识
---
name: api-doc-generator
skills:
  - api-generating  # 预加载 API 文档生成知识
---
```

配置了 Skill 的子代理好比一个经过培训的专业技师。

而同一个 Agent，注入不同的 Skill，就变成不同的专家。这就是组合的力量。

![](https://static001.geekbang.org/resource/image/d2/10/d20b530b1b00e8569b53722ef4e93410.jpg?wh=2445x1492)

现在我们来完成最后一步——把 Skill 装进 SubAgent，组装一个领域专家。

下面就是从单独用 Skill 到组合使用的进化路线：参考我的代码库 06-agent-skill-combo 。

05 \-api-generator 06 \-agent-skill-combo

───────────── ─────────────────

Skill 独立运行 Skill + SubAgent 组合

SKILL.md（ 6 组件全展示） SKILL.md（精简为 3 组件）

┌─────────────────────────────┐ ┌──────────────────────────────────────┐

│ Quick Reference │ │ 工作流程 — MANDATORY │

│ Process: Step 1 \-4 │ → │ Step 1: Route Discovery（脚本） │

│ Automation: 可选 │ │ Step 2: Route Analysis（分析） │

│ 6 个引用文件 │ │ Step 3: Documentation Generation │

└─────────────────────────────┘ └──────────────────────────────────────┘

无 SubAgent 定义 新增 SubAgent 定义

┌──────────────────────────────────────┐

│ api-doc-generator.md │

│ skills: [api-generating] │

│ "You are an API doc specialist." │

└──────────────────────────────────────┘

无测试目标 新增 Express 测试路由

┌──────────────────────────────────────┐

│ users.js — 标准 CRUD（ 5 条路由） │

│ orders.js — 含链式路由（ 5 条路由） │

│ 共 10 条路由，验证覆盖率 │

└──────────────────────────────────────┘

首先做的一件事是 Skill 精简化。 你可能注意到，新的项目中的 Skill 和刚才创建的独立 Skill 有点不一样了，从原来的 6 个组件精简为 3 个组件。为什么？因为 SubAgent 已经有了角色定义，Skill 只需提供工作流程和工具：

精简原则是参考型知识（PATTERNS、STANDARDS、EXAMPLES）在主对话场景中有用，但 SubAgent 场景下通常只需要执行流程。这里你可以回顾一下 Skill 和 Sub-Agent 的职责划分—— Skill 负责 HOW，SubAgent 负责 WHO/WHAT。

> [!note] 为什么 SubAgent 的 Skill 可以精简？
> 主 Agent 的 Skill 需要渐进式披露（Quick Reference → 按需加载 PATTERNS/STANDARDS/EXAMPLES），因为主对话上下文珍贵且任务多变。而 SubAgent 的 Skill 通过 `skills:` 字段**全量注入**——所有内容一次性加载，SubAgent 的生命周期短、任务单一，不需要"按需"这套机制。所以 PATTERNS、STANDARDS 等参考型知识可以直接嵌入 SKILL.md 的流程步骤中，无需独立文件。

> [!question] 延伸问题：大规模开发规范怎么处理？
> 上面讨论的是 Skill 本身不大（比如 API 文档生成）的场景。但如果开发规范本身就很庞大——500 页编码标准、完整的安全基线、多套数据库设计规范——全量塞进 SubAgent 会 token 爆炸，但用渐进式披露又只能在主 Agent 中用，失去了 SubAgent 的隔离分工优势。这看起来是个二选一的困局，实际有三条路：

### 补：大规模规范的三种解耦模式

**核心洞察**：渐进式披露的"触发者"不一定是用户。主 Agent 中用户输入是触发器，SubAgent 场景中**任务本身可以是触发器**。把"检索规范"和"执行设计"解耦到不同 Agent 上。

#### 路径 1：主 Agent 检索 → 提取子集 → 传 SubAgent（推荐）

不让架构师 SubAgent 吞整本手册，而是**主 Agent 先用渐进式披露 Skill 提取与本次任务相关的规范子集**，再将子集作为上下文传给 SubAgent：

```
主 Agent（渐进式 Skill）                    SubAgent（轻量 Skill）
│                                           │
├─ 用户："设计用户认证模块"                   │
├─ /coding-standards 按需检索：               │
│   → auth 相关规范（5 条）                    │
│   → API 命名约定（3 条）                     │
│   → 错误处理模式（2 条）                     │
│                                           │
├──────── 传递上下文 ──────────────────────→ │
│   "本次设计需遵循：                          │  skills:
│    1. JWT token 有效期 ≤ 24h               │    - design-patterns  ← 只管"怎么画图"
│    2. API 路径: /api/v1/auth/*             │
│    3. ..."                                 │
│                                           ├─ 产出：架构图 + 接口定义
│←──────── 返回方案 ──────────────────────── │
│                                           │
├─ 审查、补充、落地                           │
```

SubAgent 拿到的不是 500 页手册，而是**和本次任务相关的 10 条规范**。检索在主 Agent、执行在 SubAgent，各司其职。

#### 路径 2：双层 Skill 架构

把规范 Skill 拆成两层——主 Agent 用完整版（渐进式披露），SubAgent 用精简版（全量加载，但只含高频规则）：

```
.claude/skills/coding-standards/
├── SKILL.md           # 完整入口（主 Agent 用）
├── PATTERNS.md        # 完整附录
├── STANDARDS.md
└── distilled/
    └── SKILL.md       # 精简版（SubAgent 用）：只含 2000 token 的高频规则 + 自检清单
```

```yaml
name: architect
skills:
  - coding-standards/distilled  # 只加载高频底线规则
```

精简版不追求完整，追求**覆盖 80% 场景的底线规则**。边缘情况设计完后再由主 Agent 用完整 Skill 审查。

#### 路径 3：SubAgent 回查主 Agent

SubAgent 不加载规范 Skill，遇到不确定的规范时向主 Agent 发起查询：

```yaml
# .claude/agents/architect.md
---
name: architect
model: sonnet
tools: [Read, Write, Bash]
# 注意：不给 skills:，不给 glob/grep 访问规范文件
---
```

```
## Critical Rule
When you need to verify a coding standard, DO NOT GUESS.
Instead, return a question in this format:

[NEED_STANDARD] <what you need to know>

The main agent will look up the standard and respond.
```

规范检索完全留在主 Agent，SubAgent 只负责设计。缺点是交互轮次增加，适合高精度要求的复杂设计任务。

#### 选型速查

| 场景 | 推荐路径 |
|------|----------|
| 规范 < 3000 token | 直接塞 SubAgent `skills:`（全量加载损失可控） |
| 规范 3000~10000 token，任务聚焦 | **路径 1**：主 Agent 检索 → 传 SubAgent |
| 规范 > 10000 token，多场景复用 | **路径 2**：双层 Skill，SubAgent 用精简版 |
| 高精度要求，接受多轮交互 | **路径 3**：SubAgent 回查主 Agent |

**路径 1 最实用**：既不牺牲 SubAgent 的隔离优势，又利用了渐进式披露的 token 效率。本质是把"检索"和"执行"解耦到两个 Agent 上——这才是 Skills 与 SubAgent 组合的真正威力。

然后是 SubAgent 的角色定义。 SubAgent 的 .md 文件只定义角色和使命，具体的工作流程由 Skill 提供。其中 `skills: [api-generating]` 这一行——就是把"操作手册"交到 SubAgent 手里的那一刻。

```yaml
# .claude/agents/api-doc-generator.md
---
name: api-doc-generator
description: Generate comprehensive API documentation by scanning Express route files.
model: sonnet
tools: [Read, Grep, Glob, Write, Bash]
skills:
  - api-generating  # ← 预加载 Skill
---
```

```markdown
You are an API documentation specialist.

## Your Mission

Generate or update API documentation for Express.js routes.

### Workflow

1. Run the route detection script as specified in the Skill
2. For each discovered route, analyze the handler code
3. Generate documentation using the Skill's template
4. Verify all routes are covered (cross-check with script output)

### Output

- Write documentation files to `docs/api/`
- Return a summary to the main conversation:
  - Number of routes documented
  - Any routes that could not be fully analyzed (with reasons)
  - Warnings (missing auth, undocumented parameters, etc.)
```

完整项目结构如下所示。

06-agent-skill-combo/

├──.claude/

│ ├── agents/

│ │ └── api-doc-generator.md # SubAgent：角色 + 使命（WHO/WHAT）

│ ├── skills/

│ │ └── api-generating/

│ │ ├── SKILL.md # Skill：工作流程 + 规则（HOW）

│ │ ├── scripts/

│ │ │ └── detect-routes.py # 路由检测脚本（处理链式路由）

│ │ └── templates/

│ │ └── api-doc.md # 文档模板

│ └── settings.local.json # 权限预配置

├── src/

│ └── routes/

│ ├── users.js # 标准 CRUD（5 条路由）

│ └── orders.js # 含链式路由（5 条路由）

├── docs/api/ # 生成的文档（输出目录）

└── README.md

下面，在 Claude Code 中，运行 SubAgent：

> 用 api-doc-generator 为 src/ 目录生成 API 文档

![](https://static001.geekbang.org/resource/image/05/12/0583fd15cc4fecfe3c262b6978a0cb12.png?wh=865x519)

检查生成的文档：

docs/api/users.md 应包含 5 个端点

docs/api/orders.md 应包含 5 个端点（含 GET /:id/status 和 PUT /:id/status ）

需要认证的端点应有 🔒 标记

观察它的工具调用记录，SubAgent 确实执行了脚本、使用了模板，并严格遵循了 SKILL.md 的工作流程。 Skill 注入真的有效。

## 模式二：Skill + context: fork（方向 B 的直接应用）

Skill 自带任务指令，Skill 包含 SubAgent，通过 context: fork 派一个子代理去执行。这种模式的适用场景是一个独立完整的任务，不需要与主对话交互，执行完把结果送回来就行。

模式二的配套项目位于： 04-Skills/projects/07-skill-fork-demo/ 。项目结构如下：

07-skill-fork-demo/

├──.claude/

│ └── skills/

│ └── code-health-check/

│ └── SKILL.md # context: fork + agent: general-purpose

├── src/

│ ├── app.js # 含硬编码密钥

│ ├── routes/

│ │ ├── products.js # 含 SQL 注入、缺少 try/catch

│ │ └── categories.js # 含未使用函数、重复逻辑

│ └── utils/

│ └── db.js # 含 eval() 使用

└── README.md

在这个项目中，你想做一次代码质量扫描，但不想让大量中间文件内容污染主对话。这正是 context: fork 的典型场景—— Skill 自己派一个子代理去做，做完把报告送回来 。

这个项目和上一个项目（模式一）的区别是模式一（Project 05）需要 SubAgent .md 定义文件 + skills: 字段。而模式二（本项目 Project 04） 只需要 SKILL.md 一个文件 ， context: fork \+ agent: 自动搞定其它。

这个 Skill 的设计如下。

```yaml
---
name: code-health-check
description: Perform a comprehensive code health check on a directory.
context: fork        # ← 关键：在隔离子代理中执行
agent: general-purpose  # ← 子代理类型
allowed-tools: [Read, Grep, Glob]  # ← 只读，不修改代码
---
```

```markdown
# Code Health Check

Analyze the codebase at `$ARGUMENTS` and produce a structured health report.

## Checks to Perform

1. File Organization - 文件大小、目录结构
2. Error Handling - try/catch、错误传播
3. Security Basics - 硬编码密钥、eval()、SQL 注入
4. Code Quality - 重复代码、未使用变量

## Output Format

Return a structured report:
- Overall health score (A/B/C/D/F)
- Issues found (categorized by severity: CRITICAL/WARNING/INFO)
- Top 3 recommendations
```

项目 07 的 src/ 中故意埋了多个安全和质量问题。

![](https://static001.geekbang.org/resource/image/a9/23/a9e90a23a800eb43807119fec578cc23.jpg?wh=2362x1440)

这些“已知答案“让你可以验证子代理的检查质量。

# 进入项目目录后，在 Claude Code 中执行：

> /code-health-check src/

此时 Claude Code 的执行流程如下：

/ code -health-check src /

│

├─ SKILL.md 被激活（context: fork）

├─ 自动创建 general-purpose 子代理

├─ 子代理在隔离上下文中：

│ ├─ Glob 扫描 src/ 下所有.js 文件

│ ├─ Read 每个文件

│ ├─ Grep 搜索 eval (), hardcoded secrets 等模式

│ └─ 生成健康报告

└─ 返回报告到主对话（主对话上下文干净）

此处的验证要点是：

是否发现了 3 个 CRITICAL 问题？（硬编码密钥、SQL 注入、eval）

是否发现了 WARNING 和 INFO 级别问题？

主对话上下文是否干净——你看不到子代理读文件的中间过程。

模式二的适用场景总结如下。

![](https://static001.geekbang.org/resource/image/f4/8f/f4c1762aebd1e57dc0273461d885358f.jpg?wh=2770x1129)

## 模式三：流水线中的 Skill 分工（方向 A 的多阶段串联）

下面我们再往前走一步。思考一下模式一的自然延伸——多个子代理各自预加载不同的 Skill，按阶段串联执行。每个阶段的输出作为下一阶段的输入。这种模式是和复杂的多阶段任务（我们之前子代理部分也介绍过类似的示例），但此处每个阶段需要通过 Skill 来配备不同的专业知识。

配套项目： 04-Skills/projects/08-skill-pipeline/

项目整体结构如下。

一个完整的 API 文档流程可以拆分为三个阶段，每个阶段需要不同的专业能力。

![](https://static001.geekbang.org/resource/image/77/f6/7763a614505ecbb63fd4b0047c9013f6.jpg?wh=3171x1149)

每一个阶段的输入输出流程如下。

**阶段 1：Route Scanner。** 对应 Skill `route-scanning/SKILL.md`：包含扫描脚本 `scan-routes.py`，输出 JSON 格式的路由清单。

```yaml
# .claude/agents/route-scanner.md
---
name: route-scanner
model: haiku  # 轻量任务用 haiku
tools: [Read, Grep, Glob, Bash]
skills:
  - route-scanning  # 预加载扫描知识
---
```

You are a route scanning specialist. You are Stage 1 of a documentation pipeline.

**阶段 2：Doc Writer。** 对应 Skill `doc-writing/SKILL.md`：包含文档模板 `endpoint-doc.md`，按模板生成标准化文档。

```yaml
# .claude/agents/doc-writer.md
---
name: doc-writer
model: sonnet  # 需要理解代码逻辑，用 sonnet
tools: [Read, Write, Glob]
skills:
  - doc-writing  # 预加载文档编写知识
---
```

You are a documentation writing specialist. You are Stage 2 of a documentation pipeline.

**阶段 3：Quality Checker。** 对应 Skill `quality-checking/SKILL.md`：包含质量标准规则 `doc-standards.md`，逐项检查文档质量。

```yaml
# .claude/agents/quality-checker.md
---
name: quality-checker
model: haiku  # 规则检查用 haiku 足够
tools: [Read, Grep, Glob]  # 只读，不修改文档
skills:
  - quality-checking  # 预加载质量检查知识
---
```

You are a documentation quality specialist. You are Stage 3 of a documentation pipeline.

流水线的编排逻辑写在 CLAUDE.md 中：

```markdown
## API Documentation Pipeline

When the user asks to run the documentation pipeline:

### Stage 1: Route Scanning
Use the `route-scanner` agent to scan the source directory.
Collect the route manifest (JSON) from its output.

### Stage 2: Documentation Generation
Use the `doc-writer` agent to generate documentation.
Pass the route manifest from Stage 1 as input context.

### Stage 3: Quality Validation
Use the `quality-checker` agent to validate the generated docs.
Report the quality verdict to the user.
```

编排的关键在于每个阶段的输出是下一阶段的输入。Claude 主对话扮演“项目经理”角色，依次调用三个专家。

![](https://static001.geekbang.org/resource/image/2d/e8/2d08b185331d353711f91314918a29e8.jpg?wh=2390x761) ![](https://static001.geekbang.org/resource/image/59/fb/59ef2ef9fd80d3925b9b48feb76939fb.jpg?wh=3379x1046)

下面进行运行与验证。

# 先验证扫描脚本

python3.claude/skills/route-scanning/scripts/scan-routes.py src/

# 预期：发现 12 条路由（products 7 条 + categories 5 条）

# 运行完整流水线

> 对 src/ 目录运行文档流水线

# 或者分阶段手动运行

> 用 route-scanner 扫描 src/ 目录的路由

> 用 doc-writer 根据上面的路由清单生成文档

> 用 quality-checker 验证 docs/ 目录下生成的文档

验证时需要注意下面几件事：

阶段 2 是否生成了 docs/products-api.md 和 docs/categories-api.md ？

阶段 3 的质量报告是 PASS 还是 NEEDS_REVISION？

对于流水线模式的设计，需要注意下面几个要点。

要点一：定义清晰的阶段间接口

每个阶段的 SKILL.md 都明确写了输出格式。

阶段 1 输出 JSON 路由清单

阶段 2 输出文件列表 + 路由覆盖数

这些就是阶段间的“接口合约”。

要点二：每个 Skill 只关注一件事

route-scanning 只扫描路由，不生成文档。

doc-writing 只生成文档，不检查质量。

quality-checking 只检查质量，不修改文档。

单一职责 让每个 Skill 更简洁、更可测试、更可复用。

要点三：编排逻辑集中管理

流水线的顺序、数据传递逻辑都放在 CLAUDE.md 中。如果要调整流程（比如跳过阶段 3），只需修改 CLAUDE.md ，不需要动 Skill 或 SubAgent。

> [!warning] 设计商榷：编排逻辑不该放在 CLAUDE.md
> 文章把流水线编排写在 CLAUDE.md 里，是为了教学便利——把所有东西放在一个文件中方便展示。但**生产环境不推荐这样做**。
>
> **CLAUDE.md 应该像 `.gitignore` 一样精简**——只放不可商量的底线规则（项目背景、命名约定、禁止事项）。编排逻辑属于"怎么做"，应该封装成一个独立的 **pipeline Skill**。
>
> 正确做法是把它从 CLAUDE.md 抽出来：
>
> ```
> .claude/skills/doc-pipeline/SKILL.md   ← 编排 Skill（替代 CLAUDE.md 中的流水线逻辑）
> .claude/agents/route-scanner.md        ← SubAgent（WHO/WHAT）
> .claude/agents/doc-writer.md
> .claude/agents/quality-checker.md
> .claude/skills/route-scanning/         ← 领域 Skill（HOW）
> .claude/skills/doc-writing/
> .claude/skills/quality-checking/
> ```
>
> 调用方式从"对 src/ 目录运行文档流水线"变成 `/doc-pipeline src/`——和 `awesome-skills` 的 `/go-review-lead` 完全一致的入口模式。

> [!note] 三层架构：Skill → SubAgent → Skill——复杂吗？
> 抽成 pipeline Skill 后，确实多了一层：**编排 Skill → SubAgent → 领域 Skill**。看起来层数多了，但实际上**层数不等于复杂度，层间职责清晰才是简单**。
>
> ```
> 编排 Skill（doc-pipeline）
>   │  管"做什么"：阶段顺序、数据传递、失败处理
>   │  运行在主会话中（不是 SubAgent）
>   │
>   ├─→ SubAgent（route-scanner）
>   │       │  管"隔离做"：独立上下文、独立工具
>   │       │
>   │       └─→ 领域 Skill（route-scanning）
>   │             管"按什么标准做"：扫描脚本、路由识别模式
>   │
>   ├─→ SubAgent（doc-writer）
>   │       └─→ 领域 Skill（doc-writing）
>   │
>   └─→ SubAgent（quality-checker）
>           └─→ 领域 Skill（quality-checking）
>   ```
>
> 三层各管各的，去掉任何一层反而更乱：
> • 去掉编排层 → 回到手动依次调用三个 Agent（模式三退化为模式一 × 3）
> • 去掉 SubAgent 层 → 领域 Skill 在主会话中执行，上下文污染
> • 去掉领域 Skill 层 → SubAgent 变成通用 Agent，失去专业能力
>
> 这不只是代码审查流水线的设计选择——它和 **SDD（Spec-Driven Development）三阶段工作流** 在结构上是同构的：编排 Skill 相当于 Spec 阶段（定义做什么）、SubAgent 相当于 Plan 阶段（在隔离上下文中规划怎么做）、领域 Skill 相当于 Implement 阶段（按标准执行）。三层不是冗余，是把 SDD 思想编码到了 Agent 运行时里。

## 三种模式完整对照

为了帮你更好地理解和区分，我将这三种模式总结成了一张表。

![](https://static001.geekbang.org/resource/image/33/f2/33070451d0f81ceac595d93dd9ed6af2.jpg?wh=3304x1779)

实际项目中，你觉得哪种模式最常用？——我感觉当然是第一种。

### 延伸案例：分诊-派发-汇总架构（awesome-skills）

> 来源：[awesome-skills / bestpractice / 架构篇](https://github.com/johnqtcg/awesome-skills/blob/main/bestpractice/%E6%9E%B6%E6%9E%84%E7%AF%87.md) — 以 Go 代码审查为例，从"单 Skill 漏报"到"Multi-Agent + Grep-Gated 全覆盖"的完整进化记录

#### 问题起点：提示词解决不了的注意力稀释

单 Skill 同时审查安全、并发、性能、错误处理、代码质量、测试、业务逻辑 7 个维度时，**High 级 Findings 会系统性压制 Medium 级 Findings**——这不是提示词写得不够好，而是 LLM 在单上下文多维度场景下的结构性限制。

实证：一段含 4 个 High 并发缺陷的代码，单 Skill 审查漏报了 Slice 预分配问题（Medium），模型事后承认："当 4 个 High 级别的并发缺陷占据了注意力后，我在走 Performance checklist 时不够仔细。"

#### 架构总览：Orchestrator-Workers + Grep-Gated

```
PR Diff / 代码片段
          │
          ↓
[主会话 + go-review-lead Skill]
   职责：分诊 + 派发 + 汇总
   不加载垂直审查 Skill，不直接审查代码
          │
  Phase 1-4: 四级分诊（grep + 模式匹配）
  无触发 → 跳过；有触发 → 派发
          │
┌────┬────┬────┬───┼───┬────┬────┬────┐
↓    ↓    ↓    ↓   ↓   ↓    ↓    ↓    ↓
[Security][Concurr][Perf] [Error] [Quality] [Test] [Logic]
 Agent    Agent   Agent  Agent   Agent    Agent   Agent
  │        │       │      │       │        │       │
加载安全 加载并发 加载性能 加载错误 加载质量  加载测试 加载逻辑
 Skill    Skill   Skill  Skill   Skill    Skill   Skill
  7 个 Agent 并行审查，每个只看一个维度
└────┴────┴────┴───┴───┴────┴────┴────┘
          │
          ↓
     主会话汇总：去重 + 合并 + 按严重度排序
          │
          ↓
      最终报告（含 Execution Status 审计矩阵）
```

#### Grep-Gated 执行协议（核心创新）

Multi-Agent 解决了**跨维度**注意力稀释，但同一个 Agent 内部，High 级 Findings 仍会压制 Medium 级。解法不是继续拆 Agent，而是**把模型当机器用——能 grep 的不要让模型用注意力扫**：

```
每个 Worker Agent 的执行流程：

1. 加载垂直 Skill（checklist + grep pattern）
2. 对 grep-gated 项 → 按 pattern 机械扫描
3. grep HIT  → 模型做语义确认（true/false positive）
4. grep MISS → 自动标记 NOT FOUND，跳过语义分析
5. 纯语义项 → 模型全量推理
6. 只上报 FOUND 项
7. 审计行：Grep pre-scan: X/Y items hit, Z confirmed
```

**覆盖率**：7 个 Skill、86 条 checklist，其中 **65 条（75%）可 grep 化**，模型注意力集中在剩余 25% 的纯语义项上。

Pattern 设计采用**宽门槛**策略：宁有 HIT 假阳性（交给模型过滤），不漏 MISS 假阴性（一旦 MISS 就跳过语义分析，没有补救机会）。

#### 三轮迭代验证

同一段代码，从单 Skill 到最终架构：

| 指标 | 轮次 1：单 Skill | 轮次 2：Multi-Agent v1 | 轮次 3：+ Grep-Gated |
|------|:---:|:---:|:---:|
| 架构 | 1 Agent + 重度 Skill | 7 Worker + 主会话编排 | 7 Worker + Grep-Gated 协议 |
| Slice Pre-allocation | ❌ 漏报 | ❌ 分诊盲区 | ✅ REV-009 正式上报 |
| 总 Findings | 8/9（漏 1）| 不稳定 | **13/13 全部捕获** |

关键是轮次 2 → 3 的跳跃：分诊盲区修复 + Grep-Gated 之后，**两个前两轮从未被捕获的 Finding 正式上报**。

#### 实证支撑（来自原文引用）

- **Anthropic 多 Agent 研究系统**：Claude Opus 4（Lead）+ Sonnet 4（Workers）比单 Agent Opus 4 性能高出 **90.2%**；token 使用量单独解释了 80% 的性能差异
- **AgentCoder 学术研究**：多 Agent 在 HumanEval 达到 **96.3% pass@1**（单 Agent SOTA 90.2%），用更少 token（56.9K vs 138.2K）达到更高准确率

#### 核心洞察："架构换模型"

> 对于多维度任务，**Sonnet × N 个聚焦 Agent 的组合效能，可以超过 Opus × 1 个泛化 Agent**。

多 Agent 并行累计 token 更高，但每次用更低价位的模型——整体账单往往持平，质量显著提升。更好的问法不是"用哪个最强的模型"，而是"我的任务架构能否把模型从需要同时擅长多件事，改造成只需专精一件事"。

#### 与文章三种模式的对比

| 对比维度 | 模式三（流水线） | 分诊-派发-汇总 |
|----------|-----------------|----------------|
| 执行方式 | **串行**：阶段 1 → 2 → 3 | **并行**：N 个 Agent 同时审查 |
| 数据流 | 单向：上游输出 → 下游输入 | 汇聚：所有 Agent → 主会话合并 |
| Skill 粒度 | 每个阶段一个 Skill | 每个垂直维度一个 Skill |
| 触发方式 | 全量启动 | **按需派发**：grep 分诊跳过无关维度 |
| 主会话角色 | 项目经理：依次调用 | 分诊台 + 汇总台 + 审计台 |
| 抗稀释 | 仅跨阶段 | 跨维度 + **维度内**（Grep-Gated） |

**关键设计要点**：

1. **go-review-lead 必须是 Skill，不能是 Agent 定义文件**——Claude Code 禁止子代理生成子代理。Lead 作为 Skill 在主会话中运行，才能派发 Worker
2. **分诊四级递进**：Import 扫描 → Diff Pattern → 文件路径 heuristic → 变更范围评估。无触发 → 跳过，不无差别全量启动。简单 PR 成本从 $0.16 降到 $0.02
3. **Checklist 上限原则**：每个垂直 Skill 不超过 15 条。超过说明该维度还可继续拆分——但拆分依据应是可重现的漏报症状，而非条目数量的绝对阈值
4. **部分成功优于全量失败**：某个 Worker 超时/报错不阻断整份报告，标记 `SKIPPED` 后继续汇总其他维度

## 本讲小结

好，这一讲就到这里。本讲结构清晰，学习线路明确，比起重复要点，我更想分享一系列选型指南。

首先是各种场景下的选型矩阵和反模式警告。

![](https://static001.geekbang.org/resource/image/f6/6e/f61eef2abffd4023e9b3f036bfd99d6e.jpg?wh=3725x1443) ![](https://static001.geekbang.org/resource/image/53/57/538d116e96c06ed720fbd78572e54257.jpg?wh=3377x1450)

说白了，还是用第一性原理区分什么时候用啥。这基于我们对于他们不同的职能的内化理解。

![](https://static001.geekbang.org/resource/image/6d/41/6d92f9c0cff9dbcbbbfdd334eb058441.jpg?wh=3066x1881)

这种职责划分原则，贯穿几个项目的设计：

┌──────────────────────────────────────────────────────────────┐

│ 职责划分 │

│ │

│ SubAgent（.md 文件）负责： │

│ ──────────────────── │

│ • WHO: "You are an API documentation specialist" │

│ • WHAT: "Generate API documentation for Express routes" │

│ • WHERE: "Write to docs/api/" │

│ • OUTPUT: "Return summary with route count and warnings" │

│ │

│ Skill（SKILL.md + 附属文件）负责： │

│ ────────────────────────── │

│ • HOW: "Step 1: Run detect-routes.py → Step 2: Analyze" │

│ • WITH WHAT: scripts / detect - routes.py, templates / api - doc.md │

│ • BY WHAT STANDARD: "Check auth middleware, mark with 🔒" │

│ • QUALITY: "All routes documented, schemas match code" │

└──────────────────────────────────────────────────────────────┘

这里我也给出组合过程中的排错速查表，供你参考。

![](https://static001.geekbang.org/resource/image/29/18/292a70f5ee30dacde5341775303dba18.jpg?wh=3316x1710)

最后的最后，就记住一句话。 SubAgent 定义“是谁、做什么”，Skill 定义“怎么做、用什么做”。两者各有所长，组合才完整。

## 思考题

如果你的 Skill 需要调用外部 API（如翻译服务），你会如何设计？

模板和 CLAUDE.md 中的 prompt 有什么区别？什么时候用模板，什么时候用 prompt？

同一个 Skill 被不同角色的 SubAgent 预加载，执行效果会有什么不同？试着设想一个具体场景。

在项目 08 的流水线中，如果阶段 3 报告 NEEDS_REVISION，你会如何设计“自动修复”流程？需要修改哪些文件？

设计一个你自己项目中的流水线：画出阶段划分、每阶段的 SubAgent 角色和 Skill 职责、阶段间的数据接口。

### 答：调用外部 API 的 Skill 设计

> 基于实战经验，调用外部 API 的 Skill 有三种主流实现方式，以及一套可复用的目录结构设计。

#### 三种 API 调用方式

| 方式 | 适用场景 | 示例 |
|------|----------|------|
| **curl（SKILL.md 内嵌）** | 简单 API，无复杂业务逻辑 | 天气查询、汇率转换 |
| **JS/Python 脚本** | 需要编码处理业务逻辑、引入三方依赖 | 翻译服务、数据清洗 |
| **Shell/Bat/PowerShell 脚本** | 系统安装、环境初始化，不依赖语言运行时 | 环境检测、配置初始化 |

**方式对比**：curl 最轻量但能力有限；脚本方式最灵活——可以封装业务逻辑、做错误处理、调用 SDK，但依赖对应运行时；Shell 类不依赖语言环境但跨平台兼容性需要注意。

#### 推荐的 Skill 目录结构

```
.claude/skills/translator/
├── SKILL.md              # 入口：基本信息 + 脚本路由表 + 高频命令示例 + 约束
├── scripts/
│   ├── tran.js           # Node.js 实现（优先尝试）
│   ├── tran.py           # Python 实现（回退方案）
│   └── env-check.sh      # 环境检测脚本
├── references/
│   ├── translation-guide.md   # 翻译知识文件
│   ├── api-docs.md            # 翻译 API 接口文档
│   └── best-practices.md      # 最佳实践
└── config/
    └── settings.yaml     # 授权 Key、默认源语言/目标语言、字数限制等
```

#### 设计要点

**1. 通用性优先——双运行时回退**

SKILL.md 中明确指示 AI：优先尝试 `node scripts/tran.js`，失败则回退 `python scripts/tran.py`。也可以先跑 `env-check.sh` 检测环境再决定用哪个。

**2. 环境变量检测**

脚本内检测 `TRANSLATE_API_KEY` 等环境变量，未配置时引导用户设置，而不是直接报错退出。

**3. 配置集中管理**

`config/` 目录维护授权 Key、默认参数、字数限制等。可以通过引导式问答让用户首次使用时完成配置，后续直接读取。

**4. 交互式选项（AskUserQuestion）**

SKILL.md 可以主动和用户交互——提供选项让用户选择源语言/目标语言、翻译风格（直译/意译/学术）、输出格式等。确认需求后再调用脚本执行：

```markdown
Before translating, confirm with the user:
1. Source language? (auto-detect / specify)
2. Target language? (EN / ZH / JA / ...)
3. Translation style? (literal / natural / academic)
Use the AskUserQuestion tool to present these options.
```

很多 AI 工具已原生支持此类交互工具，无需额外实现。这让 Skill 从"被动执行命令"升级为"主动引导式工作流"。

本讲我们掌握了 Skills 的高级模式和 SubAgent 配合实战。

下一讲我们将站在更高的视角—— Skills 在 Claude Code 五层架构中的定位 ，从五层架构全景定位到四种设计模式，看清 Skills 在整个技术栈中的角色，最终画出 Skills 在整个技术栈中的全景定位图。

欢迎你在留言区和我交流讨论。如果这一讲对你有启发，别忘了分享给身边更多朋友。