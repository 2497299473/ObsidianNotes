---
lark_doc_url: https://my.feishu.cn/docx/Tf0ZdjncsowM40xJQMLc7iO1nfe
lark_doc_token: Tf0ZdjncsowM40xJQMLc7iO1nfe
---

> 业务背景：制冷机性能测试工具的数字化升级，通过软件仿真替代部分物理测试，提升研发效率。
> 项目定位：集参数配置、实时仿真运算、数据可视化于一体的桌面端工具，辅助工程师分析压缩机内部热力学性能。
> 技术栈：Python + PyQt5（桌面 GUI）+ Plotly（交互式图表）+ Jenkins（CI/CD）+ DynamoDB（数据存储）
> 核心改造：前后端不分离 → MVC 分离架构 + 多线程异步 + 自动化测试/部署流水线 + 数据持久化
> 交付版本：V1.0 → V2.0，持续迭代 2 年

---

## 一、项目背景 & 接手时的状态

### 接手时的情况

| 维度 | 状态 |
|------|------|
| **架构** | 前后端不分离——PyQt5 界面代码与仿真计算逻辑混在一起 |
| **UI** | 界面设计简陋，布局不合理，视觉效果差 |
| **交互** | 每次点击按钮后，界面**完全卡死**，等待后端计算返回才能操作其他按钮 |
| **部署** | 手动打包、手动测试，无自动化流水线 |
| **数据** | 仿真配置和测试结果无持久化存储，关闭即丢失 |

### 交互卡顿的根因

```
用户点击按钮
    │
    ▼
PyQt5 主线程 ──► 调用仿真计算函数（同步阻塞）
    │                │
    │                ▼
    │           复杂热力学/流体计算（可能耗时数秒~数分钟）
    │                │
    │                ▼
    │           等待返回结果
    │
    ✗ 主线程被占用 → 界面无法响应任何操作
    ✗ 按钮灰色不可点击
    ✗ 窗口无法拖动、最小化
```

> 本质问题：**GUI 事件循环被长时计算任务阻塞**。PyQt5 的主线程同时承担"界面渲染"和"业务计算"两个职责，任何一个卡住都会导致整个应用无响应。

---

## 二、核心改造：MVC 架构 + 前后端分离

### 2.1 改造思路

> 采用 **MVC（Model-View-Controller）模式**重构 PyQt 界面代码，实现 UI 逻辑与业务逻辑的彻底分离。这一思路也复用了上一个项目（AppEegServer）中「Web 前后端分离」的设计经验。

**核心理念**：界面永远是界面，计算永远是计算，两者不应该在同一个线程里相互等待。

**MVC 三层划分**：

| 层 | PyQt5 对应 | 职责 |
|----|-----------|------|
| **Model** | 仿真计算引擎、数据模型类 | 热力学/流体计算逻辑、数据存取 |
| **View** | `.ui` 文件 / QWidget 子类 | 界面布局、图表渲染、用户交互 |
| **Controller** | MainWindow 事件处理 + Worker 调度 | 用户操作响应、线程调度、信号路由 |

### 2.2 架构对比

```
【改造前：单体架构】
┌─────────────────────────────────┐
│          PyQt5 主线程            │
│  ┌──────────┐  ┌──────────────┐ │
│  │ UI 组件   │  │ 仿真计算逻辑  │ │
│  │ (按钮/表格)│  │ (热力学/流体) │ │
│  └──────────┘  └──────────────┘ │
│         ↑ 同步调用 ↓              │
│    界面卡死等待计算结果           │
└─────────────────────────────────┘

【改造后：MVC + 多线程分离架构】
┌──────────────────────┐     ┌─────────────────────┐
│   View + Controller  │ ←→  │   Model (Worker)    │
│   (主线程)            │     │   (工作线程)          │
│                      │     │                     │
│ • 界面渲染           │     │ • 仿真计算            │
│ • 用户交互           │     │ • 数据处理            │
│ • 图表更新           │     │ • 结果封装            │
│ • 进度展示           │     │                     │
│                      │     │                     │
│  事件循环不阻塞       │     │  独立执行不卡UI       │
└──────────────────────┘     └─────────────────────┘
         ↑                           ↑
         │      Signals/Slots        │
         └───────────────────────────┘
         • started / finished / progress / error
```

### 2.3 具体实现方式

在 PyQt5 中实现 MVC + 前后端分离，核心是利用 **QThread + 信号槽（Signals/Slots）** 机制：

| 组件 | MVC 角色 | 职责 | 线程 |
|------|---------|------|------|
| **MainWindow** | View + Controller | 界面布局、按钮事件、进度条更新、信号路由 | 主线程（GUI 线程） |
| **Worker 类** | Model | 封装仿真计算逻辑，继承 `QObject` | 工作线程（`QThread`） |
| **Signals** | 跨层通信 | `started` / `progress(int)` / `finished(result)` / `error(str)` | 跨线程通信 |

**核心流程**：

```
用户点击"开始仿真"
    │
    ▼
MainWindow.simulate_btn_clicked()           ← Controller 层
    │
    ├── 1. 禁用按钮（防止重复点击）
    ├── 2. 创建 QThread + Worker
    ├── 3. 连接信号：
    │      worker.progress.connect(progress_bar.setValue)   ← View 更新
    │      worker.finished.connect(on_result_ready)          ← Controller 处理
    │      worker.error.connect(on_error)
    ├── 4. worker.moveToThread(thread)
    └── 5. thread.start()
           │
           ▼
    Worker 在独立线程中执行计算                              ← Model 层
           │
           ├── 计算过程中 emit progress(n)
           │      → 主线程实时更新进度条
           │
           └── 计算完成 emit finished(result)
                  → 主线程展示结果 + 恢复按钮
```

### 2.4 解决了什么问题？

| 问题 | 改造前 | 改造后 |
|------|--------|--------|
| **界面卡顿** | 计算时整个窗口冻结 | 主线程始终响应，可操作其他按钮 |
| **代码耦合** | UI 代码和计算逻辑混在一起 | MVC 三层分离，各自修改互不影响 |
| **进度反馈** | 只能干等，不知道还要等多久 | 实时进度条 / 状态文字 |
| **可取消** | 计算开始后无法中止 | Worker 检查取消标志，用户可随时取消 |
| **错误处理** | 异常直接崩溃 | error 信号捕获异常，弹窗提示但不崩溃 |
| **可测试性** | 必须启动整个 GUI 才能测试计算逻辑 | Worker（Model）可以独立单元测试 |

### 2.5 四大信号的设计逻辑与协作 🔥

> `started` / `progress(int)` / `finished(result)` / `error(str)` 不是随意定义的四个信号——它们构成了一套 **完整的任务生命周期协议**，覆盖了仿真任务从发起到结束的所有状态。

---

#### 跨线程通信到底怎么工作？（以这四个信号为例）

先抛开抽象概念，看一个仿真任务从启动到结束，信号是如何跨越线程边界的：

```
 时间轴       主线程 (GUI Thread)                    工作线程 (Worker Thread)
  │          ┌──────────────────────┐              ┌──────────────────────┐
  │  用户点击  │ btn_start.clicked()  │              │                      │
  │     ↓     │     │                │              │                      │
  │           │ ① 创建 QThread+Worker│              │                      │
  │           │ ② worker.moveToThread│              │                      │
  │           │ ③ connect 四个信号    │              │                      │
  │           │ ④ thread.start()    │──────────────► 线程创建，进入 run()   │
  │           │                      │              │     │                │
  │  T0       │                      │   started ◄──│ ⑤ emit started()    │
  │           │ ⑥ 按钮变灰 + 状态文字  │              │     │                │
  │           │                      │              │  进入计算循环         │
  │           │                      │              │     │                │
  │  T1       │                      │  progress(20)◄│ emit progress(20)   │
  │           │ ⑦ progress_bar=20%   │              │     │                │
  │           │                      │              │  继续计算...          │
  │  T2       │                      │  progress(50)◄│ emit progress(50)   │
  │           │ ⑧ progress_bar=50%   │              │     │                │
  │           │                      │              │  继续计算...          │
  │  T3       │                      │ progress(100)◄│ emit progress(100)  │
  │           │ ⑨ progress_bar=100%  │              │     │                │
  │           │                      │              │  计算完毕             │
  │           │                      │              │     │                │
  │  T4       │                   finished(id,path)◄│ emit finished(...)   │
  │           │ ⑩ 更新 Plotly 图表    │              │                      │
  │           │ ⑪ 恢复按钮            │              │                      │
  │           │ ⑫ thread.quit()      │              │                      │
  └           └──────────────────────┘              └──────────────────────┘

  ── 如果出错 ──

  时间轴      主线程                      工作线程
  │           │              error("除零")◄─── emit error(traceback)
  │           │ 弹窗 + 日志 + 恢复按钮          │
  │           │ thread.quit()                  │
```

**关键机制**：Worker 在**工作线程**中 `emit`，主线程的**事件循环**从队列中取出并执行 slot。整个过程 Worker 不需要知道谁在监听、监听方在哪个线程——信号槽的松耦合特性让两端的代码完全独立。

---

#### `started` — 任务已启动的通知

| 维度 | 说明 |
|------|------|
| **emit 时机** | `Worker.run()` 的**第一行**，在任何计算开始之前 |
| **emit 线程** | 工作线程 |
| **参数** | 无（纯粹的事件通知） |
| **连接到的 slot** | `MainWindow` 的状态更新方法 |

```python
class Worker(QObject):
    started = pyqtSignal()

    @pyqtSlot()
    def run(self):
        self.started.emit()        # ← 第一行，立即通知主线程
        self._do_heavy_work()
        self.finished.emit(result)

# 主线程侧
worker.started.connect(self.on_simulation_started)

def on_simulation_started(self):
    self.btn_start.setEnabled(False)   # 禁止重复点击
    self.btn_cancel.setEnabled(True)   # 允许取消
    self.status_label.setText("仿真运行中...")
    self.progress_bar.setValue(0)
```

**解决的问题**：

> 用户点击"开始仿真"按钮后，`thread.start()` 是异步的——OS 线程的创建和 Worker.run() 的首次执行之间有微小延迟。如果没有 `started` 信号，主线程只能在 `thread.start()` 调用后**假设**任务已经开始了。有了 `started`，主线程**收到确认后才切换 UI 状态**，保证状态一致性。

**如果去掉会怎样？**

```
点击按钮 → thread.start() → 立即禁用按钮
                                 ↑
                  Worker 可能还没真正开始（线程调度延迟）
                  如果 Worker 初始化失败（如配置校验不通过），
                  UI 已经卡在禁用状态，用户困惑
```

---

#### `progress(int)` — 进度反馈

