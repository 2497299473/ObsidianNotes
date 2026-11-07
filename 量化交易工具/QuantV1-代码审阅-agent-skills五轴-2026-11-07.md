# QuantV1 代码审阅（agent-skills / code-review-and-quality 五轴）· 2026-11-07

- 审阅人：OpenSquilla（skill: `agent-skills` → `code-review-and-quality`，五轴 + 严重度标签）
- 对象：`D:\PythonProject\QuantV1`（main @ f1f1bcd4，工作区 9 项运行时数据未提交改动，无代码改动）
- 范围：QuantV1 全仓静态审查 + 离线验证。`QuantV1-tfm` 核实为纯 venv（无代码，不审）；`QuantV1-review-bundle` 为历史审阅包快照（不审）；`_quantv1_rebase_quarantine_20261013` 为隔离区（不审）。
- 纪律：全程零东财请求（铁律 7/8）；未改任何项目代码；触网仅 GitHub 只读核查（非东财，不属铁律 7 管束面）。

## 裁决

**Request changes** —— 工程质量整体良好（1171 测试全绿、审计 0 FAIL），但存在 1 项正在持续泄露的隐私缺陷（C-1），必须先修。

## 验证证据（本机实测，非转述）

| 项 | 结果 |
|---|---|
| fast 层 | `Ran 941 tests in 79.9s — OK`（55 文件，0 skip） |
| slow 层 | `Ran 230 tests in 141.9s — OK (skipped=3)`（3 skip = test_holdings_visibility 的 RUN_SLOW_TESTS 门控，符合 tests/README 声明） |
| audit_project --no-write | 18 PASS / 0 FAIL / 6 WARN；AUDIT_HEALTH=PASS_WITH_WARNINGS；PRODUCTION=BLOCKED（model_ready=False，符合预期） |
| 静态扫描 | eval/exec=0、shell=True=0、裸 except=0、verify=False=0、硬编码密钥=0 |

## Critical

### C-1 影子流水把真实持仓（shares/cost_nav）写进公开仓库，且仍在每天新增

- **事实链（全部已核实）**：
  1. GitHub 仓库 `2497299473/QuantProject` **匿名可访问（PUBLIC）**（主页 200 + raw 直读 200 双重确认）。
  2. `shadow_policy.py:561-563` 把 `holdings.json` 的 `shares`/`cost_nav` **原样内嵌**进每条 shadow 记录。
  3. `output/shadow_actions.jsonl`（git 跟踪）132 条中 **66 条 shares≠0 且 cost_nav 非空**（002112/025687 两只，2026-09-14 → 2026-11-06 共 33 个交易日）；HEAD 版本 64 条非零，工作区未提交增量又添 2 条。
  4. `backups/shadow_actions_pre_legacy_invalid_20260911.jsonl`（git 跟踪）36 条中 18 条同样非零。
  5. **公开 raw 实测**：`raw.githubusercontent.com/.../main/output/shadow_actions.jsonl` 首条记录 holdings 字段齐全、shares 非零——线上此刻可读。
- **加重情节**：`holdings.json` 文件头自带「勿提交/勿分享」声明；09-16 曾做过一轮隐私清理（4fbba1e 退出跟踪 3 份 holdings.json），但 journal 内嵌副本漏网；此后每次 evidence 提交都在扩大泄露面（最近一次 ce8256d，2026-11-05）。
- **为何审计没拦**：P0-2/P2-4 只查「文件名入库 + config 禁键」，不查跟踪内容里的持仓语义字段（`audit_project.py:317-370`）。测试盲区佐证：`test_shadow_policy.py` 对 holdings 字段零断言。
- **消费者核查**：全仓 grep 无任何代码读取 journal 记录的 holdings 字段（纯留档）→ 可安全改为只留 `as_of_date` 或直接去掉该块。
- **修复建议（需 Summer 授权后执行）**：
  1. 改 `shadow_policy.py:561`：holdings 块只保留 `as_of_date`（或整块删除），补一条「journal 不得含 shares/cost_nav」守护测试；
  2. 就地清洗两份 jsonl 的 holdings 块（本地留原件备份）；
  3. 历史清除：`git filter-repo` 洗 `output/shadow_actions.jsonl`、`backups/shadow_actions_pre_legacy_invalid_20260911.jsonl`、3 份历史 holdings.json、`output/zcode_runs/2026-09-08.md`（含真实代码）→ **force push 属破坏性操作，先请示**；或先例选项：直接把仓库转 private（立即止血，历史仍在但不再公开）；
  4. 审计加一条内容级检查：跟踪的 jsonl 抽样断言无 `shares`/`cost_nav` 非零值。
