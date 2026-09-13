---
lark_doc_token: WmrLdUNg2oyWdIxIEFBcrGO0nQe
lark_doc_url: https://my.feishu.cn/docx/WmrLdUNg2oyWdIxIEFBcrGO0nQe
---
# Mnemosyne 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/mnemosyne-oss/mnemosyne
> - **官网**：https://mnemosyne.site
> - **协议**：MIT License（© 2026 Abdias J）
> - **主要语言**：Python（纯 Python 依赖设计）
> - **Stars / Forks**：2.3k+ / 194
> - **创建 / 最近推送**：2026-04-05 / 2026-08-10（活跃开发中，最新 release v3.15.1 @ 2026-07-30）
> - **Topics**：agents、ai、hermes、hermes-agent、ml、nousresearch
> - **定位**：零云依赖的通用 AI Agent 记忆层（Hermes-first，兼容一切 Agent 框架）
> - **记录日期**：2026-08-11

---

## 一句话定位

**一个 SQLite 文件 + 一个纯 Python 依赖实现的"零云"AI 记忆层：以 Hermes Agent 为第一公民，同时一条命令接入 Cursor / Claude Code / Codex / Windsurf / OpenWebUI / Pi / OpenClaw / 任意 MCP 客户端 / 任意 Python Agent。数据默认永不离开本机。**

> ⚠️ 注意：GitHub 上同名的 mnemosyne 项目很多（间隔重复闪卡软件 mnemosyne-proj、Hermes 记忆插件 AxDSan/mnemosyne、Rust 堆转储分析引擎 bballer03/Mnemosyne 等），本笔记特指 **mnemosyne-oss/mnemosyne**，即 Hermes 生态的 BEAM 记忆层。

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **Agent 记忆碎片化** | 每个 Agent 框架各搞一套记忆方案，换框架记忆就丢了 |
| **云记忆隐私风险** | 很多记忆方案把数据传到云端，用户无法掌控 |
| **部署门槛** | 向量数据库 + ANN 索引 + embedding 服务一套下来太重 |
| **检索质量差** | 纯向量检索易漏关键词命中，纯关键词检索不语义；两者都缺时序性 |

---

## 🏗️ 核心架构：BEAM（Bilevel Episodic-Associative Memory）

三层记忆，全部落在单个 SQLite 文件里：

```
┌────────────────────────────────────────────────────────────┐
│  Mnemosyne（单 SQLite 文件，零外部服务）                      │
│  ─────────────────────────────────────────────────────────  │
│  ① Working Memory（工作记忆）                                │
│     热上下文，LLM 调用前自动注入，TTL 过期驱逐                 │
│  ② Episodic Memory（情景记忆）                               │
│     长期存储，sqlite-vec + FTS5 混合检索                     │
│  ③ TripleStore（时序知识图谱）                               │
│     带版本链的时间感知三元组                                  │
└────────────────────────────────────────────────────────────┘
```

### 混合检索评分（全在 SQLite 内完成）
- **50% 向量相似度**（sqlite-vec，Hamming 距离，无 ANN 索引）
- **30% FTS5 关键词命中**
- **20% 重要度加权**

### MIB 二进制向量
- 384 维 float32 向量 → **48 字节（32× 压缩）**
- Hamming 距离直接用 SQLite 计算 → **不需要外部向量库 / ANN 索引**

### 空间效率
- 10M 条消息 → 仅 **7.2MB**（情景压缩再省 9.4× 空间）
- 检索延迟 35ms，弃权准确率 100%（宁可不说也不瞎编）

---

## 📊 基准成绩（注意版本说明）

| 基准 | 成绩 | 说明 |
|------|------|------|
| **LongMemEval**（ICLR 2025 检索基准，2026-04 实测） | **98.9% Recall@All@5** | 压过 Mempalace 96.6% / Backboard 93.4% / Hindsight 91.4% |
| **BEAM**（ICLR 2026 端到端 QA，v3.0.0 测） | 65.2%（100K） | ⚠️ 官方已注明过时待重跑，judge 配置与 Hindsight 73.4% 不可直接对比 |

