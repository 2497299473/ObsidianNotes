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
- v1.29.0 release 解读：https://ai-tldr.dev/releases/akitaonrails-ai-memory-1-29-0