- **不修复的替代**：若判定「份额/成本可公开」则降级为 FYI——但与文件自述及 09-16 清理动作矛盾，不建议。

## Required（无标签 = 应修）

### R-1 重复 helper 继续扩散，违反项目自身 ponytail lite 规则

- AGENTS.md 记录（10-01 快照）：pearson ×3、_window_slices ×3、spread ×2、_fit_one_horizon ×2，规则明令「先提公共、不再复制粘贴」。
- 本次实测：pearson **×4**（`audit_candidate_flips.py:127` 于 10-20 新增第 4 份拷贝，晚于规则生效）；backtest_*.py 18→**21** 个、`__main__` 样板 ×21。
- AST 对比：factors/fusion/lookthrough 三份 pearson 完全等价（可直接收敛）；audit_candidate_flips 版本有差异、_window_slices 三份互不等价（收敛前须先对齐语义，不能盲合并）。
- 建议：等价三份先提 `core/`（或 backtest 公共模块）；不等价的标注差异原因；`# ponytail:` 注释补上限说明。属工具/胶水层，不碰「不碰清单」。

## Consider / Optional

- **O-1 依赖老化**：`requests==2.31.0`（CVE-2024-35195 verify=False 会话复用绕过 / CVE-2024-47081 .netrc 泄露，均 2.32.x 修复）。生产链路零使用（仅 experiments/chanlun 6 个探针 import），实际暴露面小。建议升 2.32.x 并单独提交、跑绿 fast+slow（依赖纪律：一次一个包）。
- **O-2 大文件**：`audit_project.py` 1908 行、`core/model_registry.py` 1168 行，超 1000 行检查信号。下次再长之前先拆（audit 可按 P0/P1/P2 轴拆模块）。本轮不动。
- **O-3 netutil v5 cookie 死路径**：docstring 自认「cookie 假设被否证、死代码无害保留」，但每次东财请求仍先 prime 一次主页 GET（300s 退避）。属已知留档决策，仅提示：若确认永不回松可删 prime 链路省真实发包。

## FYI

- 审计 6 WARN 均为运维时效项：数据快照 50 天未重生成、drift 报告 60 天、release tip 落后、工作区 9 项未提交（全是运行时数据流，符合「纵向证据入库」约定）、当日无证据运行（周六，正常）。
- pickle 反序列化 5 处全部先过 SHA 校验（生产 3 处走 registry `read_verified_model_bytes`；score_expert_a 走 sidecar meta sha——同目录同信任域，可接受）。fail-closed 设计到位。
- 隐私基建正面项：run_manifest 基金代码掩码（assert_no_fund_codes）、`tracked_path` 防绝对路径入库、崩溃兜底「宁缺证据不泄露」注释与实现一致。
- 第一次 fast 跑测 shell 报 exit=1 为 PowerShell NativeCommandError 包装假象；Python subprocess 直跑 exit=0 全绿。后续留档以 subprocess 口径为准。
- 工程亮点：测试分层 fail-closed（磁盘/登记漂移即红）、零网络守卫含 curl_cffi C 层旁路封堵 + selftest、熔断器/wire 计数与铁律条文互相引用、原子写 + 三态运行清单。

## 未验证项（如实）

