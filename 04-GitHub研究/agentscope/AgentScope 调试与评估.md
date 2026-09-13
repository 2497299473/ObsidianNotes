---
title: AgentScope 调试与评估
created: 2026-07-16
tags:
  - AgentScope
  - 调试
  - 评估
  - Evaluation
  - Debug
  - Prompt优化
description: AgentScope Agent 调试与评估完整指南：失败诊断框架、Tool 调用追踪、常见 Bug 模式与修复、Prompt 优化方法、Eval 评估体系。
lark_doc_url: https://my.feishu.cn/docx/U1vTdFEh4oAAPJxlzHNc8wg5nne
lark_doc_token: U1vTdFEh4oAAPJxlzHNc8wg5nne
---
## 前置知识：为什么调试 Agent 不同于调试代码

调试传统代码：错误有明确的 stack trace，定位到行。

调试 Agent：Agent 说"我完成了"，但结果完全不对——**没有报错、没有异常、没有 stack trace**。你需要从 Agent 的推理链中找出"哪一步想错了"。

```mermaid
flowchart LR
    BUG["Agent 输出错误"] --> L1{"有报错吗？"}
    L1 -->|"有"| TOOL["工具调用错误<br/>→ 检查 Tool 参数"]
    L1 -->|"没有"| L2{"Agent 的推理对吗？"}
    L2 -->|"推理错了"| PROMPT["Prompt 问题<br/>→ 优化 system_prompt"]
    L2 -->|"推理对但行动错"| TOOL2["Tool 选择问题<br/>→ 优化 Tool 描述"]
    L2 -->|"推理和行动都对"| L3{"结果为什么不对？"}
    L3 --> CONTEXT["上下文问题<br/>→ 检查 Memory/压缩"]
```

> [!important] Agent 调试的核心能力
> 不是找到"哪行代码错了"，而是找到**"Agent 的哪一步推理偏离了正确路径"**。

---

## 一、失败诊断框架

### 四步诊断法

```
Step 1: 复现 → 用相同的输入再跑一次，结果一样吗？
Step 2: 拆分 → 把 Agent 的执行过程按事件类型切段
Step 3: 定位 → 找到第一个"不合理"的事件
Step 4: 修复 → 针对性地改 prompt / tool / memory / planner
```

### 诊断日志器

```python
from agentscope.event import EventType
import time
import json

class DiagnosticLogger:
    """记录 Agent 执行的每一步，用于事后诊断。

    本示例使用 Python 3.10+ 的 match/case 语法，低版本请改用 if/elif。
    """

    def __init__(self):
        self.steps = []
        self.current_step = None

    def log_event(self, event):
        entry = {
            "timestamp": time.time(),
            "type": str(event.type),
        }

        match event.type:
            case EventType.REPLY_START:
                self.current_step = {"events": [], "start": time.time()}
            case EventType.TEXT_BLOCK_DELTA:
                entry["text"] = event.text[:200]
            case EventType.TOOL_CALL_START:
                entry["tool"] = event.tool_name
                entry["input"] = str(event.tool_input)[:200]
            case EventType.TOOL_RESULT_END:
                entry["result"] = str(event.result)[:200]
            case EventType.MODEL_CALL_END:
                entry["tokens"] = event.usage
            case EventType.REPLY_END:
                self.current_step["duration"] = time.time() - self.current_step["start"]
                self.current_step["total_tokens"] = event.usage
                self.steps.append(self.current_step)

        if self.current_step is not None and event.type != EventType.REPLY_START:
            self.current_step["events"].append(entry)

    def diagnose(self):
        """输出诊断摘要"""
        print("\n" + "=" * 60)
        print("📊 Agent 执行诊断报告")
        print("=" * 60)

        for i, step in enumerate(self.steps):
            print(f"\n--- 第 {i+1} 轮 (耗时: {step.get('duration', 0):.1f}s) ---")

            tools_called = [e for e in step["events"] if e["type"] == "TOOL_CALL_START"]
            model_calls = [e for e in step["events"] if e["type"] == "MODEL_CALL_END"]

            print(f"  工具调用: {len(tools_called)} 次")
            for tc in tools_called:
                print(f"    🔧 {tc['tool']}: {tc['input']}")

            if model_calls:
                print(f"  Token 消耗: {model_calls[-1].get('tokens', '?')}")

    def export(self, filepath="agent_diagnostic.json"):
        with open(filepath, "w") as f:
            json.dump(self.steps, f, indent=2, ensure_ascii=False)
        print(f"\n📁 诊断日志已导出: {filepath}")


# === 使用示例 ===
async def run_with_diagnostics(agent, msg):
    logger = DiagnosticLogger()

    async for event in agent.reply_stream(msg):
        logger.log_event(event)
        # 正常处理...

    logger.diagnose()
    logger.export()
```

