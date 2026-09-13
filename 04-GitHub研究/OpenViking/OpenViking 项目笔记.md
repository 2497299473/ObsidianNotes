# OpenViking 项目笔记

> 记录时间：2026-08-31 · 来源：GitHub README（一手）
> 链接：https://github.com/volcengine/OpenViking

## 一句话定位

字节火山引擎开源的「自进化上下文数据库」：把 Agent 的**记忆 + 知识 RAG + 技能统一成一个 `viking://` 虚拟文件系统**——Agent 用 `ls` / `tree` / `find` / `grep` 浏览自己的上下文，而不是查黑盒向量库（官方原话："like a developer working with files"）。

## 基本信息

| 项 | 值 |
|---|---|
| 语言 | Python 3.10+ |
| 协议 | **主项目 AGPLv3**（CLI 与 examples 为 Apache 2.0） |
| Stars | 30k+ |
| 版本 | 0.3.x 活跃开发 |
| 论文 | VLDB 2026（VikingMem，arXiv:2605.29640） |

## 核心机制

### 1. 三级加载（token 经济学核心）
- 写入时自动处理：**L0 摘要**（~100 tok，快速判相关性）→ **L1 概览**（~2k tok，规划用）→ **L2 全文**（按需才读）
- **每个目录也有自己的 L0/L1**：可先看目录摘要，再决定要不要钻进去

### 2. 目录递归检索（可调试）
- 向量搜索先定位最高分目录，再逐层下钻，结果自带周边上下文
- **每次检索保留完整浏览轨迹**——可观测、可调试（对比黑盒 RAG 的关键差异）

### 3. 会话自动变记忆
- session commit 后**异步**抽取用户偏好和 Agent 经验进长期记忆

## 官方基准（0.3.22，Doubao 2.0 Pro + 火山 embedding）

### LoCoMo 用户记忆

| 宿主 | 原生 | +OpenViking |
|---|---|---|
| OpenClaw | 24.2% | **82.1%** |
| Hermes | 33.4% | 82.9% |
| Claude Code | 57.2% | 80.3% |

- 同时输入 token **降 34–91%**，查询延迟降 58–66%
- tau2-bench 任务成功率：零售 +6.87pp、航空 +11.87pp

## 生态

- 官方集成 10+ 工具：Claude Code / Codex / Cursor / Trae / OpenCode / Hermes 等
- **OpenViking Helper** 桌面控制台：macOS + Windows x64（beta），一键配置插件 / MCP / Hook，可看 session trace
- **VikingBot**：`pip install "openviking[bot]"` 直接起一个基于 OpenViking 的 Agent
- 在线 Studio（openviking.ai/studio）免安装试玩
- 合作方：deer-flow、NoKV、loopx、Hermes Agent

## 商业结构

- 开源版**明确不阉割**：无 feature gate、无激活
- 火山引擎托管 SaaS：个人版 50 文件免费试用
- 企业自托管：BYOC / 全离线

## 注意点

1. **AGPLv3**：个人学习 / 内部使用无碍；若未来把量化 Agent 做成对外 SaaS 并链接它，需评估 AGPL 义务（与 MIT 的 ai-memory 的关键差异）
2. 需要 LLM，但 embedding 支持 Ollama 本地（init 向导自动检测 / 拉取模型）→ **可全本地跑**

## 与已跟踪项目定位

- 与 headroom（上下文压缩）、mnemosyne（记忆）、MyContext（供料）构成完整对照系
- **OpenViking = 目前「Agent 运行时记忆基础设施」里基准数据最硬 + 工程最完整的开源选项**
- 已有对比笔记：`04-GitHub研究\Agent上下文与记忆方案对比\Headroom vs OpenViking vs TencentDB Agent Memory.md`（2026-08-23）

## 试用路径（建议）

1. 浏览器开 openviking.ai/studio，10 分钟感受 `viking://` 交互
2. 本地 `pip install openviking` + Ollama
3. 把量化项目文档 `ov add-resource` 进去
4. 验证三级加载 + 检索轨迹 + token 节省