> 官方在 README 里诚实标注了旧版本基准的局限，这点值得好评。

---

## 🔌 生态接入（最大卖点）

| 平台 | 方式 | 接入成本 |
|------|------|----------|
| **Hermes Agent** | MCP + 原生插件 | 开箱即用，默认启用 |
| Cursor | MCP | 加 `.cursor/mcp.json` |
| Claude Code | MCP | 加 `claude.json` |
| OpenAI Codex CLI | MCP | 加 `.codex/mcp.json` |
| Windsurf | MCP | 加 `.windsurf/mcp_config.json` |
| OpenWebUI | 原生 @tool | 把 bridge 文件丢进 `data/tools/` |
| Pi | Pi extension + skill | `pi install npm:@mnemosyne-oss/pi-mnemosyne` |
| OpenClaw | 原生 provider | `pip install mnemosyne-memory[openclaw]` |
| Hermes Tweet | 伴生插件 | 记忆会话需要 X/Twitter 上下文时挂 Xquik-dev/hermes-tweet |
| 任意 MCP 客户端 | MCP（stdio/SSE） | 一行配置 |
| 任意 Python Agent | 直接 SDK | `import mnemosyne` |

---

## 🔐 隐私与同步

- **本地优先、零遥测**：不开 sync，数据永不离开本机
- **可选客户端加密同步**：
  - Fernet AES-128-CBC / PyNaCl SecretBox XSalsa20-Poly1305
  - 密钥不出本机，远端服务器只能看到元数据
  - README 声称这是同类方案中独一份

---

## ⚙️ 硬件档位（pip 三档）

| Profile | 内存占用 | 适用场景 |
|---------|----------|----------|
| `mnemosyne-memory` | ~50MB | 树莓派 4 都能跑，远端 embedding API |
| `[embeddings]` | ~800MB | 桌面单用户，fastembed 本地向量 |
| `[all]` | ~1.5GB | 全功能：本地向量 + 本地 LLM（ctransformers） |

---

## 🌐 中文用户注意

默认嵌入模型是英文优化的 **bge-small-en-v1.5**，中文可换：
- `BAAI/bge-small-zh-v1.5`（中文专用小模型）
- 多语言模型：MiniLM-L12-v2 / e5-base / bge-m3

---

## 🧭 同赛道对比（Agent 上下文 / 记忆基础设施）

| 项目 | 定位 | 关键差异 |
|------|------|----------|
| **Mnemosyne** | 记忆层 | SQLite 单文件、零云、MIB 二进制向量、Hermes-first、8+ 平台接入 |
| **headroom**（headroomlabs-ai） | 上下文压缩层 | CCR 可逆压缩，砍 60–95% token，Apache 2.0 |
| **CodeGraph**（colbymchenry） | 代码知识图谱 | Rust 内核预索引，34 语言，SQLite 存符号/调用边 |
| TencentDB-Agent-Memory | 记忆存储 | 腾讯云的 Agent 记忆方案（见同目录对比笔记） |

---

## 📝 备注 / 观察

- Releases 链接曾指向 `AxDSan/mnemosyne`，推测是作者 Abdias J 个人仓库迁移到 org 后的产物
- Topics 带 `nousresearch`，属于 **Nous Research Hermes Agent 生态**
- 商业层面有 **Atlas Cloud** 作为计算赞助方（提供 nightly recall 基准的 inference credits），说明项目有社区运营但核心代码保持 MIT 开源
- 开发活跃度：最新 commit 2026-08-10（`fix(hermes): validate the discovered interpreter and honor --python`），版本迭代到 v3.15.1（2026-07-30），65 个开放 issue

---

## 🔗 相关链接

- 仓库：https://github.com/mnemosyne-oss/mnemosyne
- 官网：https://mnemosyne.site
- 赞助合作：https://mnemosyne.site/partners
- 伴生插件 Hermes Tweet：https://github.com/Xquik-dev/hermes-tweet
