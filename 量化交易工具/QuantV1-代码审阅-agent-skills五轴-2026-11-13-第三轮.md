# QuantV1 代码审阅（agent-skills / code-review-and-quality 五轴）· 2026-11-13 第三轮

- 审阅人：OpenSquilla（skill: `agent-skills` → `code-review-and-quality`，五轴 + 严重度标签）
- 对象：`D:\PythonProject\QuantV1`（main @ `6ec7285ce06e2e2684ca161c3983092b30814e37`，实测 `git rev-parse HEAD`；远端 `git ls-remote origin main` 同值，无分叉）
- 性质：**同日第二轮（[[QuantV1-代码审阅-agent-skills五轴-2026-11-13-第二轮]]）C-5 修复批的验收复核轮**——对象 = `9cb6756..6ec7285` 三个提交（fix `5f6e172` / evidence `eb0c571` / hygiene `6ec7285`），逐项独立复算 C-5 的三条验收标准，并对修复批本身做五轴过一遍；顺带核了第二轮 FYI-2 的落实与 PR-20260913-01 的续期留痕。
- 纪律：全程零东财请求、零生产代码改动（铁律 7/8）；验证全部离线（全量测试 subprocess 直跑 + 审计 --no-write + 内存判据探针 + 一次真实文件突变实验，突变后逐字节还原实证）。触网仅 GitHub 只读 `ls-remote` / `fetch`（非东财）。

## 裁决

**Approve**——C-5 修复批全部验收标准独立复核通过，无 Critical、无 Required、无新增 Consider。1 项 Nit（判据函数与 PUBLIC_API 之间两个空行，纯排版）+ 2 项 FYI 留痕。

## 验证证据（本机独立实测，非转述修复报告）

| 项 | 结果 |
|---|---|
| 全量测试（本轮重跑） | `run_tests.py --all` → **Ran 1276 tests in 197.653s — OK (skipped=3)**，exit=0（subprocess 直跑，managed 执行实测；与落档 1276 一致，skip=test_holdings_visibility RUN_SLOW_TESTS 门控 ×3） |
| 审计（本轮重跑） | `audit_project.py --no-write` → **23 PASS / 0 FAIL / 2 WARN**；AUDIT_HEALTH=PASS_WITH_WARNINGS；PRODUCTION=BLOCKED（model_ready=False，符合预期）；PUBLISH_GATE=PASS（当日 2/2 只干净）；2 WARN = P1-11 运行时数据 21 项未提交（纵向证据约定常态）+ P1-2 已处置档案 ×2（D3-07 既有裁决） |
| C-5 验收① 突变必抓 | 独立复刻旧判据（从 `9cb6756` 源码抽出并集剔除逻辑）：主突变副本旧判据 `[]` 放行 / 新判据 `['helper']` 抓住 ✅；**真实文件突变**：evidence.py 追加掩护形态 → `test_no_undeclared_module_globals` 红（抓 `['helper']`，exit=1）→ 还原后**逐字节一致**（bytes-identical=True）→ 守护全套 **35/35 OK**（3.104s）✅ |
| C-5 验收② 七域零命中 | 用新判据对七个域模块现源码全量重算：**7/7 零命中** ✅（不误报） |
| C-5 验收③ 全量不回归 | 上表 1276 OK；守护测例 AST 计数 **35**（31+4，与声明一致）✅ |
| 判据边界探针（10 组，内存字符串，仓库零改动） | 主突变 / 无掩护未声明（C-1 原语义）/ 合法局部（参数·赋值·推导·嵌套）/ 声明通道（import·绑定·sync·builtin）/ 类方法 / 类体 / lambda / async / 闭包自由变量（合法）/ `global` 显式声明形态（已知收紧面，七域 grep 零 `global` 语句）——**10/10 符合预期** ✅ |
| 落档证据完整性 | `review_c5_all_20261113.log` sha256 前 16 位 **35084e092a66a098 与 meta 声明逐位相符**；log 纯 UTF-8 无 BOM；L661/663 `Ran 1276 … OK (skipped=3)` 与 INDEX 行一致 ✅ |
| FYI-2 落实复核 | 收集器 `code_commit` 在测试开工前 `git rev-parse HEAD` 取值（9cb6756）；reflog 实测 fix 提交于 14:53:45、meta recorded_at=14:50:35——**测试确实跑在修复批未提交工作树 + 开工时 HEAD 口径**，note 已写明 ✅ |
| release tip 对账 | 指针 = `eb0c571d2c28…`，HEAD=`6ec7285`（hygiene 批）→ P1-12 **PASS** ✅ |
| 变更体量 | fix 批 3 文件 +94/−31（生产代码零改动，全部在 tests/ + 两处 docs 留痕）；evidence 批 4 文件 +782（log/meta/收集器/INDEX）；hygiene 批 1 文件 ±1——单一逻辑变更，健康区间 ✅ |
| PR-20260913-01 续期留痕（顺带核） | 审计 P1-14 显示 expiry=2026-12-31；`git log -S` 定位续期提交 `710abee`（10-12→12-31，renewals 审计留痕 + 复核报告哈希 + 13 测例）——**非静默续期，合规** ✅ |

