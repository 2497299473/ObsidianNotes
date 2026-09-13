---
lark_doc_url: https://my.feishu.cn/docx/L5g4dn0Rjo4fiXxVGvgcvcIsnqf
lark_doc_token: L5g4dn0Rjo4fiXxVGvgcvcIsnqf
---
# TencentDB Agent Memory 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/TencentCloud/TencentDB-Agent-Memory（默认分支 `feat/server_team`）
> - **官网**：无独立官网（GitHub 即主页）
> - **安装文档**：仓库根 INSTALL.md / INSTALL_CN.md
> - **协议**：MIT（已核实 LICENSE 原文，GitHub 显示 NOASSERTION 只是检测器误判）
> - **主要语言**：TypeScript（~92%）、Python（~3.5%）、Shell、CSS
> - **Stars**：13.1k（Forks 1.2k）
> - **定位**：团队级 AI Agent 记忆中枢（Team Memory Hub）
> - **出品方**：腾讯云
> - **最新版本**：v2.0.0（2026-08-03）
> - **记录日期**：2026-08-04

---

## 一句话定位

**一个团队级的 Agent 记忆中枢，把对话、文档、代码沉淀为四类"可复用、可治理、可配装"的记忆资产（Chat Memory / Skill / LLM-Wiki / Code-Graph），让下一位 Agent"直接读档"，不必从零重新学习。**

官方 slogan："Agents remember. Humans innovate." / "让 Agent 沉淀经验，让人专注创造。"

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **经验不复用** | 每个新 Agent/新会话从零开始，学习成本反复支付（README 原话："Most Agents' first task is re-learning your project"） |
| **高代价上下文靠人肉提醒** | 如"别重构旧鉴权模块，移动端还在用"这类关键约束，每次都要人重复说 |
| **聊天历史 ≠ 记忆** | 普通 RAG 只回答"能查到什么"，回答不了"谁可以用、哪个版本有效、该给哪个 Agent" |
| **框架锁定** | 记忆与具体 Agent 框架耦合，换框架就得重新训练 |
| **冷启动成本** | 新团队/新 Agent 没有存量经验可用 |

> [!important] 核心理念
> 记忆不只是"记住对话"，而是**任何能帮下一个 Agent 避免重复造轮子的信息**都值得保存、组织、复用：已有信息 → 可复用记忆资产 → 更少 Turns → 更少返工 → 更稳定的结果和更高的效率。

---

## 🏗️ 核心机制

### 四类记忆资产

| 资产 | 是什么 | 怎么工作 |
|------|--------|---------|
| **Chat Memory** | 跨会话的偏好、事实、决策、交互历史 | 对话写入 L0，异步 Pipeline 逐层蒸馏到 L1/L2/L3；每个 Agent 创建时自动获得独立记忆 |
| **Skill** | 可复用 SOP（带版本、资源文件、触发边界、执行步骤、验证规则） | 复杂任务跑通后由 LLM 自动从对话和工具调用中提炼；默认私有，审核后共享并配装给其他 Agent |
| **LLM-Wiki** | 结构化文档页面 + 链接图谱 | 上传文档 → LLM 抽取结构化页面 → FTS5 全文检索 + 知识图谱，可沿链接下钻 |
| **CodeGraph** | 代码符号、文件、调用关系、影响路径的预索引图谱 | 导入仓库 → 自动索引；Agent 改代码前先做 impact analysis（callers/callees/影响路径） |

### L0 → L3 分层蒸馏

| 层 | 保存什么 | 用途 | 接入方式 |
|----|---------|------|---------|
| **L0 Conversation** | 原始对话与完整上下文 | 核对原话、时间、来源 | 每轮由 proxy 写回 core（SQLite） |
| **L1 Atom** | 提取的事实、偏好、约束、事件 | 精确召回可执行信息 | 通过工具让模型**按需召回**（不直接注入，保护 KV cache） |
| **L2 Scenario** | 围绕项目/场景组织的知识块 | 快速恢复工作场景 | 直接注入 system prompt（Agent Profile） |
| **L3 Core/Persona** | 长期画像、稳定模式、高层认知 | 让 Agent 快速进入用户/团队语境 | 直接注入 system prompt |

蒸馏是**异步 Pipeline**。L3 人格的生成有 5 个触发条件（Agent 显式请求、冷启动、内容丢失重建、首个 Scene 提取完成、达到记忆增量阈值）。

### 检索机制

- **生成和召回都分层**：平时用 L2/L3 快速进入语境；需要具体事实时通过 **BM25 + 向量检索 + RRF** 回落到 L1/L0
- RRF 为标准 Reciprocal Rank Fusion（`RRF_K=60`，`score = Σ 1/(k+rank+1)`）
- **无 Embedding Provider 也能跑**（回退 BM25/FTS5）；存储后端 SQLite + sqlite-vec 或 Tencent VectorDB
- **token 预算防护**：结果受条数、字符预算、超时三重限制，"避免记忆反过来占满上下文"
- **知识不整库注入**：Agent 先 `/v3/tools/list` 发现能力，再 `/v3/tools/call` 按需读取页面/源码/影响路径

