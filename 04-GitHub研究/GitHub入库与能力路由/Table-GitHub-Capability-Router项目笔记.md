---
lark_doc_url: https://my.feishu.cn/docx/XMCYdfnwwovIzQxE8G6ccpkmn4g
lark_doc_token: XMCYdfnwwovIzQxE8G6ccpkmn4g
---
# Table-GitHub-Capability-Router 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/duoduoler-ops/Table-GitHub-Capability-Router
> - **协议**：MIT
> - **定位**：Markdown-first 的 AI 知识库工作流模板（不是软件/CLI/服务）
> - **核心场景**：Obsidian + coding agent（Codex / Claude Code）
> - **记录日期**：2026-07-24

---

## 一句话定位

**一套 Markdown 模板 + SOP + 提示词，让 AI Agent 自动帮你做 GitHub 项目入库和工具能力管理。** 零安装，Markdown 即工作流。给 Agent 一张极薄的地图，一次只开一个抽屉。

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **GitHub 项目入库难** | 收藏了一堆 repo，过段时间忘记为什么收藏、怎么用 |
| **工具泛滥** | Skill、Plugin、MCP、CLI 越装越多，全挤进上下文互相抢触发，误触发、调用不稳定 |
| **上下文浪费** | Agent 经常扫描整库，token 消耗大，且容易选错工具 |
| **限制条件被截断** | 客户端为省 token 裁剪暴露文本时，放在末尾的"不要用于…"可能被悄悄删掉 |
| **总管型能力失控** | 某些启动型宽触发 Hook 会抢占第一层路由，不因包名放行整包 |

---

## 🏗️ 核心机制

### 1. 两条入库流水线

#### GitHub 项目入库

```
URL → 收件箱 → 读 SOP → 30秒定位 → 重复筛查 → 项目卡 → 能力槽对比 → 提炼可复用知识 → 冷库 manifest → 索引+日志
```

**产出**：
- 项目卡（为什么值得保留/否决）
- 提炼页（抽出了什么可复用方法）
- reference manifest

#### 已有能力筛查

```
Skill/Plugin/MCP/脚本/CLI → 盘点 → 来源/作用域/权限/重复组检查 → 状态分级 → 冷库 manifest → 薄 Registry 判断
```

**能力状态分为五级**：`active` / `cold` / `disabled` / `reference` / `retired`

### 2. 分层路由（最核心设计）

```
L0 客户端原生可见性策略 + 常驻规则短指针
→ L1 一级薄路由：任务分类，每项截断安全一句话
→ L2 Registry 行：触发与禁用条件
→ L3 能力卡：单个能力完整说明
→ L4 能力本体：启用时才进入上下文
```

> [!important] 核心理念
> **"先给 Agent 一张极薄的地图，一次只开一个抽屉"**，而不是把所有工具摊在桌上让 Agent 自己挑。Agent 从不扫描整库。

### 3. 可执行能力调用流程

```
Need Gate（是否需要额外能力）
→ Search（搜索 1-3 候选，含"不调用额外工具"）
→ Describe（读取 Top-1 详情）
→ Preflight（检查健康/权限/风险/回滚）
→ Execute（执行最小有用步骤）
→ Verify（验证结果）
→ Write Back（只写回有意义的状态变化）
→ Recycle（把临时启用的能力恢复到任务前状态）
```

### 4. 关键设计理念

| 理念 | 说明 |
|------|------|
| **截断安全** | 所有暴露给 agent 的路由入口，用途和限制前置，防止预算裁剪丢掉"不要用于…"那半句 |
| **薄路由 = 软决策层** | L1 只是软引导，不会自动覆盖原生发现。要真正接通持久路由，需用户手动在 `AGENTS.md`/`CLAUDE.md` 加一条短指令指向 `level1-router.md` |
| **总管型能力拆包治理** | 识别启动型宽触发、SessionStart Hook 注入这类抢占第一层路由的能力，不因包名放行整包 |
| **用户主权** | 不自动修改 Agent 配置，所有变更需用户确认 |

---

## 📁 项目文件结构

| 文件/目录 | 用途 |
|-----------|------|
| `QUICKSTART.zh-CN.md` | 中文 5 分钟快速入口 ⭐ |
| `docs/context-and-routing.md` | 第一性原理：Agent 看到什么、截断机制、路由权 |
| `docs/how-it-works.md` | 端到端工作流说明 |
| `docs/customization.md` | 自定义指南 |
| `docs/privacy-and-sanitization.md` | 隐私与脱敏 |
| `docs/optional-capability-optimizer.md` | 可选跨客户端审计 Skill |
| `templates/capability-router.md` | 薄路由模板 |
| `templates/github-project-card.md` | GitHub 项目评价卡模板 |
| `templates/capability-manifest.md` | 能力冷库 manifest 模板 |
| `prompts/github-intake-prompt.md` | 复制即用的提示词 |
| `examples/sanitized-demo/` | 虚构、可公开的演示输出 |
| `integrations/optimize-agent-capabilities/` | 可选跨客户端能力审计入口 |

---

## ✅ 优点

