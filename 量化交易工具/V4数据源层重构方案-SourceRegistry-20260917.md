---
title: V4 数据源层重构方案（SourceRegistry + Fallback Chain）
created: 2026-09-17
tags:
  - 量化交易
  - 数据源
  - 架构
  - QuantV1
description: 基于 core/ 现有实现的真实接口，给出多源注册表 + 健康度 + fallback 链 + 来源标记的分步重构方案（不改 46 处调用面）
---

# V4 数据源层重构方案（2026-09-17）

> 参考：FundVal-Live `backend/api/sources/{base,registry}.py`（**AGPL，只读设计不抄码**）、
> fundviewer `data_sources/fallback.py`（MIT）。
> 目标：解决「一个数据源 = 一个函数」——现在每加一个源就要改业务函数体。

---

## 一、现状盘点（读源码得出，非印象）

| 文件 | 真实行为 | 缺口 |
|---|---|---|
| `core/data_loader.py` | 东财**单源**（pingzhongdata + lsjz）；缓存 TTL 12h；`_source` 三态：`fresh` / `cache` / `cache:fallback(异常)` | 无第二源；降级只有「读缓存」，不是「换源」 |
| `core/stock_data.py` | **已有事实上的三源链**：腾讯 → 东财 → Tushare，`source` 字段标注，`attempts` 列表累积每源失败原因 | 链**硬编码在 `fetch_stock_kline` 函数体**里；无注册表、无健康度；脚本里 `fetch_stock_kline` 被 6+ 处直接调用 |
| `core/real_time.py` | 腾讯 `qt.gtimg.cn` 单源；失败**静默返回空 dict**，调用方降级不展示 | 失败无痕迹、无备源 |
| `core/netutil.py` | 逐 IP failover（v4 优先）/ 无代理 / curl_cffi 指纹兜底 / 东财 cookie 种会话 / 瞬断重试 | **这是 V4 强项，保持不动** |
| `experiments/sina_nav_redundant/pull_sina_nav.py` | 新浪净值抓取，**仅实验脚本，未进生产链** | 真正的冗余源存量，被埋着 |
| `core/market_context.py` | 已有 primary/fallback 代理切换 + `proxy_switch` 标记 + Data Quality `degraded` | 这套「降级留痕」思想可直接复用到数据源层 |

**结论**：V4 缺的不是传输层，也不是降级意识，而是**「源」这个对象**——现在源是函数，不是可登记、可排序、可健康记账的实体。

---

## 二、目标结构（最小新增，不改调用面）

```
core/datasource/                    # 纯新增包
    __init__.py
    base.py        # Provider 协议 + FetchResult 契约
    registry.py    # SourceRegistry：登记 / 排序 / 源级超时 / 健康度
    chain.py       # fallback 链执行器 + source trace
    health.py      # 连续失败 → 降级到链尾（进程内，不落盘）
    providers/
        fund_eastmoney.py     # 迁 data_loader.fetch_pingzhongdata / fetch_lsjz
        fund_sina.py          # 提升 experiments/sina_nav_redundant/pull_sina_nav.py
        stock_tencent.py      # 迁 stock_data._fetch_page
        stock_eastmoney.py    # 迁 stock_data._fetch_stock_kline_eastmoney
        stock_tushare.py      # 迁 stock_data._fetch_stock_kline_tushare
        realtime_tencent.py   # 迁 real_time.fetch_realtime
```

### 核心契约

```python
@dataclass(frozen=True)
class FetchResult:
    ok: bool
    source: str                 # 'eastmoney' | 'tencent' | 'sina' | 'tushare' | 'cache'
    payload: dict | None
    error: str | None           # 单源失败原因，保持现有 attempts 的可诊断性
    latency_ms: int
```

---

## 三、四条硬约束（违反即回滚）

1. **调用面冻结**：`load_fund(code, force_refresh)`、`fetch_stock_kline(code, market, ttl_hours, min_start)`、
   `fetch_realtime(holdings)`、`weighted_estimate(holdings, quotes)` **签名一律不动**。
   已核实 46 处调用点（`run.py`、16 个 `backtest_*.py`、`lookthrough.py`、`market_context.py`、
   `experiments/forecast_lab/refresh_panel_cache.py` 等）。第一版**只换内部实现**。
2. **`_source` 口径不倒退**：`fresh` / `cache` / `cache:fallback` 三态必须保留（`report_generator.py`
   已按 `cache:fallback` 前缀出显式 ⚠️ 告警），新增源名以 `source:<name>` 追加，**不改旧语义**。
