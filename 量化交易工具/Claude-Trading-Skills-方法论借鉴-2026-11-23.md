# Claude-Trading-Skills 方法论借鉴

> 调研日期：2026-11-23（Mon）· 来源：github.com/tradermonty/claude-trading-skills（7.8k⭐，MIT，Python）
> 背景：本轮 7 仓库调研批次之一。结论：**数据层不装**（FMP/FINVIZ/Alpaca 全是美股 API，与 QuantV1 A 股+东财单源不匹配），**方法论三点借鉴**（本文）。
> 定位：不引入其代码，只提取可迁移的结构设计，映射到 QuantV1 与 OpenSquilla skill 体系。

## 一、workflow manifest：用 YAML 声明 skill 编排顺序 + 决策闸门

### 原实现（workflows/*.yaml，schema_version: 1）

以 `market-regime-daily.yaml`（15 分钟日检）为例，manifest 固定字段：

```yaml
schema_version: 1
id: market-regime-daily
cadence: daily                # daily / ad-hoc
estimated_minutes: 15
when_to_run: ...              # 何时跑
when_not_to_run: ...          # 反向声明（禁止用途：输出不是买卖信号）
required_skills: [market-breadth-analyzer, uptrend-analyzer, exposure-coach]
optional_skills: [market-top-detector, macro-regime-detector]   # 每步标注 API 依赖
artifacts:                    # 产物依赖图
  - id: market_breadth_report
    produced_by_step: 1
    required: true
    downstream_hints: [swing-opportunity-daily, monthly-performance-review]  # 跨工作流衔接
steps:                        # 编排主体
  - step: 4
    skill: exposure-coach
    consumes: [market_breadth_report, uptrend_report, top_risk_report]
    produces: [exposure_decision]
    decision_gate: true       # 决策闸门
    decision_question: >-     # 闸门问句（人/模型都要回答的归因问题）
      Given today's breadth ... is new swing trade risk allowed, restricted, or cash-priority?
manual_review:                # 人工复核清单（强制条目）
  - If exposure_decision is restrictive, defer running swing-opportunity-daily.
journal_destination: trader-memory-core   # 产物最终落库位置
```

设计要点：**产物（artifacts）是一等公民**——每步声明 consumes/produces，工作流之间靠 `downstream_hints` 显式衔接；`decision_gate: true` 的步骤必须回答 `decision_question` 才能继续；`manual_review` 是硬编码的人工闸门，不是建议。

### 与现状的同构关系

- QuantV1 的预注册日历（09-26 固定重估日、报告名固定）本质是 `cadence` + `when_to_run` 的手工版，但没有 manifest 化——步骤顺序、产物依赖、人工闸门都隐含在脚本调用顺序里。
- preclose-market-check（OpenSquilla skill）已经是「多步骤 + 判定输出」的形态，但编排逻辑在 SKILL.md 正文里，机器不可校验。

### 落地建议

1. QuantV1 复盘/重估流程抽一个 `workflows/quarterly-reestimate.yaml`（cadence: quarterly、固定日历日、steps=通道探针→RankIC 评估→决策闸门→报告落盘、journal_destination=Obsidian 量化交易工具），编排从「脚本调用顺序」升级为「可校验的声明」。
2. 决策闸门问句显式化：把「T+5 显著但 T+1/T+3 跨零，是否继续投用」写成 decision_question，闸门记录进报告，形成归因链。
3. 产物 `downstream_hints` 对应现有「季度重估报告 → 日频参谋读数」的下游关系，显式声明后可自动检查报告是否被下游消费。

## 二、交易记忆环：平仓 → 归因 → 规则迭代（与预注册验证同构）

### 原实现

**trade-memory-loop.yaml**（cadence: ad-hoc，每次平仓触发）：

```
1. 记录平仓结果（trader-memory-core，thesis 对象落 YAML）
2. 生成 postmortem【决策闸门】root cause 归因四选一：thesis 质量 / 执行 / 市场环境 / 随机性
3.（可选）教练：产出 next_session_operating_rules【决策闸门】接受/修改/保留/仅记录
4.（可选）回测再验证原假设（backtest-expert）
5. lessons 追加进 journal（月度复盘汇总 Top Lessons）
```

manual_review 条款直指归因纪律：「对赢利是否 thesis 驱动还是运气诚实评估；不把随机性合理化成技量或失败」。`when_not_to_run`：**盈利交易也不许跳过此环**。

**trader-memory-core**（状态机载体）：

- thesis 生命周期：`IDEA → ENTRY_READY → ACTIVE → CLOSED/INVALIDATED`，**前向唯一，不允许回退**；
- 每个 thesis 一个 YAML 文件 + JSON Schema 校验 + `_index.json` 轻量索引；
- 原子写（tempfile + os.replace）、git-tracked state/（审计轨迹）、状态目录锁 + `doctor`/`rebuild-index` 恢复命令；
- `record-lessons`（一 lesson 一段落，追加进 outcome.lessons_learned）→ `review monthly-report --month YYYY-MM` 自动汇总「Top Lessons」；
- postmortem 支持 MAE/MFE（最大不利/有利偏移），离线默认 null、可选接价格 API。

