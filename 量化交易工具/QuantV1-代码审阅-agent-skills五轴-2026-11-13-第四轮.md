# QuantV1 代码审阅（agent-skills / code-review-and-quality 五轴）· 2026-11-13 第四轮

- 审阅人：OpenSquilla（skill: `agent-skills` → `code-review-and-quality`，五轴 + 严重度标签）
- 对象：`D:\PythonProject\QuantV1`（main @ `c8e889d52a4eda45e3ec6a497480104f212f1b4e`，实测 `git rev-parse HEAD`；远端 `git ls-remote origin main` 同值，无分叉——首次尝试遇瞬时 reset，重试即通）
- 性质：**第三轮（[[QuantV1-代码审阅-agent-skills五轴-2026-11-13-第三轮]]）Approve 后的首个功能提交审阅轮**——对象 = `6ec7285..c8e889d` 单提交（feat：012863 入池 + 穿透经 proxy_map 改道目标 ETF 159796，方案 B，Summer 拍板）。4 文件 +130/−1，生产代码改动仅 `core/lookthrough.py` 17 行 + config 7 行。
- 纪律：全程零东财请求、零生产代码改动（铁律 7/8）。验证全部离线：全量测试 subprocess 直跑落档 + 审计 `--no-write` + 零网络探针（`netutil.http_get` 全程 mock 成炸弹，任何真实网络尝试立即炸；突变实验缓存重定向 tempdir，真实文件零触碰）。

## 裁决

**Approve**——单点小改道，无 Critical、无 Required、无新增 Consider。2 项 FYI 留痕（release tip 待收口 / 新池首次生产运行待观察）+ 既有 Nit/FYI-1 维持原裁决不重开。

## 验证证据（本机独立实测，非转述提交信息）

| 项 | 结果 |
|---|---|
| 全量测试（本轮重跑） | `run_tests.py --all` → **Ran 1286 tests in 248.720s — OK (skipped=3)**，exit=0；76 个测试文件；与第三轮 1276 的差 = 新增守护 10 测例，账目吻合。落档 `D:\PythonProject\_review_full_tests_20261113_r4.log`（sha256 前 16 位 `502eb9f0126a4fd3`，59,178 B；skip=test_holdings_visibility RUN_SLOW_TESTS 门控 ×3，既有口径） |
| 审计（本轮重跑） | `audit_project.py --no-write` → **21 PASS / 0 FAIL / 4 WARN**；AUDIT_HEALTH=PASS_WITH_WARNINGS；PRODUCTION=BLOCKED（model_ready=False，预期）；PUBLISH_GATE=BLOCKED(no_evidence_today)。4 WARN 逐项归因见下节，**无一是本提交引入的代码回归** |
| 守护测例 | `test_lookthrough_proxy.py` 10/10 绿（映射/接线/PIT/生产配置契约四组）；fast 层登记正确（P2-3：76 文件 76 登记，四环完整） |
| 突变实验 A（改道失效） | mock `proxy_code` 为 identity → `TestHoldingsHistoryReroute` 下 **`test_mapped_fund_fetches_target` 必红**（run=3 fail=1）；恢复后 10/10 绿——守护对本提交核心行为真有牙 |
| 突变实验 B（目标码改错） | proxy_map 指向 999999 + 缓存目录重定向空 tempdir → 缓存 code 校验 miss → 网络炸弹响 **21 次**（7 年 × 3 试）→ 缺年告警点名 `holdings_history(999999)` + 返回空——改道决定缓存键/请求码实证；契约行为正确（异常不上抛、转告警，lt_missing 降级链可接住） |
| 池指纹复核 | `fund_pool_fingerprint` 实测 **6921a728** 与提交声明逐位相符（pool=002112/002207/012863/022853/025687） |
| 端到端声明离线复核 | 真实缓存（159796_2022..2026，TTL 内零请求）读回 **19 期**持仓史；`effective_snapshot(2026-11-13)` = **2026-09-30 期**（生效日 2026-10-30，30 天滞后，PIT 口径与提交声明一致）；Q3 头号重仓宁德时代 300750 9.6% |
| 无映射基金不变性 | 002112 持仓史原路径仍 27 期可得；守护测例钉死 fetch 收到原码；`proxy_code` 对四只老基金恒等返回 |
| 消费端键控核查 | 报告侧 `run.py:534/642`、`report_generator:169`、回测侧 `backtest_{chanlun,fusion,lookthrough,spread}` 全部以**原基金码**键控/迭代 `fund_pool`——改道只发生在 `holdings_history` 入口一处，`evaluate_lookthrough` 的 `agg["fund"]=code` 保持原码，无旁路直调 `fetch_holdings_year` 的生产路径（grep 全仓核实，experiments/backups 不在生产链） |
| 联接基金微量持仓实证 | 012863 自身 F10 缓存：2024 三期前十合计 **0.55/0.50/0.52%**、2025/2026 EMPTY_CONFIRMED——「直接穿透会拿陈旧微量持仓算误导性估算」的动因成立（微量 pct 在 aggregate 里按 w_valid/wsum 归一，权重被放大到 100%，确实不触发 lt_missing） |
| 净值数据面 | `data/klines/012863.json`（32,811 B，11-13 15:59 落盘）——新基金净值缓存已就位，入池不是空壳 |
| 探针脚本 | `D:\PythonProject\_scratch\review_r4_probe.py`，11/11 OK（含上述指纹/PIT/突变/守护复跑） |

