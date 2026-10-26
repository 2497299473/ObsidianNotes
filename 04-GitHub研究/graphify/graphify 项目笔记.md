# graphify 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/Graphify-Labs/graphify
> - **官网 / 文档**：https://graphify.com / https://docs.graphify.com
> - **协议**：Apache-2.0
> - **主要语言**：Python（tree-sitter AST + Leiden 社区检测）
> - **Stars / Forks**：135.3k / 12.9k（2026-10-26 GitHub API 实拉）
> - **创建 / 最近推送**：2026-04-03 / 2026-10-23（活跃，默认分支 v8，open issues 1637）
> - **Topics**：knowledge-graph、graphrag、tree-sitter、leiden、claude-code、codex、cursor、mcp、skills
> - **背景**：YC S26（Graphify-Labs）
> - **定位**：把代码库 + 文档 + SQL schema + 配置 + PDF + 图片/视频变成**可查询的知识图谱**（代码本地确定性 AST 解析、每条边有解释、无向量库）
> - **记录日期**：2026-10-26

---

## 一句话定位

**在 AI 编码助手里敲 `/graphify`，把整个项目（代码、docs、PDF、图片、视频）映射成一张可查询的知识图谱——用图遍历（query / path / explain）替代 grep 翻文件。代码部分 tree-sitter 本地 AST 确定性解析（零 LLM、不出网），每条边带 EXTRACTED/INFERRED 置信标记，不是向量索引。**

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **Agent 读代码烧 token** | 每轮会话重新 grep/读文件解释代码库；官方口径是砍 80% token（Cloud 叙事，本地版未实证） |
| **向量 RAG 答不了关系** | "A 怎么连到 B"需要路径，embedding 相似度给不出 hop 链路 |
| **代码与文档两套世界** | 研究笔记/PDF/ADR 与代码符号无法互查 |
| **架构全貌不可见** | 调用关系、子系统边界只能靠人脑拼 |

---

## 🏗️ 核心架构

```
┌──────────────────────────────────────────────────────────┐
│  提取（两条通道）                                          │
│  ① 代码：tree-sitter AST 本地确定性解析（~40 语言）          │
│     零 LLM、零网络、零 API key                              │
│  ② docs/PDF/图片/视频：语义通道（调助手模型或配置的 API key） │
├──────────────────────────────────────────────────────────┤
│  图模型                                                    │
│  节点：符号 / 文件 / 概念 / rationale 注释（# NOTE/# WHY、   │
│        ADR/RFC 引用是一等节点）                             │
│  边：calls / imports / inherits / mixes_in / uses /        │
│      references，每条标 EXTRACTED（源码显式）或             │
│      INFERRED（解析推断，带置信度）                          │
├──────────────────────────────────────────────────────────┤
│  分析：Leiden 社区检测（LLM-free）→ 子系统划分 + god nodes   │
├──────────────────────────────────────────────────────────┤
│  输出：graphify-out/                                        │
│  ├── graph.html       浏览器点开节点、滤社区、搜索            │
│  ├── GRAPH_REPORT.md  亮点报告（关键概念/意外连接/建议问题）   │
│  └── graph.json       全图，随时查询不用重读文件              │
└──────────────────────────────────────────────────────────┘
```

**不是向量索引**：无 embedding、无 ANN，纯图遍历。查询命令（本地 CLI，均可独立调用）：
- `graphify query "问题"` — BFS 子图（支持 jieba 中文分词 `[chinese]`）
- `graphify path "A" "B"` — 最短路径逐 hop 展示
- `graphify explain "X"` — 节点 + 邻居白话解释
- `graphify god-nodes` / `affected "X"` — 架构枢纽 / 反向影响面
- `graphify export html|obsidian|wiki|svg|neo4j|graphml` — 多格式导出（`export obsidian` 直接出 vault 笔记 + canvas）

**增量与同步**：manifest 增量（`/graphify . --update` 只重提变更文件，无 LLM）；`[watch]` 文件监听自动重建；`graphify hook install` 挂 post-commit/post-checkout git hook 自动重建 + union-merge 两个 graph.json（merge-driver）；`merge-graphs` 跨仓库合并图。