- GitHub 侧是否有 fork/clone 已带走历史：无法从本机核实；已按「已泄露」处理（份额成本视为公开过近两个月）。
- slow 层测试对真实冻结件只读复算通过，但 G-B K线指纹 DRIFTED 属预期留档（尺子漂移声明），不构成本轮新问题。

---

## 处置记录（2026-11-07 21:0x Summer 三步全批，当晚执行完毕）

### ① 止血：仓库转 private ✅

- GitHub API PATCH `private=true` 成功（before False → after True）；匿名 raw 复测 **404**（不可再匿名读取）。
- 凭证经 git credential fill 内存中使用，未落盘未打印。

### ② 修代码 ✅（3 个主题提交，全量 1185 OK (skipped=3)）

1. `shadow_policy.py`：holdings 块只留 `as_of_date`；新增 `HOLDINGS_PRIVATE_KEYS` + `holdings_leak_paths()` + `append_records` 写入门 fail-closed（异常只报字段路径不带值）。守护测试 ×5。
2. `audit_project.py`：新增 **P0-6 内容级检查**（扫全部跟踪 json/jsonl，禁键命中 FAIL，detail 只写路径不写值；*.example 白名单；不可解析 WARN）。守护测试 ×9（含 AST 钉住两端禁键清单一致）。实测：清理前 FAIL 402 处 → 清理后 PASS（176 份）。
3. 存量清理：三条流水（活跃 132 / legacy 36 / 备份 36）剥离 shares/cost_nav 共 408 字段，行数/顶层键集不变；原件备份 `_scratch/c1_scrub_backup_20261107/`。
4. 追加发现并修：`output/backup_daily_runs_20260918/` 两份 md 含真实代码×运行上下文，撤出跟踪 + gitignore（10-04 裁决当时漏网）。
- 误报修正留痕：首版禁键清单误列 `max_position_pct`/`consecutive_adds`（实为 config 公开风险参数），实测 FAIL 后查证修正，两处清单同步。

### ③ 洗历史 ✅（两轮 filter-repo + force push，全分支）

- **轮 1（blob 级）**：重写三条流水全部历史版本，剥离禁键字段。
- **轮 2（路径级）**：删除历史中 `holdings.json` ×3、`output/zcode_runs/`、`output/daily_runs/`、`output/backup_daily_runs_20260918/`（基金代码本身是公开研究论域保留；删的是代码×运行上下文的持仓关联证据）。
- **核验**：全历史 1121 blobs 扫描 **0 残留**（判据：非零 shares / 非零 cost_nav）；敏感路径 git log 全分支 0 命中。
- **推送**：8 分支 force push 成功，远端 main = 本地 main（aa121d6）；forks=0。
- **回滚保险**：重写前全量 bundle ×2 在 `_scratch/c1_pre_rewrite/`（含完整旧历史，**勿入库勿外发**）。
- 前置处置：3 个 _scratch worktree 已移除（r4fix 仅 2 个未跟踪探针脚本，随目录保留）；运行时数据改动 stash → 重写 → pop 还原。

### 残留风险（如实，需 Summer 知悉）

1. **GitHub 服务端 dangling commits**：旧 tip（3c78e03 等）经 API 按 SHA 仍可读（GitHub 不立即 GC 不可达对象）。仓库已 private + forks=0，暴露面≈0；彻底清除需 GitHub Support 工单或删库重建（当前 token 无 delete 权限，403）。**建议：提 Support 工单请求 purge，或接受 private 下的残留。**
2. **已泄露定性不变**：公开期近两个月，无法排除被 clone；份额/成本应按已公开处理（后续若介意可考虑换仓位移仓，但那属投资决策不在本次范围）。
3. **所有历史 commit SHA 已变**：registry/evidence 里绑定的旧 git_commit 在新历史不可解析（审计 P1-12 类对账会报漂移）——这是历史重写的必然代价，旧→新映射在 filter-repo 的 commit-map（`.git/filter-repo/commit-map`）可查。
4. R-1（重复 helper）/ O-1（requests CVE）本轮未动，仍开放。