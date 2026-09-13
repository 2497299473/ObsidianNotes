---
lark_doc_token: UilcdVqvyoLuvDxz70jcB7LCnRf
lark_doc_url: https://my.feishu.cn/docx/UilcdVqvyoLuvDxz70jcB7LCnRf
---
# CodeGraph 项目笔记

> 创建日期：2026-08-10
> 仓库：[colbymchenry/codegraph](https://github.com/colbymchenry/codegraph)
> 资源：[getcodegraph.com](https://getcodegraph.com)（托管版 CodeGraph Platform，PR 影响分析，waitlist 中）
> 状态：开源 MIT，已关注 🔥（2026 年 5 月底 GitHub Trending 黑马）

---

## 项目概述

**CodeGraph** 是面向 AI 编程工具（Claude Code、Cursor、Codex、Gemini CLI、opencode 等）的**本地代码知识图谱预索引工具**。核心思路：先把整个代码库解析成一张知识图谱（符号、函数、类、调用边、依赖关系全部预索引），让 AI Agent 直接「查图」而不是逐个文件 grep/read——把探索过程从「读很多文件」变成「问一次图谱」，官方自称「外科手术式上下文」（surgical context）。

一句话定位：**给 AI 编程助手装一层「代码地图」**。

- **内核**：Rust（快速完整结构抽取 + 跨文件解析）
- **许可证**：MIT
- **形态**：自带运行时的一键安装脚本 / npm 包（`@colbymchenry/codegraph`），无需本机 Node.js
- **隐私**：100% 本地运行，代码不上传任何服务器
- **语言支持**：34 种（TS/JS/ArkTS/Python/Go/Rust/Java/C#/PHP/Ruby/C/C++/Swift/Kotlin/Scala/Dart/Svelte/Vue/Lua/Solidity/Terraform/Nix/COBOL 等），零配置按扩展名自动识别，自动遵循 `.gitignore`
- **适配 Agent**：Claude Code、Cursor、Codex CLI、Gemini CLI、opencode、Hermes Agent、Antigravity IDE、Kiro、GitHub Copilot（VS Code / Copilot CLI / JetBrains）

⚠️ GitHub 上同名项目很多，注意区分（见下文「与其他项目的区分」）。

---

## 核心指标

| 指标 | 数值 |
|------|------|
| Star | 65.7k |
| Fork | 4.1k |
| Watchers | 149 |
| 许可证 | MIT |
| 内核 | Rust（自带运行时，无需 Node.js） |
| 支持语言 | 34 种 |
| 适配 Agent | 9 种主流 AI 编程工具 |
| 平台 | Windows / macOS / Linux |
| 热度 | 2026 年 5 月底一周 +14k stars，GitHub Trending #2 |

---

## 安装与使用

```bash
# macOS / Linux 一键脚本（无需 Node）
curl -fsSL https://raw.githubusercontent.com/colbymchenry/codegraph/main/install.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/colbymchenry/codegraph/main/install.ps1 | iex

# 或用 npm（任意 Node 版本）
npm i -g @colbymchenry/codegraph
```

装完分三步：

```bash
# 1. 接入 AI 工具：自动检测并配置 Claude Code/Cursor/Codex/opencode 等的 MCP server
codegraph install

# 2. 进入项目初始化索引（创建 .codegraph/ 并全量建图）
cd your-project && codegraph init

# 3. 之后不用管——文件监听器自动增量同步，索引永不陈旧
```

其他命令：`codegraph upgrade`（自动检测安装方式原地升级）、`codegraph uninstall`（从所有 Agent 移除，`--keep-cli` 保留 CLI）、`codegraph uninit`（移除项目索引）。

也可以作为库嵌入（二次开发 / CI 影响分析）：

```js
import CodeGraph from '@colbymchenry/codegraph';
const cg = await CodeGraph.init('/path/to/project');
await cg.indexAll({ onProgress: (p) => console.log(`${p.phase}: ${p.current}/${p.total}`) });
const results = cg.searchNodes('UserService');
const callers = cg.getCallers(results.node.id);
const context = await cg.buildContext('fix login bug', { maxNodes: 20, includeCode: true, format: 'markdown' });
cg.watch(); // 自动同步文件变化
```

---

## 关键特性

- 🗺️ **预索引知识图谱**：每个符号、调用边、依赖关系进图，含 grep 追不上的动态分派（dynamic-dispatch）调用跳转
- 🎯 **外科手术式上下文**：Agent 一次调用拿到「相关源码 + 符号间调用路径 + 改动爆炸半径（blast radius）」，而非逐文件搜索
- 📡 **自动增量同步**：文件一改图就更新，Agent 改代码、增删文件都不用重跑
- 🔒 **100% 本地**：无外部服务依赖，代码零上传
- 📦 **自带运行时**：无需编译、无原生构建
- 🧩 **可作库使用**：searchNodes / getCallers / getImpactRadius / buildContext，便于接入自定义脚本、CI、二次开发

---

## 效果数据

**官方 2026-08-05 复测**（Claude Opus 4.8，7 个真实开源仓库含 VS Code ~11k 文件，每臂中位数 4 次；对照臂屏蔽 CLI 防污染）：

- **通用优势（所有仓库、所有规模）**：工具调用 −88% · 速度 +53% · Token −62% · 成本 −44% · 文件读取归零
- 无图谱时 Agent 最多烧 43 次工具调用 + 19 次文件读取「重新推导结构」；窄问题也有 +35% 提速，宽问题最高 3.6×
- 2026-08 成本复测：成本取决于问题所需的「探索量」而非仓库体积——问 28–43 次工具才答得上的问题省 57–78%，Agent 7 次调用就答的问题几乎打平

**第三方实测**（2026 年 5 月底，4 个真实仓库一周）：工具调用中位数 −70%、Token −59%、响应时间 −49%、费用 −35%（「I Gave Claude Code a Map of My Repo」作者从怀疑到基本被打动）。

### ⚠️ 官方自曝的另一面（诚实数据）

以上数字衡量的是**吞吐**（处理多少 token / 调用 / 花多少钱），**不计上下文残留**。CodeGraph 回答是「一次密集、逐字返回的载荷」，答完仍常驻上下文窗口：多轮会话结束时残留的检索上下文比纯读文件 Agent 多约 **80%**（VS Code 仓库 67k vs 18k token）。即**处理的 token 更少、但持久占用的上下文更大**——小窗口长会话要为此做预算（README 提供 `docs/benchmarks/residual-context-occupancy.md` 细测数据）。

### 适用边界

- ✅ 大仓库（上千文件）、多语言混合工程：调用链、路由、测试影响面查询价值最大
- ✅ 在意隐私/延迟、不想把源码交给外部服务
- ✅ 想把「代码影响分析」做成脚本/自动化（库 API）
- ⚠️ Agent 反正要读文件的小场景下，CodeGraph 反而变成额外开销

---

## 与其他项目的区分 ⚠️

GitHub 上多个同名/近似项目：

1. **colbymchenry/codegraph**（⭐65.7k，本笔记主角）——预索引代码知识图谱，Rust 内核，面向 AI 编程 Agent 的上下文优化
2. **codegraph-ai/CodeGraph**（Apache 2.0）——42 个 MCP 工具 + 38 语言 tree-sitter 解析 + VS Code/JetBrains 插件 + 持久记忆层，偏 IDE/插件生态
3. **CodeGraphContext/CodeGraphContext**（⭐4.0k）——MCP server + CLI，文件/符号/调用/继承/导入关系图供 AI 查询
4. **FalkorDB/code-graph**——GraphRAG-SDK + FalkorDB 的代码知识图谱可视化 demo（Web UI + CLI `cgraph`，Python/Java/C#）

> 注：本库「Agent上下文与记忆方案对比」「TencentDB-Agent-Memory」笔记里提到的 CodeGraph 是**资产类型概念**（符号/调用/影响路径的预索引图谱），与本文开源项目是同一思路的不同落地，可互相参照。

---

## 与你相关

| 领域 | 关联度 | 说明 |
|------|--------|------|
| AI 研究 | 🔥 强相关 | 「预索引图谱而非让模型硬读代码」是上下文工程前沿，与 headroom（可逆压缩）、book-to-skill（按需切片）同一思路谱系 |
| 全栈开发 | 🔥 强相关 | 大仓库问「认证链路」「改 X 会影响谁」，直接查图比翻文件快一个量级 |
| DevOps | ⚡ 间接 | SQLite + FTS5 + 文件监听器的本地索引架构可借鉴；CI PR 影响分析可脚本化 |
| 量化交易 | ⚡ 间接 | Funder 项目代码量小收益有限；若扩展成多策略大工程、用 AI Agent 维护时可复用 |

---

## 社区与资源

- GitHub：[colbymchenry/codegraph](https://github.com/colbymchenry/codegraph)
- 作者 X：[getcodegraph](https://x.com/getcodegraph)
- 托管版（CodeGraph Platform，每个 PR 知道测什么/什么会挂/哪些流程受影响）：[getcodegraph.com](https://getcodegraph.com)（waitlist 中）
- 官方基准：`docs/benchmarks/residual-context-occupancy.md`
- 第三方解读：[腾讯云 CodeGraph 实战](https://developer.cloud.tencent.com/article/2696888)、[博客园 省35%费用/减70%工具调用](https://www.cnblogs.com/itech/p/20125317)
- 独立实测：[I Gave Claude Code a Map of My Repo — CodeGraph Killed 70% of Its Tool Calls](https://levelup.gitconnected.com/i-gave-claude-code-a-map-of-my-repo-codegraph-killed-70-of-its-tool-calls-9a7f8400a97d)

---

## 探索路线

1. `npm i -g @colbymchenry/codegraph` → `codegraph install` → 找个大仓库 `codegraph init` 跑起来
2. 用 Claude Code 问架构问题，对比装与不装的工具调用次数（验证 70% 削减）
3. 试试库 API（getCallers / getImpactRadius / buildContext）给 Funder 或大项目加影响分析
4. 关注 CodeGraph Platform（PR 影响分析）waitlist 开放

---

## 日志

- 2026-08-10：首次了解 CodeGraph，建立笔记（colbymchenry/codegraph，65.7k stars，MIT，Rust 内核；已核实官方 2026-08-05 基准 + README 自曝的上下文残留代价；记录同名项目区分）