**扩展 extra（按需装）**：`[pdf]` `[office]` `[sql]`（SQL schema 提取 + PostgreSQL 实时内省）`[postgres]` `[chinese]`（jieba）`[watch]` `[leiden]` `[neo4j]` `[falkordb]` `[svg]` `[video]`（faster-whisper + yt-dlp）`[ollama]`/`[openai]`/`[gemini]`/`[anthropic]`/`[bedrock]`/`[azure]` 等本地/云端后端、`[terraform]`/`[pascal]`/`[ocaml]`/`[commonlisp]`/`[robot]`/`[dm]` 等语言提取器。

---

## 📊 基准（⚠️ 全部自报，打折看）

| 基准 | graphify | 对照 |
|------|----------|------|
| LOCOMO (n=300) | recall@10 **0.497**，QA 45.3% | mem0 0.048 / supermemory 0.149（QA 49.7% / 27.3%） |
| LongMemEval-S (n=50) | QA **76%** | 与 dense RAG 打平 |
| ERPNext 跨工具 (n=6) | 关键事实覆盖 **82.0%** | grep/read 基线 70.8% |
| 代码建图 LLM 成本 | **0 credit** | 多数系统按 token |

> ⚠️ 注意：LOCOMO / LongMemEval 是**对话记忆基准**——它拿这两个出来，说明公司在做「agent 记忆层」叙事，这就是它与 Mnemosyne / ai-memory 地盘沾边的原因（本地版核心仍是代码图谱，记忆是 Cloud 的 always-on 卖点）。基准出自同一 harness + 双 judge 盲评（kappa 0.81），但数据主体仍是厂商自测。

---

## ⚙️ 安装（本机已装，2026-10-26）

