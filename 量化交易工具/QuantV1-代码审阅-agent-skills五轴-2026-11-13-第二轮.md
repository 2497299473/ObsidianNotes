# QuantV1 代码审阅（agent-skills / code-review-and-quality 五轴）· 2026-11-13 第二轮

- 审阅人：OpenSquilla（skill: `agent-skills` → `code-review-and-quality`，五轴 + 严重度标签）
- 对象：`D:\PythonProject\QuantV1`（main @ `9cb675602cb3861744136d84570bf6a013d15f19`，实测 `git rev-parse HEAD`；远端 `git ls-remote origin main` 同值）
- 性质：**同日第一轮（[[QuantV1-代码审阅-agent-skills五轴-2026-11-13]]）修复批的验收复核轮**——对象 = `07046d9..9cb6756` 三个提交（fix `77b0eb6` / evidence `b02dc0d` / hygiene `9cb6756`），逐项独立复算第一轮 R-A/C-1/C-2/C-4 的验收标准，并对修复批本身做五轴过一遍。
- 纪律：全程零东财请求、零代码改动（铁律 7/8）；验证全部离线（全量测试 subprocess 直跑 + 审计 --no-write + 隔离探针，探针只写 tempdir / 内存字符串）。触网仅 GitHub 只读 `ls-remote`（非东财）。

## 裁决

**Approve**——修复批全部验收标准独立复核通过，无 Critical、无 Required 新增。1 项 Consider（新守护判据自身有一个已实证的假阴性通道，当前七域零命中，不阻塞）+ 2 项 FYI 留痕口径。

## 验证证据（本机独立实测，非转述修复报告）

| 项 | 结果 |
|---|---|
| 全量测试（本轮重跑） | `run_tests.py --all` → **Ran 1272 tests in 221.935s — OK (skipped=3)**，exit=0（subprocess 直跑口径，log 纯 UTF-8 无 BOM，sha16=83739f72dfaac850，落 `_review_full_tests_20261113_r2.log`） |
| 审计（本轮重跑） | `audit_project.py --no-write` → **23 PASS / 0 FAIL / 2 WARN**；AUDIT_HEALTH=PASS_WITH_WARNINGS；PRODUCTION=BLOCKED（model_ready=False，符合预期）；2 WARN = P1-11 运行时数据 19 项未提交（纵向证据约定常态）+ P1-2 已处置档案 ×2（D3-07 既有裁决） |
| R-A 行为复核 | 独立构造 tempdir 假日历（2099-03-05/06，不复用守护测例夹具）注入 `ap.PIT_CALENDAR` 后**裸调** `ap.pit_trading_days()` → 返回注入值 ✅，且 `_research.PIT_CALENDAR` 已同步 ✅ |
| R-A 突变复核 | 仅内存把两 helper 退回裸别名 → 守护 2 测例（注入语义 + 旧路径死亡断言）**双双变红**（2 run / 2 failures）→ 还原后 `tests.test_audit_facade` **31/31 OK**（3.272s）✅ |
| C-1 复核 | fresh 子进程不经门面直调 `evidence.check_worktree_clean` → exit=0、1 check 落账（第一轮实证的 NameError 已消失）✅ |
| C-2 复核 | `TestSyncNamesCoverage` 四项在 31 项内全绿；豁免集 13 名与门面未登记裸绑定精确相等 ✅ |
| 落档证据完整性 | `review_ra_c1c2_all_20261113.log` sha256 前 16 位 **149b1a1e4baed27f 与 meta 声明逐位相符**；meta 无 BOM；log 尾 `Ran 1272 … OK (skipped=3)` 与 INDEX 行一致 ✅ |
| 消费面 | `pit_trading_days/pit_lag_floor` 全部调用点 = 门面包装 / 域内直调 / tests（test_pit_guard ×6、test_audit_facade ×5）；包装化不改任何调用方语义 ✅ |
| 变更体量 | fix 批 6 文件 +178/−7（其中守护测例 +163），单一逻辑变更，健康区间 ✅ |

## Consider

### C-5 `test_no_undeclared_module_globals` 的 local_names 全局并集有一个假阴性通道（已实证，当前零命中）