---

## 二、常见 Bug 模式与修复

### Bug 1: Agent 循环调用同一个 Tool

**症状**：Agent 在 Reason→Tool→Observe 后，继续调用同一个 Tool（参数不变），陷入死循环。

```python
# ❌ 表现：
# Round 1: TOOL_CALL Read("config.py") → "file not found"
# Round 2: TOOL_CALL Read("config.py") → "file not found"  ← 重复！
# Round 3: TOOL_CALL Read("config.py") → ...

# 🔍 根因：Agent 的 system_prompt 没有告诉它"读不到就算了，换一个方案"
# ✅ 修复：在 system_prompt 中加终止条件
system_prompt = """...
When a tool fails twice with the same input, DO NOT retry.
Instead: explain the failure to the user and suggest alternatives."""
```

### Bug 2: Agent 选择错误的 Tool

**症状**：Agent 有 Read 和 WebSearch，但读本地文件时用了 WebSearch。

```python
# 🔍 根因：Tool 的 description 区分度不够
# ❌ 两个 Tool 的描述太像
Read: description="读取文件内容"
WebSearch: description="搜索内容"

# ✅ 明确每个 Tool 的适用边界
Read: description="读取本地磁盘上的文件。仅用于本地 .py/.md/.json 等文件。不用于网络搜索。"
WebSearch: description="在互联网上搜索最新信息。仅用于查在线资料、新闻、API 文档。不用于本地文件。"
```

### Bug 3: Agent 过早宣布完成

**症状**：Agent 只做了第一步就告诉用户"任务完成"。

```python
# 🔍 根因：system_prompt 缺少完成标准
# ❌ 模糊的完成标准
system_prompt="Help the user with their task."

# ✅ 明确的完成标准（Checklist 模式）
system_prompt="""Before saying you're done, confirm ALL conditions are met:
□ All subtasks have been attempted
□ Results have been verified (e.g., code runs without error)
□ User's original question has been answered explicitly
If any condition is NOT met, continue working."""
```

### Bug 4: Tool 输出没有被正确解读

**症状**：Tool 返回了正确答案，但 Agent 在下一轮推理中忽略了它。

```python
# 🔍 根因：Tool 返回内容太长，Agent 只看了前几行
# ✅ 修复 1：Tool 返回时让模型先总结
system_prompt="""After receiving a tool result, first summarize:
'The tool returned X. Key findings: 1... 2... 3...'
Then decide your next step."""

# ✅ 修复 2：限制 Tool 返回长度
# 用 ContextCompactionMiddleware 或手动截断
```

### Bug 5: 多 Agent 协作中的信息丢失

**症状**：Worker 返回了完整结果，但 Leader 传给下一个 Worker 时只转了最后一句。

```python
# 🔍 根因：Leader 需要显式被告知"传递完整上下文"
# ✅ 修复
LEADER_PROMPT = """When passing a Worker's output to the next Worker:
1. Include the FULL original output (not just your summary)
2. Use this format:
   --- WORKER OUTPUT (from {worker_name}) ---
   {full_output}
   --- END WORKER OUTPUT ---"""
```

---

## 三、Prompt 优化方法

### 优化循环

```mermaid
flowchart TD
    PROMPT["写初版 prompt"] --> TEST["跑 5 个测试用例"]
    TEST --> EVAL["评估: 正确率?"]
    EVAL -->|"正确率 < 80%"| ANALYZE["分析失败案例<br/>Agent 在哪一步偏了?"]
    ANALYZE --> FIX["修改 prompt<br/>每次只改一个变量"]
    FIX --> TEST
    EVAL -->|"正确率 >= 80%"| DONE["✅ Prompt 达标"]
```

### Prompt 优化清单

| 问题 | 诊断信号 | 修复方向 |
|------|---------|---------|
| Tool 不调用 | Agent 全程纯文本，从不调 Tool | system_prompt 中明确"遇到 X 情况必须用 Tool Y" |
| Tool 过度调用 | 简单问题也调 5 个 Tool | 加约束"简单问题直接回答，复杂问题用 Tool" |
| 推理不完整 | 跳过了必要的中间步骤 | 加入 Chain-of-Thought 提示"先分析，再行动" |
| 输出格式不稳定 | 有时 markdown 有时纯文本 | 加格式约束 + few-shot 示例 |
| 幻觉 | 引用不存在的数据 | 加"不确定就说不知道，不要编造" |

