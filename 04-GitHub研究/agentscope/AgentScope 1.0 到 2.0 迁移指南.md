---
title: AgentScope 1.0 到 2.0 迁移指南
created: 2026-07-16
tags:
  - AgentScope
  - 迁移
  - 1
  - 2
  - MsgHub
  - Pipeline
description: AgentScope 从 1.0 迁移到 2.0 的完整对照指南：核心概念变更对照表、MsgHub→Orchestrator 迁移、Pipeline→agent_spawn 迁移、API 变化、迁移 Checklist。
lark_doc_url: https://my.feishu.cn/docx/PYPJdeKfyojN2wxO2RKc56K3ngb
lark_doc_token: PYPJdeKfyojN2wxO2RKc56K3ngb
---
## 前置知识：为什么要迁移

AgentScope 2.0 不是小修小补，而是一次**重新设计**。迁移的驱动力来自 LLM 能力的质变：

| 1.0 的假设 | | 2.0 的现实 |
|-----------|--|-----------|
| LLM 不够聪明，需要框架预设流程 | → | LLM 具备强推理和工具调用能力 |
| 用 `MsgHub` 广播消息最自然 | → | Leader 显式分发更可控 |
| `Pipeline` 硬编码足够灵活 | → | Agent 动态决策更灵活 |
| 开发和部署是两个阶段 | → | Agent Service 把部署融入开发 |

---

## 核心概念变更对照表

| 1.0 概念 | 2.0 替代 | 变化性质 |
|----------|---------|---------|
| `MsgHub` | 不需要（Leader 隐式管理） | 删除 |
| `SequentialPipeline` | Leader 串行 `agent_spawn` | 替换 |
| `FanoutPipeline` | Leader 异步 `agent_spawn` | 替换 |
| `Pipelines.conversation()` | Leader 轮询 `agent_spawn` | 替换 |
| `Pipeline.run(x)` | `agent.reply(x)` | 替换 |
| `agent_server` (独立仓库) | Agent Service (主库内置) | 合并 |
| Callback 回调 | Event System (28种事件) | 增强 |
| 无 | Permission System (三层引擎) | 新增 |
| 无 | Middleware (六大Hook) | 新增 |
| 无 | Workspace (Local/Docker/E2B) | 新增 |

---

## 迁移一：MsgHub → Orchestrator + Workers

### 1.0 写法

> [!note] 以下 1.0 代码为示意写法
> AgentScope 1.0 的精确 import 路径（如 `agentscope.core` vs `agentscope.agents`）可能因小版本而异，请以实际安装的 1.0 包路径为准。核心概念（MsgHub、Pipeline）的用法是正确的。

```python
from agentscope.core import MsgHub, Agent  # 示意路径

# 创建 Agent
alice = Agent(name="alice", ...)
bob = Agent(name="bob", ...)
charlie = Agent(name="charlie", ...)

# 放入 MsgHub，任意 Agent 的消息自动广播给其他人
hub = MsgHub()
hub.add(alice, bob, charlie)

# 启动群聊——所有 Agent 自动收到消息并回复
hub.broadcast("讨论一下系统架构设计")
```

### 2.0 写法

```python
from agentscope.agent import Agent
from agentscope.tool import Toolkit
from agentscope.team import TeamTools, SubagentDeclaration

# 创建 Agent
alice = Agent(name="alice", ...)
bob = Agent(name="bob", ...)
charlie = Agent(name="charlie", ...)

# Host Agent 显式管理参与者
host = Agent(
    name="host",
    system_prompt="""You are hosting a group discussion with Alice, Bob, and Charlie.
Discussion topic: 系统架构设计

Format:
1. Ask Alice to speak first
2. Then Bob responds to Alice
3. Then Charlie responds to both
4. Repeat for 2 rounds
5. Summarize the discussion""",
    model=model,
    toolkit=Toolkit(tools=[
        TeamTools(subagents=[
            SubagentDeclaration(agent=alice, description="系统架构师"),
            SubagentDeclaration(agent=bob, description="后端开发"),
            SubagentDeclaration(agent=charlie, description="DevOps"),
        ]),
    ]),
)

# Host 启动讨论——Host 自己决定谁什么时候说
await host.reply(UserMsg("user", "讨论一下系统架构设计"))
```

> [!important] 关键差异
> 1.0: MsgHub 自动广播 → Agent 自动收到 → 框架决定发言顺序
> 2.0: Host 显式 spawn → Agent 被动接收 → **LLM 决定**发言顺序

---

## 迁移二：Pipeline → agent_spawn

### 1.0 SequentialPipeline

```python
# 1.0: 硬编码的串行流程
from agentscope.core.pipeline import SequentialPipeline

pipeline = SequentialPipeline([
    code_generator,
    code_reviewer,
    doc_writer,
])
result = pipeline.run("实现用户登录功能")
```

### 2.0 Sequential (串行 agent_spawn)

```python
# 2.0: Leader 动态决策串行
leader = Agent(
    name="tech_lead",
    system_prompt="""For each task:
1. First → spawn code_generator to write code
2. After code is done → spawn code_reviewer to review
3. After review passes → spawn doc_writer to write docs
4. If review finds issues → re-spawn code_generator with feedback""",
    model=model,
    toolkit=Toolkit(tools=[
        TeamTools(subagents=[
            SubagentDeclaration(agent=code_generator, ...),
            SubagentDeclaration(agent=code_reviewer, ...),
            SubagentDeclaration(agent=doc_writer, ...),
        ]),
    ]),
)

await leader.reply(UserMsg("user", "实现用户登录功能"))
```