---

## 🧱 架构组成（三服务）

```
memory-core（:8420）     记忆与元数据内核：L0–L3 存储+异步蒸馏、Skill、鉴权
      │                  User/Team/Agent/Task 元数据；SQLite+本地文件，零外部依赖
memory-hub（:8125/:8424）团队管理面板 + Wiki/CodeGraph 引擎
proxy（:8096）           透明 LLM 请求代理（Anthropic+OpenAI 双协议原样转发）
```

**Proxy 请求处理 8 阶段**：auth → systemUser → sessionInit（首会话弹 team→agent→task 表单）→ injection（注入 skill/knowledge/L2/L3）→ rateLimit → forward → extract（对话回流：Skill 归档 + L0 写入）→ report（可观测 + 计费）。

**权限模型**：
- 可见性四档：`private`（仅 Owner）/ `team`（全队可读）/ `restricted`（User/Role/Agent ACL 精确授权）/ `agent`（同团队 Agent 定向装配）
- 角色两层：**全局 System Admin** + **Team 内 Admin/Member**；资产归属用 Owner 标记
- 新 Chat Memory 和 Skill **默认私有**，共享是显式动作——"共享经验，不共享隐私"

### 冷启动（"先读档，再开工"）

| 导入什么 | 生成什么资产 |
|---------|-------------|
| 代码库（公开 HTTPS 仓库） | CodeGraph 自动索引符号、文件、调用关系、影响路径 |
| 文档与文件 | Wiki 自动生成结构化页面 + 链接图谱 |
| 历史对话 Session | 自动提取 Skill 与 Chat Memory（也支持从 Opik trace 导入） |

---

## ✅ 优点

- **定位独特**：不止"记得住"，还管"谁能用、哪个版本、配给谁"，填补 RAG 与聊天记录之间的空白
- **一键部署体验好**：一条命令三件套、自动建 admin、打印 claude 启动命令、verify.sh 预检 LLM 通路
- **本地优先、零外部依赖内核**：SQLite + BM25 兜底，无 Embedding 也能跑，多架构镜像公开可拉
- **框架解耦**：资产可跨框架迁移，"换 Agent 只需重新装配，不必重新训练"
- **工程细节成熟**：RRF 混合检索、KV cache 友好的工具化召回、token 预算三重限制、proxy 限流/可观测/多节点存储降级链
- **MIT 协议**：商用友好（已核实 LICENSE 为实质 MIT）

---

## ⚠️ 潜在局限

| 方面 | 说明 |
|------|------|
| **部署偏重** | 三服务 + 首会话表单流程，个人单 Agent 场景略显冗余 |
| **CodeGraph 限制** | 当前优先支持**公开 HTTPS 仓库**，私有仓库/SSH 凭证仍在完善 |
| **接入约束** | Hermes/OpenClaw 接入 `x-task-id` 必填，`x-conversation-id` 需静态指定、换会话要手改；部分客户端 tool call 后续请求可能丢 header 导致该轮跳过注入 |
| **客户端兼容 bug** | CodeBuddy 4.10.2–4.10.4 有 sessionId 丢失 bug（需 ≥4.10.5 或 ≤4.10.1）；OpenClaw hooks 字段与版本强相关 |
| **依赖两套 LLM 配置** | 内部蒸馏组 + 上游组，蒸馏/召回均消耗 LLM token |
| **默认凭据弱** | 本地默认 gateway key=`local`，官方警告生产暴露前必须替换 |
| **版本迭代激进** | 0.x→1.x→2.x 四个月内跨越，数据格式 v2→v3 需迁移脚本，504 个 open issues |

---

## 🧩 适合谁用

- 👥 **多 Agent 团队 / "一人公司"**：给 Scout/Builder/Reviewer 等角色化 Agent 小队建立共享、可继承经验（官方主打玩法）
- 💻 **重度 Claude Code / CodeBuddy / OpenClaw / Hermes 用户**：让 coding agent 跨会话记住项目背景、排障经验、发布 checklist
- 📚 **有存量资产的组织**：已有文档库、代码库、历史会话（含 Opik trace），想冷启动注入而非从零积累
- 🛠️ **Agent 平台/框架开发者**：可参考其 adapter 模式（写 L0 → 召回 L1/L2/L3 → 有界注入）与资产治理模型

> [!warning] 不太适合
> 只想单会话临时记忆的个人轻量场景（偏重）；需要私有仓库 CodeGraph 的场景（暂不支持）。

---

## 🚀 如何使用（上手步骤）

> [!tip] 前置：macOS/Linux + Docker + bash 4+；源码方式另需 Node ≥22.16；准备两个 LLM API key（可为同一家的）

### 第一步：克隆并配置