## 审计 4 WARN 逐项归因（均非本提交回归）

1. **P1-11 工作区 21 项未提交**：全部为运行时数据（intraday_features/market_context/audit_history/run_manifest 等，11-10~11-13 定时任务产物）——纵向证据约定常态，与第三轮同性质。
2. **P1-2 已处置档案 ×2**：D3-07 既有裁决，维持。
3. **P1-9 no_evidence_today（新增 WARN，时间口径）**：本轮审计跑在 **11-14 00:03**（跨零点）。固定 `now` 探针实证：`publish_gate(now=11-13 23:50)` → **ok=True，3/3 只干净可发布**；`now=11-14 00:03` → no_evidence_today。11-13 当日 mid/post 两次运行均 SUCCESS 零降级（manifest 实读），门禁本身无恙——纯「当日」glob 跨零点失配，2026-09-18 gate.py docstring 已记载的已知边界。**不是缺陷，不立案。**
4. **P1-12 release tip 落后（新增 WARN，待收口）**：指针=`eb0c571`（第三轮收口），HEAD^=`6ec7285`——本提交是收口后首个功能提交，指针尚未 re-close。见 FYI-1。

## FYI（留痕，无需本轮行动）

1. **FYI-1 · release tip 待收口**：按既有节奏，下次 evidence/hygiene 批次把 `evidence/release_code_tip.txt` re-close 到 `c8e889d` 即可转 PASS。远端同步已核：QuantV1 远端 main = `c8e889d` 与本地一致（ls-remote 实测）；本笔记已提交并推送 ObsidianNotes（`a9521b1..682b12e`）。
2. **FYI-2 · 新池首次生产运行待观察**：提交于 11-13 17:24，当日 mid(11:30)/post(14:55) 两班均跑在旧 4 基金池（manifest `n_funds=4`、池指纹 `f19f8e7d`）。**5 基金新池的首次生产运行 = 11-14 11:30 mid**。提交信息的端到端实测（覆盖 100%/综合 0/防狼术 True）是盘后手动验证，非生产班次。建议 11-14 mid 后看一眼 manifest：`n_funds=5`、`pool_sha256_8=6921a728`、lookthrough.missing 为空、012863 无 lt_missing。
3. **既有遗留不重开**：第三轮 Nit（test_audit_facade.py:82-83 空行，本轮实读仍在）与 FYI-1（24→31 的 +1 口径）维持「并入下次 docs/hygiene 批次」原裁决；est_chg live 误差带满 30 交易日重跑、PR-20260913-01 到期 **2026-12-31** 三选一留痕，均不变。

## 五轴小结

- **正确性**：改道语义、PIT 口径、无映射不变性三条全部独立实证（上表）；突变 A/B 证明守护测例对「改道失效」「目标码错」两类回归必抓；1286 全量绿。微量持仓动因有缓存数据实证支撑，非臆断。
- **可读性**：`proxy_code` 17 行带完整 docstring（动因/单点改道/无映射行为），`holdings_history` docstring 同步更新；`_note` 文档键约定与 config 既有风格一致且有测例钉死。无新嵌套、无聪明技巧。
- **架构**：**单点改道是本提交最大的优点**——映射只在 `holdings_history` 入口生效一次，报告/回测/个股 universe/缓存键/告警自动同源，消费端零改动（grep 全仓核实无旁路）；`proxy_map` 放 config 而非硬编码，目标码经季报核实且有生产配置契约测例防手滑。概念数净增 1（映射表），换来的是消除「联接基金穿透失真」整类问题——复杂度是挣来的。
- **安全**：零新增网络面（改道后请求数不增：012863 的 7 年请求转为 159796 的 7 年，缓存命中后归零）；零密钥面；P0-2/P0-6 全 PASS（192 份跟踪 json/jsonl 内容级扫描零命中）；目标 ETF 不入 fund_pool 有测例钉死（防代理码混入持仓链）。
- **性能**：`proxy_code` 每次调用读一遍 config.json（`_cfg()` 既有模式，非本提交引入的新模式）；每基金每年一次、毫秒级，无热路径影响。新池 +1 基金 = 首跑 +7 个 F10 请求 + 前十个股 K 线，一次性，缓存后归零——东财 IP 频控预算内（铁律 7 由生产班次自身闸门管）。

## 未验证项（如实）

- 东财真实拉取路径（铁律 7/8 未授权不触网）：159796 缓存是提交者 17:05 实拉的产物，本轮只读复核，未重拉。
- 5 基金新池的生产班次表现（首班 11-14 11:30，见 FYI-2）。
- 本轮全量测试落档在工作区根 `_review_full_tests_20261113_r4.log`（与历轮 `_review_*` 同位置同约定），未入 QuantV1 evidence 目录——落档件以本笔记哈希声明为准。

## 下一步

1. 无待拍板项——本提交 Approve。FYI-1 的 tip re-close 并入下次 evidence/hygiene 批次（连同第三轮 Nit/FYI-1）。
2. **11-14 11:30 mid 后看新池首班 manifest**（FYI-2 三指标），这是本轮唯一值得主动盯的点。
3. PR-20260913-01 到期日 2026-12-31 不变。
