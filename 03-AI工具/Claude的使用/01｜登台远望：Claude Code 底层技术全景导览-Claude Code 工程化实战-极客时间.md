---
title: 01｜登台远望：Claude Code 底层技术全景导览
course: Claude Code 工程化实战
author: 黄佳
source: https://time.geekbang.org/column/article/942438
created: 2026-06-14
published: 2026-01-27
description: Claude Code是一个可编程、可扩展、可组合的AI Agent框架，支持用户自定义工作流。它包括Skills、SubAgents、Hooks和集成层等核心组件，以及Agent SDK接口层。
tags:
  - clippings
  - claude-code
  - ai-agent
  - 极客时间
lark_doc_url: https://my.feishu.cn/docx/AZMUddO9mo4tDWxlh6xc5RernVf
---
# 01｜登台远望：Claude Code 底层技术全景导览

> 讲述：黄佳 · 时长 17:44

## 你眼中的 Claude Code 是什么？

我们可能会得到很多个不同的答案：

- 一个能读懂代码的 AI 助手
- 命令行里的 ChatGPT
- 帮我写代码的工具
- 比 GitHub Copilot 更强大的东西

这些答案都对，但都不完整。

**Claude Code 的真正身份是：一个可编程、可扩展、可组合的 AI Agent 框架。**

它不只是一个"工具"，而是一个"平台"——你可以在上面构建自己的 AI 工作流。就像你不会说 "VS Code 是一个文本编辑器"（它是，但远不止于此），Claude Code 也远不只是一个 AI 助手。

---

## Claude Code 5 分钟快速上手

### 安装

