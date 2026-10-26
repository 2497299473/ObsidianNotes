# ai-memory 项目笔记

> 记录时间：2026-08-31 · 来源：GitHub README（一手）+ AI/TLDR release notes
> 链接：https://github.com/akitaonrails/ai-memory

## 一句话定位

给 AI 编码 Agent 的长期记忆：会话结束时把该会话**编译成 Karpathy 式 Markdown wiki**（而非回放原始日志），存进 Git 仓库，实现跨 Agent 接力——"退出 Claude Code，在同一目录启动 Codex，继续干，不用重新解释架构、失败方案和未决问题"。

## 基本信息

| 项 | 值 |
|---|---|
| 语言 | Rust（要求 Rust 1.95+） |
| 协议 | MIT |
| Stars | ~3.3k（2026-08-19） |
| 版本 | v1.29.0（2026-08-19，迭代极快） |
| 热度 | 2026-08-18 曾登 GitHub Rust 日榜 #1 |

## 核心机制

### 1. 会话结束 → 编译 wiki（不是日志回放）
- lifecycle hooks 捕获提示词、工具调用、会话边界
- session-end 时 LLM 把该会话的观察**编译成 wiki 页面**，写入记忆仓库
- 每个页面有 supersession 版本链：`checkpoints` / `restore-page` / `git log` 时间旅行

### 2. 纯 Markdown + Git 是唯一事实源
- 可 grep、可 rsync 备份、**可直接用 Obsidian 打开**
- SQLite FTS5 只是索引（可重建），没有必须维护的向量库（向量检索是可选项）

### 3. 混合检索（RRF 融合）
- FTS5 全文 + 实体匹配（每页 frontmatter 声明 ≤10 个规范名词）+ 图邻居排序 + 可选向量
- `_rules/`、`decisions/`、`gotchas/` 等维护页在截断前获得权威性加权
- 向量 provider 可选：OpenAI / Voyage / Gemini / Ollama 等

### 4. 跨 Agent 接力（主卖点）
- 支持 12+ 工具：Claude Code / Codex / Cursor / Gemini CLI / OpenCode / Kimi / Kiro / Antigravity / Grok Build / OpenClaw 等
- `ai-memory run claude` 干一半 → 退出 → `ai-memory run codex` 在**同一逻辑工作流**继续
- 下一个 Agent 在 SessionStart 收到「where you left off」交接块（未决问题 + 下一步 + 会话摘要）

### 5. LLM 完全可选
- zero-LLM 模式：FTS5 + 规则式摘要仍可用
- 配 LLM 才有：页面整合、矛盾 lint、auto-improve（后台把新会话提炼成 wiki 改进提案，可设人工审批）

### 6. 架构与工程细节
- 薄客户端 CLI：一切操作走 HTTP server，server 是唯一事实源（可部署 LAN/VPN/云 → 团队共享记忆）
- 每仓库捕获排除：`[capture] ignore_paths`，敏感文件不进记忆
- 页面可设 `expires_at` TTL 自动遗忘
- `bootstrap`：把已有项目的 git log / README / docs 一次性总结成种子页面
- 内置 `/web` 只读浏览器：项目列表、文件树、FTS5 搜索、Markdown 渲染

## 已知坑（README 自述）

1. **bootstrap 幻觉**：首次总结时 LLM 可能编造"似是而非"的页面 → 靠 git diff 审查 + 回滚兜底
2. **部分 harness 无真正 session-end hook**（Codex / Kiro / Pool 等）→ 需手动 `finalize-session`
3. **Windows 原生是 Experimental**，WSL2 是正式支持路径

## 与已跟踪项目对照

| 维度 | **ai-memory** | **mnemosyne** | **MyContext** |
|---|---|---|---|
| 存储形态 | Markdown + Git + SQLite FTS5 | 单 SQLite | 本地 SQLite（版本化迁移） |
| 人可读性 | ⭐⭐⭐⭐⭐（= Obsidian） | ⭐⭐（程序优先） | ⭐⭐⭐ |
| 公开基准 | 无 | LongMemEval Recall@5 98.9% | 无 |
| LLM 依赖 | 可选 | 需要 | 需要 |
| 协议 | MIT | MIT | Elastic License 2.0 |
| 一句话 | 给 Agent 用的 Obsidian | 给 Agent 用的数据库 | 工作上下文供料 |

## 试用路径（建议）

1. WSL2 里装 release（release 二进制或 cargo install）
2. 挑一个量化项目（如 `quant_test`）跑 `ai-memory bootstrap`
3. 用任一已支持 Agent 跑一场真实会话
4. 重点验证：bootstrap 产出页面是否编造（git diff 审查）、跨工具接力交接块质量