| 维度 | 说明 |
|------|------|
| **emit 时机** | 计算循环中，阶段性报告进度（0-100） |
| **emit 线程** | 工作线程 |
| **参数** | `int`，表示完成百分比 |
| **连接到的 slot** | `QProgressBar.setValue`、状态文字更新 |

```python
class Worker(QObject):
    progress = pyqtSignal(int)

    @pyqtSlot()
    def run(self):
        steps = self._build_steps()
        total = len(steps)
        last_pct = -1

        for i, step in enumerate(steps):
            step.compute()
            pct = int((i + 1) / total * 100)

            # 节流：只在跨 5% 时 emit
            if pct - last_pct >= 5:
                self.progress.emit(pct)
                last_pct = pct

# 主线程侧（零代码！直接连到控件方法）
worker.progress.connect(self.progress_bar.setValue)
```

**括号里的数字到底是什么？——逐行拆解**：

```python
progress = pyqtSignal(int)          # ① 声明：这个信号携带一个 int 参数
                                    #    int 是参数的类型约束

self.progress.emit(50)              # ② 发送：把整数 50 作为信号参数发出去
                                    #    50 表示「完成 50%」（0-100 的百分比）

worker.progress.connect(            # ③ 接收：信号连接到 QProgressBar.setValue
    self.progress_bar.setValue)     #    setValue 也接收一个 int → 进度条显示 50%
```

参数流动路径：

```
Worker 线程                           主线程事件队列                  主线程
    │                                     │                          │
emit progress(50)  ───►  Qt 把 50 拷贝进事件队列  ───►  事件循环取出  ───►  setValue(50)
    │                                     │                          │
    │                              [progress(50)]                     ▼
    │                                                          ┌──────────┐
    │                                                          │ ██████░░ │ 50%
    │                                                          └──────────┘
```

> `50` 的本质：一个 `int` 类型的函数参数，通过 Qt 信号槽机制从 Worker 线程「运送」到主线程，最终喂给 `QProgressBar.setValue(50)`。整个过程 Worker 不需要知道谁在监听、监听方怎么用这个数字——松耦合。

| 概念 | 代码中的体现 | 含义 |
|------|------------|------|
| **信号声明** | `progress = pyqtSignal(int)` | 我发送一个整数 |
| **信号发射** | `self.progress.emit(50)` | 我现在发送整数 50 |
| **信号连接** | `worker.progress.connect(bar.setValue)` | 把收到的整数传给进度条的 setValue |
| **参数语义** | `50` 代表 50% | 约定：0=刚开始，100=已完成 |

**三个常被追问的细节**：

| 追问 | 回答 |
|------|------|
| "能传 150 吗？或者负数？" | 语法上可以（`int` 不限范围），但 `QProgressBar.setValue` 内部会 clamp 到 0-100，超出部分直接截断。所以实际上传 150 和传 100 效果一样 |
| "emit 之后，50 这个数字是传引用还是拷贝？" | 跨线程走 `QueuedConnection`，Qt 对 `int` 做**值拷贝**（`int` 是 C++ 基本类型，拷贝成本几乎为零）。如果传的是大对象才需要担心拷贝开销 |
| "`pyqtSignal(int)` 和 `pyqtSignal(float)` 混用会怎样？" | connect 时 PyQt 不检查参数类型匹配（C++ Qt 会编译期检查），但运行时如果类型不兼容可能静默失败或收到 0。最佳实践：**信号和槽的参数类型严格一致** |

| 决策 | 理由 |
|------|------|
| **参数用 `int` 而非 `float`** | 进度条控件天生用整数，减少类型转换；0-100 的语义所有人都懂 |
| **节流 emit（5% 阈值）** | 如果有 500 个迭代步骤，每步 emit 一次会产生 500 个排队事件——主线程事件队列被淹没，UI 反而卡。节流后最多 20 次 |
| **不传中间数据，只传百分比** | 进度信号的唯一职责是告诉用户"到哪了"。中间计算结果通过 `finished` 一次性传递，职责分离 |

**解决的问题**：

> 改造前用户最大的痛点不是"慢"，而是**不知道还要等多久**。进度条把"未知"变成"已知"——心理学上，有进度反馈的等待比无反馈的等待，用户感知时间要短得多。

**如果去掉会怎样？**

```
改造前的体验：点击"开始仿真"
→ 按钮变灰
→ ... (沉默 30 秒) ...
→ 结果突然出现
→ 用户：这工具是不是卡死了？要不要强制关掉？🤔

有 progress 之后：点击"开始仿真"
→ 按钮变灰 + 进度条开始走
→ 5%... 25%... 50%... 75%... 100%
→ 结果出现
→ 用户：哦，它在跑，快了 ✅
```

**进度百分比是怎么算出来的？——四种策略**：

> 面试官追问「你怎么知道进度到了 50%？」时，本质在考察你是否真的处理过**计算量不可预知**的实际场景。

---

**策略一：已知步骤数（最简单，但很少真的够用）**

```python
steps = self._build_steps()
total = len(steps)                     # 事先确定总步数
for i, step in enumerate(steps):
    step.compute()
    pct = int((i + 1) / total * 100)   # 直接算比例
    self.progress.emit(pct)
```

> 适用：每一步计算量大致相等，且总步数可提前确定。简单但脆弱——如果某一步突然耗时 10 倍，进度条会"卡在 30% 很久然后瞬间跳到 100%"。

---

**策略二：多阶段加权（本项目压缩机仿真的实际用法）**

压缩机仿真不是单一循环，而是多个计算阶段的流水线。每个阶段耗时差异巨大，简单数步数会导致进度条严重失真。

```
压缩机仿真流程：
┌─────────────────────────────────────────────────────────┐
│  阶段 1: 参数初始化 (快)         占 5%   = 0-5          │
│  阶段 2: 主热力学迭代循环 (核心)  占 70%  = 5-75         │
│  阶段 3: 收敛验证与修正 (中)      占 15%  = 75-90        │
│  阶段 4: 结果后处理 (快)          占 10%  = 90-100       │
└─────────────────────────────────────────────────────────┘
```

```python
class SimulationWorker(QObject):
    progress = pyqtSignal(int)

    def run(self):
        # 阶段 1: 参数初始化 (0 → 5)
        self._init_params()
        self.progress.emit(5)

        # 阶段 2: 主迭代循环 (5 → 75)
        max_iterations = 100
        for i in range(max_iterations):
            self._iterate_thermodynamics()

            # 在阶段内线性插值
            stage_pct = int((i + 1) / max_iterations * 70)  # 0→70
            global_pct = 5 + stage_pct                       # 5→75
            self.progress.emit(global_pct)

            if self._has_converged():
                break

        # 阶段 3: 收敛验证 (75 → 90)
        self._verify_convergence()
        self.progress.emit(90)

        # 阶段 4: 结果后处理 (90 → 100)
        self._post_process()
        self.progress.emit(100)
```

**加权表的设计思路**：

| 阶段 | 权重 | 理由 |
|------|:---:|------|
| 参数初始化 | 5% | 纯内存操作，毫秒级 |
| 主迭代循环 | 70% | **核心耗时**，每次迭代做热力学方程组求解 |
| 收敛验证 | 15% | 反算检查，中等耗时 |
| 结果后处理 | 10% | 整理数据 + 写盘，秒级 |

> **这个方案的核心价值**：进度条对用户**感知上很均匀**——不会出现"卡在 30% 很久然后瞬间飙到 100%"的情况。因为总进度的权重是按**实际耗时占比**分配的，而不是按步数。

---

**策略三：基于工况数量（批量仿真场景）**

工程师经常一次跑多个工况（如不同蒸发温度下的性能曲线）。如果拆成多个 Worker 各自跑，进度反馈会复杂——需要聚合。

```python
class BatchSimulationWorker(QObject):
    progress = pyqtSignal(int)

    def run(self):
        conditions = [
            {"evap_temp": 5,  "cond_temp": 45},
            {"evap_temp": 10, "cond_temp": 45},
            {"evap_temp": 15, "cond_temp": 45},
            # ... 可能十几个工况
        ]
        total = len(conditions)

        for idx, cond in enumerate(conditions):
            result = self._run_single(cond)

            # 每个工况完成后报告总体进度
            overall_pct = int((idx + 1) / total * 100)
            self.progress.emit(overall_pct)
```

> 更进一步：如果单个工况本身也很长，可以用 `工况进度 + 工况内进度` 的双层进度，或者分成 `current_condition` 和 `condition_progress` 两个信号。

---

**策略四：无法预知总量时——估算 + 上限兜底**

某些仿真算法（如迭代求解器）事先不知道要迭代多少次才能收敛。这时候没办法精确计算百分比。

```
方案 A：用最大迭代次数做分母（最常用）
  pct = min(current_iter / max_iterations * 100, 95)
  最后 5% 留给后处理

方案 B：基于收敛残差的启发式估算
  pct = int((1 - current_residual / initial_residual) * 100)
  问题：残差通常指数下降 → 前 90% 很快，后 10% 极慢 → 进度条"假快真慢"

方案 C：混合方案（本项目的选择）
  pct = 0.5 * (迭代进度) + 0.5 * (残差进度)
  迭代进度 = current_iter / max_iterations * 100
  残差进度 = (1 - log10(current_residual / initial_residual) / log10(tolerance)) * 100
  对残差取 log——因为残差是指数收敛的，取 log 后进度才线性
```

---

**四种策略的选择指南**：

| 策略 | 什么时候用 | 本项目中哪里用到了 |
|------|-----------|-----------------|
| **固定步数** | 计算步骤完全确定、每步耗时接近 | ❌ 基本不用——仿真场景太简单 |
| **多阶段加权** | 流水线式多阶段计算，各阶段耗时差异大 | ✅ **主仿真流程**（压缩机单工况计算） |
| **基于工况数** | 批量跑多个参数组合 | ✅ **批量仿真**（多蒸发温度性能曲线） |
| **估算 + 上限** | 迭代收敛算法，总步数不可预知 | ✅ **收敛循环内部**（热力学方程组迭代求解） |

> **面试话术**：压缩机仿真一次有多个阶段——参数初始化、迭代循环、收敛验证、后处理。如果简单按步数算进度，迭代循环占了 90% 的步数但只占 70% 的时间，进度条会失真。我用了多阶段加权策略，按**实际耗时占比**给每个阶段分配权重，让进度条走得均匀、符合用户直觉。

---

#### `finished(result)` — 任务完成，传递结果

| 维度 | 说明 |
|------|------|
| **emit 时机** | 计算**正常结束**或**被取消**后 |
| **emit 线程** | 工作线程 |
| **参数** | 轻量标识（`str` 类型的 result_id + cache_path），**不传原始数据** |
| **连接到的 slot** | 结果展示、按钮恢复、线程清理 |

