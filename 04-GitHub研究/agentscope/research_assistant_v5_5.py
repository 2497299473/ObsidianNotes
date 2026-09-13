#!/usr/bin/env python3
"""
AI 研究助手 v5.5：事件流追踪与可观察性

在 v5 多 Agent 编排的基础上，增加 ResearchEventTracer，
对一次完整的研究任务进行全链路事件追踪，并生成结构化分析报告。

前置依赖：
    pip install agentscope
    set DASHSCOPE_API_KEY=your_key    # Windows
    export DASHSCOPE_API_KEY=your_key # macOS/Linux

运行：
    python research_assistant_v5_5.py

对应笔记: [[AgentScope 2.0 核心架构]] - Event System
"""

from __future__ import annotations

import asyncio
import os
import time
from collections import Counter, defaultdict
from typing import Any

from agentscope.agent import Agent
from agentscope.credential import DashScopeCredential
from agentscope.event import EventType
from agentscope.message import UserMsg
from agentscope.model import DashScopeChatModel
from agentscope.team import SubagentDeclaration, TeamTools
from agentscope.tool import Bash, Read, Toolkit, WebFetch, WebSearch, Write


# ============================================================================
# ResearchEventTracer — 事件流追踪器
# ============================================================================

class ResearchEventTracer:
    """追踪多 Agent 研究全流程的事件流，实现可观察性。

    本类使用 Python 3.10+ 的 match/case 语法，低版本请改用 if/elif。
    """

    def __init__(self) -> None:
        self.timeline: list[dict[str, Any]] = []
        self.current_round = 0

    async def trace(self, agent: Agent, user_msg: UserMsg) -> list[dict[str, Any]]:
        """追踪 Agent 执行的完整事件流，实时打印并返回结构化 timeline。

        Args:
            agent: 要追踪的 Agent 实例（通常是 Director）
            user_msg: UserMsg 对象

        Returns:
            list[dict]: 完整事件时间线
        """
        round_start = time.time()
        current_tool: str | None = None

        async for event in agent.reply_stream(user_msg):
            entry: dict[str, Any] = {
                "timestamp": time.time(),
                "elapsed_ms": round((time.time() - round_start) * 1000),
                "type": str(event.type),
            }

            match event.type:
                case EventType.REPLY_START:
                    self.current_round += 1
                    entry["round"] = self.current_round
                    print(f"\n{'=' * 50}")
                    print(f"🔄 Round {self.current_round} 开始")

                case EventType.TEXT_BLOCK_DELTA:
                    print(event.text, end="", flush=True)

                case EventType.TOOL_CALL_START:
                    current_tool = event.tool_name
                    entry["tool"] = current_tool
                    if current_tool == "agent_spawn":
                        agent_name = "?"
                        if isinstance(event.tool_input, dict):
                            agent_name = event.tool_input.get("agent_name", "?")
                        entry["spawn"] = agent_name
                        print(f"\n  📢 委派 → {agent_name}")
                    else:
                        print(f"\n  🔧 工具调用: {current_tool}")

                case EventType.TOOL_RESULT_END:
                    result_len = len(str(getattr(event, "result", "")))
                    entry["result_chars"] = result_len
                    entry["tool"] = current_tool
                    print(f"  ✅ 结果: {result_len} 字符")

                case EventType.TOOL_CALL_END:
                    current_tool = None

                case EventType.REPLY_END:
                    entry["usage"] = str(getattr(event, "usage", "N/A"))
                    print(f"\n  📊 Round {self.current_round} 完成")
                    print(f"{'=' * 50}")

                case EventType.ERROR:
                    entry["error"] = str(getattr(event, "error_message", "unknown"))
                    print(f"  ⚠️  错误: {entry['error']}")

            self.timeline.append(entry)

        return self.timeline

    def generate_report(self) -> None:
        """生成事件流分析报告并打印到控制台。"""
        if not self.timeline:
            print("（无事件记录）")
            return

        print("\n" + "=" * 60)
        print("📊 事件流分析报告")
        print("=" * 60)

        total_duration = self.timeline[-1]["elapsed_ms"] - self.timeline[0]["elapsed_ms"]

        # 1. 执行概况
        print(f"\n## 执行概况")
        print(f"  总耗时: {total_duration} ms")
        print(f"  ReAct 总轮次: {self.current_round}")
        print(f"  总事件数: {len(self.timeline)}")

        # 2. Worker 耗时
        spawn_events = [e for e in self.timeline if "spawn" in e]
        spawn_counter = Counter(e["spawn"] for e in spawn_events)
        # 估算每个 spawn 到下一个事件的延迟
        spawn_latency: dict[str, list[int]] = defaultdict(list)
        for i, e in enumerate(self.timeline):
            if "spawn" in e and i + 1 < len(self.timeline):
                next_e = self.timeline[i + 1]
                spawn_latency[e["spawn"]].append(next_e["elapsed_ms"] - e["elapsed_ms"])

        print(f"\n## 子 Agent 委派统计")
        for agent_name, count in spawn_counter.most_common():
            latencies = spawn_latency.get(agent_name, [])
            avg_ms = round(sum(latencies) / len(latencies)) if latencies else 0
            print(f"  {agent_name}: {count} 次, 平均耗时 {avg_ms} ms")

        # 3. 工具调用统计
        tool_events = [e for e in self.timeline if e.get("tool") and "spawn" not in e]
        tool_counter = Counter(e.get("tool", "unknown") for e in tool_events)
        print(f"\n## 工具调用统计")
        print(f"  总工具调用次数: {len(tool_events)}")
        print("  频率排行:")
        for name, count in tool_counter.most_common(5):
            print(f"    - {name}: {count} 次")

        # 4. 事件类型分布
        type_counter = Counter(e["type"] for e in self.timeline)
        print(f"\n## 事件类型分布 (Top 10)")
        for t, c in type_counter.most_common(10):
            print(f"    - {t}: {c}")

        print("=" * 60)