- **位置**：`tests/test_audit_facade.py:432-437`（`local_names` 递归收集**所有**函数作用域的局部名并集）+ `:456-457`（`needed − … − local_names(top)`）。
- **机理**：名字 X 在函数 A 里是局部变量、在函数 B 里是未声明全局依赖时，X 会进 `local_names(top)` 并集而被从 `needed` 里减掉——守护放行。这正是 C-1 要堵的「用了没导入」形态，只是多了一个同名局部变量作掩护。
- **实证（内存字符串突变，仓库零改动）**：向 states.py 源码副本追加 `_mut_a`（局部 `ghost_dep=1`）+ `_mut_b`（裸调 `ghost_dep()`）→ 现行判据 undeclared=[] **放行**；per-scope 修正判据 undeclared=['ghost_dep'] **抓住**。
- **当前暴露面**：用 per-scope 判据对七个域模块全量重算，**零额外未声明名**——今天没有假绿，属守护自身的精度缺口，非现行代码缺陷。
- **建议修法**（≤15 分钟含测例）：把剔除集从「全树局部名并集」改为「**当前作用域自身**的局部名」（walk 每个 symtable 子表时用该表自己的 `is_assigned/is_parameter` 集合）；补一条「同名局部掩护」突变测例钉住（先红后绿）。
- **验收标准**：□ 上述 ghost_dep 型突变副本被新判据抓住 □ 七域现源码仍零命中（不误报）□ 全量不回归。

## FYI（留痕口径，两处，均不构成行为问题）

1. **「守护 24→31」的 24 不是任何 commit 的实数**：`07046d9`（修前 HEAD）实数 23、`9cb6756` 实数 31（AST 计数）。24 = HEAD 23 + RED→GREEN 流程中先写好的 R-A 测例 1——口径成立但 README/AGENTS/笔记三处都没写明这个「+1」，将来按 commit 对账会差 1。建议下次 docs 批次顺手补半句「（24 含先写的 RED 测例）」。
2. **meta.code_commit 与「测试时 HEAD」表述有一步之差**：收集器在 12:26:58 记录 `code_commit=77b0eb6`，但 reflog 显示 `77b0eb6` 提交于 12:31:17——测试实际跑在**未提交工作树**上（当时 HEAD=07046d9）。内容无问题（工作树与终态提交逐字节一致，第一轮已声明「双记」口径），但「meta 双记测试时 HEAD」的措辞不准：meta 记的是**终态** commit，非测试时 HEAD。属 evidence 措辞精度，历史件不改写，下次收集器把字段名改为 `terminal_commit` 或加 `head_at_run` 即可。

## 五轴小结

- **正确性**：R-A/C-1/C-2/C-4 四项验收全部独立复核通过（上表）；修复批未动因子/门禁/回测口径/冻结件，审计四层状态与第一轮终态一致。
- **可读性**：包装处注释写明动因与守护测例名；C-4 docstring 如实声明容忍面扩大。无新增晦涩结构。
- **架构**：`_check` 同款包装复用既有注入契约，未引入第二套机制；守护判据（symtable 对偶 + 豁免集钉死）方向正确，精度缺口见 C-5。
- **安全**：零新增网络面/密钥面；P0-6 内容级扫描 191 份跟踪 json/jsonl 零命中（本轮审计实测）；持仓私有字段防线无回潮。
- **性能**：包装每次调用多一次 `_sync()`（34 名 × 8 模块 setattr），微秒级，且与 25 个 check 函数同款既有开销，无新热路径。

## 未验证项（如实）

- 东财真实拉取路径（铁律 7/8 未授权不触网）：pull 族运行时行为结论上限仍为**推断级**。
- GitHub 服务端 dangling commits 处置状态未复查（11-07 已裁决接受 private 下残留，本轮不重开）。
- C-5 的 per-scope 修正判据只在探针里验证，未落为守护测例（属修复动作，待 Summer 拍板）。

## 下一步

1. Summer 拍板 C-5 是否修（建议修，≤15 分钟含测例；也可裁决「登记 backlog 不修」——当前零命中，风险低）。
2. FYI 两条留痕口径可并入下次 docs/evidence 批次，不单独开提交。
3. 第一轮遗留观察项不变：est_chg live 误差带满 30 交易日后零网络重跑刷新（面 4 遗留）。