下载匹配系统的 [Claude Code](https://claude.ai) 版本：

```bash
# macOS / Linux / WSL（推荐，自动更新）
curl -fsSL https://claude.ai/install.sh | bash

# Windows PowerShell
irm https://claude.ai/install.ps1 | iex

# 或使用 Homebrew（需手动更新）
brew install --cask claude-code
```

首次运行：

```bash
claude  # 首次运行会提示登录
```

**Claude Code 是付费软件**，支持的账户类型包括：

- Claude Pro / Max / Teams / Enterprise（推荐）
- Claude Console（API 访问，需预付费）

### API Key + 中转

如果你不想订阅 Max，可以用 Console API Key（按 Token 付费）。国内直连 Anthropic API 不通，但 Claude Code 支持自定义 API 端点：

```bash
export ANTHROPIC_BASE_URL=https://your-relay-service.com/v1
export ANTHROPIC_API_KEY=sk-xxx
```

中转服务（API Relay）在国内有很多选择，也可以用 OpenRouter 作为更稳定的中间层——它聚合了 Anthropic、OpenAI、Google 等多家模型 API，支持国内访问。

> ⚠️ **注意**：中转服务存在 Key 泄露和服务中断风险，选择信誉好的平台，不要用来处理敏感代码。

### 同生态位的开源替代

如果你暂时搞不定网络和支付，下面这些工具和 Claude Code 是同一个物种——终端 AI Agent，同一套 Agentic Loop 内核，本课程的概念框架几乎可以迁移。

| 工具 | 说明 |
|------|------|
| **OpenCode** | 最好的平替，支持 DeepSeek、Qwen、Kimi、GLM 等国内模型 API。安装：`curl -fsSL https://opencode.ai/install | bash` |
| **Cursor** | 基于 VS Code 的 AI 编辑器，内置 Claude 和 GPT 模型，支持国内网络，有免费额度 |
| **Cline** | VS Code 开源插件，支持多种 LLM 后端（DeepSeek、Qwen 等），灵活度高 |
| **Windsurf** | Codeium 推出的 AI IDE，有免费版，支持 agentic 编码 |
| **通义灵码** | 阿里云国产 AI 编程助手，IDE 插件形式，完全国内可用 |
| **自建方案** | Cline + DeepSeek API，或 Cline + 本地部署开源模型（如 Qwen-Coder） |

### 多账户、多模型切换

推荐社区工具 **CC Switch**（`npx @songhe/cc-switch`），功能包括：

- **模型切换**：内置 Anthropic Claude、OpenAI GPT、DeepSeek、Qwen、GLM、MiniMax、Kimi 等模板
- **MCP 管理**
- **Skills 管理**

本质上是帮你改写项目级 `.claude/settings.json` 中的 API 配置。

如果需要多账户切换 + 代理管理，可以用 **CCS**（Claude Code Switch），支持 300+ 模型 via OpenRouter，有可视化 Dashboard。

> 💡 **一句话总结**：CC Switch 管"用哪个模型 + 哪个 MCP"，CCS 管"哪个账户 + 哪个代理"。选一个顺手的就行，核心功能都是改 `settings.json`。

---

## 从使用者到驾驭者

两种使用模式对比：

| 模式 | 流程 | 说明 |
|------|------|------|
| **被动使用** | 用户 → 输入问题 → Claude 回答 → 完成 | 你问，它答。就像用计算器 |
| **主动驾驭** | 用户 → 配置 Agent → Agent 自主工作 → 自动完成任务 | 你设计，它执行。就像编写程序 |

用学开车打个比方：

- **使用者**：知道方向盘转哪边车往哪走，油门让车动，刹车让车停
- **驾驭者**：理解发动机、变速箱、刹车系统的工作原理，能改装车辆

对于 Claude Code：

- **使用者**：知道怎么提问，怎么让 Claude 帮你写代码
- **驾驭者**：理解记忆系统、子代理、技能包、钩子的工作原理，能构建自定义工作流

> 🎯 **这门课的目标：把你从被动使用者变成主动驾驭者。**

---

## Claude Code 底层技术全景图

Claude Code 的底层能力从技术上拆解可以分为四个层次：

```
编程接口层：Agent SDK
     ↑
集成层：Headless（无头模式）+ MCP（Model Context Protocol）
     ↑
扩展层：Commands + Skills + SubAgents + Hooks（四大核心组件）
     ↑
基础层：Memory（记忆系统）
```

### 基础层：Memory（记忆系统）

CLAUDE.md 就是 Claude 的"新员工手册"。

**示例**：

```markdown
# Project: E-commerce Platform

## Tech Stack
- Frontend: React + TypeScript
- Backend: Node.js + Express
- Database: PostgreSQL

## Code Style
- Use functional components
- Prefer async/await over .then()
- Maximum line length: 100 characters

## Important Rules
- NEVER commit to main directly
- Always run tests before pushing
```

**三级记忆系统**：

| 层级 | 路径 | 作用域 |
|------|------|--------|
| 全局 | `~/.claude/CLAUDE.md` | 所有项目共用 |
| 项目 | `项目根目录/CLAUDE.md` | 当前项目 |
| 模块 | `项目根目录/.claude/rules/*.md` | 特定目录 |

### 扩展层：四大核心组件

#### Commands（斜杠命令）

- **触发**：用户手动输入 `/command`
- **适合**：标准化操作（团队统一的 commit 格式、固定的部署流程等）

```
用户输入: /review
Claude 执行: 根据 .claude/commands/review.md 的指令审查代码
```

#### Skills（技能）

- **触发**：Claude 自动判断（语义推理）
- **适合**：有强烈"领域感"的能力（安全、架构、性能），判断依赖上下文而非关键词，执行路径可能变化

```
用户说: "帮我看看这段代码有没有安全问题"
Claude 思考: 这是代码安全审查任务 → 激活 security-review Skill
Claude 执行: 按照 Skill 中定义的流程审查代码
```

> 💡 **Tool vs Skill**：Tool 解决的是"我能不能做"；Skill 解决的是"我该不该做、怎么做、做到什么程度"。

Claude 的隐式判断流程：

1. 这是代码吗？→ 是
2. 这是哪一类代码？→ Node.js 后端
3. 上下文是否涉及用户输入？→ 是
4. 是否存在鉴权逻辑？→ 是
5. 是否值得深入做安全审查？→ 是

然后自动激活 `security-review` Skill。

#### SubAgents（子代理）

- **触发**：由 Claude 决定或用户指定
- **适合**：隔离执行——高噪声任务、需要特定权限的任务

```
主 Claude: 这个任务需要跑大量测试，让我创建一个子代理来处理
子代理（test-runner）: 执行测试，只把结果汇报给主 Claude
```

#### Hooks（钩子）

- **触发**：事件自动触发
- **适合**：自动化检查——格式化、安全检查、日志记录等

```
事件: Claude 即将执行 Edit 工具
Hook: 自动检查是否有安全敏感内容
结果: 如果发现问题，阻止执行并警告
```

### 集成层：连接外部世界

#### Headless（无头模式）

让 Claude Code 在没有人工交互的情况下运行，适合 CI/CD 集成。

```yaml
# GitHub Actions 中
- name: Auto-fix code issues
  run: claude --headless "Fix all linting errors in src/"
```

#### MCP（Model Context Protocol）

让 Claude 连接外部工具和服务：

```
Claude → MCP → 数据库
Claude → MCP → Jira
Claude → MCP → 自定义 API
```

### 编程接口层：Agent SDK

当配置式的扩展不够用时，用代码来驱动 Claude：

```python
from claude_sdk import ClaudeSDKClient

client = ClaudeSDKClient()

result = client.query(
    prompt="Review this code for security issues",
    tools=["Read", "Grep"],
    max_turns=10
)
```

---

## 组件关系和技术选型指南

### 触发方式

| 组件 | 触发方式 | 确定性 |
|------|----------|--------|
| Commands | 用户手动 `/command` | 100% |
| Hooks | 事件自动触发 | 100% |
| Skills | Claude 语义推理激活 | 概率性 |
| SubAgents | Claude 决定或用户指定 | 可控 |

**为什么"确定性"很重要？**

- 需要**每次都执行**的操作（如代码格式化）→ 选 Commands 或 Hooks
- 希望 Claude **智能判断何时使用**（如安全审查）→ 选 Skills
- 任务可能很重，需要**手动触发或自动决定** → 选 SubAgents

### 数据流向

一个典型请求的生命周期（以"帮我修复 src/api.js 中的安全漏洞"为例）：

1. **Memory 层**：加载 CLAUDE.md，了解项目规范
2. **扩展层分发**：
   - 没有斜杠命令 → Commands 不参与
   - 识别"安全漏洞"关键词 → 激活 security-review Skill
   - Skill 指示创建子代理执行测试
3. **Hooks 监控**：Edit 工具执行前，自动运行预检查脚本
4. **工具执行**：通过 Read、Edit 等工具完成代码修改
5. **MCP 连接**：如果配置了 Jira MCP，自动更新相关 ticket

> 🔑 **关键洞察**：Memory 是基础设施，始终存在；扩展层是能力中心，按需激活；Hooks 是守门人，监控一切。

### Plugins：打包容器

Plugins 不是一种新能力，而是**打包机制**——把一组相关的 Claude Code 扩展打包在一起。

```
my-team-plugin/
├── commands/         # 斜杠命令
│   └── review.md
├── skills/           # 技能
│   └── security-check/
│       └── SKILL.md
├── agents/           # 子代理
│   └── test-runner.md
├── hooks/            # 钩子
│   └── pre-edit.sh
└── plugin.json       # 插件配置
```

**Plugin 的价值**：可复用、可版本化、可分发。

### 技术选型指南

决策流程：

```
这是"能力"吗？
  ├── 是 → 希望手动触发还是自动识别？
  │   ├── 手动触发 → Commands
  │   └── 自动识别 → Skills
  ├── 不是 → 这是"检查机制"吗？
  │   ├── 是 → 需要在工具执行时自动检查？→ Hooks
  │   └── 不是 → 需要连接外部系统？→ MCP
  └── 需要完全编程控制？→ Agent SDK
```

**场景实操**：

| 需求 | 分析 | 方案 |
|------|------|------|
| 团队统一 commit message 格式 | 能力问题，手动触发更合适 | **Commands**（创建 `/commit`） |
| Claude 修改代码时自动安全检查 | 检查机制，工具执行时自动触发 | **Hooks**（创建 pre-Edit hook） |
| Claude 查询内部知识库 | 连接外部数据源 | **MCP**（创建知识库 MCP server） |

### 组合使用

真实世界的问题很少能用单一技术解决。示例：**自动 PR 审查流程**

1. **Headless** 模式在 CI 中触发（GitHub Actions 监听 PR 事件）
2. 调用 **code-review SubAgent**（隔离审查任务）
3. SubAgent 使用 **security-check Skill**（自动识别安全代码）
4. **Hooks** 记录审查日志（便于审计和调试）
5. 结果通过 **MCP** 发送到 Slack（通知相关人员）

> 🧩 **这就是可组合的威力**：每个组件做好自己的事，组合起来完成复杂任务。

---

## 总结

1. **认知转变**：Claude Code 不只是一个聊天工具，而是一个**可编程的 AI Agent 框架**

2. **四层架构**：
   - **基础层**：Memory，让 Claude 记住你的项目
   - **扩展层**：Commands、Skills、SubAgents、Hooks 四大核心组件
   - **集成层**：Headless 融入 CI/CD，MCP 连接外部世界
   - **接口层**：Agent SDK 提供完全编程控制

3. **技术选型方法**：能力问题 vs 流程问题？确定性触发 vs 智能识别？隔离执行 vs 集中处理？

4. **组件可组合**：Headless + SubAgent + Skill + Hook + MCP 可以组成完整的自动化流程

---

## 思考题

1. 回顾你目前使用 Claude Code 的方式，属于"被动使用"还是"主动驾驭"？
2. 你的工作中有哪些重复性任务可以通过 Commands 或 Skills 自动化？
3. 如果要构建一个"自动代码审查 + 自动修复 + 自动提交 PR"的流程，你会组合哪些技术？

---

## 彩蛋：扩展层四大核心组件对照表

| 组件 | 触发方式 | 适合场景 | 类比 |
|------|----------|----------|------|
| Commands | 手动 `/command` | 标准化操作 | 快捷键 |
| Skills | 语义推理自动激活 | 领域专家能力 | 专家模式 |
| SubAgents | Claude 决定/用户指定 | 隔离执行 | 外包任务 |
| Hooks | 事件自动触发 | 自动化检查 | 门禁系统 |

---
