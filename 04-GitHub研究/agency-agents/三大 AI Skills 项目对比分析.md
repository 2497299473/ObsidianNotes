---
lark_doc_url: https://my.feishu.cn/docx/EOUKdaw0UoBrSsxb8kNcjfeznAe
lark_doc_token: EOUKdaw0UoBrSsxb8kNcjfeznAe
---
# 三大 AI Skills 项目对比分析

> 对比 `agency-agents`、`agent-skills` (Addy Osmani)、`mattpocock/skills` (Matt Pocock) 三个项目的定位、设计哲学和互补关系。

---

## 1. 一句话区分

| 项目 | 作者 | 本质 | 解决的问题 |
|------|------|------|-----------|
| **agency-agents** | Mike Sitarzewski | 角色人格库（281 张角色卡） | "AI 应该**以谁的身份**思考" |
| **agent-skills** | Addy Osmani (Google Chrome) | 工程工作流剧本（24 个 skill） | "AI 应该按**什么标准流程**做代码" |
| **mattpocock/skills** | Matt Pocock (Total TypeScript) | 个人实战工具箱（41 个 skill） | "AI 应该像**我本人**一样干活" |

---

## 2. 设计哲学

### agency-agents：身份层（WHO）
- 281 个角色卡，覆盖 18 个部门（工程/营销/安全/设计/财务/医疗/游戏/GIS…）
- 每个角色定义：人格设定、记忆模型、核心使命、交付物
- **不含工程流程**——只管"你是谁"，不管"你怎么做"
- 格式：YAML frontmatter（name/emoji/vibe）+ 散文

### agent-skills：流程层·标准化（HOW · 工厂模式）
- 24 个 skill 覆盖软件工程全生命周期：spec → plan → build → test → review → ship
- Addy Osmani 从 Google 工程文化中提取
- 核心机制：**反合理化表**（预判 AI 偷懒借口并逐条反驳）+ **强制验证门**
- 引用：Hyrum's Law, Beyonce Rule, trunk-based dev, Shift Left
- 有 evals 测试、70+ 工具支持、`npx skills add` CLI

### mattpocock/skills：流程层·方法论（HOW · 工匠模式）
- 41 个 skill，22 个 promoted（engineering + productivity）
- Matt Pocock 从个人日常实战中提炼，直接公开 `~/.agents` 目录
- 核心创新：**Grilling 拷问**（AI 追问你直到需求完全清晰）+ **共享语言**（CONTEXT.md）
- 引用：Kent Beck, John Ousterhout, Eric Evans, Pragmatic Programmer
- 独特概念：User-invoked vs Model-invoked 分层、deepening opportunities、wayfinder

---

## 3. 核心对比表

| 维度 | agency-agents | agent-skills (Osmani) | mattpocock/skills (Pocock) |
|------|--------------|----------------------|-------------------|
| 规模 | 281 个 Agent | 24 个 Skill | 41 个 Skill（22 promoted） |
| 覆盖域 | 全领域 | 软件工程全生命周期 | 软件工程 + 生产力 |
| 核心格式 | YAML frontmatter + 人格散文 | SKILL.md（Process/Gates/Red Flags） | SKILL.md（极简引用 + 附属文档） |
| Slash 命令 | ❌ | 8 个（/spec /plan /build…） | 无独立命令文件（skill 名即命令） |
| 反偷懒机制 | ❌ | ✅ 反合理化表 | ✅ Grilling 天然防偷懒 |
| 强制验证门 | ❌ | ✅ | ❌（靠 TDD 回路） |
| **Grilling 拷问** | ❌ | ❌ | ✅ 核心创新 |
| **共享语言/领域建模** | ❌ | ❌ | ✅ CONTEXT.md + ADR |
| **架构抢救** | ❌ | ❌ | ✅ HTML 可视化报告 |
| **大项目路径规划** | ❌ | ❌ | ✅ /wayfinder |
| 问题追踪集成 | ❌ | ❌ | ✅ GitHub/GitLab/Linear/本地 |
| 问题分诊 | ❌ | ❌ | ✅ /triage 状态机 |
| 原型验证 | ❌ | ❌ | ✅ /prototype |
| 合并冲突解决 | ❌ | ❌ | ✅ |
| 跨 session 交接 | ❌ | ❌ | ✅ /handoff |
| 教学 | ❌ | ❌ | ✅ /teach |
| Eval 测试 | ❌ | ✅ JSON fixture | ❌ |
| User/Model 分层 | ❌ | ❌ | ✅ |
| Changesets 版本管理 | ❌ | ❌ | ✅ |
| 已废弃技能管理 | ❌ | ❌ | ✅ deprecated/ 目录 |
| 协议 | MIT | MIT | MIT |

