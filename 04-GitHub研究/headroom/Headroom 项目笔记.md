---
lark_doc_url: https://my.feishu.cn/docx/JNb0dMgaPo2iMjx7yYhc2mmJnod
lark_doc_token: JNb0dMgaPo2iMjx7yYhc2mmJnod
---
# Headroom 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/headroomlabs-ai/headroom
> - **官网**：https://headroomlabs.ai
> - **文档**：https://headroom-docs.vercel.app/docs
> - **协议**：Apache 2.0
> - **主要语言**：Python（核心）、TypeScript（SDK）
> - **Stars**：61.3k+（2026-06 曾登顶 GitHub Trending #1）
> - **定位**：AI Agent 的上下文压缩层（Library · Proxy · MCP）
> - **记录日期**：2026-07-24

---

## 一句话定位

**一个本地运行的上下文压缩层，拦截并压缩 Agent 发送给 LLM 的所有内容（工具输出、日志、JSON、代码、文件、RAG 结果），在保证答案不变的前提下砍掉 60–95% 的 token。**

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **Agent 大量重读而非思考** | 跑一小时的 coding agent，真正"思考"的 token 很少，大量 token 花在反复读取失败 CI 日志、200 行 JSON blob 等 |
| **工具输出冗余** | API 返回 200 行 JSON 但 agent 只需要 4 个字段；CI 日志全量灌入上下文 |
| **RAG 检索膨胀** | 检索回来的 chunk 70–95% 是模板/样板内容，只有少量真正有信息量 |
| **成本与延迟** | token 多 = 贵 = 慢；长上下文还会触发模型注意力衰减 |
| **对话历史膨胀** | 多轮对话后历史越来越长，每次都要重复发送 |

---

## 🏗️ 核心机制

### 整体架构

```
你的 Agent / 应用（Claude Code, Cursor, Codex, LangChain, Agno, Strands, 自研代码…）
    │  prompts · tool outputs · logs · RAG results · files
    ▼
┌────────────────────────────────────────────────────────┐
│  Headroom（本地运行 — 数据不出本机）                        │
│  ──────────────────────────────────────────────────────  │
│  CacheAligner → ContentRouter → CCR                      │
│    ├─ SmartCrusher（JSON 压缩）                           │
│    ├─ CodeCompressor（AST 代码压缩）                      │
│    └─ Kompress-v2-base（通用文本压缩，HF 模型）            │
│                                                          │
│  Cross-agent memory · headroom learn · MCP              │
└────────────────────────────────────────────────────────┘
    │  压缩后的 prompt + 检索工具
    ▼
LLM Provider（Anthropic · OpenAI · Bedrock · …）
```

### 压缩管线三阶段

| 阶段 | 作用 |
|------|------|
| **CacheAligner** | 对齐缓存，避免重复压缩相同内容 |
| **ContentRouter** | 按内容类型路由到对应压缩器（JSON → SmartCrusher，代码 → CodeCompressor，文本 → Kompress-v2-base） |
| **CCR（Compress-Cache-Retrieve）** | 压缩后缓存原文，LLM 可随时通过检索工具取回原始内容——压缩是**可逆的** |

> [!important] 核心理念
> Headroom 不是"摘要后祈祷模型还能答对"。它通过 CCR 机制保证**可逆压缩**：先压缩，LLM 需要细节时再检索原文。答案不变，token 大幅减少。

### 三种压缩器

| 压缩器 | 针对内容 | 技术 |
|--------|---------|------|
| **SmartCrusher** | JSON 数组/对象 | 结构感知，去掉冗余字段和样板 |
| **CodeCompressor** | 源代码 | AST 分析，保留语义去掉噪音 |
| **Kompress-v2-base** | 通用文本/日志/RAG chunk | HuggingFace 上的自研模型 |

### 四种交付形态

| 形态 | 安装方式 | 适用场景 |
|------|---------|---------|
| **Python 库** | `pip install "headroom-ai[all]"` | 内联调用，代码中直接 `from headroom import compress` |
| **TypeScript SDK** | `npm install headroom-ai` | Node/前端项目集成（不含 CLI） |
| **HTTP 代理** | `headroom proxy --port 8787` | 零代码改动，OpenAI/Anthropic 兼容的 drop-in 代理 |
| **MCP Server** | 内置工具 | Claude Code / Cursor / 任意 MCP 宿主，工具：`headroom_compress`、`headroom_retrieve`、`headroom_stats` |

---

## ✅ 优点

- **直击 token 成本**：不是省一点点，是 60–95% 级别的削减（JSON 场景），coding agent 也有 20%
- **可逆压缩**：CCR 机制让 LLM 随时能取回原文，不是有损摘要，答案质量有保障
- **本地优先**：数据不出本机，压缩全部在本地完成
- **零侵入**：Proxy 模式不改一行代码就能接入；MCP 模式适配主流 agent 客户端
- **多形态**：库、代理、MCP 三选一，覆盖从"嵌入代码"到"包住整个 agent"的各种用法
- **生态广**：支持 Claude Code、Codex、Cursor、Aider、LangChain、CrewAI、Agno、LiteLLM、MCP、AWS Strands
- **开源 Apache 2.0**：可商用，无锁定
- **Netflix 出品**：源于 Netflix 内部工程实践，经过大规模验证后开源

