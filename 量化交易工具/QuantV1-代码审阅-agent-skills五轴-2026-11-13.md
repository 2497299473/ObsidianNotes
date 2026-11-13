# QuantV1 代码审阅（agent-skills / code-review-and-quality 五轴）· 2026-11-13

- 审阅人：OpenSquilla（skill: `agent-skills` → `code-review-and-quality`，五轴 + 严重度标签）
- 对象：`D:\PythonProject\QuantV1`（main @ `07046d9e3c543db486ed4d8f1cbe5ce4ff4a8b22`，实测 `git rev-parse HEAD`）
- 性质：**11-07 五轴审阅的续轮**——上轮 C-1/R-1/O-1/O-2 已全部处置（见 [[QuantV1-代码审阅-agent-skills五轴-2026-11-07]]）；本轮聚焦上轮之后新增的 **R-2 收敛（原子写 + 交易日历）与 R-3 拆分（audit 门面 + core/audit 八域）** 及其收尾修复（`af34c3c^..07046d9`，11-09 全天 9 个代码提交）。
- 纪律：全程零东财请求、零代码改动（铁律 7/8）；验证全部离线（全量测试 + 审计 --no-write + 隔离子进程探针，探针只写 tempdir）。

## 裁决

**Request changes**（轻量）——R-2/R-3 主体质量高：50 个函数 AST 逐字节搬运核实、1264 测试全绿、审计 0 FAIL。但 R-3 的「门面注入」契约存在 **1 项已实证的旁路（Required）** 与 2 项 Consider 级结构问题；无 Critical、无隐私/授权回归。

## 验证证据（本机实测，非转述）

| 项 | 结果 |
|---|---|
| 全量测试 | `run_tests.py --all` → **Ran 1264 tests in 204.046s — OK (skipped=3)**，exit=0（skip=test_holdings_visibility RUN_SLOW_TESTS 门控 ×3，符合 tests/README） |
| 审计 | `audit_project.py --no-write` → **21 PASS / 0 FAIL / 4 WARN**；AUDIT_HEALTH=PASS_WITH_WARNINGS；PRODUCTION=BLOCKED（model_ready=False，符合预期）|
| 4 WARN 定性 | P1-11 工作区 17 项未提交（全是运行时数据流，符合纵向证据约定）；P1-12 release tip 指针落后（见 C-3）；P1-2 历史绑定已处置档案 ×2（D3-07 既有裁决）；P1-9 当日无证据运行（审阅时点 11:0x，当日 mid 尚未跑，正常） |
| 拆分保真 | R-3 前后同名函数 AST dump 对比：**50 个逐字节一致**；唯一差异 `check_release_code_tip`（`07046d9` 有意修复，带 6 新测例）；域模块零新增函数 |
| 消费面 | run.py 实际 import：`assert_no_fund_codes / mask_manifest_funds / publish_gate` + 属性 `is_trading_day / last_expected_run_date / load_json / refresh_current / AUDIT_CURRENT`——逐一实测语义完好 |
| 11-09 落档证据 | `review_all_final_20261109.log` sha256 前 16 位与 meta 声明**逐位相符**（2486acd56b9e1d1e）；meta UTF-8 无 BOM 无 mojibake |

## Required（无标签 = 应修）

### R-A `pit_trading_days` / `pit_lag_floor` 是门面裸别名，注入契约对它们不成立（已实证）

- **位置**：`audit_project.py:265-266`（`pit_trading_days = _research.pit_trading_days`，直接绑定）；`core/audit/research.py:33-47`（函数体读模块级 `PIT_CALENDAR`）。
- **事实链（全部已核实，隔离子进程复现）**：
  1. R-3 的注入契约是「对 `ap.*` 赋值 → 下次经门面调用时 `_sync()` 推给域模块」。25 个 check 函数都走 `_check` 包装，契约成立（守护测试 7 项注入测例钉住）。
  2. 但 `pit_trading_days` / `pit_lag_floor` 是**裸别名**（与 `load_json` 等不同：后者在 `_SYNC_NAMES` 里，前者不在）。测试若按既有手法 `ap.PIT_CALENDAR = fake` 后**直接调 `ap.pit_trading_days()`**，不经过任何 `_check` 包装 → `_sync` 不触发 → 函数仍读域模块里的**旧真实日历**。
  3. 实测：注入 `2099-01-02` 假日历后直接调门面别名，返回的是真实 510300 日历（2013-07-29 起 3000+ 行）；`ap._sync()` 后才返回注入值。**同一注入手法，check 函数生效、这两个函数不生效**——契约不一致。
