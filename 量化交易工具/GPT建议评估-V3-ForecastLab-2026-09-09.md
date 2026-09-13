# GPT 建议逐条评估 · V3 Forecast Lab 改造方案（2026-09-09）

> - 评估对象：2026-09-08 23:32 Summer 转入的 GPT 对 V3 的建议（17 节，原文存于会话 `agent:main:webchat:5zwvefsv`，本次已全文取回逐条核对）。
> - 涉及项目：Microsoft Qlib、TimesFM 3.0、Chronos-2、TimeXer、FinRL-X/FinRobot、TSLib、TFB/TSFM-Bench、Time-MoE。
> - 口径基线：v7 诚实口径（2026-08-29）· WF 首跑（2026-09-01）· TimesFM-3 判读（2026-09-08）· V3 数据层评审（2026-09-08）。
> - 产出方式：OpenSquilla 定时任务（cron cfdb0938），外部事实逐项本机核实；**本笔记现归档于「量化交易工具」（2026-09-09 14:46 由 `04-GitHub研究\GPT建议评估-V3\` 移入）。**

---

## 〇、结论（放最前）

**17 条总判定：值得偷 9 · 暂不做 6 · GPT 需修正 2（§4 TimesFM 优先级、§2/§14 Qlib 基准数字），另有 2 处附带事实修正——原第 3 处（§10 FinRL-X 未证实）已于 01:42 复核撤回：GPT 该句正确。**

1. **总评：GPT 主线主张成立**——"别再堆单一模型，改建可插拔的模型委员会（V3 Forecast Lab）"。这与 V3 现状吻合：单模型 HistGBT + 正态近似分位数确实到了边际收益递减点，而 WF/registry/评分卡基建恰好是委员会框架的现成底座。
2. **但它的两个招牌建议必须修正**：① "TimesFM 第二值得测" **已过期**——V3 已于 09-08 跑完全量 PIT 零样本回测并被准入门否决（002112 T+3/T+5 RankIC 显著负、002207 CI 跨零、方向命中≈抛硬币），P5 取消不接主线；② Qlib 基准 IC 数字与官方 README 有出入（详见 §四）。
3. **三专家起步**（不是 GPT 的六专家）：`existing_v7`（对照）→ `lgbm_a158`（特征体系压力测试）→ `chronos2`（唯一外部零样本概率专家候选）。TimesFM 席位**关闭**，TimeXer 排队候补——许可证与数据供给两关未过。
4. **上线门槛写死**：集成必须同时满足"T+5 RankIC ≥ existing_v7 同冻结样本 WF 当次复算值"且"配对 cluster bootstrap 差值 CI 下界 > 0"，并逐级通过 baseline hierarchy；不过即留档出局，不进 `model_ready`。（09:35 修正：原"+0.080，WF 口径"系混标——+0.080 是 v7 purge-bootstrap 与 09-01 rolling 口径冻结值，09-08 WF 复算为 +0.089 CI[+0.034,+0.155]，历史三版本有漂移，见 §六）
5. **缠论按 GPT 的定位收编（结构信号专家，非预测模型），但入池前置可证伪闸门**，沿用 2026-08-23 做法。
6. **（13:24 补充）各暂缓项目的复活闸门已预注册，见 §九**——"验收不理想"须先分解为特征层 / 集成层 / 目标层三类失败，不同层对应不同复活处方，防止退化成"不成就换更大模型"。

---

## 一、17 条逐条判定

判定分三类：✅值得偷 ／ ⏸暂不做 ／ ✋GPT 说错·需修正（表内标签）。

| § | GPT 主张（摘要） | 判定 | 对 V3 的理由 |
|---|---|---|---|
| 1 | Qlib 全平台：数据层→特征→…→回测一条龙 | ✅（只偷模块）⏸（框架迁移） | V3 已有等价分层（PIT 样本、WF 63 日/折、registry、评分卡）。Qlib bin/exi 数据层过重、与现有 PIT 管道冲突，不整体迁移；只读 A158 特征表与 DoubleEnsemble 实现 |
| 2 | 偷 Alpha158/Alpha360 特征体系；引用基准：LGBM IC≈0.0448 等 | ✅ + ✋（数字） | 特征体系值得偷：A158-lite（约 50~70 个滞后/滚动/量价结构特征，PIT 化）。但官方 README 现为 **LightGBM IC 0.0399 / RankIC 0.0482**，GPT 的 0.0448/0.0469、XGBoost 0.0498、Transformer 0.0264 与 README 不符（疑混不同 setting）；且 Qlib 的 IC 是 CSI300 **横截面**日频，V3 RankIC 是时序/小横截面混合——**口径不可平移**，0.04~0.05 不能当 V3 收益预期 |
| 3 | DoubleEnsemble / 动态模型选择器是下一版升级点 | ✅（lite 版） | 问题框架（"什么时候该信模型"）与 v7 一致性因子、最近窗衰减发现完全同源。落地为 DE-lite：WF 每折用上一折 RankIC 加权；**不移植** DE 全套数据增广+样本重加权（3343 样本太薄） |
| 4 | TimesFM 3.0 是"第二个最想让你测试的" | ✋（建议过期）＋ ❌（席位关闭） | **已测已否决**（2026-09-08，P5 取消）：全量 PIT（002112/002207 各 ~2115 origins）准入门未过；档案 `experiments\timesfm\RESULTS.md`。multivariate+covariates/330M/zero-shot 事实本身说对了 |
| 5 | TimesFM 不必训练、zero-shot 当专家；权重非商用许可 | ✅（许可提醒对）⏸（不再投入） | 许可提醒属实且 V3 已照办：权重 `timesfm-non-commercial-license-v1.0` 仅在隔离环境 `D:\PythonProject\QuantV1-tfm` 使用、禁入生产——合规。但据实测结果，专家席位关闭，不重复实验 |
| 6 | Chronos-2 与 TimesFM 应成"双模型基线" | ✅（Chronos 部分） | Chronos-2 事实全部核实无误（2025-10-20 / 120M / 21 quantiles / context 8192 / covariate-informed / Apache-2.0 无 Weight 商用限制）。但"与 TimesFM 成双"的另一半已被实测拆掉——**Chronos-2 是唯一的外部零样本概率专家候选**，且同样必须先过 TimesFM 式准入门 |
| 7 | 概率预测（P(ret>x)）→ Forecast+Uncertainty+Regime → DecisionScore | ✅ | 直接解 v6/v7 遗留：现行 q10/q90 是 ±1.282σ 正态近似（P0-5 妥协项），Chronos 真 21 分位可替换；接入现有 confidence 与 policy 层即可，不推翻 forecast 协议 |
| 8 | TimeXer 专治"ETF + 市场/板块外生变量" | ⏸（排队专家 #4） | 问题框架贴合，但三关未过：① 官方仓库首页**未见 LICENSE**（README 留"use the code 请联系作者邮箱"）——商用性无据；要玩该架构走 TSLib（MIT）内的同名实现 ② 需自训，日频少量标的下 Transformer 最易过拟合 ③ 外生供给受东财封锁制约（BK0457 单一忠实源、板块 K 线缓存 62/65、09-08 刚修完陈旧盲点 P0-0） |
| 9 | 真问题是领先-滞后（指数→券商 ETF→主题 ETF）而非自相关 | ⏸（先过代理质量闸） | 问题提法有价值，但 V3 已在 2021-23 段用同类路径证伪过一次（板块代理→净值相关性衰减，08-23 行业指数版②收口）。任何 lead-lag 特征先过分时段相关性闸门，再谈专用模型 |
| 10 | FinRL-X/FinRobot 属决策/Agent 层非价格预测层；"先不被 Agent 带跑" | ✅判断（同意）＋ ✅名称（复核成立） | 同意 GPT 自己的降权：V3 瓶颈在信号→Forecast，决策层已有 Policy/account 层承接，不引 LLM agent。**01:42 复核撤回上一轮修正**：GPT 这半句是对的——AI4Finance 官方仓库 `FinRL-Trading` 标题即 "FinRL-X: An AI-Native Modular Infrastructure for Quantitative Trading"（Apache-2.0；arXiv 2603.21330；PAKDD 2026 DMO-FinTech Workshop 论文并获 Best Presentation），FinRL 主线 README 亦明确把生产向管线指向 FinRL-X/FinRL-Trading。其论文口径（日频流动性好的美股/ETF、决策严格只用 t 时点前信息、offline 评估与部署分离）与 V3 纪律同源，可作为决策层参考。**但存在≠该用**：本条判定不变（不是当前瓶颈，暂不引入） |
| 11 | TSLib 当成熟实现库、**不当** SOTA 榜 | ✅（仅抄代码） | GPT 的维护警告属实：官方 News (2026.04) 原文——维护带宽有限、不再积极加新功能、老 benchmark 已无意义、"baseline implementations remain correct"。V3 用途＝抄 PatchTST/TimeXer 等实现；优先级低（瓶颈不在网络实现层） |
| 12 | TFB/TSFM-Bench 解决"到底哪个模型有效" | ✅（偷协议不接框架） | TFB＝PVLDB 2024 Best Paper Nomination（MIT），2025.06 开源 TSFM-Bench（zero/few/full-shot 统一切分），带 characteristics_extractor（trend/seasonality/stationarity/shifting/transition/correlation）。同类可参考 ICML 2026 "It's TIME"（MIT，专治 zero-shot 语料污染的 fresh-dataset 基准）。偷三样：特征诊断回答"数据瓶颈还是模型瓶颈"、按 regime 切段评估、多指标协议。**不接框架**：其长程点预测 MSE/MAE 任务设定与 V3 的 RankIC/Brier/WF 门禁不同口径 |
| 13 | Time-MoE 不推荐（2.4B/300B 时间点与 V3 数据量不匹配） | ✅同意（即暂不做） | 同意。V3 缺的是 signal quality + 截面信息 + regime 适应 + ensemble，不是模型容量 |
| 14 | "树模型不输普通 Transformer"→委员会大于大模型 | ✅（原则）＋ ✋（数字） | 方向性结论成立，作为"先加 LightGBM 专家、不再写新 Transformer"的依据。引用的具体 IC 一律按 §2 修正后数字使用 |
| 15 | Forecast Engine 改 6 专家 + Ensemble + Regime-aware Router + Policy | ✅（降配版） | 架构方向对，但 6 专家超配——3343 样本先养 2~3 个。Router 第一版不做在线切换，只做"regime 分桶诊断表 + 只降权不上调"（见方案 §3） |
| 16 | 缠论不做 Forecast Model，做"结构信号专家"入委员会 | ✅（定位）⏸（入池） | 定位认同。但入池前补可证伪条件（沿用 08-23 做法）：① 线段终结特征序列判定 ② 背驰正规判据（现"末笔<前笔×0.8"近似）③ 中枢按线段构建（现用笔）+ 熊/震市样本"下方<ZD"显著跑输检验（事件 N≥30）。补齐前入池＝把不可验证信号洗成委员会票数 |
| 17 | V3 Forecast Lab：models/ 可插拔 + 统一输出 schema | ✅（最值） | 全案核心容器。schema 与 v7 完全兼容，与 `core/model_registry.py`（sha256 拒载）和 registry feature_protocol（排期中）天然衔接；详见方案 §1 |

---

## 二、V3 Forecast Lab 最小可执行改造方案

**先定纪律**：本方案全部在**研究层**——不动生产报告/推送/`model_ready`，不改数据层（09-08 数据层 P0 系列与 BK0457 单源问题不在本方案范围），所有评估沿用 v7 口径：**PIT + label-end purge（max_horizon=5）+ 按日块 cluster bootstrap 999 次（seed 42）+ baseline hierarchy**。

### 步骤 1｜统一专家接口（0.5 天，2 文件 + 1 测试）
```jsonc
{ "expert": "lgbm_a158", "asof": "2026-09-08",        // PIT：只用 asof 收盘前可见信息
  "horizon": 5,
  "expected_return": 0.0031,                           // E[return]，沿用 P0-5 口径
  "prob_up": 0.58,
  "q10": -0.012, "q50": 0.0029, "q90": 0.0187,
  "q_source": "normal_1.282sigma",                     // 必填：normal | empirical_21q | conformal —— 分位数出身必须显式（v6 假分位数教训）
  "confidence": 0.55 }                                 // 复用 v7 复活的两因子（coverage 映射 + 跨周期一致性合成）