---

## ⚠️ 潜在局限

| 方面 | 说明 |
|------|------|
| **压缩不等于无损** | 虽有 CCR 可逆机制，但极端情况下 LLM 可能因未自动检索原文而遗漏细节 |
| **本地依赖** | 需要本机运行 proxy 或 MCP server，增加了一层基础设施 |
| **Python 生态为主** | TypeScript SDK 功能相对有限（无 CLI） |
| **压缩效果因内容而异** | 自然语言对话压缩率低于结构化 JSON；已很精炼的 prompt 提升有限 |
| **新项目快速迭代** | Stars 增长极快，API 和 CLI 可能频繁变动 |
| **需要监控** | `headroom doctor` / `headroom perf` 需定期检查确保路由和压缩正常工作 |

---

## 🧩 适合谁用

- 🤖 **重度 Coding Agent 用户**：每天跑 Claude Code / Codex / Cursor，token 消耗大、成本高
- 🔗 **RAG / Agent Pipeline 开发者**：检索结果和工具输出经常灌入大量冗余 token
- 💰 **关注成本的人**：想在不换模型、不改代码的前提下把 API 费用砍掉一半以上
- 🏢 **企业内网部署**：本地优先 + 可逆压缩，数据不出内网
- ⚡ **长对话场景**：多轮 agent 对话历史不断膨胀，需要压缩历史上下文

---

## 🚀 如何使用（上手步骤）

> [!tip] 建议按以下顺序逐步进行

### 第一步：安装

```bash
# 方式一：uv（推荐，自包含虚拟环境）
uv tool install --python 3.13 "headroom-ai[all]"

# 方式二：pip（全局安装 CLI）
pip install "headroom-ai[all]"

# 方式三：npm（仅 TypeScript SDK，不含 CLI）
npm install headroom-ai
```

### 第二步：选择使用模式

```bash
# 模式 A：一键部署 + Agent 配置（最省心）
headroom deploy

# 模式 B：包裹某个 coding agent（如 Claude Code）
headroom wrap claude

# 模式 C：Drop-in 代理（零代码改动）
headroom proxy --port 8787

# 模式 D：内联库（嵌入自己的代码）
# from headroom import compress
```

### 第三步：验证与查看效果

```bash
# 健康检查 — 确认路由和压缩管线正常工作
headroom doctor

# 性能报告
headroom perf

# 实时节省仪表盘（需先启动 proxy）
headroom dashboard
```

### 第四步（可选）：作为 MCP Server 接入

如果你的 agent 客户端支持 MCP（如 Claude Code、Cursor），可直接注册 Headroom 的 MCP 工具：

- `headroom_compress` — 压缩指定内容
- `headroom_retrieve` — 检索压缩前的原文
- `headroom_stats` — 查看压缩统计

---

## 📊 实际效果参考

| 场景 | 压缩前 | 压缩后 | 压缩率 |
|------|--------|--------|--------|
| Claude Code 实时请求 | 3,576 tokens | 666 tokens | 81% |
| 编码 Agent（综合） | — | — | ~20% |
| JSON 工具输出 | — | — | 60–95% |
| 基准测试 | 10,144 tokens | 1,260 tokens | ~88% |

> 来源：YouTube 演示视频 + Trendshift 描述，实际效果因内容类型而异。

---

## 🔗 相关资源

| 资源 | 地址 |
|------|------|
| GitHub 仓库 | https://github.com/headroomlabs-ai/headroom |
| 官网 | https://headroomlabs.ai |
| 文档 | https://headroom-docs.vercel.app/docs |
| 压缩模型 | https://huggingface.co/chopratejas/kompress-v2-base |

---

## 💡 个人思考

Headroom 解决的是一个在 Agent 时代被很多人忽视但成本巨大的问题：**Agent 的 token 不是花在"思考"上，而是花在"重读"上。** 每次工具调用返回一大坨 JSON、每次 CI 失败灌入全量日志、每次 RAG 检索带回一堆样板 chunk——这些内容 70–95% 是信息密度极低的冗余数据。

它的核心创新不在于"压缩"本身（压缩技术早就有了），而在于 **CCR（Compress-Cache-Retrieve）可逆机制**：压缩后缓存原文，LLM 随时能检索回来。这让它区别于"有损摘要"方案——不会因为压缩而丢掉关键细节。

从工程角度看，四种交付形态（库 / 代理 / MCP / SDK）覆盖了几乎所有接入方式，尤其是 Proxy 模式的"零代码改动"对已有项目极为友好。作为 Netflix 出品的开源项目，经过大规模内部验证后开源，可信度较高。

> [!quote] 一句话总结
> 如果你每天用 coding agent 烧大量 token，又不想换模型或改代码——Headroom 是目前最值得尝试的上下文压缩方案。

> 状态：✅ 已分析，⏳ 待实践
