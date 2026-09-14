---
title: QuantV1数据流图-archify实测与310复现-20260914
created: 2026-09-14
tags:
  - 量化交易
  - 项目结构
  - GitHub工具实测
  - archify
description: archify 一次性渲染 QuantV1 全链路数据流图；Windows 8.3 短名路径下 live preview 崩溃（issue #310）复现与规避验证
---

# QuantV1 数据流图（archify 实测）+ #310 复现结论

> [!success] TL;DR
> - **图**：`assets/quantv1-dataflow.html`（自包含，四道闸门全过）· **IR 源**：`assets/quantv1.dataflow.json`（改图只改这个）
> - **#310 复现成立**：8.3 短名（alias）路径下 `preview` 在首次文件变更事件即崩（libuv `fs-event.c:72` 原生断言）；纯长路径稳定；**`render` 一次性路径零 watcher，免疫本坑**——本图就是这么出的。

## 1. 产物与闸门证据（2026-09-14 实测）

| 闸门 | 命令 | 结果 |
|---|---|:---:|
| validate | `archify validate dataflow IR.json --json` | `ok:true`（schema/layout/composition 全绿） |
| render | `archify render dataflow IR.json out.html` | exit 0 · 825,087 B |
| check | `archify check out.html` | `ok:true` · 4 条 warning 级 |
| visual-check | `archify visual-check out.html --json` | `status:"pass"` · sha256 `0dbe2311…859064c` · `visualReview:"pending"` |

版本：`tt-a1i/archify` @ `a07fa1d`，CLI **v2.17.0-dev.1**——与上游 #310 标注「仍复现」的版本一致。

warning 留档（不阻塞，改图时复查）：
1. 3× `composition/proper-crossing`：e14/e15×e16、e15×e17 三处线交叉
2. 1× `desktop-readability`：sublabel `lsjz / pingzhong` 7px 在 1440 视口投影 4.4px < 6px 下限（修：加宽节点或删 sublabel）

## 2. 怎么跑对（坑位清单）

1. 入口在仓库嵌套的 `archify\bin\archify.mjs`（仓库根不是包）；`node … doctor` 一键环境自检。
2. **`render` = 一次性出图（无 watcher）；`preview` = watcher 路径（#310 死面）**。规避 = 用 `render`；要用 `preview` 则确保 `dirname(input.json)` 不含 8.3 alias。
3. dataflow 布局硬约束（本次全部踩过一遍）：
   - `stages` **≤ 5**；同 stage 行号 0..4
   - `core.xxx` 长 label 超 112px 默认宽 → 给节点显式 `width` 或缩短 label
   - 手写 `fromSide/toSide` 易触发 `endpoint-side-direction` 拒绝，优先自动路由
   - `preview` 要求 `meta.output` 为**相对路径**（`render` 不强制）
4. 同列上下节点（正下方）连线是安全的竖线；跨列别画横线绕进同列。
5. 终端侧：PowerShell 管道会截断大 JSON 输出，先 `>` 落盘再解析。

## 3. #310 复现记录（双轨对照）

| 轨 | 被 watch 路径 | 触发 | 结果 |
|---|---|---|---|
| alias | `D:\PYTHON~1\ARCHIF~1\quantv1.dataflow.json`（两段全短名） | 3s 时 touch IR 文件 | **进程立即退出**，stderr：`Assertion failed: !_wcsnicmp(filename, dir, dirlen), file src\win\fs-event.c, line 72` |
| long | `D:\PythonProject\archify310probe\…`（同内容，纯长路径） | 同 touch | 10s 存活无崩溃（harness 主动 kill） |

- 崩溃点在 `bin\preview.mjs` L591 `fs.watch(dirname(inputPath))`：事件返回的 filename 与 dir 前缀（长/短名不一致）比对失败，命中 libuv 原生断言——**JS 层捕获不到**，与 #310 描述吻合。
- 环境事实：`D:\PythonProject` → `PYTHON~1`（**有** alias）；`QuantV1` 本身无 alias（`dir /x` 短名列空）。所以按长路径写全 QuantV1 目录是安全的；危险动作是把 `dir /x` 里的短名复制进路径。
- 未核实：单段 alias（如 `D:\PYTHON~1\QuantV1`）+ 事件触发的组合（裸 `fs.watch` 测试未 touch、不算数）——不外推。
- 处置：上游修 #310 前，本机约定 **只用 `render` 出图**；`preview` 仅限确认无 alias 的长路径临时用，用完即杀。

## 4. 图里是什么（全部来自 import 反查实测，非印象）

五列链路：

1. **外部数据源**：天天基金（lsjz/pingzhongdata，`config.json data.*_url`）· 东财板块页 · 新浪备用
2. **采集层**：`core.netutil`（重试·频控，4 引用）· `pull_sector_klines*` ×4 · probe ×2
3. **本地仓 + 读数**：`data/`（klines·models·registry，`data.cache_dir`）→ `core.data_loader` 统一读数（8 引用）
4. **特征/模型/因子**：`forecast_engine`（9 引用·最重）· `path_forecast` · `signal_engine` · `chanlun`+`lookthrough` · `intraday_*` ×3
5. **决策与验证**：`run.py`（单文件 import 14 个 core 模块）→ `output/`（128 文件·90 md）；`backtest_*.py` ×16 节点带 `ponytail lite 适用` tag——与同日胶水层收敛决策衔接。

## 5. 重跑（代码结构变化后）

```powershell
$cli = 'D:\archq\archify\archify\bin\archify.mjs'
Set-Location 'D:\Obsidian\My-First-Obsidian\量化交易工具\assets'
node $cli validate dataflow .\quantv1.dataflow.json --json   # 须 ok:true
node $cli render  dataflow .\quantv1.dataflow.json .\quantv1-dataflow.html
node $cli check .\quantv1-dataflow.html
```

布局由渲染器确定性重算，改 IR 不改坐标——这正是 archify 相对手画图的卖点。