```python
class Worker(QObject):
    finished = pyqtSignal(str, str)  # (result_id, cache_path)

    @pyqtSlot()
    def run(self):
        try:
            result_id = str(uuid4())
            cache_path = f"/tmp/sim_{result_id}.parquet"

            df = self._compute()
            df.to_parquet(cache_path)          # 大数据落盘

            self.finished.emit(result_id, cache_path)  # 只传路径

        except Exception:
            self.error.emit(traceback.format_exc())

# 主线程侧
worker.finished.connect(self.on_simulation_finished)

def on_simulation_finished(self, result_id: str, cache_path: str):
    if result_id == "cancelled":
        self.status_label.setText("已取消")
    else:
        # 主线程按需加载数据（懒加载）
        df = pd.read_parquet(cache_path)
        self._update_plotly_chart(df)          # 更新图表
        self.status_label.setText(f"仿真完成: {result_id}")
        self._save_to_dynamodb(result_id, cache_path)

    # 无论完成还是取消，都要做这些
    self._restore_buttons()
    self._cleanup_thread()
```

**为什么 `finished` 只传标识符不传原始数据？**

> 仿真结果可能是几万行的 DataFrame。如果通过信号传递，Qt 会深拷贝一份塞进事件队列——这不但消耗大量内存，主线程从队列取出时还要做反序列化，导致 UI 瞬间卡顿。**策略：信号只传「去哪拿数据」的路径，不传数据本身。**

**`finished` 的两种结果状态**：

| 状态 | result_id | 主线程行为 |
|------|-----------|-----------|
| **正常完成** | 随机 UUID | 加载数据 → 更新 Plotly → 保存 DynamoDB |
| **用户取消** | `"cancelled"` | 跳过数据加载 → 恢复 UI → 清理临时文件 |

> **设计理念**：取消不是错误，是一种**合法的结束状态**。用同一个 `finished` 信号承载两种结果，而非单独搞一个 `cancelled` 信号——让主线程在**一个 slot 里统一处理**结束逻辑，避免代码分叉。

---

#### `error(str)` — 异常兜底

| 维度 | 说明 |
|------|------|
| **emit 时机** | Worker.run() 的 `except` 分支 |
| **emit 线程** | 工作线程 |
| **参数** | `str`，完整的异常堆栈（`traceback.format_exc()`） |
| **连接到的 slot** | 错误弹窗、日志记录、停止加载动画 |

```python
class Worker(QObject):
    error = pyqtSignal(str)

    @pyqtSlot()
    def run(self):
        try:
            self.started.emit()
            # ... 计算逻辑 ...
            self.finished.emit(result_id, cache_path)
        except Exception:
            self.error.emit(traceback.format_exc())
            # 注意：不在这里调 finished！
            # error 和 finished 是互斥的——要么成功，要么失败

# 主线程侧
worker.error.connect(self.on_simulation_error)

def on_simulation_error(self, err_msg: str):
    QMessageBox.critical(self, "仿真出错", f"错误详情：\n{err_msg[:500]}")
    self.logger.error(f"Simulation failed:\n{err_msg}")
    self.status_label.setText("仿真失败")
    self._restore_buttons()
    self._cleanup_thread()
```

**三个为什么**：

| 为什么这样设计 | 解释 |
|---------------|------|
| **为什么传 `str` 而不是 `Exception` 对象？** | `str` 是 PyQt 内置类型，跨线程拷贝零成本；`Exception` 对象可能包含不可序列化的引用（如 traceback 中的 frame 对象） |
| **为什么 `error` 和 `finished` 不共存？** | 语义清晰——一个任务只有两种结局：成功走 `finished`，失败走 `error`。如果两信号都 emit，主线程要处理"先 error 后 finished"这种复杂状态 |
| **为什么 error 里不自动恢复 UI？** | error 信号只负责**传递错误信息**，恢复 UI 是主线程 slot 的职责——信号本身保持纯粹 |

**解决的问题**：

> 改造前：Worker 线程内部抛异常 → 线程直接死亡 → 主线程永远收不到 `finished` → 按钮永远灰色 → **用户只能强制关窗口**。
>
> 有了 `error` 信号：异常被 `try/except` 捕获 → emit error → 主线程弹窗 + 恢复按钮 → **用户可以修正参数后重试**。

---

**报错后日志记录在哪里？——三层落盘架构**：

> 面试官问这个，是在考察你是否考虑了「用户看不到日志 → 无法反馈问题 → 开发无法复现」这条链路。

```
Worker 线程捕获异常
    │
    │ traceback.format_exc()
    ▼
emit error(traceback字符串)
    │
    │  QueuedConnection → 主线程
    ▼
MainWindow.on_simulation_error(err_msg)
    │
    ├── ① 弹窗给用户看（QMessageBox）
    │      内容：截取前 500 字符 + 提示"完整日志已保存"
    │
    ├── ② 写入本地滚动日志文件（Python logging → RotatingFileHandler）
    │      路径：%APPDATA%/CarrierSim/logs/simulation.log
    │      保留最近 10 个文件，每个最大 10MB
    │
    ├── ③ 写入 DynamoDB 错误表（结构化记录）
    │      Primary Key: timestamp
    │      Sort Key:     error_type (如 "ZeroDivisionError")
    │      字段:         traceback, config_snapshot, version, operator
    │
    └── ④ 写入 Jenkins 构建日志（如果在 CI 环境）
           Jenkins 控制台输出自动捕获 stderr
```

**各层的角色**：

| 层级 | 存储位置 | 用途 | 谁看 |
|------|---------|------|------|
| **弹窗** | 内存（不持久化） | 即时告知用户出了问题 | 操作工程师 |
| **本地日志文件** | `%APPDATA%/CarrierSim/logs/` | 完整堆栈 + 时间戳，支持按日期翻查 | 驻场开发排查 |
| **DynamoDB** | AWS 云端 | 结构化查询：某版本某类型的错误频率、某参数的报错关联 | 开发团队做缺陷分析 |
| **Jenkins** | Jenkins 服务器 | CI 自动化测试的 stdout/stderr | 测试失败的上下文 |

**logging 配置的实际代码**：

```python
# app_logger.py —— 在应用启动时初始化
import logging
from logging.handlers import RotatingFileHandler
from pathlib import Path

def setup_logger() -> logging.Logger:
    log_dir = Path.home() / "AppData" / "Roaming" / "CarrierSim" / "logs"
    log_dir.mkdir(parents=True, exist_ok=True)

    logger = logging.getLogger("CarrierSim")
    logger.setLevel(logging.DEBUG)

    # 文件处理器：滚动覆盖，保留最近 10 个文件
    file_handler = RotatingFileHandler(
        log_dir / "simulation.log",
        maxBytes=10 * 1024 * 1024,  # 10 MB
        backupCount=10,              # 保留 10 个历史文件
        encoding="utf-8",
    )
    file_handler.setLevel(logging.DEBUG)
    file_handler.setFormatter(logging.Formatter(
        "%(asctime)s [%(levelname)s] %(name)s: %(message)s"
    ))
    logger.addHandler(file_handler)

    return logger

# ── MainWindow 中使用 ──
class MainWindow(QMainWindow):
    def __init__(self):
        self.logger = setup_logger()

    def on_simulation_error(self, err_msg: str):
        # 1. 弹窗（用户看到）
        QMessageBox.critical(
            self, "仿真出错",
            f"错误详情：\n{err_msg[:500]}\n\n"
            f"完整日志已保存，请联系开发人员。"
        )

        # 2. 本地日志文件（完整堆栈）
        self.logger.error(f"Simulation failed:\n{err_msg}")

        # 3. DynamoDB 错误记录（结构化）
        self._log_error_to_dynamodb(err_msg)

        # 4. 恢复 UI
        self.status_label.setText("仿真失败")
        self._restore_buttons()
        self._cleanup_thread()

    def _log_error_to_dynamodb(self, err_msg: str):
        """结构化错误记录：方便后续按版本/错误类型/参数做聚合分析"""
        try:
            error_type = err_msg.strip().split("\n")[-1].split(":")[0]
            item = {
                "timestamp": datetime.now().isoformat(),
                "error_type": error_type,
                "traceback": err_msg[:4000],       # DynamoDB 单条 400KB 上限
                "config_snapshot": json.dumps(self.current_config()),
                "version": APP_VERSION,
                "operator": os.getlogin(),
            }
            self.dynamodb_table.put_item(Item=item)
        except Exception:
            # DynamoDB 写入失败不能影响主流程
            self.logger.warning("Failed to write error to DynamoDB")
```

**为什么弹窗只展示前 500 字符？**

- 完整堆栈可能几百行，弹窗会撑爆屏幕
- 用户不需要看到底层的 `File "site-packages/numpy/..."` 调用链
- 提示"完整日志已保存" → 用户截图发给开发 → 开发去日志文件或 DynamoDB 查完整信息

> **面试话术**：错误日志走了三层落盘——弹窗即时通知用户、本地滚动文件保留完整堆栈、DynamoDB 做结构化存储方便按版本和错误类型聚合分析。用户看到弹窗截图发给我的时候，我直接去 DynamoDB 查时间戳就能拿到完整上下文，包括当时的仿真参数和软件版本，不需要让用户再跑一遍。

---

#### 四信号协作全景：一个仿真任务的生命周期

```
                    ┌─────────────┐
                    │  用户点击    │
                    │ "开始仿真"   │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │ Controller  │
                    │ 创建线程+Worker│
                    │ connect 信号 │
                    │ thread.start│
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
         started ◄─────────┤            │  ← Worker 确认启动
              │            │            │
       UI: 禁用按钮         │            │
       显示"运行中"         │            │
              │            │            │
              ├─── progress(5) ◄────────┤  ← 计算循环中
              ├─── progress(25)◄────────┤
              ├─── progress(50)◄────────┤
              ├─── progress(75)◄────────┤
              ├─── progress(100)◄───────┤
              │            │            │
              │            │      ┌─────┴─────┐
              │            │      │ 正常结束？  │
              │            │      └─────┬─────┘
              │            │     ┌──────┼──────┐
              │            │    Yes     │      No
              │            │     │      │    (异常)
              │            │     ▼      │      ▼
              │    finished(id,path)◄───┤  error(traceback)◄──┐
              │            │     │      │      │              │
              │            │     ▼      │      ▼              │
              │            │  加载数据    │   弹窗+日志          │
              │            │  更新Plotly │   恢复按钮            │
              │            │  保存DDB   │   清理线程            │
              │            │     │      │                      │
              │            │     └──────┼──────────────────────┘
              │            │            │
              │            └────────────┼── 共同终点：
              ▼                         ▼   恢复按钮、清理线程
         ┌─────────────────────────────────┐
         │  thread.quit() → wait() →       │
         │  worker.deleteLater() →          │
         │  用户可再次点击"开始仿真"          │
         └─────────────────────────────────┘
```