- 要求 Python 3.10+；`uv tool install "graphifyy[leiden,chinese,sql,watch]"`（**PyPI 包名 `graphifyy` 双 y**，其它 `graphify*` 包均非官方；本机 uv 已备，73 个包 ~30s 装完）
- `graphify install [--platform P]` 一键注册助手 skill（Claude Code / Cursor / Codex / Gemini CLI / OpenCode / Copilot / Kimi / Amp / Kiro / Pi / Devin / Antigravity / `agents` 通用目录等 20+ 平台）
- **OpenSquilla 兼容性（已核实）**：
  - `--platform agents` 写 `~/.agents/skills/`，但 **OpenSquilla 不自动加载该目录**（本机 `tabbit` 已置于该目录却未出现在 available skills，反例实证）
  - **主路径 = CLI 独立使用**：`graphify query/path/explain` 等经 exec_command 直接调用，零依赖 skill 注册
  - 若想要自动发现：把其 SKILL.md 拷入 `C:\Users\Turn-\.opensquilla\skills\`（nihaixia 同目录，加载正常）
  - 查询日志默认写 `~/.cache/graphify-queries.log`（JSON Lines，`GRAPHIFY_QUERY_LOG_DISABLE=1` 可关）

---

## ✅ QuantV1 实测（2026-10-26，零网络零 LLM 全流程）

| 步骤 | 命令 | 结果 |
|------|------|------|
| 建图 | `graphify extract D:\PythonProject\QuantV1 --code-only` | 366 代码文件 → **5200 节点 / 11135 边 / 353 社区**；52 个 docs 跳过（无语义提取）；16 worker |
| 报告 | `graphify cluster-only --no-label` | GRAPH_REPORT.md + graph.json + graph.html；5200>5000 自动聚合为社区视图（353 社区节点 / 1114 跨社区边） |
| 质量 | — | **98% EXTRACTED / 2% INFERRED**（218 条，平均置信 0.86）；token cost 0 input/0 output |
| 新鲜度 | — | built from commit `89e1ff1e`；改代码后 `graphify update .`（零 API 成本） |

**god-nodes top（与 QuantV1 实际架构核对一致）**：
1. `resolve_samples()` 57 边 · 2. `rank_ic()` 55 · 3. `load_registry()` 49 · 4. `load_samples()` 47 · 5. `register_model()` 46 · 6. `Audit` 45 · 7. `get_model_entry()` 45 · 8. `split_date_oos()` 43 · 9. `bind_validation()` 42 · 10. `FetchResult` 39 · `ForecastEngine` 36 · `verify_approval()` 32
→ registry 晋升链 + 回测样本层 + V4 数据源契约，全部在图上显形。

**中文查询实测**：`graphify query "数据源降级链的失败分类是怎么实现的"`（jieba 分词）→ 直接命中 `core/datasource/{chain,registry,health,base}.py`、`run_chain()` / `SourceRegistry` / `HealthTracker` / `FetchResult` + `tests/test_datasource_chain.py` 等 61 节点（1200 token 预算截断到 38）。

**⚠️ 待办（触网，按铁律 8 先请示）**：docs/PDF 语义通道未跑（`--code-only` 跳过 52 个文档）；Obsidian 笔记库入图同理。

---

## 🧭 同赛道对比

### vs CodeGraph（colbymchenry，已跟踪）

| | CodeGraph | graphify |
|---|---|---|
| 内核 | Rust，自带运行时（免 Node） | Python + tree-sitter |
| 存储 | SQLite（符号/调用边/依赖） | `graph.json`（可选推 Neo4j/FalkorDB） |
| 覆盖 | 纯代码 34 语言 | 代码 ~40 语言 + **docs + PDF + 图片/视频 + SQL schema** |
| 增量 | 文件监听器自动增量同步 | `--update` 增量 + `[watch]` + git hook 重建 + union-merge |
| 可视化 | 无 | `graph.html` 力导向图 + 社区着色 + D3 树 + Mermaid 调用流 |
| 边置信度 | — | EXTRACTED / INFERRED 标记 |
| 基准 | 官方（Opus 4.8，7 仓库）：工具调用 −88%、token −62%、成本 −44%、速度 +53%、文件读取归零；代价：多轮会话末上下文残留 +80% | 自报 recall@10 0.497 等（见上表） |
| 规模 | 65.7k stars，MIT | 135.3k stars，Apache-2.0，YC S26 |

**结论：互补，不是替换。** CodeGraph 强在纯代码增量 + 编码工具深度集成；graphify 强在「代码 + docs/PDF 同一张图 + 可视化 + 中文查询 + 论文/视频入图」。本机目前 graphify 已装、CodeGraph 未装；两者并存不冲突（不同存储、不同目录）。

### vs Mnemosyne / ai-memory（记忆层）

- 本地 graphify = **代码 + 语料地图**，不是会话记忆系统，与两者无直接功能冲突
- 但 LOCOMO 基准 + Cloud「always-on memory」叙事表明公司在向记忆层扩张；Cloud 的跨仓库/形式化验证/Sentry-Jira 连接器是付费区，本地版不涉及

---

## ⚠️ 风险与注意

1. **基准全部自报**（同 harness 双 judge，但厂商主导），量化收益（如「砍 80% token」）以 Cloud 口径为主，本地版未独立实证
2. **Graphify Cloud 付费 upsell**（14 天试用）：monorepo/跨仓库/形式化验证/code review/Sentry-Jira——本地开源版已够单仓库使用，Cloud 可无视
3. **5200 节点即触 graph.html 上限**（自动转聚合社区视图；`export obsidian` 可出节点级全量）
4. **docs/媒体语义通道触网调模型**（成本 + 本项目铁律 8 约束：先请示）；代码通道严格零网络零 LLM
5. **1637 open issues**（135k stars 的副作用，热门但 issue 量大）
6. 包名 `graphifyy` 易拼错；PyPI 上其它 `graphify*` 包均非官方
7. README 有 32 种语言翻译（`docs/translations/`），营销做得足

---

## 追加实测（2026-10-26 下午）：docs 语义入图

- 路径：`extract <root> --backend deepseek`（deepseek-v4-flash @ api.deepseek.com，`GRAPHIFY_DISABLE_THINKING=1`；密钥经临时 helper 脚本注入子进程环境，不落 argv/日志）
- 成本实账：首轮 52 docs = **$0.1345**（145k in/53k out）；三轮增量重试（16/2/1 文件）= $0.059 + $0.028 + $0.016 → 合计 **≈$0.24**
- 结果：**51/52 入图**，5476 节点 / 11543 边 / 369 社区；docs 节点与代码节点正确联通（AGENTS.md「授权闸门」↔ shadow_policy.py ↔ data/promotion_prereg.json）
- ⚠️ 坑 1：**大 md 单文件恒截断**——deploy/preregistered_items.md（31.6KB）的语义 chunk 打到 max_completion_tokens，5 次重试全失败（确定性，非抖动）；shrink guard 每轮正确拦住降级写
- ⚠️ 坑 2：docs 入图会带出运行产物 md 的引用节点（`output/_pre_fix_*` 等），graphify 自动按「本 run 未派发」丢弃，无需干预
- ⚠️ 坑 3：deepseek 后端依赖 `openai` SDK，基础安装不含，需 `uv tool install "graphifyy[openai,...]" --force` 补装
- 社区命名仍为 `Community N` 占位（零网络建图的代价）；后续可 `cluster-only --label` 花几分钱补语义名
- graph.html 已用 `export html --node-limit 12000` 重建为全节点级视图（5.7MB，5476 节点）；默认 5000 上限会在 >5000 节点时降级为社区聚合圆点视图（即「只见 Community N 小圆点」的原因）

---

## 追加实测（2026-10-26 傍晚）：369 个社区补中文语义名

- 结果：**269 个有成员的社区全部中文命名**（如「项目审计检查」「验证证据装配」「Rank IC 指标」「路径预测建模」「穿透信号回测」），零 `Community N` 占位；graph.html 5479 处 `community_name` 全中文
- 成本实账：36,179 in / 15,907 out → **$0.037**（deepseek-v4-flash，与预估 $0.03 同量级）
- ⚠️ **坑 1：graphify 没有 label 语言开关**。命名 prompt 硬编码英文（`llm.py` 的 `_label_batch_with_retry`，示例 `"Order Management"`），deepseek 照抄英文。**做法：直接改 site-packages 里的 prompt 为中文指令**，示例换成「项目审计检查 / 验证证据装配」。副作用：该改动对**所有**项目生效（以后别的库建图也是中文名），升级 graphify 会丢失需重打
- ⚠️ **坑 2：`.graphify_labels.json` 是强缓存**。改了 prompt 或想重新命名，必须先删这个文件；否则 `cluster-only --label` 只打印 `--backend ignored: reusing saved labels` 然后原样返回旧名（踩了 3 次）
- ⚠️ **坑 3：`cluster-only --label` 不带 backend 会退回 `claude` CLI**（本机无 Claude CLI → `no API key found`），必须用注入 `DEEPSEEK_API_KEY` 的 helper 脚本跑
- ⚠️ **坑 4：每次 cluster-only 都会重写 graph.html**，且默认又降级回聚合视图；跑完必须再 `export html --node-limit 12000` 重建全节点视图
- 复用脚本：`D:\PythonProject\_scratch\_cluster_label.py`（从 Agent 配置取 key 注入子进程，不落 argv/日志）

---

## 🔗 相关链接

- 仓库：https://github.com/Graphify-Labs/graphify
- 文档：https://docs.graphify.com（安装/排障/命令参考/always-on 指南）
- 基准全文：https://github.com/Graphify-Labs/graphify/blob/v8/BENCHMARKS.md
- 原理：`docs/how-it-works.md`（提取管线/社区检测/置信度）· `ARCHITECTURE.md`（模块拆解/加语言）
- 关联笔记：`../CodeGraph/CodeGraph 项目笔记.md` · `../mnemosyne/mnemosyne 项目笔记.md` · `../ai-memory/ai-memory 项目笔记.md`