# ============================================================================
# Agent 定义（同 v5）
# ============================================================================

def create_agents() -> tuple[Agent, Agent, Agent, Agent]:
    """创建 Researcher / Analyst / Writer / Director 四个 Agent。"""
    api_key = os.environ.get("DASHSCOPE_API_KEY")
    if not api_key:
        raise RuntimeError(
            "请设置环境变量 DASHSCOPE_API_KEY\n"
            "  Windows: set DASHSCOPE_API_KEY=your_key\n"
            "  macOS/Linux: export DASHSCOPE_API_KEY=your_key"
        )

    model = DashScopeChatModel(
        credential=DashScopeCredential(api_key=api_key),
        model="qwen-plus",
    )

    # Researcher: 负责搜索和收集资料
    researcher = Agent(
        name="researcher",
        system_prompt="""你是研究资料搜集专家。对于给定的子课题:
1. 用 WebSearch 搜索相关中文和英文资料
2. 用 WebFetch 抓取最重要的 3-5 篇文章
3. 提取关键事实、数据、方法
4. 以结构化格式输出（含来源链接）

输出格式:
## 子课题: [名称]
### 关键发现
- 发现1 (来源: url)
- 发现2 (来源: url)
### 主要方法
### 重要数据""",
        model=model,
        toolkit=Toolkit(tools=[WebSearch(), WebFetch(), Read(), Write()]),
    )

    # Analyst: 负责深度分析
    analyst = Agent(
        name="analyst",
        system_prompt="""你是深度分析师。收到研究资料后:
1. 找出不同来源之间的共同发现和矛盾
2. 评估每条信息的可信度（来源权威性、时效性）
3. 发现研究空白（文献中没有回答的问题）
4. 提出值得深入的方向

输出格式:
## 分析报告
### 共识发现（多来源交叉验证）
### 矛盾与争议
### 研究空白
### 建议的深入方向""",
        model=model,
    )

    # Writer: 负责撰写综述
    writer = Agent(
        name="writer",
        system_prompt="""你是学术写作专家。收到分析报告后:
1. 撰写结构完整的综述报告
2. 包含: 摘要、引言、方法综述、关键发现、争议与展望、参考文献
3. 使用学术语言但保持可读性
4. 所有引用标注来源

输出格式: Markdown 格式的完整综述报告""",
        model=model,
    )

    # Director: 编排整个研究流程
    director = Agent(
        name="director",
        system_prompt="""你是研究项目主管。对于用户的每个研究问题:

流程:
1. 将问题分解为 3-5 个子课题
2. 并行 spawn 所有 researcher 处理各自的子课题
3. 等待所有 researcher 完成后，将汇总结果交给 analyst
4. 将分析报告交给 writer 生成最终综述
5. 向用户交付最终报告

质量要求:
- researcher 必须在 180 秒内完成
- 如果某个 researcher 的结果质量不够，重新 spawn 一次
- 最终报告必须包含至少 5 个引用来源""",
        model=model,
        toolkit=Toolkit(tools=[
            TeamTools(subagents=[
                SubagentDeclaration(
                    agent=researcher,
                    description="研究资料搜集专家: 搜索网络资料并提取关键信息",
                    max_concurrent=3,
                ),
                SubagentDeclaration(
                    agent=analyst,
                    description="深度分析师: 交叉验证多源信息并发现研究空白",
                    max_concurrent=1,
                ),
                SubagentDeclaration(
                    agent=writer,
                    description="学术写作专家: 将分析结果撰写为结构化的综述报告",
                    max_concurrent=1,
                ),
            ]),
        ]),
        max_rounds=15,
    )

    return researcher, analyst, writer, director


# ============================================================================
# 主流程
# ============================================================================

async def main() -> None:
    print("=" * 60)
    print("🤖 AI 研究助手 v5.5 — 事件流追踪与可观察性")
    print("=" * 60)

    _, _, _, director = create_agents()
    tracer = ResearchEventTracer()

    topic = "大语言模型在医疗诊断中的可靠性研究"
    print(f"\n📋 研究课题: {topic}\n")

    # 使用 tracer.trace() 替代直接 reply_stream()
    timeline = await tracer.trace(
        director,
        UserMsg("scientist", f"研究课题: {topic}"),
    )

    # 生成分析报告
    tracer.generate_report()

    print("\n✅ 研究完成")


if __name__ == "__main__":
    if "DASHSCOPE_API_KEY" not in os.environ:
        raise SystemExit(
            "请先设置环境变量 DASHSCOPE_API_KEY\n"
            "  Windows: set DASHSCOPE_API_KEY=your_key\n"
            "  macOS/Linux: export DASHSCOPE_API_KEY=your_key"
        )
    asyncio.run(main())
