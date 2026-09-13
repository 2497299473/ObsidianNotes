---
title: 自动化修复子代理：permissionMode 选型与风险控制
date: 2026-06-16
tags:
  - Claude
  - Sub-Agent
  - permissionMode
  - hooks
  - 安全
aliases:
  - auto-fixer 配置
  - permissionMode 怎么选
status: ✅ 已完成
related:
  - "[[Claude的使用/03｜分而治之：Sub-Agents 的核心概念与应用价值]]"
  - "[[Claude的使用/04｜量体裁衣：从 Sub-Agents 到 Multi-Agent 的工程指南]]"
  - "[[Sub-Agent 上下文隔离、信息流与 CLAUDE.md 继承]]"
lark_doc_url: https://my.feishu.cn/docx/XuT2dGHrzon3dDx9MQ3c8ryenNf
---

> [!ABSTRACT] 核心问题
> 创建一个"自动化修复"子代理，允许它自主修改文件而不需要每次都弹窗确认——该选哪种 `permissionMode`？风险是什么？如何用实际可用的字段降低风险？

---

## Q1：permissionMode 选什么？

### 常见误区

❌ `automatic` —— 这个值**不存在**，是你自己起的名字。

### 实际可用取值

| 值 | 效果 |
|----|------|
| `default` | 继承主对话的权限上下文 |
| `plan` | 系统级只读，写操作被物理阻止 |
| `acceptEdits` | 自动接受文件编辑，不弹窗确认 |
| `bypassPermissions` | 跳过所有权限检查（包括 Bash） |

### 正确答案

选择 **`acceptEdits`**：

- 既满足"自主修复 → 不打扰用户"的核心需求
- 又不至于像 `bypassPermissions` 那样连危险命令都静默执行
- `plan` 太保守（完全不能写），`bypassPermissions` 太激进（什么都拦不住）

---

## Q2：这种权限模式的风险是什么？

1. **误改代码导致 bug** — 自动修 lint 顺手改了逻辑
2. **删错文件、改错逻辑** — 没有人工确认环节
3. **无限循环自动修复** — 修了又坏、坏了又修
4. **越权修改不该碰的文件** — 碰了 `package.json`、配置文件、`.env`
5. **破坏未提交的手工代码** — 覆盖用户正在写的代码

一句话：**`acceptEdits` = 方便，但失去了人工确认这道最后防线。**

---

## Q3：配合哪些字段降低风险？

### 关键认知

❌ 常见的"想当然"字段：

```
fileAllowList    ← 不存在
fileDenyList     ← 不存在
maxAutoFixPerHour ← 不存在
requireGitClean  ← 不存在
operationScope   ← 不存在
dryRunFirst      ← 不存在
```

**Claude Code 子代理实际只有三种约束手段：**

| 机制 | 约束力 | 用途 |
|------|:---:|------|
| `tools` / `disallowedTools` | 系统级硬约束 | 控制能用哪些工具 |
| `hooks`（PreToolUse） | 系统级硬约束 | 每次工具调用前执行校验脚本 |
| agent prompt 正文 | 行为约定 | 流程指引、软约束 |

### 你的 6 个思路 → 实际落地映射

| 你的想法 | 类型 | 实际落地方式 |
|---------|:---:|------------|
| `fileAllowList` — 只允许改指定目录 | 硬约束 | `hooks` → PreToolUse → 脚本校验文件路径 |
| `fileDenyList` — 禁止改配置文件 | 硬约束 | 同上（黑白名单是一个逻辑） |
| `maxAutoFixPerHour` — 每小时限流 | 软约束 | prompt 中写"一次最多修 N 个问题" |
| `requireGitClean` — 工作区干净才执行 | 硬约束 | `hooks` → PreToolUse → `git status --porcelain` |
| `operationScope` — 限制操作类型 | 硬约束 | `tools` 白名单 + prompt 禁止 delete/rename |
| `dryRunFirst` — 先预览再写入 | 软约束 | prompt 指令："先 diff，确认后再写入" |

---

## 修正后的完整配置

```markdown
---
name: auto-fixer
description: Automatically fix lint errors, formatting issues, and simple bugs.
  Use after code changes are detected.
tools: Read, Grep, Glob, Edit, Bash
disallowedTools: Write
model: sonnet
permissionMode: acceptEdits
hooks:
  PreToolUse:
    - matcher: "Edit|Bash"
      hooks:
        - type: command
          command: |
            # 校验文件在白名单目录内
            case "$CLAUDE_TOOL_INPUT" in
              *src/*|*tests/*) exit 0 ;;
              *package.json*|*.env*|*.config.*) exit 1 ;;
              *) exit 0 ;;
            esac
        - type: command
          command: |
            # 要求工作区干净
            [ -z "$(git status --porcelain)" ] && exit 0 || exit 1
---

你是一个自动化修复专家。

## 修复流程
1. 先用 `git diff` 确认当前改动范围 → 评估影响面
2. 针对 lint / format / 简单 bug 进行修复
3. 修复后 `git diff` 展示变更 → 确认无意外改动
4. 一次最多连续修复 5 个问题，超过则汇报后停止

## 禁止
- 不修改 package.json、配置文件、.env 等
- 不执行 delete / rename / 破坏性 git 操作
- 不引入新依赖
```

---

## 配置逐行解读

| 配置行 | 作用 | 对应你的哪个想法 |
|--------|------|:---:|
| `tools: Read, Grep, Glob, Edit, Bash` | 白名单——只给必要工具 | `operationScope` |
| `disallowedTools: Write` | 禁止 Write（用 Edit 更安全，只做替换不做全量覆盖） | `operationScope` |
| `permissionMode: acceptEdits` | 自动接受编辑，不弹窗 | 核心需求 |
| `hooks: PreToolUse` — 文件路径校验 | 每次 Edit/Bash 前检查是否在 `src/` 或 `tests/` | `fileAllowList` + `fileDenyList` |
| `hooks: PreToolUse` — git 工作区检查 | 不干净则拒绝执行 | `requireGitClean` |
| prompt: "先用 git diff 确认" | 先预览再改 | `dryRunFirst` |
| prompt: "一次最多修 5 个" | 防止无限循环 | `maxAutoFixPerHour` |

---

## 一句话总结

> **`permissionMode` 不是安全配置——它是便利性配置。真正的安全靠 `hooks` + `tools` 白名单。**
>
> 你列的 6 个约束思路完全正确，只是不能凭空发明字段名——把它们写成 `hooks` 里的校验脚本，就从"我希望"变成了"你做不到"。