**一句话总结四个信号的协作关系**：

> `started` 告诉 UI "我开始了，锁住按钮" → `progress` 告诉 UI "我在跑，到 xx% 了" → `finished` 告诉 UI "我跑完了，这是结果" 或者 `error` 告诉 UI "我挂了，这是原因"。四个信号覆盖了**一个异步任务的完整状态机**，让主线程从"不知道 Worker 在干什么"变成"对 Worker 的一举一动都知情"。

---

## 三、高性能数据可视化（Plotly）

### 3.1 为什么需要专门的绘图方案？

仿真计算产生**大量时序数据**——压缩机一个完整工况可能产生数千到数万个数据点（温度曲线、压力变化、功率曲线等）。原有方案直接用 Matplotlib 静态渲染，存在三个问题：

| 问题 | 表现 |
|------|------|
| **交互缺失** | 静态图片无法缩放、hover 查看数值、局部框选 |
| **性能瓶颈** | 大数据量下 Matplotlib 渲染缓慢，切换图表有明显延迟 |
| **多曲线对比困难** | 不同工况的对比需要在多张图之间来回切换 |

### 3.2 为什么选 Plotly？

| 维度 | Matplotlib（旧方案） | Plotly（新方案） |
|------|---------------------|-----------------|
| **交互性** | 静态图片 | 缩放、平移、hover 数据点、框选区域 |
| **大数据渲染** | 数据点多时卡顿 | 基于 Web 引擎（Plotly.js），支持大数据量 |
| **多曲线叠加** | 代码量大，布局复杂 | 原生支持 subplot + 多 trace |
| **导出** | 仅图片 | 图片 + 交互式 HTML |
| **PyQt5 集成** | 原生支持 | 通过 `QWebEngineView` 嵌入 |

### 3.3 实现架构

```
┌─────────────────────────────────────┐
│            MainWindow                │
│  ┌───────────────────────────────┐  │
│  │     QWebEngineView             │  │
│  │  ┌─────────────────────────┐  │  │
│  │  │   Plotly HTML (iframe)   │  │  │
│  │  │   • 热力学曲线           │  │  │
│  │  │   • 压力-体积图          │  │  │
│  │  │   • 效率曲线             │  │  │
│  │  └─────────────────────────┘  │  │
│  └───────────────────────────────┘  │
│                                      │
│  QThread (后台计算)                   │
│     │                                │
│     ├── NumPy/SciPy 计算引擎          │
│     └── 结果 → JSON → Plotly 渲染    │
└─────────────────────────────────────┘
```

**关键实现细节**：

- **多线程配合**：计算线程（QThread）产出数据 → `finished` 信号携带结果回主线程 → 主线程调用 Plotly 生成 HTML → `QWebEngineView` 渲染
- **避免界面假死**：Plotly 图表渲染本身在主线程，但数据量可控（只渲染当前视图），不会阻塞事件循环
- **图表切换**：不同工况的对比通过 Plotly 的多 trace / subplot 实现，无需重新加载整个页面

### 3.4 面试话术

> 仿真计算会产生大量时序数据，原来的 Matplotlib 静态图既不能交互，大数据量下也卡。我引入了 Plotly 做交互式图表渲染，工程师可以缩放、hover 查看具体数值、框选局部区域做精细分析。同时把计算放在 QThread 里异步跑，出结果后 emit 信号回主线程更新图表，避免了界面假死。

---

## 四、Jenkins 自动化流水线

### 4.1 为什么引入 Jenkins？

接手项目时的痛点：
- 仿真工具频繁迭代（模型参数调整、计算逻辑优化）
- 每次发布需要**手动测试 → 手动打包 → 手动部署**，耗时且容易出错
- 没有回归测试，改了一个参数可能影响其他仿真场景
- 测试报告靠人工截图 + Excel，格式不统一，回溯困难

### 4.2 流水线架构

```
Git Push (代码提交)
    │
    ▼
┌──────────────────────────────────────────────┐
│              Jenkins Pipeline                 │
│                                               │
│  Stage 1: Checkout                            │
│    └── 拉取最新代码                            │
│                                               │
│  Stage 2: 静态检查                             │
│    └── flake8 / pylint 代码风格检查             │
│                                               │
│  Stage 3: 单元测试                             │
│    └── pytest 运行 Worker 计算逻辑测试          │
│    └── 覆盖核心仿真算法 + 边界条件               │
│                                               │
│  Stage 4: 集成测试                             │
│    └── 用预置的仿真参数运行完整计算流程          │
│    └── 对比输出结果与预期值的误差范围（Golden Test）│
│                                               │
│  Stage 5: 生成测试报告                         │
│    └── 收集所有测试结果                         │
│    └── 生成 HTML 格式的自动化测试报告           │
│    └── 直观展示回归测试结果（Pass/Fail/误差率）  │
│                                               │
│  Stage 6: 打包                                │
│    └── PyInstaller 打包成 exe                  │
│    └── 归档到制品库                            │
│                                               │
│  Stage 7: 通知                                │
│    └── 企业微信 / 邮件通知构建结果              │
└──────────────────────────────────────────────┘
```

### 4.3 量化成果

| 指标 | 改造前 | 改造后 |
|------|--------|--------|
| **单次测试报告生成时间** | ~X 分钟（手动汇总） | **缩短 40%** |
| **测试触发方式** | 手动 | Git Push 自动触发 + 每日定时构建 |
| **回归测试覆盖** | 基本无 | 预置参数矩阵，批量验证 |
| **报告格式** | 人工截图 + Excel | **HTML 自动生成**，带 Pass/Fail 汇总 |

### 4.4 HTML 测试报告的设计

> 面试官可能会追问"HTML 报告怎么做"——这里提前准备好细节。

- **数据来源**：pytest 运行结果（`--json-report` 或自定义 conftest.py 收集）
- **报告内容**：
  - 顶部汇总卡片：Total / Passed / Failed / Error / 通过率
  - 失败用例详情：用例名 + 错误堆栈 + 预期值 vs 实际值
  - 回归对比表：本次 vs 上次测试结果差异（哪些用例从 Pass 变 Fail）
  - 历史趋势图：近 30 次构建的通过率折线（用 Plotly 生成嵌入 HTML）
- **生成方式**：Python 脚本读取 pytest JSON 输出 → Jinja2 模板渲染 → 产出独立 HTML 文件
- **存储**：报告上传到 Jenkins 制品归档，同时写入 DynamoDB 方便历史回溯

### 4.5 面试话术

> 我引入 Jenkins 搭建了自动化流水线，覆盖了代码检查、单元测试、集成测试、报告生成、打包发布五个阶段。以前手动测试一轮要花半天，现在代码提交后 20 分钟内自动出结果。另外，我设计了 HTML 格式的自动化测试报告模板，一目了然地展示回归测试结果，单次测试报告生成时间缩短了 40%。仿真计算的回归测试本身很难手工做——参数组合太多——用 Jenkins 批量跑参数矩阵，效率提升了 10 倍以上。

### 4.6 关键设计决策

| 决策点 | 选择 | 原因 |
|--------|------|------|
| **触发方式** | Webhook（Git Push 触发）+ 每日定时构建 | 既保证即时反馈，又兜底夜间完整回归 |
| **测试环境** | Jenkins Agent 用虚拟桌面环境 | PyQt5 某些组件依赖显示驱动，headless 跑不了 |
| **打包工具** | PyInstaller | 将 Python 环境 + 依赖全部打包进单个 exe，交付给非技术人员直接双击使用 |
| **制品管理** | 保留最近 10 个版本的 exe | 方便快速回滚到上一版本 |
| **通知** | 构建失败立刻通知，成功则每日汇总 | 避免构建成功通知刷屏 |
| **脚本对接** | Python 脚本对接 Jenkins API | 实现测试用例的定时触发与结果解析，无需手动操作 Jenkins 界面 |

---

## 五、数据持久化方案（DynamoDB + JSON）

### 5.1 为什么需要持久化？

仿真工具的典型使用场景：
- 工程师配置一套仿真参数（制冷剂类型、压缩机转速、蒸发/冷凝温度等），运行计算
- 后续可能需要在**不同时间、不同工况下反复对比**
- 如果每次关闭软件配置就丢失，每次都要重新输入十几个参数，效率极低

### 5.2 为什么选 DynamoDB？

| 考量 | DynamoDB | 本地 SQLite | JSON 文件 |
|------|----------|------------|-----------|
| **多机共享** | ✅ 云端存储，团队成员可共享配置 | ❌ 仅本机 | ❌ 仅本机 |
| **查询能力** | ✅ 按工况类型、时间范围扫描 | ✅ SQL 查询 | ❌ 需要手动解析 |
| **运维成本** | ✅ 托管服务，免运维 | 需要备份策略 | 需要管理文件路径 |
| **数据结构** | ✅ JSON 文档模型，天然匹配仿真参数结构 | 需要建表，参数变更要 ALTER | ✅ 灵活 |
| **成本** | 按用量付费，小规模几乎免费 | 免费 | 免费 |

> 实际采用 **DynamoDB 为主 + 本地 JSON 兜底** 的混合方案：联网时用 DynamoDB 同步 + 共享；离线时本地 JSON 缓存，联网后自动同步。

### 5.3 数据模型设计

```
┌────────────────────────────────────────┐
│           SimulationRecord             │
├────────────────────────────────────────┤
│  PK: config_id (UUID)                  │
│  SK: timestamp (ISO 8601)              │
│                                        │
│  config: {                             │
│    refrigerant: "R410A",               │
│    compressor_speed: 3000,  // rpm     │
│    evaporator_temp: 5,     // °C       │
│    condenser_temp: 45,     // °C       │
│    ...                                 │
│  }                                     │
│  results: {                            │
│    cop: 3.42,                          │
│    power_consumption: 2.8,  // kW      │
│    mass_flow_rate: 0.12,  // kg/s      │
│    ...                                 │
│  }                                     │
│  status: "completed" | "failed" | ...  │
│  operator: "username"                  │
│  duration_ms: 1523                      │
│  version: "2.0.1"                      │
└────────────────────────────────────────┘
```

### 5.4 核心功能

| 功能 | 实现方式 |
|------|---------|
| **保存配置** | 仿真完成后 → Worker emit finished 信号 → Controller 序列化为 JSON → 写入 DynamoDB |
| **历史回溯** | 主界面"历史记录"面板 → 按时间倒序查询 DynamoDB → 点击任意记录一键回填参数 |
| **工况对比** | 选中两条历史记录 → Plotly 双曲线叠加对比 |
| **测试记录** | Jenkins 每次运行将测试结果写入 DynamoDB → 前端可查询历史测试通过率趋势 |