- **直击真实痛点**：AI agent 时代"工具越来越多但管理跟不上"
- **理念成熟**：截断安全、分层渐进、Need Gate + Preflight、总管型能力拆包，说明作者对 Codex/Claude Code 的上下文机制有真实理解
- **零安装、不绑客户端**：Markdown-first，适配主流 coding agent
- **隐私意识强**：连发布前脱敏扫描命令都给了，区分"真实 vault"和"公开 starter kit"
- **双语友好**：中英双语 README，有专门中文快速开始

---

## ⚠️ 潜在局限

| 方面 | 说明 |
|------|------|
| **不是开箱即用软件** | 本质是模板+规则，需复制到自己的 vault、替换路径、配置 agent 规则，初始搭建有认知成本 |
| **依赖 Agent 执行质量** | 路由效果高度依赖 agent 对 `AGENTS.md`/`CLAUDE.md` 指令的遵守程度，不同客户端表现有差异 |
| **L1 只是软决策层** | 未配合客户端禁用/隐藏/显式调用设置时，不能强制限制 Agent |
| **需要持续维护** | 长期运行需持续维护 Registry、能力卡、薄路由等多个文件，否则容易变成另一个"收藏夹坟墓" |
| **适用场景偏窄** | 主要针对重度 AI 工具用户和知识工作者 |
| **强依赖 Obsidian** | 不用 Obsidian 的人需要自己适配知识库载体 |

---

## 🧩 适合谁用

- 🔧 **重度 AI Agent 用户**：同时用多个 Skill/Plugin/MCP，感觉上下文混乱、工具互相抢触发
- 📚 **知识工作者**：想把 GitHub 收藏和项目笔记系统化，而不是一直扔在临时上下文里
- 🗂️ **Obsidian + Codex/Claude Code 用户**：这是项目的核心场景
- 🎛️ **想掌控"路由权"的人**：不希望 Agent 自主决定用什么工具，而是由自己定义决策路径

---

## 🚀 如何使用（上手步骤）

> [!tip] 建议按以下顺序逐步进行，不要一次吞完

### 第一步：快速阅读

1. 克隆仓库到本地
   ```bash
   git clone https://github.com/duoduoler-ops/Table-GitHub-Capability-Router.git
   ```
2. 先读 `QUICKSTART.zh-CN.md`，不要一次啃完整 README

### 第二步：复制模板到 Vault

1. 把 `templates/` 目录下的模板文件复制到你的 Obsidian vault
2. 把 `docs/` 目录下的文档复制到 vault（可作为参考文档）
3. 替换模板里的占位符路径（`{{VAULT_PATH}}`、`{{CAPABILITY_LIBRARY}}` 等）

### 第三步：跑通第一个项目入库

1. 从最近收藏的 1-2 个 GitHub 项目开始
2. 使用 `prompts/github-intake-prompt.md` 中的提示词
3. 让 Agent 执行完整的入库流程
4. 检查生成的项目卡和提炼页

### 第四步：接入薄路由

1. 在 `AGENTS.md` 或 `CLAUDE.md` 中添加一条指向 `level1-router.md` 的规则
2. 观察 Agent 是否按照薄路由的分层路径选择工具
3. 根据实际效果调参

### 第五步：扩展能力管理

1. 盘点你现有的 Skill/Plugin/MCP/CLI
2. 按模板填写能力 manifest
3. 状态分级（active / cold / disabled / reference / retired）
4. 逐步将能力纳入薄路由 Registry

### 第六步（可选）：跨客户端审计

1. 查看 `integrations/optimize-agent-capabilities/`
2. 使用可选的跨客户端能力审计 Skill
3. 统一管理不同 Agent 客户端的能力

---

## 📌 后续研究清单

> [!todo] 等有时间后逐项研究
> - [ ] 读 `docs/context-and-routing.md`，理解第一性原理
> - [ ] 读 `docs/how-it-works.md`，理解端到端工作流
> - [ ] 实际跑通一个 GitHub 项目入库流程
> - [ ] 尝试接入薄路由到自己的 `CLAUDE.md`
> - [ ] 盘点现有 AI 工具，做能力 manifest
> - [ ] 研究"截断安全"机制在 Obsidian 中的具体实现
> - [ ] 研究隐私与脱敏流程
> - [ ] 考虑是否需要跨客户端审计

---

## 💡 个人思考

这个项目的本质是一个**工作流方法论**，而不是一个工具。它解决的是 AI Agent 时代一个非常前沿的问题：**当你的 AI 助手装了太多 Skill 和 MCP 之后，谁来管理这些能力？** 作者的答案是：**你来管，Agent 来执行**。

它跟 Prompt 工程、Skill 设计不同——它不是教你写更好的提示词，而是教你设计一套**路由系统**，让 Agent 在正确的时间、用正确的工具、做正确的事。

当前 vault 已有 `AI协作方法论` 和 `Claude的使用` 等目录，这个项目正好可以作为"方法论落地"的实践工具，把之前积累的方法论真正跑起来。

> [!quote] 一句话总结
> 适合已经感受到"工具泛滥"痛点、愿意花时间搭工作台的人；不适合想要一键安装、立即生效的被动型用户。

> 状态：✅ 已分析，⏳ 待实践