### 1.0 FanoutPipeline

```python
# 1.0: 硬编码的并行广播
from agentscope.core.pipeline import FanoutPipeline

pipeline = FanoutPipeline([researcher_1, researcher_2, researcher_3])
results = pipeline.run("研究最新的 AI Agent 框架")
# 3 个 researcher 同时收到相同的问题
```

### 2.0 Fanout (异步并行 agent_spawn)

```python
# 2.0: Leader 决定并行
leader = Agent(
    name="research_lead",
    system_prompt="""For research tasks:
1. Decompose the question into independent sub-topics
2. Spawn ALL researchers IN PARALLEL (timeout_seconds=0)
3. Wait for all to complete
4. Synthesize results""",
    model=model,
    toolkit=Toolkit(tools=[
        TeamTools(subagents=[
            SubagentDeclaration(agent=researcher, ...),
        ]),
    ]),
)

await leader.reply(UserMsg("user", "研究最新的 AI Agent 框架"))
```

---

## 迁移三：API 变化速查

### Agent 创建

```python
# 1.0
from agentscope.agents import DialogAgent
agent = DialogAgent(name="assistant", sys_prompt="...", model_config_name="qwen")

# 2.0
from agentscope.agent import Agent
from agentscope.model import DashScopeChatModel
from agentscope.credential import DashScopeCredential

model = DashScopeChatModel(
    credential=DashScopeCredential(api_key=os.environ["DASHSCOPE_API_KEY"]),
    model="qwen-plus",
)
agent = Agent(name="assistant", system_prompt="...", model=model)
```

### 发送消息

```python
# 1.0
from agentscope.message import Msg
response = agent(Msg("user", "Hello", role="user"))

# 2.0
from agentscope.message import UserMsg
response = await agent.reply(UserMsg("user", "Hello"))
```

### 流式输出

```python
# 1.0: 通过 callback
def my_callback(text):
    print(text, end="")
agent.register_callback(my_callback)

# 2.0: 通过 async event stream
from agentscope.event import EventType
async for event in agent.reply_stream(msg):
    if event.type == EventType.TEXT_BLOCK_DELTA:
        print(event.text, end="")
```

### 工具注册

```python
# 1.0: 在 Agent 级别注册
agent = DialogAgent(
    name="coder",
    sys_prompt="...",
    functions=[bash_func, read_func, write_func],
)

# 2.0: Toolkit 抽象
from agentscope.tool import Toolkit, Bash, Read, Write
agent = Agent(
    name="coder",
    system_prompt="...",
    toolkit=Toolkit(tools=[Bash(), Read(), Write()]),
)
```

---

## 迁移 Checklist

### ✅ 删除的内容（2.0 不再存在）

- [ ] `MsgHub` — 已删除
- [ ] `SequentialPipeline` — 已删除
- [ ] `FanoutPipeline` — 已删除
- [ ] `Pipelines.conversation()` — 已删除
- [ ] `DialogAgent` — 已合并到 `Agent`
- [ ] 旧 `agent_server` 独立仓库 — 已合并到主库的 Agent Service

### ✅ 需要添加的内容

- [ ] `Agent` 类替代所有旧 Agent 类型
- [ ] `Model` + `Credential` 替代 `model_config_name`
- [ ] `Toolkit` + `Tool` 替代 `functions` 列表
- [ ] `TeamTools` + `SubagentDeclaration` 替代 `MsgHub` + `Pipeline`
- [ ] `EventType` + `reply_stream()` 替代 callback
- [ ] `PermissionMode` 配置权限策略

### ✅ 可选添加（1.0 没有的新能力）

- [ ] `Middleware` — 扩展 Agent 行为的 Hook 机制
- [ ] `Workspace` — 代码执行的沙箱隔离
- [ ] `Agent Service` — 生产级 API 服务
- [ ] `ContextCompactionMiddleware` — 自动上下文压缩

---

## 迁移原则

> [!important] 迁移三原则
>  ① 
>  ② 
>  ③ 

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能说出 MsgHub / Pipeline 在 2.0 中的对应替代方案 |
| 🟡 进阶 | 能将一段 1.0 的 MsgHub + Pipeline 代码迁移到 2.0 的 Orchestrator + agent_spawn |
| 🔴 挑战 | 能为一个现有 1.0 项目制定完整的迁移 Checklist 并估算工作量 |

---

## 相关笔记

- [[AgentScope 基础概念与环境搭建]] -- 2.0 基础概念
- [[AgentScope 多Agent协作]] -- 2.0 协作模式详解
- [[AgentScope 2.0 核心架构]] -- Event System、Permission、Agent Service
- [[AgentScope 调试与评估]] -- 2.0 调试方法
- [[AgentScope 单Agent开发]] -- 2.0 单 Agent 开发指南
- [[AgentScope 生产部署与实战]] -- 迁移后的部署方案
- [[AgentScope 毕业项目：AI研究助手]] -- 迁移后的完整项目参考

---

*最后更新：2026-07-16*