- **当前暴露面**：`tests/test_pit_guard.py:59,80` 恰好只读真实日历（不注入），所以今天没有假绿；这是**埋着的测试幻觉通道**——下一个想注入 PIT_CALENDAR 的测例会静默测错对象。
- **建议修法**（二选一，都很小）：① 把这两个名字改成 `_check` 同款转发包装（`functools.wraps` + 先 `_sync()`）；② 或把它们加进一个「调用前同步」的轻量包装并在 `test_audit_facade.py` 补一条注入测例钉住。
- **验收标准**：□ `ap.PIT_CALENDAR = <tempdir 假日历>` 后直接调 `ap.pit_trading_days()` 返回假日历内容 □ 新增测例先红（现实现）后绿（修后）□ 既有 1264 全量不回归。

## Consider / Optional

### C-1 域模块「占位 None + 运行时注入」使静态可读性变差，且漏配即 NameError（已实证一例）

- `core/audit/evidence.py:325` 调 `git_worktree_clean()`，但该名字**既不在 evidence 的 import 清单、也不在占位 None 清单**——只靠 `_sync()` 无条件 setattr 补上。实测：fresh 进程里直接调 `evidence.check_worktree_clean(a)` → `NameError: name 'git_worktree_clean' is not defined`；`_sync()` 后才可用。
- 生产路径全部经门面，**无运行时风险**；但「导入面收紧为显式清单」的守护（TestExplicitImports）只查「导入了没用的」，不查「用了没导入的」——两个方向只堵了一半。读代码的人（或未来拆分者）无法从 import 清单推断依赖全集。
- 建议：给 TestExplicitImports 加对偶判据——域模块自由变量 ⊆（import 名 ∪ 模块级赋值 ∪ `_SYNC_NAMES` ∪ builtins），把「隐式依赖」显式化；顺手把 `git_worktree_clean` 加进 evidence 的占位注释清单。零行为改动。

### C-2 `_SYNC_NAMES` 是手工维护的 34 名清单，与门面属性集无一致性守护

- `audit_project.py:224-232`。R-A 正是这个清单漏了名字的实例。新增门面属性时忘登记 → 静默不注入（不报错）。
- 建议：守护测试加一条——门面公开可调用/常量名（排除明确豁免的 stdlib 重导出如 `Path/datetime`）⊆ `_SYNC_NAMES` ∪ 包装函数集；或反向：`_sync` 改为遍历白名单外的全部公开名。与 R-A 一并修最省。

### C-3 release tip 指针已落后 5 个提交（运维时效，非代码缺陷）

- `evidence/release_code_tip.txt` = `3b86f148e204`（11-09 12:21），当前 HEAD^ = `0bc8806`。P1-12 WARN 如实呈现。11-09 晚间的 R-2/R-3 收尾提交后未再 re-close 指针。
- 处置：下次 evidence 提交时按既有惯例 `hygiene: re-close release code tip pointer` 一并闭合；不单独为它开提交。

### C-4 `holidays_calendar` 失败语义比旧实现**更宽**，方向是安全侧但值得留痕

- 旧 audit 侧只捕 `(OSError, ValueError)`：`years` 下畸形结构（如某年值是 list）会**裸崩**；新 `load_holidays` 捕 `Exception` → empty 退化。实测对照：畸形输入旧实现抛 AttributeError、新实现返回空集。
- 定性：audit/pull 侧「宁可退化只看周末」的既有裁决方向一致，**不是回归**；但「结构容忍与原三处实现一致」的 docstring 表述过强——实际是「容忍面扩大」。建议 docstring 改一句实话即可，代码不动。

## FYI

- **R-2 原子写收敛质量高**：12 消费方 AST 接线守护 + 行为黄金值 + 突变实验（`e95e765` 非 OSError 清理修复走 RED→GREEN 实测）；`write_json_atomic` 序列化失败在写盘前抛、不产 tmp 的边界有专测。`shadow_policy` 两处局部 import 与 fe/mr 同风格，无循环依赖。
- **R-3 拆分纪律执行到位**：50 函数逐字节搬运（本轮 AST 独立复核）；门面 432 行 / 最大域模块 433 行，均回到健康区间；`_sync` 无条件 setattr 的设计取舍在 docstring 里写明了理由（避免第二种事实源）。
- **`07046d9`（git_lines 收敛）是真修复**：旧内联 subprocess 无 timeout，git 挂起可无限阻塞审计；新路径 fail-closed（查不到 ⇒ WARN）+ 6 测例含「旧内联路径死亡」AST 断言。
- 审计 25 检查项、四层状态机、P0-6 内容级隐私扫描（190 份跟踪 json/jsonl 零命中）在本轮全部复验通过；C-1（11-07）的修复没有回潮。
- `tests/README` fast 59 / slow 16 / 总 75 与 `--list` 实数一致（0bc8806 已刷新）。
- 工作区 17 项未提交改动全部是 `data/`、`output/` 运行时数据流（11-10~11-12 三个交易日的 mid/post 清单与快照），符合「纵向证据入库」约定，待下次 evidence 批次提交。

