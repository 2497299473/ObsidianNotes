---
title: 发布资格门禁-last-run-wins设计
created: 2026-09-17
tags:
  - 量化交易
  - QuantV1
  - 证据链
  - 门禁设计
description: V4-A 飞书发布资格门禁（publish_gate）设计笔记：方案A整批裁决 + WARN定位防自锁 + 逐基金last-run-wins / 运行级全天粘性 + 持仓防泄露；含真实运行验证与全部提交号
---

# 发布资格门禁：last-run-wins 设计（2026-09-17）

> 触发：V4-A 收尾项 C——「报告可以生成」≠「报告可以发布」。飞书推送前加一道资格门禁。
> 代码：`D:\PythonProject\QuantV1` `audit_project.py::publish_gate()` + `run.py` 推送前置。
> 结论先行：**门禁单一事实源在审计器、拦截执行在 run.py、审计呈现只 WARN 不计 FAIL**；逐基金以当日最后一次完整评估为准（mid 抖动不拦 post 决策推送），运行级问题全天粘性。实测 356 测试 OK + 真实 post 运行 exit=0 双重验证。

---

## 一、三个关键决策（及理由）

| 决策 | 选择 | 理由 |
|---|---|---|
| 门禁粒度 | **方案 A 整批裁决**：任一只持仓基金证据不干净 ⇒ 当天整份报告不推 | 缺一只的报告本身就是残缺，推半份更危险；`push_feishu()` 是整份推送，逐基金拦截在执行层只能退化为布尔 |
| 审计定位 | **WARN 不计 FAIL** | 防自锁：门禁若进 FAIL 列表，「审计 FAIL ⇒ 资格 FAIL ⇒ 永无干净清单 ⇒ 永不可推」死循环；同时保住「0 FAIL」对账口径。真正拦截在 run.py，审计只呈现事实 |
| 时间语义 | **逐基金 last-run-wins + 运行级全天粘性**（见下） | 最初实现是全天并集：11:30 行情抖动会拦掉 14:55 最重要的决策推送——最坏情况恰好是最不该发生的 |

## 二、last-run-wins 规则细则

```
证据 = 当日已落盘 run_manifest（按时间序） + 本次内存 extra（恒为最后一次）
├─ 逐基金前缀（fund_data_partial / lookthrough_missing / realtime_failed / realtime_degraded）
│    → 以该基金当日最后一次「完整评估」轮为准；末轮干净即洗白早轮
│    → 「完整评估」判据 fund_evidence_complete()：data/lookthrough/realtime
│      四节键位齐全且 data.ok=True；末轮残缺 ⇒ 回落全天并集（不得用没评估过的轮次洗白）
├─ 运行级（market_context_* / shadow_failed / intraday_features_empty / 归因漂移）
│    → 全天粘性：市场背景缺位不会因末轮恢复而变可信，末轮干净也照拦
└─ 排除项（GATE_EXCLUDED_REASONS）：feishu_push_failed + 清单 notification 节
     → 门禁的输出永不作门禁输入（另一条自锁路径）
```

观测字段：`superseded`（早轮脏、末轮已恢复，仅记录不影响裁决）、`n_assess_runs`（完整评估轮数）。

## 三、过程中挖出的两个持仓泄露（都是本次改动自己引入的）

隐私边界澄清：`config.json` 的 fund_pool 4 只代码**本就公开在仓库**，真正敏感的是「**哪几只 shares>0**」。

| 泄露点 | 触发路径 | 修法 |
|---|---|---|
| manifest `codes` 字段 | 清单里写论域代码 → `output/run_manifest/` 被 git 跟踪（.gitignore 只忽略 `output/*.md`、`logs/`） | 删除该字段；论域只从 `holdings.json` 取（shares>0） |
| 拦截详情 `contaminated` | gate 明细进 `notification` 节 → 进清单 → 进仓库 | 清单只落 `n_contaminated`/`n_universe` 计数；审计入库侧做位置别名掩码（F1/F2，`mask_fund_codes`），真实代码只进本地控制台 |

两条均有回归守护测例（源码级：断言 run.py 不再出现 `'sorted(gate["contaminated"])'` 等）。

## 四、验证记录（2026-09-17）

- 单测：`tests/test_publish_gate.py` 26 例（tempdir 造证据、零网络），含 TestLastRunWins 五场景：
  S1 恢复放行 ✓ / S2 末轮仍脏照拦 ✓ / S3 残缺回落并集 ✓ / S4 运行级全天粘 ✓ / S5 内存 extra 恒为最后 ✓
- 全量：`run_tests.py` **356 tests OK (skipped=3)**
- 真实运行：`run.py --slot post --no-push` **exit=0**，清单 status=SUCCESS；
  P1-9 判「当日 2 次证据运行（1 次完整评估）→ 2/2 干净 → 可发布」
  （11:30 mid 那份为 09-16 旧格式、无 realtime 节 ⇒ 正确地不被计为完整评估轮）
- 泄露复查：audit_current.json + 2 份当日清单，持仓代码出现次数 = 0

## 五、提交对应

| Commit | 内容 |
|---|---|
| `d2908a9` | audit_project：publish_gate + P1-9 WARN 呈现 + 掩码 |
| `6d22645` | run.py 推送前置门禁 + --no-publish-gate + 21 测例 + README |
| `afb1f5a` | 审计指针与历史留档 |
| `b22de2b` | **last-run-wins 改造** + fund_evidence_complete + 5 测例 |
| `61e8a67` | 真实 post 运行验证产物 |

逃生口 `--no-publish-gate` 不静默：记 `publish_gate_bypassed` ⇒ exit=2 留痕。
关联既有机制：四层状态机（`audit_health`/`model_promotion`/`action_enable`/`production_status`）、三态退出码（0/2/1）、证据通道隔离。

## 六、遗留与下一步

- 022853/002207 两只池内基金若 shares 归零，其数据问题仍会以运行级拦全批（022853 失败无法归因到论域 ⇒ 升运行级）——这是 A 方案的刻意保守，如觉误伤可再议「非持仓池内基金降为逐节标注」。
- 推送链路（不带 --no-push）待真实飞书环境验证一次。
- 明日 mid→post 双清单自然运行后，复盘 last-run-wins 在真实抖动下的表现（看 `superseded` 是否出现）。