## Nit（可忽略）

- `tests/test_audit_facade.py:82-83`：`undeclared_module_globals` 函数体结束后与 `PUBLIC_API` 之间只有 1 个空行（PEP8 顶层定义间 2 空行）。纯排版，不影响任何行为；下次顺手批次带上即可。

## FYI（留痕，无需行动）

1. **FYI-1（24→31 的「+1」口径）仍开放**：第一轮历史留痕行（README/AGENTS/笔记）尚未补「24 含先写的 RED 测例」半句，维持「留待下次 docs 批次」的既有裁决，本轮不重开。
2. **`global` 显式声明形态是已知收紧面**：`global X` + 赋值会被新判据标记（合理——该形态本就该显式声明依赖）；七域现源码零 `global` 语句（本轮 grep 实测），处置记录已留痕，无暴露面。

## 五轴小结

- **正确性**：C-5 三条验收标准全部独立复核通过（上表）；判据函数化后 10 组边界探针零意外；修复批未动因子/门禁/回测口径/冻结件，审计四层状态与第二轮终态一致。
- **可读性**：判据从内嵌闭包抽为模块级函数并带完整 docstring（C-1 引入 / C-5 修正的来龙去脉 + 测例指引），比首版更易读；`TestUndeclaredGlobalsJudge` 四测例名即语义。唯一排版瑕疵见 Nit。
- **架构**：删掉整个 `local_names` 并集剔除、直接用 `is_global()` 的真实作用域解析——**比原建议（per-scope 重算）更简**，概念数净减少，符合「删抽象优于抛光抽象」；判据可单测直打，守护与实现解耦。
- **安全**：零新增网络面/密钥面；P0-6 内容级扫描 192 份跟踪 json/jsonl 零命中（本轮审计实测）；持仓私有字段防线无回潮；PR-20260913-01 续期有完整审计留痕。
- **性能**：判据仅在测试期运行（symtable 解析 7 个 ≤520 行模块，毫秒级）；生产路径零改动、零新热路径。

## 未验证项（如实）

- 东财真实拉取路径（铁律 7/8 未授权不触网）：pull 族运行时行为结论上限仍为**推断级**。
- GitHub 服务端 dangling commits 处置状态未复查（11-07 已裁决接受 private 下残留，本轮不重开）。
- 本轮全量测试为 managed 执行直跑（stdout 由执行层捕获，未另落 evidence 件）；落档件以 `eb0c571` 批的 log+meta 为准，本轮只复核其哈希与内容一致性，不重复落档。

## 下一步

1. 无待拍板项——C-5 已闭环，本轮 Approve。Nit 与 FYI-1 并入下次 docs/hygiene 批次即可，不单独开提交。
2. 第一轮遗留观察项不变：est_chg live 误差带满 30 交易日后零网络重跑刷新（面 4 遗留）。
3. PR-20260913-01 下次到期日 **2026-12-31**：到期须三选一并留痕（续期/废止/转正），禁止静默续期——本轮已确认上次续期合规。