## 未验证项（如实）

- 东财真实拉取路径（铁律 7/8 未授权不触网）：本轮结论对 pull 族运行时行为上限为**推断级**（静态 + 离线测例已核实）。
- GitHub 远端状态未核查（本轮零外网）；11-07 残留的 dangling commits 处置状态维持 Summer 已知悉口径。
- `test_pit_guard` 在 PIT_CALENDAR 注入场景下的行为未测（现有测例不注入，见 R-A）——这正是 R-A 要补的测例。

## 下一步

1. Summer 拍板 R-A 修法（包装 vs 清单，工作量都 ≤30 分钟含测例）；C-1/C-2 可同批顺手做。
2. 修复后按惯例：本地 commit → 全量回归 → evidence 落档 → push（fetch 先行）。
3. C-3 指针随下次 evidence 提交闭合；C-4 一句 docstring 修正可并入同批。

---

## 处置记录（2026-11-13 12:0x Summer 拍板「①包装 + ②守护测例同批，C-1/C-2 顺手做」，当日执行完毕）

**✅ 全部落地并推送**：QuantProject main `07046d9..9cb6756`（3 个主题提交）。

### 修复批 `77b0eb6`（fix，6 文件 +178/−7）

1. **R-A**：`pit_trading_days`/`pit_lag_floor` 改 `_check` 同款转发包装（`audit_project.py`）。守护测例 `test_pit_helper_injection_reaches_domain` **修前红/修后绿实测**（RED：注入假日历后仍返回真实日历 3195 项；GREEN：返回注入值）。
2. **C-1**：`git_worktree_clean` 补进 evidence 的 common 显式导入清单；`TestExplicitImports` 增对偶判据 `test_no_undeclared_module_globals`（symtable：模块级自由变量 ⊆ import ∪ 模块级绑定 ∪ `_SYNC_NAMES` ∪ builtins）+ 实例钉住测例。
3. **C-2**：新增 `TestSyncNamesCoverage` 四项——需注入裸绑定全登记 / **静态豁免集与实际未登记裸绑定精确相等**（13 个只读词表/类 re-export 显式钉死，新增豁免须显式改测例）/ 豁免名不得是函数 / `_SYNC_NAMES` 无死条目；另加 pit 包装旧路径死亡断言（`assertIsNot` 域实现）。
4. **C-4**：`holidays_calendar.load_holidays` docstring 如实声明容忍面宽于旧实现（代码零改动）。
5. **突变实验 3/3 如期变红后还原**：拆 R-A 包装 → 2 测例红；拆 C-1 导入 → 实例钉住红（对偶判据按设计放行已登记 `_SYNC_NAMES` 名，属预期）；造未登记裸绑定 → 2 测例红。
6. 守护 `test_audit_facade` **24→31 项**；README「Project State」与 AGENTS ponytail lite 节均已留痕。

### evidence 批 `b02dc0d` + hygiene 批 `9cb6756`

- **全量 1272 OK (skipped=3)**（203.080s，较 1264 基线 +8 测例），log+meta+收集器落档 `evidence/probes/review_ra_c1c2_*_20261113.*`，INDEX 已登记；meta 双记测试时 HEAD 与终态 commit（内容逐字节一致）。
- **C-3 已闭合**：release tip 指针 re-close 至 `b02dc0d`，审计 P1-12 由 WARN 转 **PASS**。
- 终态审计：**23 PASS / 0 FAIL / 2 WARN**（P1-11 运行时数据未提交=纵向证据约定常态；P1-2 已处置档案=D3-07 既有裁决）。PRODUCTION=BLOCKED 不变，未动任何门禁。

### 过程如实记录

- 全量测试两次被工具超时误杀（PowerShell 包装层 kill 不带走 python 子进程 + `*>` 重定向产 UTF-16 污染日志）→ 改用 subprocess 直跑收集器落盘，残件已清理，终态 log 为纯净 UTF-8 无 BOM。
- push 时 openssl 后端被间歇重置（与 11-08 相反方向）→ 一次性 `-c http.sslBackend=schannel` 即通，**未改仓库配置**（repo 级仍 openssl）。
- 本轮零东财请求、零门禁/冻结件改动。