```
- 落点：`core/expert_protocol.py`（dataclass+校验）；现有 ForecastEngine 输出适配为第一个专家 `existing_v7`（即 GPT 图中的 `existing_model.py`，零侵入）。
- 专家与集成权重版本全部进 `core/model_registry.py` 同一 sha256 机制。

### 步骤 2｜先接 2~3 个专家（顺序即优先级）
| 序 | 专家 | 成本 | 定位 |
|---|---|---|---|
| 1 | `existing_v7`（HistGBT） | 0 | "单模型 T+5"对照基线：基准＝同冻结样本 WF 当次复算值（09-08 为 +0.089；+0.080 系 v7/rolling 口径，勿混用，见 §六） |
| 2 | `lgbm_a158` | 1~1.5 天 | A158-lite ~50-70 特征（滞后/滚动/量价结构，剔除需全市场截面的项），LightGBM 回归 + 分位回归；回答"**是不是特征瓶颈**" |
| 3 | `chronos2` | 1~1.5 天 | 隔离环境 `D:\PythonProject\QuantV1-c2`（复刻 tfm 模式：hf-mirror + `HF_HUB_DISABLE_XET=1`，CPU 够），zero-shot 滚动 origin 打分；q10/q50/q90 取真 21 分位；**准入门与 TimesFM 同款**——大概率同样被拒，拒了也是"第二个外部零样本基线已排除"的证据 |
| — | TimesFM 3.0 | 不排 | **席位已关闭**（09-08 实测否决 + 非商用许可） |
| 候补 | TimeXer | 不排 | 触发条件：① regime 分桶显示外生信息确有增益 ② 走 TSLib（MIT）实现解决许可证 ③ 东财封锁/外生数据供给解决 |

### 步骤 3｜Regime-aware Router / DoubleEnsemble-lite（~2 天，本轮只建 V1）
1. **V1（唯一允许本轮上线）**：过准入专家等权平均；分歧（p_up 符号分歧或 q90-q10 宽度过大）→ 只**降 confidence/降权**，不上调——安全形式。
2. **V2（DE-lite）**：每个 WF 折内用前一折 RankIC 静态加权（fold 内不碰未来、无泄漏）；不移植 DE 样本重加权/数据增广。
3. **regime 部分先只做诊断表**：按 V3 现有 MarketContext/Regime 定义分桶统计各专家 RankIC——为 Router 与 09-26 Drift（PSI/KS）里程碑共用证据；不做在线切换。
4. **Router 本身是模型**：必须进 baseline hierarchy 逐级"不劣于"裁决，不许绕过。

### 步骤 4｜集成 vs T+5 单模型：评估协议（5 条）
1. **冻结样本**：样本拉取版本固定 + 记录 sha256（002112 持仓快照失败造成 ±0.03~0.05 IC 漂移，**大于多数新专家的预期增益**；不冻结版本，"A 比 B 好"可能只是拉取噪声）；两次独立重跑均过才留档。
2. **主判据**：pooled WF T+5 RankIC（63 日/折 expanding、每折重训 + label-end purge）；冻结口径 OOS 作对照列。
3. **配对 bootstrap**：同一按日块重采样同时作用于集成与单模型 T+5 逐样本分数，取**差值分布** CI（999 次）——消除共同日冲击，是"集成>单模"的唯一公平判法。
4. **准入闸门（三条全满足才"准上"）**：① 集成 RankIC 点估计 ≥ existing_v7 同冻结样本 WF 当次复算值（09-08 复算 +0.089；勿沿用历史 +0.080——那是 v7 purge-bootstrap/rolling 口径）；② 配对差值 CI 下界 > 0（显著不劣于→实际要求显著更好或至少同点值）；③ baseline hierarchy（瞎猜→多数类→est_chg→existing_v7→集成→Router）逐级不劣于。任一断档：该级弃用留档，不做择好看。
5. **裁决去向**：全过 → 填入现有 T+5 评分卡 + 走 STALE_EVIDENCE 门禁，`model_ready` 翻转仍由 Summer 拍板；本方案无权翻旗。

### 步骤 5｜缠论入池前置闸门（并行，0 代码，0.5~1 天）
沿用 08-23 可证伪做法，三项已知限制各写 1 条证伪指标并出判定文档：特征序列判定（熊段终结早于基线率）、背驰正规判据（"下方<ZD"后 20 日收益在熊/震市样本显著<0，事件 N≥30）、中枢线段化（ZG/ZD 稳定性）。**任一未达标：结构信号不进统一接口。**

---

## 三、风险提示（必写四条 + 补一条）

1. **TSFM 语料隐性泄漏**：TimesFM/Chronos 预训练语料可能含 A 股指数/ETF 日线（Google 语料含 Web/GCS 时序、构成未公布）。若含，zero-shot 的"IC"实为记忆，真未来必然衰减。**缓解**：① 按 regime 切段验证（牛/熊/震荡段 RankIC 逐段一致，重点查近期段），并与 WF 最近窗衰减列（-0.051 跨零）交叉对照；② **发布日后净段检验（最干净的一招，本轮新增）**——zero-shot 专家只在**模型发布日之后**的样本上计分：Chronos-2 取 2025-10-20 之后（V3 现有约 10.5 个月、~215 个交易日，够做一次裁决），TimesFM-3 取 2026-08-31 之后（不足 1 周、样本不可用，这本身就是它席位关闭的第三条理由）。该段按构造不可能在预训练语料内，能直接把"记忆"与"泛化"分开。
2. **分布差异**：V3 是日频 × 4 基金（T1 池 12 项）× horizon≤5；TSFM benchmark 多为长程多变量、海量点位、能源/交通/零售域。**TimesFM 实测（方向命中≈抛硬币）已是前车之鉴**——对 Chronos-2 的预期同样按"大概率被拒"设定，拒了就留档，成了才算惊喜。
3. **许可合规**：TimesFM 权重 = `timesfm-non-commercial-license-v1.0`，**仅限研究/回测，禁入生产与商用输出**（代码 Apache-2.0 不改变权重限制）；V3 现处理合规（隔离环境、未接主线）。Chronos-2（Apache-2.0）无此限制，是它当首选外部专家的另一半理由。附带：TimeXer 官方仓库 LICENSE 未见展示，"可自由使用"无据。
4. **缠论入池**：定位（结构信号专家）可采纳，但可证伪条件未补全前不得入池——否则委员会只是把不可验证信号洗成票数的机器。
5. **（补）评估复现性**：002112 快照拉取失败导致的 ±0.03~0.05 IC 漂移大于本方案多数结论的量级；所有对比必须先冻结样本版本，否则白跑。

---

## 四、GPT 事实性错误的集中修正（引用时替换）

| 处 | GPT 原文 | 应修正为 |
|---|---|---|
| §2/§14 | LightGBM IC 0.0448 / RankIC 0.0469；XGBoost 0.0498 / 0.0505；Transformer 0.0264 / 0.0407 | 官方 README 现值：**LightGBM IC 0.0399 / RankIC 0.0482**；**DoubleEnsemble 0.0521 / 0.0502（GPT 引对）**；其余两条疑混 setting，按 README 重查后才可引用。方向性结论（树模型不输普通 Transformer）成立 |
| §4/§5 | TimesFM"最值得测试" | V3 已测已否决（09-08 全量 PIT、准入门未过、P5 取消）；证据 `experiments\timesfm\RESULTS.md` |
| ~~§10~~ | ~~"FinRL 已推进到 FinRL-X / FinRL-Trading"~~ | **本行撤回（01:42 复核）**：上一轮误判为"零命中/未证实"，实为 GPT 正确——`AI4Finance-Foundation/FinRL-Trading` 即 FinRL-X（Apache-2.0、arXiv 2603.21330、PAKDD 2026 DMO-FinTech Best Presentation），FinRL 主线 README 亦指向它。**故 §四 现只剩 2 行真实修正**；§10 的"暂不做"结论不因名称之争改变，理由是"非当前瓶颈 + 决策层已有 Policy 承接" |

---

## 五、排期与件数估计（合计 ~9 人日 / 09-09 ~ 09-29）

| 阶段 | 窗口 | 内容 | 件数 |
|---|---|---|---|
| P0 接口 | 09-09 ~ 09-10 | `expert_protocol.py` + existing_v7 适配 + registry 登记 | 3 文件（2+1 测试）≈300 行 |
| P1 特征专家 | 09-11 ~ 09-15 | A158-lite 特征包 + lgbm_a158 训练/打分 + WF"现有特征 vs A158-lite"消融 | 4~5 文件 + 消融留档 md ×1 |
| P2 概率专家 | — | **已取消（09-09 Summer 决议：不测 Chronos-2）**——外部大模型路线整体关闭，如未来重启需重新拍板；regime 切段表与发布日后净段检验思想并入 P3' 诊断表 | 0 文件 |
| P3 集成裁决 | 09-23 ~ 09-25 | `backtest_ensemble.py`（等权 V1 + 配对 cluster bootstrap 999 + hierarchy 逐级）；**09-25 前留档**，避开 09-26 季度重估同日双口径（§六） | 2~3 文件 + 裁决 md ×1 |
| 并行 | 09-27 ~ 09-29 | 缠论可证伪清单文档；Router V2 只出设计稿不实现 | 文档 ×1 |

与既有排期不冲突：Path 校准/T+5 评分卡/feature_protocol/Drift(09-26) 是承接项；本方案总新增 **9~13 文件、≤1500 行**，全部 paper-only 研究层。

---

## 附：证据链与路径（本机已核实存在）

- GPT 原文 17 节：会话 `agent:main:webchat:5zwvefsv`（2026-09-08 23:32 用户消息，本次全文取回）
- v7 口径 / WF 首跑：`MEMORY.md#L61-L63`、`MEMORY.md#L75-L77`；留档 `D:\PythonProject\QuantV1\backtest_walk_forward.py`、`output/backtest_walk_forward_20260901.md`
- TimesFM 判读：`MEMORY.md#L131-L134`；`D:\PythonProject\QuantV1\experiments\timesfm\RESULTS.md`；隔离环境 `D:\PythonProject\QuantV1-tfm`
- 数据层评审与 BK0457 单源结论：workspace `memory/2026-09-08.md`；`D:\PythonProject\QuantV1\output\V3_data_layer_plan_20260908.md`
- 缠论可证伪先例：`04-GitHub研究\book-to-skill\chanlun\缠论POC验证记录-20260823.md`（2026-08-23，②闸门关闭记录）
- 外部核实（2026-09-09 01:42 复核）：① TSLib 官方 News (2026.04) 原文确认——维护带宽有限、不再积极加新功能、老 benchmark 已无意义、"baseline implementations remain correct"、建议改用更新的基准；② TimeXer＝thuml 官方 NeurIPS 2024 实现（515 stars），仓库首页未见 LICENSE 展示，README 为"want to use the code, please contact 作者邮箱"；③ TFB＝PVLDB 2024 Best Paper Nomination（MIT），News (2025.06) 同时开源 TAB 与 TSFM-Bench；④ **FinRL-X 已证实存在**（`AI4Finance-Foundation/FinRL-Trading`，Apache-2.0，arXiv 2603.21330，PAKDD 2026 DMO-FinTech），上一轮"零命中/未证实"判定作废