### 5.5 面试话术

> 仿真工具的工程师经常需要在不同时间对比不同工况下的压缩机性能。我用 DynamoDB 做了配置和结果的持久化存储，支持一键保存、一键回溯历史工况。选 DynamoDB 的原因是它是托管服务、免运维，JSON 文档模型天然匹配仿真参数这种半结构化数据。同时做了本地 JSON 兜底，断网也能用。

---

## 六、技术深挖：PyQt5 多线程模式

### 6.1 QThread 的正确用法

> ⚠️ 常见误区：继承 `QThread` 然后重写 `run()`。这是 Qt4 时代的写法，Qt5 推荐「Worker + moveToThread」模式。

```
【不推荐】继承 QThread          【推荐】Worker + moveToThread
┌──────────────┐                ┌──────────┐   ┌──────────┐
│ MyThread     │                │ QThread  │   │ Worker   │
│ (继承QThread)│                │ (只管线程)│   │ (QObject)│
│  run():      │                └──────────┘   └──────────┘
│    do_work() │                    ↑                ↑
└──────────────┘                    │ moveToThread   │ 信号槽
     ↑                              │                │
     耦合了"线程管理"和"业务逻辑"     解耦，Worker 可独立测试
```

### 6.2 为什么不用多进程？

| 维度 | QThread | Multiprocessing |
|------|---------|-----------------|
| **数据传递** | 信号槽直接传 Python 对象 | 需要 pickle 序列化，大对象开销大 |
| **启动开销** | 轻量，毫秒级 | 较重，需要 fork/spawn |
| **内存共享** | 共享进程内存空间 | 独立内存空间，需 IPC |
| **适用场景** | I/O 密集型或短时计算 | CPU 密集型长时计算 |

> 仿真计算的耗时主要在于**算法逻辑**而非纯 CPU 计算（不像 ML 推理跑 GPU），QThread 足够。如果后续仿真规模扩大、单次计算超过 30 秒，可升级到 `QProcess` 或 multiprocessing 做进程级隔离。

### 6.3 取消机制的设计

```python
# Worker 内部
class SimulationWorker(QObject):
    finished = pyqtSignal(object)
    progress = pyqtSignal(int)

    def __init__(self):
        super().__init__()
        self._is_cancelled = False

    def cancel(self):
        self._is_cancelled = True

    @pyqtSlot()
    def run(self):
        for i, step in enumerate(self.simulation_steps):
            if self._is_cancelled:
                self.finished.emit({"status": "cancelled"})
                return
            step.compute()
            self.progress.emit(int(i / len(self.simulation_steps) * 100))
        self.finished.emit({"status": "done", "result": ...})
```

关键点：
- 不在主线程中直接 terminate QThread（会导致资源泄漏）
- 用标志位 + 协作式取消，每个计算步骤开始时检查
- 取消后正确清理临时文件和中间状态

### 6.4 跨线程通信面试深挖 🔥

> 面试官问「信号槽跨线程通信的细节」时，本质是在考察你对 **Qt 事件循环**和**线程模型**的理解深度。以下按追问层级递进。

---

#### Q1：信号槽跨线程通信的底层是怎么实现的？

**一句话**：Qt 通过**事件队列 + 连接类型自动选择**来实现跨线程信号槽。

```
主线程 (GUI Thread)                    工作线程 (Worker Thread)
┌─────────────────────┐               ┌─────────────────────┐
│                     │               │                     │
│  事件循环 running    │               │  Worker.run()       │
│      ↑              │               │      │              │
│      │ 取出事件       │               │      │ 计算完成       │
│      │              │               │      ▼              │
│  事件队列 (FIFO)     │  ◄──────────  │  emit finished()    │
│  ┌──────────────┐   │  信号参数复制   │                     │
│  │ finished(data)│   │  到接收方队列   │                     │
│  ├──────────────┤   │               │                     │
│  │ progress(50) │   │               │                     │
│  └──────────────┘   │               │                     │
└─────────────────────┘               └─────────────────────┘
```

**关键机制**：

| 概念 | 说明 |
|------|------|
| **事件循环（Event Loop）** | 每个线程可以有自己的事件循环。主线程默认有，QThread 需要手动调用 `exec()` 开启 |
| **连接类型** | Qt 有 4 种连接方式，跨线程时**自动选择 `QueuedConnection`** |
| **参数传递** | 信号参数在 emit 时被**深拷贝**到接收线程的事件队列中，而非传递引用 |

**四种连接类型**：

| 类型 | 行为 | 适用场景 |
|------|------|---------|
| `AutoConnection`（默认） | 同线程→同步调用；不同线程→队列调用 | 99% 的情况 |
| `DirectConnection` | **同步**：emit 后立即在**发送者线程**执行 slot | 明确知道两个对象在同一线程 |
| `QueuedConnection` | **异步**：信号参数入队，等接收者事件循环处理 | 跨线程通信的标准方式 |
| `BlockingQueuedConnection` | 异步 + **阻塞等待** slot 执行完毕才返回 | 需要同步返回结果（小心死锁！） |

---

#### Q2：为什么默认用排队的而不是直接的？这带来什么影响？

**为什么必须排队？**

如果跨线程用 `DirectConnection`，slot 函数会在**发送者（Worker 线程）**中执行。这意味着：
- Worker 线程去操作 UI 组件（如 `label.setText()`）→ **直接崩溃**（Qt 禁止非 GUI 线程操作 UI）
- 多个 Worker 可能同时操作同一 UI 组件 → 竞态条件

`QueuedConnection` 保证了 **slot 总是在接收者所在的线程中执行**——对我们来说就是 GUI 更新一定在主线程。

**代价/影响**：

| 影响 | 细节 |
|------|------|
| **参数必须是可拷贝的** | emit 时 Qt 会对参数做深拷贝（内部用 `QMetaType` 注册的类型）。传大对象（如整个仿真结果 DataFrame）会触发大量内存拷贝 |
| **不是零延迟** | 信号入队后，要等接收线程的事件循环「轮询到」才执行。如果主线程正忙于其他事件，可能延迟几到几十毫秒 |
| **不保证严格的全局顺序** | Worker 线程 emit 了 progress(80) 再 progress(90)，但主线程可能先处理 90 再处理 80（极少见，但理论可能） |

---

#### Q3：你在项目中遇到过什么具体的问题？

**问题一：传大对象导致 UI 卡顿**

```python
# ❌ 错误做法：整个 DataFrame 通过信号传递
class Worker(QObject):
    finished = pyqtSignal(pd.DataFrame)  # DataFrame 可能几十 MB

# ✅ 正确做法：Worker 把结果存在共享位置，只传轻量标识
class Worker(QObject):
    finished = pyqtSignal(str)  # 传文件路径或缓存 key

    def run(self):
        result_df = self.do_simulation()
        cache_path = f"/tmp/sim_{uuid4()}.parquet"
        result_df.to_parquet(cache_path)
        self.finished.emit(cache_path)  # 几十字节 vs 几十 MB
```

> **面试时强调**：仿真一次可能产生上万行数据，直接通过信号传 DataFrame 会让主线程在反序列化时卡顿。我改成只传文件路径，主线程按需加载——相当于「懒加载」模式。

**追问：数据体量都一样大（几十 MB），为什么传文件就比传信号快？**

> 这个问题问到点子上了——开销不在数据量，在**序列化机制、内存分配路径、以及对事件循环的阻塞方式**三者完全不同。

```
【传 DataFrame 通过信号】
Worker 线程                        Qt 事件队列               主线程
┌─────────────────┐      ┌──────────────────────┐     ┌─────────────────┐
│ DataFrame (50MB) │      │                      │     │                 │
│       │          │      │  ┌────────────────┐  │     │                 │
│       ▼          │      │  │ pickle 序列化后  │  │     │                 │
│ ① pickle.dumps() │──────►  │ 的字节流 (~50MB) │──┼────►│ ③ pickle.loads()│
│   耗时 ~2秒      │ 拷贝   │  ┌────────────────┐  │ 取出 │   耗时 ~3秒      │
│   (持有 GIL)     │ 到队列  │  │ 占用队列内存     │  │ 执行 │   (持有 GIL)     │
│                 │        │  └────────────────┘  │     │       │          │
│                 │        │                      │     │       ▼          │
│                 │        │  事件循环在此期间      │     │ DataFrame (50MB) │
│                 │        │  ★完全阻塞★          │     │                 │
│                 │        │  无法处理任何 UI 事件  │     │ 内存峰值：        │
│                 │        │                      │     │ 队列中的 50MB    │
└─────────────────┘      └──────────────────────┘     │ + 新 DataFrame 50MB│
                                                       │ ≈ 100MB           │
                                                       └─────────────────┘
总耗时：pickle 序列化 ~2s + 队列拷贝 + pickle 反序列化 ~3s ≈ 5 秒主线程卡死


【传文件路径 + 读 Parquet】
Worker 线程                         Qt 事件队列               主线程
┌─────────────────┐      ┌──────────────────────┐     ┌─────────────────┐
│ DataFrame (50MB) │      │                      │     │                 │
│       │          │      │  ┌────────────┐      │     │                 │
│       ▼          │      │  │ "/tmp/sim   │      │     │                 │
│ ① to_parquet()   │      │  │  _xxx.      │      │     │                 │
│   耗时 ~0.3秒    │      │  │  parquet"   │      │     │                 │
│   (Arrow C++,    │      │  │  (80 bytes) │      │     │                 │
│    释放 GIL)     │      │  └────────────┘      │     │                 │
│       │          │      │                      │     │                 │
│       ▼          │      │  事件循环继续运行 ✓   │     │                 │
│  Parquet 文件    │      │  可以响应鼠标/重绘     │     │ ③ pd.read_      │
│  ~10MB (压缩)    │      │                      │     │   parquet()     │
│                 │      │                      │     │   耗时 ~0.3秒    │
│  内存峰值：50MB  │      │  队列内存：80 bytes   │     │   (Arrow C++,    │
└─────────────────┘      └──────────────────────┘     │    释放 GIL)     │
                                                       │       │          │
                                                       │       ▼          │
                                                       │ DataFrame (50MB) │
                                                       │                 │
                                                       │ 内存峰值：50MB    │
                                                       └─────────────────┘
总耗时：写 parquet ~0.3s + 读 parquet ~0.3s ≈ 0.6 秒，主线程仅 0.3s 在做读盘
```

**三条根本差异**：