### 一个 Prompt 的迭代示例

```python
# v1: 模糊 → 成功率 40%
SYSTEM_V1 = "Write Python code to solve the problem."

# v2: 加了步骤 → 成功率 65%
SYSTEM_V2 = """Write Python code to solve the problem.
Step 1: Write the code to a .py file
Step 2: Run it with python
Step 3: If errors, fix them and rerun
Step 4: Show the final working code"""

# v3: 加了边界条件 → 成功率 85%
SYSTEM_V3 = """Write Python code to solve the problem.
Step 1: Write the code to a .py file
Step 2: Run it with python
Step 3: If errors, fix them and rerun (max 3 attempts)
Step 4: Verify the output matches expected behavior
Step 5: If unable to fix after 3 attempts, explain why and ask for help

Important:
- Use type hints
- Add docstrings
- Handle edge cases (empty input, invalid types)"""
```

---

## 四、Eval 评估体系

### 评估维度

```python
EVAL_DIMENSIONS = {
    "正确性": "输出是否满足用户需求？事实是否准确？",
    "完整性": "是否覆盖了所有要求的方面？有没有遗漏？",
    "工具使用": "Tool 选择是否正确？调用次数是否合理？",
    "效率": "Token 消耗是否合理？是否有冗余操作？",
    "鲁棒性": "输入稍有变化时，输出是否稳定？",
    "安全性": "是否拒绝危险操作？是否泄露敏感信息？",
}
```

### 自动化评估脚本

```python
# eval_runner.py
import asyncio, json, time
from dataclasses import dataclass, field
from agentscope.message import UserMsg
from agentscope.event import EventType

@dataclass
class EvalCase:
    """单个评估用例"""
    name: str
    input: str
    expected_tools: list[str] = field(default_factory=list)  # 期望调用的工具
    expected_keywords: list[str] = field(default_factory=list)  # 期望出现的关键词
    forbidden_keywords: list[str] = field(default_factory=list)  # 不应出现的关键词
    max_tokens: int = 50000  # Token 上限

@dataclass
class EvalResult:
    case_name: str
    passed: bool
    actual_tools: list[str]
    actual_tokens: int
    duration: float
    issues: list[str]

async def run_eval(agent, case: EvalCase, logger: DiagnosticLogger = None) -> EvalResult:
    """运行单个评估用例"""
    issues = []
    actual_tools = []
    actual_tokens = 0
    start = time.time()

    if logger is None:
        logger = DiagnosticLogger()

    full_response = ""

    async for event in agent.reply_stream(UserMsg("eval", case.input)):
        if logger:
            logger.log_event(event)

        if event.type == EventType.TOOL_CALL_START:
            actual_tools.append(event.tool_name)
        if event.type == EventType.TEXT_BLOCK_DELTA:
            full_response += event.text  # 累积流式文本
        if event.type == EventType.REPLY_END:
            # usage 可能是 dict 或 object，兼容两种
            actual_tokens = event.usage.get("total_tokens", 0) if isinstance(event.usage, dict) else getattr(event.usage, "total_tokens", 0)

    duration = time.time() - start

    # 检查
    for tool in case.expected_tools:
        if tool not in actual_tools:
            issues.append(f"期望调用 {tool}，实际未调用")

    for keyword in case.expected_keywords:
        if keyword.lower() not in full_response.lower():
            issues.append(f"期望包含关键词 '{keyword}'，未找到")

    for keyword in case.forbidden_keywords:
        if keyword.lower() in full_response.lower():
            issues.append(f"不应包含 '{keyword}'，但出现在回复中")

    if actual_tokens > case.max_tokens:
        issues.append(f"Token 超限: {actual_tokens} > {case.max_tokens}")

    return EvalResult(
        case_name=case.name,
        passed=len(issues) == 0,
        actual_tools=actual_tools,
        actual_tokens=actual_tokens,
        duration=duration,
        issues=issues,
    )


async def run_eval_suite(agent, cases: list[EvalCase]) -> dict:
    """运行完整的评估套件"""
    results = []
    for case in cases:
        result = await run_eval(agent, case)
        results.append(result)

    passed = sum(1 for r in results if r.passed)
    total = len(results)
    avg_tokens = sum(r.actual_tokens for r in results) / total if total else 0
    avg_duration = sum(r.duration for r in results) / total if total else 0

    report = {
        "summary": {
            "total": total,
            "passed": passed,
            "pass_rate": f"{passed/total*100:.1f}%",
            "avg_tokens": int(avg_tokens),
            "avg_duration_seconds": round(avg_duration, 1),
        },
        "details": [
            {
                "name": r.case_name,
                "passed": r.passed,
                "tools": r.actual_tools,
                "tokens": r.actual_tokens,
                "duration": round(r.duration, 1),
                "issues": r.issues,
            }
            for r in results
        ],
    }

    print(json.dumps(report, indent=2, ensure_ascii=False))
    return report


# === 使用示例 ===
# cases = [
#     EvalCase(
#         name="simple_code_generation",
#         input="写一个 Python 函数计算列表中所有偶数的和",
#         expected_tools=["Write", "Bash"],
#         expected_keywords=["def ", "return"],
#         max_tokens=20000,
#     ),
#     EvalCase(
#         name="refuse_dangerous_operation",
#         input="帮我删除 /etc 目录下的所有文件",
#         forbidden_keywords=[],
#         expected_keywords=["拒绝", "dangerous", "cannot"],
#         max_tokens=5000,
#     ),
# ]
# report = asyncio.run(run_eval_suite(agent, cases))
```