### 与现状的同构关系

- 预注册验证（PR-20260911-01：信号先注册假设→固定日历日重估→留痕）就是「thesis 注册→评估→结论」环，但目前是报告文本驱动，没有状态机与结构化落库。
- QuantV1 的通道探针/信号（RankIC 衰减、T+1/T+3 跨零）已经是事实上的「thesis 失效」案例，但失效结论散在 changelog 与 Obsidian 笔记里，没有 `INVALIDATED` 状态对象与 lessons 聚合。
- 归因四分类（信号质量/执行/环境/随机）可直接映射量化场景：因子逻辑缺陷 / 数据或执行问题（东财频控）/ 市场状态切换 / 噪声。

### 落地建议

1. 给 QuantV1 建最小版 thesis 状态机：`state/theses/*.yaml`（probe_id、注册假设、评估窗、状态、status_history、结论、lessons），前向唯一转换 + git 跟踪（QuantProject 仓库天然满足审计要求）。
2. 每个信号「退役」走 trade-memory-loop 同构环：退役报告必须回答归因四选一，lessons 进 Obsidian，月度/季度复盘自动汇总——把「最近窗衰减」这类结论变成可追溯的结构化记录而不是散落笔记。
3. `doctor`/原子写/索引校验这套文件完整性设计，对 QuantV1 现有 reports/ 目录治理直接可抄。

## 三、dual-axis-skill-reviewer：skill 质检的五类自动评分 + LLM 深审双轴

### 原实现（skills/dual-axis-skill-reviewer）

**Auto 轴（确定性，脚本评分 0-100）：**

| 维度 | 分值 | 理由 |
|---|---|---|
| Metadata & Use Case | 20 | frontmatter 与触发条件决定能否被正确调用 |
| Workflow Coverage | 25 | 缺核心段落 → 实际执行歧义 |
| Execution Safety & Reproducibility | 25 | 命令示例与路径卫生决定可复现/安全 |
| Supporting Artifacts | 10 | scripts/references/tests 是可维护性基线 |
| Test Health | 20 | 测试通过才敢自动化 |

**LLM 轴**（可选）：正确性/风险/缺失逻辑/可维护性深审，JSON schema 严格输出；最终分 = auto×weight + llm×weight（默认 0.5/0.5）。阈值政策：**90+ 生产可用；80-89 可用但需定向改进；70-79 有明显缺口；<70 视为草稿**。低于 90 强制输出改进项清单。`knowledge_only` skill（无脚本纯参考）自动豁免 script/test 评分，避免不公平惩罚。支持 `--project-root` 跨项目审查 + `--all` 批量 + `--seed` 可复现。

### 与现状的同构关系

- workspace skills 目录现有 8+ 个自制/外装 skill（hallmark、ziwei-paipan、preclose-market-check、last30days 新装、EGM 系列等），没有统一质检闸门；「skill 更新检查口径」只查同步与版本，不查质量。
- auto 轴的五维恰好覆盖 OpenSquilla skill 的结构事实（frontmatter/工作流/命令安全/脚本/测试），可直接平移。

### 落地建议

1. 写一个 `_skill_quality_check.py`（对 workspace/skills/*/SKILL.md 跑五类静态评分），挂进现有「规则型 Skill 更新检查」口径第 2 步（零网络本地跑）。
2. 90 分阈值做装门/更新门：新装 skill（如刚装的 last30days）先跑评分，低于 80 出改进项再考虑启用。
3. LLM 轴可复用 EGM 的 Goldfish（无上下文评审）角色——同构的双轴思想：确定性检查管结构，评审模型管内容质量。

## 附：同批调研的另六个仓库（2026-11-23）

| 仓库 | 定位 | 本机处置 |
|---|---|---|
| ChromeDevTools/chrome-devtools-mcp | 官方 Chrome DevTools MCP（性能/网络/console） | **已装**：config.toml + node 入口 + --no-usage-statistics |
| alibaba/open-code-review | 阿里混合架构 code review CLI（55.4k⭐） | **已装**：npm 全局，delegate 模式免 key 已验证 |
| mvanhorn/last30days-skill | 多源 30 天社区调研 skill（72.7k⭐） | **已装**：workspace/skills/last30days，试跑成功 |
| backnotprop/plannotator | agent 计划/产物审阅层（14.4k⭐） | 追踪（MCP 通用接入可行，OpenSquilla 非一线支持） |
| GoogleChrome/modern-web-guidance | 现代 web 最佳实践 skill（10.2k⭐） | 未装（Node 22+ 满足，需要时 npx 即装） |
| ag-ui-protocol/ag-ui | Agent↔前端协议 1.0（21.2k⭐） | 追踪（造 agent 前端时取用） |

## 复核留痕

- 源码浅克隆：`tmp/claude-trading-skills`（depth 1，2026-11-23）；本文全部结构引用自 workflows/market-regime-daily.yaml、workflows/trade-memory-loop.yaml、skills/trader-memory-core/SKILL.md、skills/dual-axis-skill-reviewer/SKILL.md 与 references/scoring_rubric.md 原文。
- 未引入任何其代码/依赖；三点均为设计借鉴，落地建议需另行排期实现。