| 差异维度 | 传 DataFrame 通过信号 | 传文件路径 + 读 Parquet |
|---------|---------------------|----------------------|
| **序列化引擎** | `pickle`（Python 纯解释执行，逐对象递归遍历 DataFrame 内部的 Index、columns、dtype、block manager……） | Apache Arrow C++ 原生库（列式二进制格式，编译优化，SIMD 加速） |
| **何时阻塞主线程** | **反序列化发生在事件分发阶段**——Qt 从队列取出事件后、调用 slot 之前，必须先 `pickle.loads()` 还原对象，此期间事件循环完全冻结 | 只有 `pd.read_parquet()` 调用期间阻塞，但 Arrow C++ 做 I/O 时**释放 GIL**，Python 层可以穿插处理其他事件 |
| **内存峰值** | 序列化字节流留在事件队列中（~50MB）+ 主线程新 DataFrame（~50MB）**≈ 100MB**，且旧 DataFrame 在 Worker 线程还没释放 | Worker 写盘后 DataFrame 可回收 → 磁盘文件 ~10MB（Parquet 压缩）+ 主线程一个 DataFrame **≈ 60MB** |
| **数据体积** | pickle 无压缩，~50MB | Parquet 列式压缩（snappy/gzip），~10MB |

**核心结论——回答「体量一样大为什么快」**：

> 瓶颈不在**数据搬运的字节数**，而在**字节和 Python 对象之间的转换速度**。pickle 是 Python 层面的递归对象图遍历，每碰到一个 Python 对象就要查 dispatch table、调 `__reduce__`；Arrow/Parquet 是 C++ 层面的列式批量转换，一整列数值一条 SIMD 指令就处理完了。同样 50MB 的 DataFrame，pickle 往返要 5 秒，Arrow 不到 0.6 秒——差距近 **10 倍**。

**还有一个重要但容易被忽略的点**：

信号传 DataFrame 时，反序列化发生在 **Qt 事件循环的内部代码路径**中——这不是你的 Python 代码，你无法控制、无法优化、甚至无法打断。而读文件发生在你自己的 `on_finished` slot 里，你是这段代码的主人：你可以在读之前先更新 UI（「正在加载结果…」），你可以用 `QApplication.processEvents()` 手动让出控制权，你可以异步读、分块读——**可控性完全不同**。

**问题二：Worker 销毁时序导致偶发崩溃**

```python
thread.quit()      # QThread 结束事件循环
worker.deleteLater()  # Worker 可能还在处理最后的事件
# → 偶尔 crash，极难复现

# ✅ 正确做法
thread.quit()
thread.wait()      # 阻塞等待线程真正退出
worker.deleteLater()  # 此时安全
```

**问题三：高频 progress 信号淹没事件队列**

仿真有几百个迭代步骤，每步 emit 一次 `progress(int)`。如果不加节制，主线程事件队列被挤爆，UI 反而响应变慢：

```python
# ✅ 节流：只在进度变化 > 5% 时才 emit
_last_emit_pct = -1
for i, step in enumerate(steps):
    step.compute()
    pct = int(i / len(steps) * 100)
    if pct - self._last_emit_pct >= 5:
        self.progress.emit(pct)
        self._last_emit_pct = pct
```

---

#### Q4：Python GIL 对 QThread 信号槽有影响吗？

这是一个很好的「区分理解」问题——答案分两层：

**第一层：信号槽机制本身不受 GIL 影响**

QThread 是 Qt 的 C++ 层创建的**真实 OS 线程**，不是 Python 的 `threading.Thread` 的封装。信号入队/出队、事件循环调度都在 C++ 层完成，不经过 GIL。

**第二层：Worker 里的 Python 代码受 GIL 影响**

```
emit 信号 → C++ 层（无 GIL）           ← 不受影响
    ↓
事件队列调度 → C++ 层（无 GIL）          ← 不受影响
    ↓
slot 函数执行 → Python 层（需 GIL）     ← 受 GIL 影响，但主线程通常持有 GIL
```

但在这个项目里影响很小：
- 仿真计算的核心（NumPy/SciPy）在 C 扩展中执行时**会释放 GIL**
- 只有纯 Python 的循环控制逻辑持有 GIL，占比很低

> 如果未来仿真规模大到纯 Python 计算成为瓶颈，升级方向是 `multiprocessing`（每个进程独立 GIL），而不是继续用 QThread。

**跟其他方案对比**：

| 方案 | GIL 影响 | 数据传递 | 适用场景 |
|------|---------|---------|---------|
| **QThread + 信号槽** | Python 代码段受影响，C 扩展可释放 | 信号槽自动拷贝，支持复杂类型 | GUI 应用的标准选择 |
| **`threading.Thread` + `queue.Queue`** | 同 QThread | 手动管理队列，无类型安全 | 非 GUI 场景的后台任务 |
| **`asyncio`** | 单线程，无 GIL 竞争 | N/A | 纯 I/O 密集（网络请求等） |
| **`multiprocessing`** | **无 GIL 影响**（独立进程） | 必须 pickle 序列化，大对象开销大 | CPU 密集 + 大计算量 |

---

#### Q5：面试现场——如何用 3 分钟讲清楚跨线程通信？

> 面试时就按这个逻辑线讲，从「问题」到「方案」到「原理」：

**① 先说问题（10 秒）**
> 仿真计算一次可能跑几十秒，如果放在主线程，整个 GUI 会卡死，用户连窗口都拖不动。

**② 再说方案（30 秒）**
> 我用 QThread 把计算逻辑移到工作线程。核心是 Worker + moveToThread 模式——Worker 是纯 QObject，只包含业务逻辑，通过信号槽跟主线程的 GUI 通信。计算过程中 emit progress 更新进度条，算完 emit finished 展示结果，出错 emit error 弹窗提示。

**③ 最后说原理亮点（1-2 分钟）**
> 信号槽跨线程通信的底层是 Qt 的事件队列机制。不同线程间默认走 QueuedConnection——信号参数被深拷贝到接收线程的事件循环队列里，保证 slot 总是在接收者线程执行，所以 Worker emit 的 finished 信号，slot 一定跑在主线程，安全地更新 UI。
>
> 实际开发中踩过几个坑：第一时间传大对象（DataFrame）导致主线程反序列化时卡顿，改成只传轻量标识符、按需加载；第二 Worker 销毁时序问题——必须先 wait 线程再 delete Worker，否则偶发崩溃；第三高频 progress 信号挤爆事件队列，加了 5% 节流阈值。
>
> 还有一点面试官可能关心：Python GIL 对这块的影响。QThread 是 OS 原生线程，信号槽调度在 C++ 层完成不经过 GIL，只有 Worker 里的纯 Python 代码受 GIL 限制，但我们仿真计算的核心在 NumPy C 扩展里跑，会主动释放 GIL，所以实际影响很小。

---

#### Q6：追问——如果 Worker 在执行中主窗口被关闭了怎么办？

```python
class MainWindow(QMainWindow):
    def closeEvent(self, event):
        if self.thread and self.thread.isRunning():
            # 1. 通知 Worker 停止
            self.worker.cancel()
            # 2. 请求 QThread 退出事件循环
            self.thread.quit()
            # 3. 等待线程真正结束（设超时防止卡死）
            if not self.thread.wait(3000):
                # 超时强制终止（最后手段）
                self.thread.terminate()
                self.thread.wait()
        event.accept()
```

> 面试要点：强调「协作式取消 + 超时兜底」。直接 `terminate()` 可能让 Worker 持有的文件句柄、DynamoDB 连接泄露，所以优先用 `cancel()` 让 Worker 自己清理。

---

### 6.5 QThread 完整生命周期 🔧

面试官如果问「从技术角度说说 QThread 是怎么运作的」，你要能画出这个状态机：

```
                          ┌─────────────┐
                          │  未启动      │  isRunning() → False
                          │  (初始状态)   │  isFinished() → False
                          └──────┬──────┘
                                 │ thread.start()
                                 ▼
                          ┌─────────────┐
              ┌──────────►│   运行中     │  isRunning() → True
              │           │  (事件循环)   │  isFinished() → False
              │           └──────┬──────┘
              │                  │
              │     ┌────────────┼────────────┐
              │     │            │            │
              │  run()返回    thread.quit()  thread.terminate()
              │  (自然结束)   (优雅退出)     (强制终止 ⚠️)
              │     │            │            │
              │     ▼            ▼            ▼
              │           ┌─────────────┐
              │           │   已结束      │  isRunning() → False
              │           │  (线程死亡)   │  isFinished() → True
              │           └─────────────┘
              │                  ▲
              └──────────────────┘
              再次 start() → 报错！
              (QThread 不可重用，必须 new 新的)
```

**各方法的行为细节**：

| 方法 | 行为 | 线程安全 | 关键细节 |
|------|------|---------|---------|
| `start()` | 创建 OS 线程，线程内调用 `run()` | ✅ | 一个 QThread 对象只能 start 一次。第二次 start 直接报错 |
| `run()` | 默认启动事件循环 `exec()` | 内部 | 继承 QThread 才需要重写。Worker 模式不需要动它 |
| `quit()` | 让事件循环退出（`exit(0)`） | ✅ | **异步**——调用后立即返回，线程可能还在跑。等价于 `exit(0)` |
| `wait(timeout)` | 阻塞当前线程，等待目标线程死亡 | ✅ | **必须调用**，否则线程变成僵尸。timeout 后返回 `False` |
| `terminate()` | 强制杀死线程 | ⚠️ | **危险**：不释放 mutex、不析构栈对象、不清理资源。仅作为超时兜底 |
| `isRunning()` | 查询线程是否在运行 | ✅ | start 后 quit 前为 True |
| `isFinished()` | 查询线程是否已结束 | ✅ | run() 返回后为 True |
| `finished` 信号 | 线程结束时自动 emit | — | 可用来触发 cleanup 逻辑 |

**完整生命周期代码模板**：

```python
import threading  # 用于调试

class MainWindow(QMainWindow):
    def start_simulation(self):
        # 1. 创建对象
        self.thread = QThread()
        self.worker = SimulationWorker()

        # 2. 移动 Worker 到新线程
        self.worker.moveToThread(self.thread)

        # 3. 连接信号（在 moveToThread 之后！）
        self.thread.started.connect(self.worker.run)          # 线程启动 → Worker 干活
        self.worker.finished.connect(self.on_result)           # Worker 完成 → 更新 UI
        self.worker.finished.connect(self.thread.quit)         # Worker 完成 → 退出事件循环
        self.thread.finished.connect(self.on_thread_done)      # 线程结束 → 清理
        self.worker.error.connect(self.on_error)               # Worker 报错 → 弹窗

        # 4. 启动线程
        self.thread.start()

    def on_thread_done(self):
        """线程彻底结束后才安全清理"""
        self.worker.deleteLater()
        self.thread.deleteLater()
        # 验证线程已死亡
        print(f"Thread alive: {self.thread.isRunning()}")  # False
```

---

### 6.6 moveToThread 与线程亲和性（Thread Affinity）🔧