3. **零网络纪律**：shadow 路径只读 `forecast_outputs/samples_frozen_*.jsonl`，**不得因重构回退到 `load_samples`**
   （否则撞缓存 TTL 会触发前复权 K 线重取，尺子漂移，09-10 实证 −0.121）。
4. **东财频控**：SourceRegistry 必须带「源级串行 + 时间盒」，并遵守项目铁律 7——**发东财请求禁止定时器自动开跑**，
   须人显式授权 + 四项征兆闸门。多源重构**不得**变成「自动多路并发打东财」。

---

## 四、迁移步序（每步可回滚，做完即验）

> **进度（2026-09-17）：步 1~5 代码全部落地；唯一遗留＝步 4「真实东财失败→回落」待授权对拍**
>
> **步 1**——`QuantV1/core/datasource/{__init__,base,registry,chain,health}.py`
> + `providers/` 空目录 + `tests/test_datasource_chain.py`（22 测，含「providers 不得有实现 /
> 包内不得 import 网络库 / 旧模块 import 面不变」三道守护），登记进 `tests/layers.py` fast 层；
> fast 全量 229/229 通过，旧路径零改动。
>
> **步 2**——新增 `core/datasource/symbols.py` 与 `providers/stock_{tencent,eastmoney,tushare}.py`；
> `base.py` 补失败前缀契约（`network:` / `data:` / `skip:` / `protocol:`）与 `classify_exc()`
> （按 OSError 家族 + MRO 类名判定，**不 import 网络库**）；`core/stock_data.py` 退化为
> **装配点 + 缓存层**（对外签名 / 返回形状 / 缓存 schema / 全灭逐源原因文本一字不变）。
> 验证：provider 离线测例 21/21、fast 全量 252/252、耦合测试 57/57。提交已推 QuantProject。
> 另补零网络回读核验（`ttl_hours=1e9` 强制命中缓存分支，不发任何请求）：
> **247 个真实生产缓存全部 json 往返恒等（0 不匹配）、行宽均为 5 列、日期严格升序**；
> 源分布 tencent 197 / eastmoney 50，均与迁移前口径一致。
> **✅ 17:20 真实逐根对拍已跑，步 2 验收全部关闭**（Summer 17:14 显式授权发东财请求；
> 当时 17:17 已过 11:20 时间盒，G1/G2/G3 全 PASS，G4 由授权人明确破例——记录在案）。
> 脚本 `experiments/datasource_parity/parity_step2_real_20260917.py`：旧实现（f0668e5 函数体
> 原样、去写盘）vs 新 providers **同参数真实拉数**，标的 600519(沪)/000651(深)/000858(深)
> 三只非科创板：
> - **tencent 3×3196 根、eastmoney 2847/2690/2787 根，全部逐根恒等（0 不一致）**，双源 6/6 PASS；
> - 东财实际请求 6 发（旧 3 + 新 3），单发不重试、全程无频控征兆；
> - 生产缓存零扰动：脚本不落盘，三标的缓存 mtime 仍为 09-16、sha256 与备份逐字节一致
>   （备份 `output/backup_stock_klines_20260917/`，可删）。
> **✅ 17:55 步 3 落地**——新增 `providers/realtime_tencent.py`（解析逻辑逐字平移，
> 「拉出 0 条」按 `data:` 失败处理）；`core/real_time.py` 退化为装配点（category
> `realtime_quote`，链上暂仅腾讯一源）。**唯一语义升级（本步验收点）**：失败不再静默
> `{}`，改返 `{"_error": "network:…|data:…"}`——`_error` 非 6 位代码键，
> `weighted_estimate` 天然免疫 → 走既有 `est_change_pct is None` 降级分支，run.py 零改动。
> 验证：离线测 9/9（`tests/test_realtime_datasource.py`，stub netutil）+ fast 全量
> **261/261** + 真实冒烟（qt.gtimg.cn 腾讯域、不受铁律 7 约束，总共仅 2 发：
> 旧 1 + 新 1）600519/000651/000858 **3×6 字段 + 加权估计逐项一致**；
> 脚本 `experiments/datasource_parity/smoke_step3_realtime_20260917.py`。
> **✅ 18:05 步 4 代码+离线层落地**（真实东财回落验收待人授权，见下）——新增
> `providers/fund_eastmoney.py`（pingzhongdata 解析逐字平移；唯一语义升级：空序列按
> `data:` 失败处理，旧单源下会静默出空 fresh）与 `providers/fund_sina.py`（升自
> `experiments/sina_nav_redundant/pull_sina_nav.py`——通路 09-08 实证 + 每日校验背书，
> 传输改走 netutil，不落盘不校验，纯取数）。`core/data_loader.py` 退化为装配点：
> `load_fund`/`fetch_pingzhongdata`/`fetch_lsjz` **签名与 `_source` 三态一字不改**，
> 全链（东财→sina 严格串行）失败才 `cache:fallback`；fresh 新增**附加键**
> `source`=实际源名（步 5 展示层用，不改三态取值域）；`fetch_lsjz`（申赎，东财 f10 独有）
> 保持单源不迁移。
> 验证：离线 15 测（链成功/回落/全灭降级/缓存 TTL 命中/失败分类；`fetch_lsjz` 已 stub
> 保证零网络）+ fast 全量 **276/276** + 真实冒烟（**仅新浪域、零东财请求，不受铁律 7
> 约束**）：provider vs 今日 16:01 脚本产物 002112 **2644 条逐条恒等**、
> sina×东财缓存交叉 mismatch=0。脚本 `experiments/datasource_parity/smoke_step4_sina_20260917.py`。
> ⚠️ **步 4 验收未全关**：表内「东财失败时回落新浪、`_source` 正确标注」需构造东财真实
> 失败或对拍，发东财请求受铁律 7 约束 → **待明日 09:30 后人授权**（四闸门 + ≤6 发）。
> **✅ 18:40 步 5 落地，重构方案 5 步代码全部完成**——`signal_engine` 透传 `source`
> 附加键；`report_generator` 新增 `source_trace_notes()` 作**单一事实源**（落盘报告与
> 飞书卡片共用，防两处判定漂移）：fresh 且实际源≠东财 → 显式「ℹ️ 数据源切换」；
> `notify._build_card` 接入同一函数。`data_loader` 在 cache/cache:fallback 命中时
> **剥除缓存里的 stale `source` 键**（对本次加载已失真，不得混进展示层）。
> 验证：新增 10 测（三态回归 + 共用判定 + 卡片含切换行 + stale 剥除）+ fast 全量
> **286/286**，零网络。提交 QuantProject `a431da7`。
> ⚠️ **全局唯一遗留**：步 4「东财真实失败→链回落 sina」真实验收待发东财请求，
> 明日（09-18 周五）09:30 后过四闸门 + 人授权再跑（≤3 发）；届时步 5 的换源提示
> 会自然在报告中亮相（若真发生回落）。

