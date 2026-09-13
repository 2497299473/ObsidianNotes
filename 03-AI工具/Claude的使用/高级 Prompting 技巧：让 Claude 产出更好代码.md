---
title: 高级 Prompting 技巧：让 Claude 产出更好代码
date: 2026-06-17
tags:
  - Claude
  - Claude Code
  - Prompting
  - 工程效率
aliases:
  - 怎么让 Claude 写出更好代码
  - Claude Code 提示技巧
status: ✅ 已完成
related:
  - "[[CLAUDE.md 使用指南：八个场景与实操]]"
  - "[[CLAUDE.md]]"
lark_doc_url: https://my.feishu.cn/docx/Nze2dBDogoSpT3xSlmGc0sTKn8e
---

> [!ABSTRACT] 快速概览
> CLAUDE.md 配置文件告诉 Claude **该做什么**；而 Prompting 技巧决定 Claude **能做到什么程度**。本文是三个高频实用招数，适用于 Claude Code 编码场景。

---

## 核心认知

CLAUDE.md 和 Prompting 是两层互补的手段：

```
CLAUDE.md → 设定规则和上下文（一次性配置，每次对话自动加载）
Prompting → 调控单次对话的质量和深度（每次交互时动态使用）
```

| 手段 | 作用 | 生效时机 |
|------|------|---------|
| CLAUDE.md | 让 Claude 知道你的规矩和偏好 | 每次对话自动 |
| Prompting 技巧 | 让 Claude 在这个对话中发挥到极致 | 单次对话中 |

---

## 三个技巧

### 技巧 a：让 Claude 做审查者 🔍

**核心思想**：不让 Claude 做执行者，让它做你的**对立面**——质疑你的每个决策，只在你说服它之后才放行。

**两种变体：**

| 指令 | 效果 | 适用场景 |
|------|------|---------|
| `"Grill me on these changes and don't make a PR until I pass your test"` | Claude 对你的改动穷追猛打，像面试官一样追问每个设计决策 | 提交 PR 前做自审 |
| `"Prove to me this works"` | Claude 主动对比 main 分支和 feature 分支的行为差异，验证功能正确性 | 改完代码后验证 |

**实操建议**：

```text
// 不要在改完后直接 /review
// 而是在对话中主动说：

"我改了 xxx.py 的缓存逻辑。Grill me on these changes——
 追问每一个可能有问题的设计决策，直到我逐个解释清楚。
 在我通过你的审查之前，不要帮我提交 PR。"
```

**为什么有效**：Claude 默认是**合作者模式**——顺着你的思路走。当你明确要求对抗性审查时，它会切换到**批判者模式**，用不同标准审视代码。

---

### 技巧 b：推倒重来 🔄

**核心思想**：先让 Claude 用最直接的方式把功能跑通，然后一句命令让它带着**全局理解**重新实现。

**指令**：

```text
"Knowing everything you know now, scrap this and implement the elegant solution"
```

**适用场景**：

| 场景 | 为什么不第一次就要求优雅？ |
|------|--------------------------|
| 需求不够清晰，边做边明确 | 第一版验证功能可行性，第二版优化架构 |
| 第一版有技术债 | 快速迭代必然产生临时方案，"知道所有坑之后"的重写才是真正优雅的 |
| 性能瓶颈 | 先跑通再 profiling，而不是过早优化 |

**实操建议**：

```text
// 第一版：快速出 MVP
"帮我在这个 PyQt5 窗口里加上实时相机预览功能，能跑就行"

// MVP 跑通后：
"Knowing everything you know now——QThread 竞争、QImage 内存管理、
 帧率控制——scrap this and implement the elegant solution."
```

**为什么有效**：Claude 在实现过程中积累了对代码库、边界条件、坑点的理解，但第一版往往是线性的"加功能"。重写时它已经知道了全貌，可以做全局最优的架构设计。

---

### 技巧 c：先写规格再动手 📐

**核心思想**：给 Claude 的指令越具体、歧义越少，输出质量越高。把模糊需求转化为精确规格。

**对比**：

| ❌ 模糊指令 | ✅ 精确规格 |
|------------|-----------|
| "加个缓存" | "在 `get_data()` 上加 Redis 缓存，TTL 300s，key 格式 `cache:user:{uid}`，缓存未命中时用 SETNX 防击穿" |
| "优化这个查询" | "把这段 SQL 的 N+1 问题用 `select_related` 改掉，explain 输出的 rows 预期从 5000 降到 3" |
| "写个测试" | "为 `merge_intervals()` 写 pytest，覆盖：空列表、单个区间、完全重叠、部分重叠、无重叠，用 parametrize 参数化" |

**规格的核心要素**：

| 要素 | 示例 |
|------|------|
| **输入/输出** | "入参是 `List[Tuple[int, int]]`，返回同类型" |
| **边界条件** | "空数组返回 `[]`，单个元素直接返回" |
| **性能约束** | "时间复杂度 O(n log n)，n ≤ 10^5" |
| **命名约定** | "函数名用 snake_case，类名用 PascalCase" |
| **测试要求** | "pytest，覆盖 5 个典型场景 + 2 个边界条件" |
| **错误处理** | "输入非法时抛 `ValueError`，不静默吞错" |

**为什么有效**：Claude 表现不佳的绝大多数情况，不是能力问题，而是**需求模糊**。你脑子里有完整的设计，但只说了一半，Claude 只能用猜的。把规格写清楚，等于把脑中的设计完整传递出去。

---

## 三招组合使用

```
                    ┌─────────────┐
                    │  c. 写规格   │ ← 开始前：把需求说清楚
                    └──────┬──────┘
                           ▼
                    ┌─────────────┐
                    │  b. 推倒重来 │ ← 第一版跑通后：全局重写
                    └──────┬──────┘
                           ▼
                    ┌─────────────┐
                    │  a. 做审查者│ ← 提交前：对抗性审查
                    └─────────────┘
```

| 阶段 | 技巧 | 目标 |
|------|------|------|
| **编码前** | c. 先写规格 | 消除歧义，确保 Claude 理解你的意图 |
| **MVP 后** | b. 推倒重来 | 用全局视角重构出优雅的最终方案 |
| **提交前** | a. 做审查者 | 用对抗性审查守住质量底线 |

---

## 相关笔记

- [[Claude的使用/CLAUDE.md 使用指南：八个场景与实操]] — CLAUDE.md 能干什么（配置层面的八种使用模式）
- [[CLAUDE.md]] — 你当前的 vault 级 CLAUDE.md 本体
- [[Claude的使用/CLAUDE.md 的期望值决策框架]] — 形式化决策框架 + 稀释效应讨论