这是 QThread 最容易被问到的底层概念。

**什么是线程亲和性？**

Qt 中每个 QObject 在创建时被打上一个「归属线程」标签：

```python
worker = SimulationWorker()         # worker.thread() → 主线程
print(worker.thread())              # <QThread object at 0x...>（主线程）

self.worker.moveToThread(self.thread)
print(worker.thread())              # <QThread object at 0x...>（工作线程）
```

> 线程亲和性决定了两件事：① QObject 的事件在哪个线程处理；② 信号槽的默认连接类型。

**`moveToThread` 做了什么？**

```
moveToThread 之前：
┌─────────────┐      ┌─────────────┐
│  主线程       │      │  工作线程     │
│  ┌─────────┐ │      │  (空)        │
│  │ Worker  │ │      │             │
│  │ (QObject)│ │      │             │
│  └─────────┘ │      │             │
└─────────────┘      └─────────────┘
     ↑ 亲和性=主线程

moveToThread(工作线程) 之后：
┌─────────────┐      ┌─────────────┐
│  主线程       │      │  工作线程     │
│  (Worker    │       │  ┌─────────┐ │
│   已移走)    │       │  │ Worker  │ │
│             │       │  │ (QObject)│ │
│             │       │  └─────────┘ │
└─────────────┘      └─────────────┘
                          ↑ 亲和性=工作线程
```

**关键规则（面试易考）**：

| 规则 | 说明 |
|------|------|
| **不能移动有父对象的 QObject** | Qt 要求父子必须在同一线程。先 `moveToThread` 再 `setParent` |
| **必须在目标线程未运行时移动** | `moveToThread` 在 `thread.start()` **之前**调用 |
| **子对象自动跟随** | Worker 内部创建的 QObject 自动继承 Worker 的线程亲和性 |
| **信号连接要在 moveToThread 之后** | 否则连接类型可能在移动前就确定了 |

**为什么 `moveToThread` + Worker 优于继承 QThread？**

```
继承 QThread 的问题：
┌──────────────────────┐
│ MyThread : QThread    │
│  ├── 线程管理 (继承)   │  ← QThread 的职责
│  └── 业务逻辑 (重写)   │  ← 你的代码
└──────────────────────┘
问题：run() 里创建的 QObject 亲和性是「新线程」，
但 MyThread 对象本身的亲和性是「创建它的线程（主线程）」。
这导致 MyThread 的信号在旧线程 emit，槽在主线程执行——形不成真正的跨线程通信。

Worker + moveToThread 的正确方式：
┌──────────┐          ┌──────────────┐
│ QThread  │          │ Worker(QObject)│
│ (线程管理)│  拥有 ──►│ (业务逻辑)     │
└──────────┘          └──────────────┘
         │                      │
    亲和性=主线程          亲和性=工作线程
    (不管业务)            (只管计算，emit 在工作线程)
```

**面试答法**：
> QThread 的职责是「管理一个 OS 线程」，不应该承载业务逻辑。我用 Worker + moveToThread 模式，Worker 的线程亲和性绑定到工作线程，这样它 emit 的信号自然就是跨线程通信——Qt 自动走 QueuedConnection，slot 回到主线程安全更新 UI。

---

### 6.7 信号槽类型注册：自定义类型如何跨线程传递 🔧

这是一个「能答出来说明确实深入用过」的问题。

**为什么需要类型注册？**

跨线程信号槽走 `QueuedConnection` 时，Qt 需要**拷贝信号参数**。拷贝的前提是 Qt 知道这个类型的结构——Qt 通过 `QMetaType` 系统管理类型信息。

```python
# 内置类型 → 自动注册，直接能用
progress = pyqtSignal(int)          # ✅ QMetaType::Int
finished = pyqtSignal(str)          # ✅ QMetaType::QString
data_ready = pyqtSignal(dict)       # ✅ PyQt 自动转换 QVariantMap

# 自定义类型 → 需要显式注册
finished = pyqtSignal(pd.DataFrame)  # ❌ Qt 不认识 DataFrame，可能：
                                     #    - 参数丢失（收到 None）
                                     #    - 运行时警告
                                     #    - 跨线程时直接报错
```

**什么类型需要注册？**

| 类型 | 是否需要手动注册 | 说明 |
|------|:---:|------|
| `int`, `float`, `str`, `bool` | ❌ | C++ 基本类型，QMetaType 内置 |
| `list`, `dict` | ❌ | PyQt 自动转换 `QVariantList` / `QVariantMap` |
| `tuple` | ❌ | PyQt 自动转换 |
| `datetime` | ❌ | PyQt5 新版本支持 `QDateTime` 转换 |
| `numpy.ndarray` | ⚠️ | 不推荐通过信号传（太大），用文件路径代替 |
| `pd.DataFrame` | ⚠️ | 同上 |
| **自定义 `dataclass` / 普通类** | ✅ | Qt 完全不认识，需要 `@pyqtSlot` 指定类型 |
| **自定义 `QObject` 子类** | ✅ | 需要 `qRegisterMetaType` |

**两种注册方式**：

```python
# 方式一：PyQt5 装饰器（简单场景）
from PyQt5.QtCore import pyqtSignal, pyqtSlot

class Worker(QObject):
    # 声明信号参数类型
    result_ready = pyqtSignal(object)  # object 可传任意 Python 对象
                                       # 代价：跨线程时走 pickle 序列化

# 方式二：qRegisterMetaType（严格场景）
from PyQt5.QtCore import qRegisterMetaType
from dataclasses import dataclass

@dataclass
class SimulationResult:
    cop: float
    power_kw: float
    mass_flow: float
    timestamp: str

# 注册自定义类型
qRegisterMetaType(SimulationResult)

class Worker(QObject):
    finished = pyqtSignal(SimulationResult)  # ✅ 现在 Qt 认识这个类型了

    def run(self):
        result = SimulationResult(cop=3.42, power_kw=2.8, mass_flow=0.12, ...)
        self.finished.emit(result)  # 深拷贝传递，不是引用
```

**本项目的最佳实践**：

```python
# 策略：不用信号传大对象，用轻量标识 + 共享存储
class Worker(QObject):
    finished = pyqtSignal(str, str)  # (result_id, cache_path)
    progress = pyqtSignal(int)
    error = pyqtSignal(str)

    def run(self):
        try:
            result_id = str(uuid4())
            cache_path = f"/tmp/sim_{result_id}.parquet"

            # 计算过程
            df = self._do_heavy_computation()
            df.to_parquet(cache_path)  # 写入共享存储

            self.finished.emit(result_id, cache_path)  # 只传轻量标识
        except Exception as e:
            self.error.emit(str(e))
```

> **面试强调**：仿真场景数据量大（上万行），我设计的原则是「信号只传标识符，数据走共享存储」。这样既避免了 Qt 的类型注册复杂度，又避免了大数据拷贝导致的 UI 卡顿。

---

### 6.8 生产级完整代码骨架 🏗️

> 面试时如果被要求「写一段 QThread + 信号槽的代码」，以下是一个可直接套用的生产级模板。

```python
from PyQt5.QtCore import (
    QObject, QThread, pyqtSignal, pyqtSlot, QMutex, QWaitCondition
)
from PyQt5.QtWidgets import QMainWindow, QPushButton, QProgressBar, QLabel
from uuid import uuid4
import traceback


# ═══════════════════════════════════════════════
# Worker：纯业务逻辑，不感知线程
# ═══════════════════════════════════════════════
class SimulationWorker(QObject):
    # ── 信号声明（类型明确） ──
    started   = pyqtSignal()
    progress  = pyqtSignal(int)            # 0-100
    finished  = pyqtSignal(str, str)       # (result_id, cache_path)
    error     = pyqtSignal(str)            # 错误信息

    def __init__(self, config: dict):
        super().__init__()
        self.config = config
        self._cancelled = False
        self._mutex = QMutex()             # 保护取消标志

    def cancel(self):
        """线程安全地设置取消标志"""
        self._mutex.lock()
        self._cancelled = True
        self._mutex.unlock()

    def _is_cancelled(self) -> bool:
        self._mutex.lock()
        val = self._cancelled
        self._mutex.unlock()
        return val

    @pyqtSlot()
    def run(self):
        """Work 的入口：由 thread.started 信号触发"""
        try:
            self.started.emit()

            # ── 主计算循环 ──
            steps = self._build_simulation_steps()
            total = len(steps)
            last_pct = -1

            for i, step in enumerate(steps):
                # 协作式取消检查
                if self._is_cancelled():
                    self.finished.emit("cancelled", "")
                    return

                step.compute()

                # 节流 emit（避免淹没事件队列）
                pct = int((i + 1) / total * 100)
                if pct - last_pct >= 5:
                    self.progress.emit(pct)
                    last_pct = pct

            # ── 结果落盘 ──
            result_id = str(uuid4())
            cache_path = f"/tmp/sim_{result_id}.parquet"
            self._save_result(cache_path)

            self.finished.emit(result_id, cache_path)

        except Exception:
            self.error.emit(traceback.format_exc())

    def _build_simulation_steps(self):
        """构建仿真步骤列表"""
        return [...]  # 具体业务逻辑

    def _save_result(self, path: str):
        """结果写入共享存储"""
        pass


# ═══════════════════════════════════════════════
# MainWindow：只管 UI + 线程调度
# ═══════════════════════════════════════════════
class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self._thread = None
        self._worker = None
        self._setup_ui()

    def _setup_ui(self):
        self.btn_start = QPushButton("开始仿真")
        self.btn_cancel = QPushButton("取消")
        self.progress_bar = QProgressBar()
        self.status_label = QLabel("就绪")

        self.btn_start.clicked.connect(self.on_start)
        self.btn_cancel.clicked.connect(self.on_cancel)
        self.btn_cancel.setEnabled(False)

    # ── 启动 ──
    def on_start(self):
        # 防止重复启动
        if self._thread and self._thread.isRunning():
            return

        self.btn_start.setEnabled(False)
        self.btn_cancel.setEnabled(True)
        self.progress_bar.setValue(0)
        self.status_label.setText("仿真运行中...")

        # 创建线程与 Worker
        self._thread = QThread()
        self._worker = SimulationWorker(config=self._gather_config())

        # ✦ 移动 Worker 到工作线程
        self._worker.moveToThread(self._thread)

        # ✦ 连接信号（注意顺序：先 moveToThread，再 connect）
        self._thread.started.connect(self._worker.run)
        self._worker.progress.connect(self.progress_bar.setValue)
        self._worker.finished.connect(self._on_finished)
        self._worker.error.connect(self._on_error)

        # ✦ 启动
        self._thread.start()

    # ── 取消 ──
    def on_cancel(self):
        if self._worker:
            self._worker.cancel()
            self.status_label.setText("正在取消...")
            self.btn_cancel.setEnabled(False)

    # ── 完成回调（主线程执行） ──
    @pyqtSlot(str, str)
    def _on_finished(self, result_id: str, cache_path: str):
        if result_id == "cancelled":
            self.status_label.setText("已取消")
        else:
            self.status_label.setText(f"仿真完成: {result_id}")
            self._load_and_display(cache_path)

        self._cleanup_thread()

    # ── 错误回调（主线程执行） ──
    @pyqtSlot(str)
    def _on_error(self, err_msg: str):
        self.status_label.setText(f"仿真出错: {err_msg[:80]}")
        self._cleanup_thread()

    # ── 线程清理 ──
    def _cleanup_thread(self):
        """安全的线程清理流程"""
        if self._thread:
            self._thread.quit()
            if not self._thread.wait(5000):
                self._thread.terminate()
                self._thread.wait()

            self._worker.deleteLater()
            self._thread.deleteLater()
            self._worker = None
            self._thread = None

        self.btn_start.setEnabled(True)
        self.btn_cancel.setEnabled(False)

    # ── 窗口关闭 ──
    def closeEvent(self, event):
        if self._thread and self._thread.isRunning():
            self._worker.cancel()
            self._thread.quit()
            if not self._thread.wait(3000):
                self._thread.terminate()
                self._thread.wait()
        event.accept()

    def _gather_config(self) -> dict:
        """从 UI 控件收集仿真参数"""
        return {}

    def _load_and_display(self, cache_path: str):
        """从共享存储加载结果并更新 Plotly 图表"""
        pass
```