```bash
git clone https://github.com/Tencent/TencentDB-Agent-Memory.git
cd TencentDB-Agent-Memory/deploy/global-images
cp .env.example .env
# 编辑 .env：
#   MEMORY_LLM_BASE_URL / API_KEY / MODEL      ← memory+hub 内部蒸馏用
#   PROXY_UPSTREAM_URL / API_KEY / MODEL       ← proxy 转发的主对话上游
./verify.sh        # 可选：预检两组 LLM 通路（--skip-llm 跳过）
./start-all.sh     # 一键拉起三件套；结束打印 claude 命令 + admin key（.admin-key）
```

### 第二步：面板初始化

1. 浏览器打开 `http://localhost:8125`，用 `.admin-key` 里的 `sk-mem-...` 登录
2. （推荐）admin 创建业务用户（`POST /v3/meta/user/create`），保存好只返回一次的 `default_user_key`
3. 建至少 1 个 Team + 1 个 Agent（填好 description/system prompt），可选建 Task

### 第三步：挂上 Claude Code

```bash
export ANTHROPIC_BASE_URL=http://127.0.0.1:8096/claude-code/default
export ANTHROPIC_AUTH_TOKEN=<业务用户 sk-mem-...>
claude --model <PROXY_UPSTREAM_MODEL>
```

新会话首轮按表单选 Team → Agent → Task；之后每轮自动注入该 Agent 的 L2/L3 + skill + knowledge。

### 第四步：观察记忆生长

面板里看 Chat Memory（L0 scene 切分）、Agent Profile（L2/L3 累积）、Skill 列表：

```bash
curl -s http://localhost:8420/health | jq .services.pipelineWorker
```

> 其他端点：Knowledge API `:8424/v3`、Swagger `:8424/docs`、Core `:8420`、Proxy `:8096`。清理：`./stop-all.sh --purge`。

---

## 📊 实际效果参考（Benchmark）

| Benchmark | 未启用 | 启用后 | 相对提升 |
|-----------|--------|--------|---------|
| **PersonaMem** | 48% | **76%** | **+59%** |

PersonaMem 检验 Agent 能否在长期交互后正确理解和运用用户信息。

> 来源：README。官方仅公布这一项，未提供基线模型与复现方式，仅供参考。

---

## 🔗 相关资源

| 资源 | 地址 |
|------|------|
| GitHub 仓库 | https://github.com/TencentCloud/TencentDB-Agent-Memory |
| 最新 Release | https://github.com/TencentCloud/TencentDB-Agent-Memory/releases/tag/v2.0.0 |
| Docker Hub | https://hub.docker.com/u/agentmemory（memory-core / memory-hub / memory-proxy） |
| npm | @tencentdb-agent-memory/memory-tencentdb、memory-sdk-ts-v2 |
| PyPI | tencentdb-agent-memory-sdk-python |
| 社区 | Discord https://discord.gg/dJQM6mKMF |
| 上游致谢 | colbymchenry/codegraph、nousresearch/hermes-agent、Karpathy "LLM Wiki" |

---

## 💡 个人思考

TencentDB Agent Memory 的差异化非常清晰：它不跟别人比"记忆召回准确率"，而是把**组织管理经验**这件事产品化了。核心命题是——RAG 只回答"能查到什么"，但团队场景还需要回答"谁可以用、哪个版本有效、应该配给哪个 Agent"。这四类资产（Chat Memory/Skill/Wiki/CodeGraph）+ 权限治理 + Agent 配装的组合，本质是把"企业知识管理"的思路搬给了 Agent 团队。

它的分层设计（L0→L3）和 OpenViking 的 L0/L1/L2、Headroom 的 CCR 一样，都认同"别把全部内容一次性灌给 LLM"，但落点不同：**Headroom 落在"传输时压缩"，OpenViking 落在"写入时预分层存储"，TencentDB 落在"对话逐层蒸馏成人格 + 资产治理"**。三者是互补关系而非竞品。

三个工程细节很见功力：一是 **L1 不直接注入而是工具化召回**，保护了上游 KV cache；二是 **token 预算三重限制**（条数/字符/超时），防止记忆反过来吃满上下文；三是 **RRF 混合检索**在无 Embedding 时自动回退 BM25，保证本地零依赖可跑。

但要清醒看待它的门槛：**部署偏重**（三服务 + 表单流程），单人轻量场景收益不明显；**CodeGraph 暂不支持私有仓库**；接入约束较多（`x-task-id` 必填、conversation-id 要手改、个别客户端版本 bug）。它真正的甜蜜点是**多 Agent 团队 + 有存量文档/代码/会话资产 + 需要权限治理**的组织场景。协议上已核实是实质 MIT，商用无忧。

> [!quote] 一句话总结
> 如果你在组建多 Agent 团队、想让经验"资产化、可继承、可治理"，TencentDB Agent Memory 是目前唯一把这块做完整的产品——代价是部署和接入比单机方案重不少。

> 状态：✅ 已分析，⏳ 待实践 · 关联：[[Headroom 项目笔记]]、[[OpenViking 项目笔记]]、[[Headroom vs OpenViking vs TencentDB Agent Memory]]