---

## 4. 重叠与互补

后两者都是工程流程 skill，但哲学不同：

| 能力 | Osmani (agent-skills) | Pocock (mattpocock/skills) |
|------|----------------------|-------------------|
| TDD | ✅ 完整流程 + 反合理化 | ✅ 更轻量可插拔 |
| Code Review | ✅ 五轴审查 | ✅ 双轴（Standards + Spec 并行 sub-agent） |
| Debugging | ✅ | ✅ 五阶段诊断循环 |
| Spec 生成 | ✅ /spec | ✅ /to-spec（合成已有对话） |
| 任务拆分 | ✅ /plan | ✅ /to-tickets |
| Grilling | ❌ | ✅ 独有 |
| 领域建模 | ❌ | ✅ 独有 |
| 架构可视化 | ❌ | ✅ 独有 |
| Wayfinder | ❌ | ✅ 独有 |
| Triage | ❌ | ✅ 独有 |
| Teach | ❌ | ✅ 独有 |
| Handoff | ❌ | ✅ 独有 |
| 反合理化表 | ✅ 独有 | ❌ |
| 强制验证门 | ✅ 独有 | ❌ |
| Eval 测试 | ✅ 独有 | ❌ |

**重叠的 3 项**（TDD / Code Review / Debugging）风格不同：
- Osmani 靠**流程和门禁**保证质量（工厂模式）
- Pocock 靠**理解和设计**保证质量（工匠模式）

---

## 5. 三者搭配使用

```
agency-agents (281)          agent-skills (24)           mattpocock/skills (41)
  WHO  ×                      HOW  ×                      WHY/ALIGN  ×
  身份层                       流程层（标准化）              流程层（方法论）
  "我是前端专家"                "按 spec→build→test→ship"    "先拷问清楚需求"
       │                           │                           │
       └───────────────────────────┼───────────────────────────┘
                                   ↓
                        完整 AI 工程工作流示例：
                        1. Pocock /grill-me 对齐需求
                        2. Pocock /to-spec 生成 spec
                        3. Osmani /build 按 TDD 实现
                        4. 任意一个的 /review 做代码审查
                        5. Pocock /improve-codebase-architecture 定期重构
```

---

## 6. 已安装位置

### OpenSquilla（workspace）

| 技能集 | 路径 | 数量 |
|--------|------|------|
| agency-agents | `workspace/agency-agents/references/agents/` | 281 个角色 |
| agent-skills | `workspace/agent-skills/references/skills/` | 24 个 skill |
| mattpocock-skills | `workspace/mattpocock-skills/references/skills/` | 41 个 skill |

### Claude Code（~/.claude）

| 组件 | agent-skills | mattpocock-skills |
|------|-------------|-------------------|
| Slash 命令 | 8 个（/spec /plan /build…） | 无（skill 名即命令） |
| Sub-Agent | 4 个 | 无 |
| 插件市场 | ✅ addy-agent-skills | ✅ mattpocock-skills |

### Obsidian

| 文件 |
|------|
| `agency-agents/Agency Agents 使用手册.md` |
| `agency-agents/Agent-Skills 与 Agency-Agents 对比分析.md` |
| `agency-agents/三大 AI Skills 项目对比分析.md` ← 本文件 |

---

## 7. 使用场景速查

| 场景 | 用哪个 |
|------|--------|
| 需要特定领域专家视角（非代码） | agency-agents |
| 需要规范化的工程流程和门禁 | agent-skills (Osmani) |
| 对话前想先拷问清楚需求 | mattpocock /grill-me |
| 项目有大量术语，AI 理解困难 | mattpocock /grill-with-docs |
| 代码库变烂了，需要抢救 | mattpocock /improve-codebase-architecture |
| 大项目不知从何下手 | mattpocock /wayfinder |
| 需要讲清楚一个复杂概念 | mattpocock /teach |
| 严格 TDD 开发 | 两者都可以，Pocock 更轻量 |
| 代码合并前做审查 | Osmani 五轴 / Pocock 双轴 |
| 需要分诊 issue | mattpocock /triage |
| 跨 session 交接工作 | mattpocock /handoff |