| 步 | 动作 | 风险 | 验收 |
|---|---|---|---|
| 1 | 建 `core/datasource/` 骨架（base/registry/chain/health），**不接任何 provider** | 零（纯新增） | import 通过，旧路径行为不变 |
| 2 | ~~迁 `stock_data.py` 三源 → providers~~ **✅ 2026-09-17 已落地** | 低（三源已存在，语义平移） | 离线层已钉（provider 21 测 + fast 252 全过）；**真实逐根对拍待授权** |
| 3 | ~~迁 `real_time.py` 腾讯源 → provider，失败**留痕**而非静默~~ **✅ 2026-09-17 已落地** | 低（单源） | ✅ 断网返回 `{"_error": …}`（离线 9 测钉死）+ 真实冒烟 6 字段一致 |
| 4 | 迁 `data_loader.py` 东财源 → provider；`pull_sina_nav.py` 升 `fund_sina` 接链 **🟡 2026-09-17 代码+离线+新浪冒烟已落地** | **中**（首次真正多源） | 东财失败时可回落新浪，`_source` 正确标注——**真实回落验收待授权**（铁律 7） |
| 5 | ~~报告层加 source trace，复用 `report_generator.py` 现有 `cache:fallback` 告警分支~~ **✅ 2026-09-17 已落地** | 低 | ✅ 落盘报告 + 飞书卡片显示实际来源（共用 `source_trace_notes` 判定） |

**测试**：新增 `tests/test_datasource_chain.py`——每源的 成功 / 失败 / 超时 三种链行为 + source trace 正确性。
现有 `tests/test_netutil.py` 不动。

---

## 五、明确不做

- ❌ 不重构 `core/netutil.py`（传输层已过硬，动了就是自伤）。
- ❌ 不引入 Django / Celery / Web UI（FundVal-Live 那套是 AGPL 且对 V4 无用）。
- ❌ 不把 Tushare 拉进任何决策备选（MEMORY.md 已定：Tushare 积分通道不在决策备选范围）。
- ❌ 第一版不落盘健康度（进程内即可，避免新增状态文件污染 `data/`）。