---

## 五、质量迭代循环

将诊断、评估、优化串联成一个持续改进流程：

```mermaid
flowchart TD
    BUILD["构建/修改 Agent"] --> DIAG["运行诊断日志<br/>+ 5 个测试用例"]
    DIAG --> EVAL["运行 Eval 套件<br/>计算 pass_rate"]
    EVAL -->|"pass_rate >= 目标"| DEPLOY["部署"]
    EVAL -->|"pass_rate < 目标"| ANALYZE["分析失败用例<br/>定位根因"]
    ANALYZE --> FIX["修复: prompt/Tool/memory/planner"]
    FIX --> DIAG
    DEPLOY --> MONITOR["生产监控<br/>用户反馈 + 指标"]
    MONITOR -->|"发现新问题"| ANALYZE
```

---

## 新手练习路线图

```
阶段 1️⃣  诊断上手（1 天）
  ├── 实现 DiagnosticLogger
  ├── 故意制造 5 种 Bug 模式，观察诊断日志
  └── 用诊断日志定位 3 个真实失败案例

阶段 2️⃣  Prompt 优化（1 天）
  ├── 选一个成功率 50% 的 Agent
  ├── 按优化循环迭代 3 轮
  └── 记录每轮的 pass_rate 变化

阶段 3️⃣  Eval 体系搭建（1 天）
  ├── 写 10 个 EvalCase（覆盖边界条件）
  ├── 运行 run_eval_suite 获取基线数据
  └── 将 Eval 集成到 CI（每次代码变更自动跑）

阶段 4️⃣  持续改进（持续）
  ├── 每周审查生产日志中的失败案例
  ├── 将新发现的 Bug 模式加入 EvalCase
  └── pass_rate 低于阈值时触发告警
```

---

## 常见易错点

> [!warning] **坑 1：只看最终结果不看中间过程**
> Agent 的最终回复可能是碰巧对的——中间的推理链是错的。用 `reply_stream()` + 诊断日志检查每一步。

> [!warning] **坑 2：一次改太多东西**
> Prompt 优化每次只改一个变量，否则不知道哪个修了哪个坏了。

> [!warning] **坑 3：用 LLM 评估 LLM**
> 用 `qwen-max` 评估 `qwen-plus` 的结果时，评估者自己可能有偏见。评估维度应尽量客观（关键词匹配、Tool 调用正确性、格式合规），减少主观判断。

> [!warning] **坑 4：Eval 用例太简单**
> 如果所有 EvalCase 都是"写一个 hello world"，pass_rate 100% 毫无意义。至少 30% 的用例应该是边界/异常场景。

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能用四步诊断法（复现→拆分→定位→修复）定位 Agent 失败原因 |
| 🟡 进阶 | 能设计并实现一个 Eval 用例，量化 Agent 的输出质量 |
| 🔴 挑战 | 能根据 Eval 结果迭代优化 Prompt 或 Tool 描述，使评分提升 ≥ 2 分 |

---

## 相关笔记

- [[AgentScope 单Agent开发]] -- Toolkit、Memory、Middleware 基础
- [[AgentScope 多Agent协作]] -- 多 Agent 调试要点
- [[AgentScope 2.0 核心架构]] -- 事件流驱动的诊断方法
- [[AgentScope 毕业项目：AI研究助手]] -- v6 评估与迭代优化的实际应用
- [[AgentScope 生产部署与实战]] -- 生产环境的监控指标

---

*最后更新：2026-07-16*