**代码骨架拆解（面试时逐个指出来）**：

| 设计点 | 代码体现 | 对应的问题 |
|--------|---------|-----------|
| **Worker 纯业务** | `SimulationWorker` 不继承 `QThread`，不 import 任何 Qt Widget | 可测试性、解耦 |
| **线程安全取消** | `QMutex` 保护 `_cancelled` 标志 | 多线程写同一变量 |
| **节流 emit** | `pct - last_pct >= 5` | 防止事件队列被淹没 |
| **大对象不传信号** | 结果写 parquet，信号只传路径 | 避免跨线程大数据拷贝 |
| **异常兜底** | `try/except` + `error` 信号 | Worker 崩溃不传播到主线程 |
| **清理顺序** | quit → wait(超时) → terminate(兜底) → wait → deleteLater | 防止资源泄漏 + 僵尸线程 |
| **防重复启动** | `_thread.isRunning()` 检查 | 用户双击按钮 |
| **窗口关闭保护** | `closeEvent` 中优雅结束 Worker | 退出时崩溃 |

---

### 6.9 调试技巧：跨线程信号槽常见错误 🔧

> 面试官问「调试经验」时，下面三个是最典型的：

**错误一：`Cannot send events to objects owned by a different thread`**

```
报错场景：
QCoreApplication::postEvent: Cannot send events to objects
owned by a different thread.

根因：在 Worker 线程中直接操作了属于主线程的 Widget。

❌ worker.run() 里写: self.parent().label.setText("xxx")
✅ 改成 emit 信号，让主线程的 slot 去 setText
```

**错误二：信号连接了但 slot 不触发**

```
排查清单：
1. 是否在 moveToThread() 之前 connect 了？
   → 如果是 DirectConnection，可能发信号时 Worker 还在主线程
   → 解决：connect 放在 moveToThread() 之后

2. 接收方线程有没有事件循环在跑？
   → 主线程默认有 (app.exec_())
   → QThread 默认有 (run() 里调 exec())
   → 裸 thread (不调 exec()) 没有事件循环 → QueuedConnection 永远不会被处理！

3. 参数类型是否注册？
   → 自定义类型没注册 → emit 时静默失败
   → 用 pyqtSignal(object) 做快速验证
```

**错误三：QThread 销毁时崩溃**

```
典型崩溃堆栈：
QThread: Destroyed while thread is still running

根因：QThread 对象被 Python GC 回收了，但 OS 线程还在跑。

✅ 永远先 wait() 再 deleteLater()
✅ 不要用局部变量存 QThread（函数返回就被回收）
✅ 存为 self._thread（跟 MainWindow 同生命周期）
```

**快速诊断脚本**：

```python
# 贴在 Worker.run() 开头，确认运行在正确的线程
def run(self):
    import threading
    print(f"[Worker] Python thread: {threading.current_thread().name}")
    print(f"[Worker] Qt thread: {self.thread()}")  # QObject::thread()
    print(f"[Worker] Is GUI thread? {self.thread() == QApplication.instance().thread()}")
    # ...
```

---

## 七、面试高频追问速查表

| 方向 | 可能的追问 | 回答要点 |
|------|-----------|---------|
| **架构** | "桌面应用做前后端分离，有必要吗？" | 只要存在长时计算 + 交互界面，分离就必要——Web 前端用 async/await，桌面用 QThread，本质一样 |
| **架构** | "MVC 在桌面应用里怎么落地？" | Model = 仿真引擎 + 数据层；View = .ui 文件；Controller = MainWindow 事件处理 + Worker 调度。三层分离后 Model 可独立测试 |
| **PyQt5** | "为什么选 PyQt5 而不是 Electron？" | 团队 Python 技术栈统一；仿真计算库（NumPy/SciPy）是 Python 生态；不需要 Web 技术的跨平台优势 |
| **PyQt5** | "信号槽和回调函数有什么区别？" | 信号槽是类型安全的、松耦合的、支持多对多连接；回调函数是紧耦合的一对一调用 |
| **跨线程通信** | "跨线程信号槽底层怎么实现的？" | Qt 自动选择 QueuedConnection，信号参数深拷贝到接收线程的事件队列，保证 slot 在接收方线程执行。主线程 GUI 更新安全 |
| **跨线程通信** | "emit 信号是同步还是异步？" | 跨线程默认异步（QueuedConnection）。emit 后立即返回，slot 等接收方事件循环调度。需要同步用 BlockingQueuedConnection，但易死锁 |
| **跨线程通信** | "传大对象（DataFrame）有什么问题？" | 信号参数在 emit 时深拷贝，大对象导致内存拷贝 + 主线程反序列化卡顿。实践：只传轻量标识（文件路径/缓存 key），接收方按需加载 |
| **跨线程通信** | "多个 Worker 同时发信号会乱序吗？" | Qt 事件队列 FIFO 保证单发送者有序。多发送者不保证全局顺序——如果顺序敏感，加时间戳或序列号，接收方排序 |
| **跨线程通信** | "Python GIL 对 QThread 有影响吗？" | 信号调度在 C++ 层不经过 GIL。Worker 里纯 Python 代码受 GIL 限制，但 NumPy C 扩展会释放 GIL，本项目中实际影响很小 |
| **QThread 生命周期** | "QThread 从 start 到销毁经历哪些状态？" | 未启动 → start() 创建 OS 线程 → 运行中 (事件循环) → quit() 退出循环 → wait() 等线程死亡 → deleteLater。terminate() 是最后兜底，危险 |
| **线程亲和性** | "moveToThread 做了什么？为什么不能继承 QThread？" | 改变 QObject 归属线程，保证 Worker 的 emit 来自工作线程而非主线程。继承 QThread 的问题是 run() 里创建的对象和 QThread 本身线程亲和性不一致 |
| **类型注册** | "自定义类型能跨线程传吗？需要做什么？" | 需要用 `qRegisterMetaType()` 注册，否则 QueuedConnection 拷贝失败。本项目的策略是避免传大对象，信号只传 str/int 标识符 |
| **调试** | "跨线程信号槽不触发，怎么排查？" | 三步：①检查 connect 是否在 moveToThread 之后 ②检查接收方线程有无事件循环 ③用 `pyqtSignal(object)` 排除类型注册问题 |
| **Plotly** | "Plotly 和 PyQt5 怎么集成的？" | 通过 QWebEngineView 嵌入 Plotly 生成的 HTML，信号槽驱动图表更新。大数据量下比 Matplotlib 交互性好、渲染快 |
| **Plotly** | "为什么不用 ECharts / Matplotlib？" | ECharts 需要维护 JS 代码，团队不熟；Matplotlib 交互性弱、大数据量卡。Plotly 在 Python 生态下调用最自然 |
| **Jenkins** | "为什么选 Jenkins 而不是 GitHub Actions？" | 公司内网环境，代码托管在内网 GitLab，Jenkins 部署在内网服务器，不需要出公网 |
| **Jenkins** | "并行 stage 怎么设计的？" | 静态检查和单元测试可以并行跑，但集成测试必须等前两步通过 |
| **Jenkins** | "HTML 测试报告怎么生成的？" | pytest JSON 输出 → Jinja2 模板渲染 → 产出独立 HTML，包含汇总卡片、失败详情、回归对比、历史趋势图 |
| **测试** | "仿真计算的测试怎么做？正确性怎么验证？" | 用已知参数的经典算例做黄金标准测试（Golden Test），对比输出误差在 1% 以内 |
| **DynamoDB** | "为什么用 DynamoDB 而不是 MySQL？" | 仿真参数是半结构化 JSON，DynamoDB 文档模型天然匹配；托管服务免运维；小规模用量几乎免费 |
| **DynamoDB** | "离线怎么办？" | 本地 JSON 文件做兜底缓存，联网后自动同步到 DynamoDB |
| **部署** | "PyInstaller 打包有什么坑？" | 大文件体积优化（用 `--onefile` + upx 压缩）；某些库需要手动指定 hidden import；杀毒软件误报 exe |
| **效能** | "40% 时间缩短怎么算的？" | 对比改造前后 10 次完整测试流程的耗时（从参数配置到拿到测试报告），取平均值。自动化消除了手动汇总截图/填 Excel 的环节 |

---

## 八、总结：面试一句话概括

> 在这个桌面仿真工具项目中，我做了四件事：一是用 MVC 模式 + QThread 信号槽机制实现了前后端分离，解决了长时仿真计算导致界面卡顿的问题；二是引入 Plotly 做交互式数据可视化，替代了静态 Matplotlib 图表；三是搭建了 Jenkins 自动流水线并设计了 HTML 测试报告模板，把测试报告生成时间缩短了 40%；四是用 DynamoDB + JSON 混合方案实现了仿真配置与测试记录的一键保存和历史回溯。这些改造让工具从"能用"变成了"好用"，显著提升了研发团队的验证效率。

---

## 🔗 关联笔记

- [[面试/面试自我介绍]] | 面试自我介绍稿（含本项目概述）
- [[面试/AppEegServer 项目面试 QA]] | 上一个项目（Web 前后端分离经验复用）
- [[面试/AI视觉检测-JD映射与面试预测]] | AI 视觉检测 JD 匹配分析