## 复核留痕（2026-09-09 01:42，第二轮）

1. **撤回 1 处修正**：§10 原判定"FinRL-X 未证实"错误，GPT 该句成立（详见 §一 表 §10 与 §附录④）。判定结果不受影响（仍属"暂不做"）。
2. **确认 3 处修正**：TSLib 维护状态、TimeXer 无 LICENSE、TFB/TSFM-Bench 事实与身份，均与官方仓库页一致，§11/§8/§12 判定维持。
3. **新增 1 项可执行检验**：TSFM 隐性泄漏的"发布日后净段检验"（§三·风险 1②），已写进 P2 交付内容。
4. **未变动项**：三专家顺序（existing_v7→lgbm_a158→chronos2）、TimesFM 席位关闭、上线三门槛、~9 人日排期、paper-only 纪律——本轮无新证据动摇。
5. 本轮另核：`04-GitHub研究\GPT建议评估-V3\` 分类已存在（上一轮新建），本次原地修订不重复建目录。
6. **（2026-09-09 14:46 归档修正）** 本笔记归属纠正：内容属量化项目（V3 Forecast Lab），已从 `04-GitHub研究\GPT建议评估-V3\` 移入 `量化交易工具\`，空目录已删。此后本项目所有笔记统一记入 `D:\Obsidian\My-First-Obsidian\量化交易工具\`。

## 六、ZCode 任务迁移影响补记（2026-09-09 09:35）

Summer 已把 ZCode 侧 3 条定时任务迁回 OpenSquilla（09-09 生效）：16:00 板块 K 线增量拉取、22:30 Shadow 纸面记录、09-26 季度证据重估；另有 09:30 zcode_runs 只读检查为原有任务、牛马 09:00 日报暂停中。核查结论：**本报告主线结论与三决策均不受影响**，落地 4 项调整：

1. **门槛口径修正**（已改 §〇4 / 步骤 2 / 步骤 4①）：单模型对照基准从历史 +0.080 改为"同冻结样本 WF 当次复算值"——+0.080 系 v7 purge-bootstrap（CI[+0.017,+0.142]）与 09-01 rolling（CI[+0.020,+0.140]）口径，09-08 WF 复算为 +0.089（CI[+0.034,+0.155]）；三个版本互相漂移这件事本身，就是"门槛不许钉历史数字"的证据。
2. **P3 错峰**（已改 §五）：集成裁决 09-25 前完成留档，避开 09-26 10:00 季度重估全管线同日双口径；两份结果并存时标注 bootstrap 口径差异（季度例行 199 次 vs 裁决级 999 次配对）。
3. **IP 频控互斥纪律**（新增，强化 §三风险 5）：东财 push2his 为 IP 级频控——Forecast Lab 一切实验性拉取（002112 持仓快照补拉、Chronos-2 样本冻结）必须避开 16:00 生产拉取窗口（前后留缓冲），且 ZCode 侧同类任务已确认停干净（09:35 只读核验：`tasks-index.sqlite` automations 表已空，无双跑）；双跑会把 IP 打进封锁、连累 evening 任务（09-08 五次白忙教训）。P2 的 CPU 长跑也错开 16:00 / 21:30（Windows 计划任务 QuantFund_KlineEvening 晚间拉取）/ 22:30 三个生产窗口。
4. **衔接利好**：09-26 季度重估即本报告步骤 3 所引"09-26 Drift 里程碑"的执行载体（drift_monitor.py 在其管线内）；regime 分桶诊断表应在 09-26 前就绪，与 drift_monitor 输出同源对齐。另：09:30 检查任务失败提示语"检查 ZCode 是否在运行"已过时（zcode_runs 现由 OpenSquilla 自产自检），任务逻辑仍自洽，暂不改动、待 Summer 定。
5. **迁移冒烟与依赖隔离（09:35 只读核验、09:46 复核补记）**：迁回任务在册——16:00 板块K线 `fa6d9cdd`（09:45 重建，原 `55c96e26` 因文案失配废弃：默认 scope=prod 4 码 BK0428/BK0457/BK0669/BK1173、分母 65→4 对齐）/ 22:30 Shadow `69b20a70` / 09-26 季度重估 `0cfaa288`，另有原有 09:30 巡检 `fe99ab2a`、牛马日报 `a5794719`（paused）、22:45 一次性验收 job `6fceb42e`（P0-1/2/3/4 收尾，只读）。今日 16:00 为迁回后首跑（09-08 ZCode 侧同任务 failed＝东财封锁日），即迁移冒烟测试，明日 09:30 巡检自动核验。另两条纪律：P3 对比基线开跑前冻结快照（不追 09-26 复算值）；P1 新依赖（lightgbm 等）装研究层独立 venv，不动日常任务主 venv；zcode_runs 目录与小节标题契约沿用不改，P0 样本冻结（sha256）与每日 data/ 增量天然隔离。

## 七、P0 实施记录 · Summer 七点决议落地（2026-09-09 10:06~10:45）

决议映射：①开 Lab→按案执行；②**不测外部大模型**→P2 删除，总人日 ~9→**~6.5**；③门槛认→写死不变（含 §六修正）；④修快照→已开工并有重大发现（下节）；⑤巡检文案→新 job `93439da7` 在册、旧 `fe99ab2a` 已删（单跑确认）；⑥缠论闸门不变；⑦Router 仅诊断不变。

**取消 P2 后的新排期**：P0 09-09~10（接口+冻结+快照加固）→ P1 09-11~15（lgbm_a158）→ P3' 09-16~18（集成裁决，09-18 留档避开 09-26）→ 09-19~21 缠论闸门。裁决问题随之收窄为：**A158-lite 特征专家 + 等权集成能否击败 existing_v7**（无第二独立专家，集成＝成员只有两个；若未来重启 Chronos-2 再扩）。

### 快照漂移根因：比预期严重——现行复现的静默 bug（非历史偶发）

- **证据链（全部今日本机实测，只读）**：① 10:27 首冻 002112 n=26（完整）；② 10:29 二冻 **002112/2020 整年 4→0 消失**，且 `failed_year_warnings` 为空——即 fetch 返回 `[]` 但**未抛异常**，`holdings_history` 只在异常时记缺年，v7 的重试加固**护不住这条路**；③ 同年直连重探 3×2 基金全部 200 且正常 4 期 → 服务端偶发降级页，旧码不留档、根因无法回溯；④ 10:38 三冻与首冻 **100% 一致**（`--compare` 退出码 0）→ 完整态可复现，可作裁决基准。
- **含义修正**：v7 笔记里的"±0.03~0.05 漂移"不是 8 月那天的代理抖动，而是**今天还在发的现行 bug**——任何一次 WF 重跑都可能静默少一整年持仓样本。
- **修复（研究层已实现并验证）**：`holdings_hardened.py` 把"HTTP 成功但零快照"从静默正常升级为三态判定——EMPTY_CONFIRMED（确证无披露，正常返空）/ SUSPECT_DEGRADED / PARSE_MISMATCH（均抛异常 + **原始响应留档** `forecast_outputs/f10_raw/`）。离线自检 5/5，在线 6 请求 4 OK + 2 真无披露正确区分，无回归。
- **冻结基准（P3' 裁决样本版本）**：002112 `81b00c09…` / 002207 `7fb85482…` / 022853 `40b9d9d2…` / 025687 `9421faa1…`（`forecast_outputs/snapshot_frozen_20260909_102726.json`）。022853/025687 的 2020-2024 零计数 = 新基金无早期披露（正常，探针已区分显示）。
- **✅ 已合入生产（10:43 Summer 拍板，早于预估的 09-10）**：`core/lookthrough.py` 新增 `DegradedResponse` / `_EMPTY_MARKERS` / `_dump_raw`，三态判定内联到 `fetch_holdings_year`（+61/−3 行）。**合入方式与研究版有一处关键取舍**：研究版自带的 `retries=2` **不合入**——否则与外层 `holdings_history` 的 3 次重试嵌套，降级年份最坏 9 次请求，东财是 IP 级频控（09-08 五次白忙），花不起；重试仍只由外层负责，降级走既有「缺年告警」通道。告警措辞按原因分流：降级页→指「服务端改版/反爬，已留档」，普通异常→仍指「检查代理」。备份：`backups/pre_lookthrough_tri_20260909/lookthrough.py.bak`。
- **验证链（全部本机跑过）**：`test_holdings_tristate.py` 9/9（零真实网络，直 mock `netutil.http_get`，不触发外层退避故属快路径）· 快路径全量回归 **249/249 OK**（含既有 235）· 合入后经**生产路径**重冻与首冻基准 **sha 完全一致**（`--compare` 退出码 0）· 研究层两自检仍绿（12/12、5/5）。
- **一个取证卫生教训（差点把假证据当实锤）**：首版单测未隔离 `_RAW_DIR`，跑完在 `f10_raw/` 留下 2 份**合成** antibot 样本，与测试夹具逐字节相同。10:54 我一度将其误读为「修复在真实链路上拦下降级」的铁证——幸而核对了时间戳（10:54 ≠ 10:56 重冻）与字节内容才抢回。**`f10_raw/` 的唯一用途是留真实事故证据，混进测试件就会失去可计数性**。已修（`_RawDirSandbox` 基类重定向临时目录）+ 清理，现真实留档计数 **0**（即：合入至今服务端未再降级，重冻一致属正常而非漏拦）。
- ~~run.py 只读探针列~~ **✅ 已补（11:20，Summer 拍板 A 项）**：`core/report_generator.py` `_lookthrough_section` 新增 `missing` 参数——穿透缺失基金在报告里显式输出🚨探针行（含「≠该基金无持仓、勿按空表解读」警示），`run.py` 接线 `lt_missing`。三态冒烟（缺失显式化/旧空态/无缺失）全对，全量回归 235 OK/skip 3。未提交，与 forecast_lab 一并等 22:45 验收后入库。背景：`evaluate_lookthrough` 对 `holdings_history` 外层层有 `except Exception: continue`，全部年份失败时会返回 `[]` → `if not history: continue` → 该基金从报告里静默消失（告警只发 log）。

**P0 文件清单**——研究层（`experiments/forecast_lab/`）：`expert_protocol.py`（schema v1 冻结，自检 12/12）· `probe_snapshot_freeze.py`（冻结+compare）· `holdings_hardened.py` · `ab_f10_hardened.py` · `probe_f10_flakiness.py`；**生产层（已合入）**：`core/lookthrough.py` + `tests/test_holdings_tristate.py`；产物目录 `forecast_outputs/` 已入 .gitignore。P0 完成度：接口 100%、快照修复 100%、生产合入 100%、run.py 探针 100%（11:20 补）。

### 11:10 双会话冲突合并留痕（另一 OpenSquilla 会话；Summer 10:52 对两会话都说了「开工」）

1. **冲突实况**：同一小时内两会话并行开工。本会话写了 `core/expert_protocol.py` + `tests/test_expert_protocol.py` + `forecast_lab_freeze_samples.py`（均未提交）；对方会话（即 §七 作者，10:27~10:38）已先落 `experiments/forecast_lab/expert_protocol.py`（§七 明载「写入即冻结」）。
2. **裁决：以对方 schema 为唯一权威**（冻结在先、校验更严：confidence 缺失用 NaN 不假装中性、asof 格式硬校验、自带自检）。本会话三份重复实现**已删除**（仅删自己新建文件，未碰对方任何文件）；独有增量已合并进权威文件尾部（只增不改已冻结段）：`from_forecast_triple`（v8 Forecast t1/t3/t5 一次转三条）、`ensemble`（V1 等权/显式权归一校验 + 分歧惩罚只降不上调 + 成员唯一/同组校验）、`write_records`/`load_records`（jsonl 往返）。
3. **验证（删除后复跑）**：权威文件自检 12/12、合并段冒烟全过、全仓 unittest 235 过 3 跳（含对方三态生产新测试；§七提的 249 含另一发现，不矛盾——仓内测试在对方推进中增长）；`git status` 确认本会话净贡献 = 仅 `expert_protocol.py` 尾部扩展段。
4. **冻结基准确认不动**：P3' 裁决样本以 §七 10:27 快照指纹（4 码 sha，`snapshot_frozen_20260909_102726.json`）为准；10:38/10:56 两次复冻 `--compare` 与首冻 100% 一致（本会话 10:5x 独立复核同样过）。**样本级冻结（samples_frozen.jsonl ~3343 行）仍未做**：今日 fundf10 已 ~70 次请求，不宜再跑 load_samples 叠频控风险；归 P1 首步（09-11，用已合入的三态加固路径跑）。
5. **未提交原因**：仓内大量并行 WIP（P0 数据层未提交 + forecast_lab 未跟踪），为避免混提交，留到 22:45 验收后或 Summer 指示时统一入库。
6. **教训（流程）**：Summer 同时在多会话发同一开工令会双写；今后开工类指令建议只发一个会话，或由收到方先查 `git status` + 本文件 §七再动（本会话即此顺序发现冲突的）。

## 八、11:18 两项决议落地 + P1 前置就绪（11:20~11:55）

1. **决议①（探针列并入 P1 窗口）**：实际已由并行会话于 11:20 提前合入（`report_generator.missing` + `run.py lt_missing`，见 §七划线条目，未提交等 22:45）——本会话**不重做**，明日只做验证；挂账清零。
2. **决议②（P1 提前 09-10）**：一次性开工任务已挂 cron `914e037f`（09-10 09:40，隔离会话），内置四道前置闸：schema 自检 12/12 · 冻结基准本地 compare 零网络 · 日期守卫（非 09-10 即空转报告）· paper-only 纪律。裁决窗口 09-16~18 不变，提前的一天即缓冲；特征规模维持 A158-lite（50~70）不再开分叉。
3. **自检红灯根因（已闭案，commit `9b3d3fb`）**：`to_dict 往返`用 dataclass `==`，往返对象 confidence=NaN——py3.12 元组比较含同对象身份短路（`nan is nan`→判真），py3.14 逐字段严格（`nan != nan`→判假）；同一文件在主 venv（3.12.13）12/12、旧 lab（3.14.6）11/12，结论随解释器版本翻转。字段级探针钉死后改为 **NaN 容差逐字段比较**，两环境均 12/12；冻结 schema 本体未动（只改自检段）。候选解释"文件保存中间态"已排除：mtime 11:03 未再变、git 状态干净。教训：**测试语义不得依赖解释器版本**。
4. **研究环境对齐生产**：`.venv-lab` 重建为 **3.12.13**（uv cpython-3.12 同源基底）+ numpy==2.5.1 + scikit-learn==1.9.0 + lightgbm==4.7.0，fit/predict 冒烟通过——消除跨版本 pickle 风险，P1 模型产物可与主 venv 互读。主 venv 无 pandas：P1 若需按同款方式补版本钉。
5. **P1 首日待办（写入 09:40 任务）**：样本级冻结 `samples_frozen.jsonl`（~3343 行，§11:10 留痕第 4 条挂账）+ 三态加固路径首跑生产真实数据验证 + run.py 探针列验证。P0 至此含前置条件全部就绪，P1 明早开跑无已知红灯。

### 8.1 P1 执行留痕（2026-09-10 接力执行 · 结论：未过准入门）

**前置闸 4/4 通过**：schema 自检 12/12 · 冻结基准 compare 四基金完全一致（退出码 0）· 全量单测 235 OK(skipped=3) · 探针列单测已并入全量。

**①样本冻结**：`samples_frozen_20260910.jsonl`，**3371 行**（预期 ~3343），sha256 `be8e55f02b39a25dc48f2efd67150c895ceebfebfe97c647b00610f61a05b034`；四基金分布 002112:1530 / 002207:1530 / 022853:276 / 025687:35；日期 2020-04-27 ~ 2026-08-12；0 告警、无频控征兆。

**②A158-lite 特征**：`experiments/forecast_lab/features_a158lite.py`——**50 个**（10 族 roc/ma/std/rsqr/resi/rsv/rank/cntp/sump/corr × 5 窗 5/10/20/30/60），PIT 只用 ≤T-1 净值；短窗不足一律 NaN 不缩窗；负下标防回绕；零网络自检 30/30。**CORR 口径替代**：基金无成交量，Alpha158 的 corr(close,log(volume)) 改用收益滞后 1 阶自相关（已显式标注）。单测 `tests/test_a158lite.py` **14/14**。

**③④消融与 hierarchy**（冻结样本 20260910 · T+5 pooled WF RankIC · cluster bootstrap 按日块 999/seed42 · 4 折 63 日）

| 方案 | RankIC | 95% CI | n |
|---|---:|---|---:|
| existing(7) · LGB | -0.0148 | (-0.0764, 0.0432) | 923 |
| **a158(50) · LGB** | **-0.0723** | (-0.1423, 0.0003) | 923 |
| union(57) · LGB | -0.1012 | (-0.1796, **-0.0228**) | 923 |

hierarchy：瞎猜 0 / 多类 0 / est_chg -0.0085 / existing_v7(HGB 7 特征) **-0.0324** / lgbm_a158 -0.0723 → **逐级不劣于检查未通过**。

**判定**：A158-lite **未过准入门**（既不 ≥ existing_v7 同样本复算值，也未通过 hierarchy），**留档出局，不进 `model_ready`**。**特征瓶颈假设不成立**——加特征反而更差（union 的 CI 上界 -0.0228 < 0，显著为负）。§九 复活闸门按「特征层失败」处方待触。

**⚠️ 附带发现：v7 基线漂移已定位机制（非随机波动）**。用**生产函数本体**（`backtest_walk_forward._fit_one_horizon`/`_eval`）在同一冻结样本复算 v7：

| 周期 | 冻结样本复算（09-10） | 09-08 WF 报告 | 漂移 |
|---|---:|---:|---:|
| T+1 | +0.0095 | +0.030 | −0.021 |
| T+3 | +0.0119 | +0.036 | −0.024 |
| T+5 | **−0.0324** | **+0.089** | **−0.121** |

机制：`data/stock_klines` 240 只缓存中 **171 只**于 09-10 09:48 冻结时因 TTL 到期被重取；东财 K 线是**前复权**价，重取会追溯改写历史价量 → 历史 `est_chg`/`composite`/`score` 随之改变（折 2 +0.142→−0.126，折 3 +0.040→−0.133）。**结论：漂移源是前复权序列的重取，不是波动**——意味着 v7 的 WF 基线在语义上不是「同一把尺子」。影响：准入门「≥ existing_v7 **同冻结样本** WF 当次复算值」这句需明确取哪把尺子（已列入日汇总待拍板）。

**⑤提交**：commit `ce65c74`（5 文件 / 946 行新增）：`features_a158lite.py` · `run_p1_ablation.py` · `freeze_samples.py` · `tests/test_a158lite.py` · `deploy/daily_summary_task.md`。**未 `git add -A`**（仓内并行会话 WIP 原样保留）。报告 `output/forecast_lab_p1_20260910.md`（`output/*.md` 按 .gitignore 留本地）。

**⑥专家输出契约**：`forecast_outputs/expert_lgbm_a158_20260910.jsonl` **266 条** ExpertOutput（expert=`lgbm_a158`、q_source=`empirical`、confidence=NaN；LightGBM 分位回归交叉 0 条）。

**踩坑留痕**：cron 任务的 `task` 正文过长会被上游截断成占位符导致 `add` 内部错误（实测两次失败、最小文本成功）→ 长指令改为「短正文 + 引用 `deploy/daily_summary_task.md`」结构。本条已固化为该任务的持久形态。

### 8.5 P1 收尾两项口径拍板落地（2026-09-10 13:13，Summer「按你的建议执行」）

1. **决策① 准入门基线（已法典化）**：后续一切验收（含 P2 chronos2、09-16~18 裁决窗口）基线一律取「**同冻结样本当次复算值**」，不再引用 09-08 的 +0.089 历史数字作门槛。P1 报告 §七已改「拍板结果」，本次合规基线 = §8.1 表中的 existing_v7 同样本复算 **-0.0324**。
2. **决策② 前复权漂移防护（已闭案）**：新增 `experiments/forecast_lab/kline_fingerprint.py`——逐标的 (date, close) 全序列 canonical sha256 → 聚合总指纹；只读缓存零网络；自检 8/8（close 改写敏感 / 非 close 列不敏感 / 追加敏感 / 坏文件显式列名）。已接入 `freeze_samples.py`：下次冻结起自动产 `kline_fingerprint_YYYYMMDD.json`，meta 增 `kline_fingerprint_sha256` 字段。**20260910 冻结事后补测指纹 `f5a2f607d335f180…`（stock=240 / fund=4，当日 09:48 后缓存无变更故可比）**。今后判定两次冻结是否等价 = 样本 sha256 + K 线指纹**双值同时相等**。
3. 日汇总待拍板清单据此清空：今晚 23:00 首跑不会重复提请这两条。

## 九、GPT 项目复活闸门（预注册 · 2026-09-09 13:24 · Summer 确认写入）

> 原则：暂缓 ≠ 永久否决，但复活条件必须写在验收点**之前**；触发后仍走同一套准入（10:27 冻结样本 sha256 + 配对 cluster bootstrap CI + baseline hierarchy），防事后换枪。本表与 §〇 第 4 条上线门槛同级，对 P1~P3 各验收点机械适用。

### 9.1 "效果不理想"先分解：失败层 → 复活处方

| 失败层 | 判定信号（最早观测点） | 该复活的 | 不该复活的 |
|---|---|---|---|
| ① 特征层 | P1（09-10 开工，裁决窗 09-16~18）：A158-lite 未跑赢 existing_v7 → 结论"不是特征瓶颈" | DoubleEnsemble（治训练方差）；TimeXer（若诊断显示结构性领先滞后） | 外部大模型（TimesFM 已证伪，同范式换皮无意义） |
| ② 集成层 | P3（09-23~26）：单模各自合格，但委员会打不过 existing_v7 单模 → 专家同质化 / 信号天花板 | **Chronos-2（唯一合理复活情形）**，且须 Summer 撤销决议② | 再加同类树模型（同质化加深） |
| ③ 目标层 | 全部打不过 baseline hierarchy（连瞎猜/多数类/est_chg 都不赢） | 无模型可复活——应怀疑问题定义（日频 T+5、3343 样本、小横截面），回到数据/label 层 | 任何 GPT 模型清单上的项目 |

### 9.2 逐项复活闸门

| 项目 | 当前状态 | 复活条件（可测） | 最早裁决点 |
|---|---|---|---|
| DoubleEnsemble | 暂缓 | P1 通过后同特征多 seed/窗口的 RankIC 波动 > 阈值（重采样能救方差）；或 P1 失败且诊断显示模型不稳定主导 | 09-16 |
| Chronos-2 | 决议②关闭 | **双条件**：①§9.1 第②类（集成层）失败出现；②Summer 明确撤销决议②。复活后裁决强制用"发布日后净段"（≥2025-10-20，样本够、构造上不可能在预训练语料内） | 09-23~26 |
| TimeXer | 候补 | 板块→ETF 滞后互相关在 ≥3 个 regime 桶稳定显著、但加进特征仍无增益（线性救不了 → 需结构建模）；前置：license 问题解决（官方仓库无 LICENSE → 走 TSLib MIT 实现） | P1 诊断后 |
| Regime 在线 Router | 降级为分桶诊断表 | 同一专家跨 regime 的 RankIC 差值配对 CI 不含 0，**且** regime 标签通过 PIT 复现检查（当时可见、不偷看），且桶内样本量达标 | 09-26 后 |
| 六专家全委员会 | 缩编 2~3 | ≥2 个专家过准入**且**专家间预测误差相关性低（多样性证据） | P3 后 |
| FinRL-X / FinRobot | 暂缓 | `model_ready=true` 达成**且**决策层回测显示更优 policy 有费后显著改善；或开新闻/情绪新信号源当独立专家 | 远期 |
| TimesFM 3.0 | **席位关闭（09-08 已证伪）** | 仅当**换问题**时复活：换频率 / 换 horizon（如 T+20）/ 换横截面排序目标。同一问题（日频 T+1/3/5 定向）下不复活 | — |
| Qlib 框架迁移 | 否决 | 扩到全市场横截面（数百标的）、自维护数据层成本 > 迁移成本 | 项目规模拐点 |
| TFB / TSFM-Bench | 暂缓 | 仅当外部 TSFM 路线复活需正式多模型 bake-off；单专家裁决现有 WF + 评分卡够用 | 随 Chronos-2 |
| Time-MoE | 否决 | **无复活条件**（数据量级差约 8 个数量级，2.4B 参数对 3343 样本无意义） | — |

### 9.3 流程纪律（两条）

1. **复活也要登记**：任何复活项目进 `core/model_registry.py`（sha256 + 10:27 冻结样本），重裁决受 09-26 季度证据重估的多重比较校正约束；每个暂缓项在一个验收周期内**只许提一次复活**，防止换种子反复摇。
2. **人为闸门与数据闸门分离**："决议类关闭"项（TimesFM、Chronos-2、外部大模型路线）即使数据条件满足也**不自动复活**，必须 Summer 拍板；纯技术条件满足的项（DoubleEnsemble、TimeXer、Router）可由验收报告直接触发，无需再议。

## 相关笔记

- [[基金日频参谋-v7-诚实口径-20260829]] · [[基金日频参谋-当前状态与下一步-20260902]] · [[板块K线数据源-东财频控诊断与兜底通道-20260908]] · [[缠论POC验证记录-20260823]]

#量化 #V3 #ForecastLab #评估报告 #GPT建议