## 参考

- 仓库：https://github.com/akitaonrails/ai-memory
- 官网：https://aimemory.sh
- v1.29.0 release 解读：https://ai-tldr.dev/releases/akitaonrails-ai-memory-1-29-0

---

## 2026-10-26 复核（最新 README，v3.0 代）

> 来源：GitHub README（main 分支实拉）+ GitHub API + 本机环境对照。原笔记（08-31）内容保留，以下为新版本增量与重新裁决。

### 数字更新（API 实拉）

- Stars：3.3k（08-19）→ **13,538**（10-26），forks 959，open issues 15，语言 Rust，MIT
- 版本：README 已含 `docs/upgrading-to-3.0.md`，docs 提及 2.4 代特性（per-tier 衰减曲线、DBSCAN 冷簇去重、访问强化、belief-strength 置信）→ 当前 **v3.x 代**

### 相对旧笔记的新增/演进

1. **支持矩阵扩到 20+ harness**：新增 Qwen Code / Cline（Hooks+MCP）/ IBM Bob / DSH（Hooks+MCP）/ Hermes Agent / GrizzyBot / FreeBuff / Prime Agent 等；Claude Desktop / Zed / VS Code Copilot 为 MCP-only
2. **handoff 协议升级为 typed + claim-once**：每个 handoff 有 owner、恰好被认领一次，防多 Agent 双抢
3. **`ai-memory run <harness>` 会话投影**：接力时下一个 Agent 除共享 workstream 外，还获得上一会话可见原生历史的**私有临时本地投影**（精确细节不受启动包截断）；远端服务器只收 harness/会话身份，**不收本地 transcript 路径与正文**
4. **跨项目 profile**：你的工作方式（包管理器/测试布局/架构习惯）到每个新仓库自动生效，位于仓库规则文件之下的默认层；单用户默认开，共享 server 上 opt-in 且按人私有
5. **零 LLM 记忆老化**（默认路径，无需 API key）：per-tier 衰减曲线 + 访问加权（越常用衰减越慢）+ 冷笔记抽取式压缩（只留文件路径/错误码/决策等持久事实）+ 近重复合并 + 矛盾标记；**supersede-not-delete + `restore-page` 可恢复，改写类功能默认全关**
6. **opt-in LLM "dream"**：空闲时把冷笔记簇改写成连贯页面，用户一回来即取消；永不删源；默认关，要过 recall eval 才可能成默认
7. **多用户团队**：按项目共享知识、个人 handoff 私有、归属 + 审计日志内置**非付费档**；可选 server 渲染管理控制台（编译期 + 运行期双 opt-in）
8. 其他：wiki 原生 **OKF v0.2** bundle、SSO OIDC device-auth、airgapped 离线安装、TypeScript SDK、桌面安装器（unsigned preview）

### 本机环境裁决（2026-10-26）

**决定：不装，继续跟踪。** 三个理由 + 一个前提：

1. 主力 harness 是 **OpenSquilla，不在支持矩阵**（顶多 MCP 连接查询；hooks 静默自动捕获是 harness 级集成，接不上——核心卖点打对折）
2. 单机单用户（QuantV1 + 笔记库）：团队 server / 多用户归属 / 审计日志均用不上
3. MiMo Code 2026-10-08 已卸载，无「Claude Code 干一半 → Codex 接手」跨 CLI 场景
4. Windows 原生仍 **Experimental**（WSL2 才是正式路径）

**重估触发条件**：将来确实需要多 coding CLI 接力、或同一仓库跨两台机器 → 与 Mnemosyne 并排 A/B（本机 Windows 环境 Mnemosyne 走 pip 原生更顺，ai-memory 需 WSL2）。

### Mnemosyne vs ai-memory（设计哲学对照）

| | Mnemosyne | ai-memory |
|---|---|---|
| 第一优先级 | **召回质量**（单 SQLite + MIB 48 字节向量 + FTS5 混合评分，LongMemEval Recall@All@5 98.9%） | **文件主权 + 跨 agent 接力**（git markdown wiki 是唯一事实源，DB 只是可随时重建的派生索引） |
| LLM 依赖 | 需要 embeddings | 默认零 LLM（capture/search/handoff 全免 API key），LLM 功能全 opt-in |
| 差异化 | Hermes-first、任意 MCP、Fernet/SecretBox 加密同步 | claim-once 类型化 handoff、会话投影、团队 server、零 LLM 记忆老化 |
| Windows | pip 原生 | 原生 Experimental / WSL2 正式 |
