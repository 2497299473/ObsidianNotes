---
title: Claude Code 代码审查全景：三层架构
created: 2026-06-28
tags:
  - claude-code
  - code-review
  - architecture
description: 梳理 Claude Code 各讲中代码审查技术的协作关系——SubAgent、Skill、Hook、Headless、MCP 不是竞争而是互补，覆盖代码生命周期的三个关键节点。
lark_doc_url: https://my.feishu.cn/docx/IhAudaj8Eo5v0px2cyFchVvOnUc
---

代码审查在 Claude Code 各讲中反复出现——SubAgent、Skill、Hook、Headless 都有涉及。这不是重复，而是因为**代码审查是一个覆盖代码全生命周期的任务**，不同技术在不同阶段各司其职。

## 全景图

```
开发者日常（交互式）         Claude 写代码时（自动）         PR 提交后（无人值守）
━━━━━━━━━━━━━━━━━━━━       ━━━━━━━━━━━━━━━━━━━━       ━━━━━━━━━━━━━━━━━━━━
  开发者触发                    Write 工具写入                  开发者 push → 创建 PR
    ↓                             ↓                              ↓
  Command                     PostToolUse Hook              GitHub Actions 触发
  /review src/api.ts          自动 lint + format               ↓
    ↓                             ↓                         Headless
  Skill                       Claude 说"我做完了"          claude -p "review this PR"
  加载 code-reviewing            ↓                              ↓
  SKILL.md（12 条标准）        Stop Hook                   自动在 PR 留 review
    ↓                         跑测试，失败就继续修              comments
  SubAgent                        ↓                              ↓
  派只读子代理审查              → 质量门控                    MCP
    ↓                                                     同时调 SonarQube
  输出结构化审查报告                                       做静态分析（外部工具补充）
```

## 各技术的角色分工

| 技术 | 解决的问题 | 在审查中的职责 |
|------|-----------|---------------|
| **Command** | 入口 | `/review` 让开发者一键触发审查 |
| **Skill** | 标准 | 定义审查方法论（检查项、输出格式、禁止项），可跨项目复用 |
| **SubAgent** | 隔离 | 以只读权限派生子代理执行审查，审查者不能改代码 |
| **Hook** | 时机 | PostToolUse 逐文件即时检查（lint/format）；Stop 做质量门控（测试通过才放行） |
| **Headless** | 无人值守 | CI/CD 中 `claude -p` 自动审查 PR，不需要人坐在终端前 |
| **MCP** | 工具链 | 连接 SonarQube、ESLint 等外部工具，补充 AI 审查之外的静态分析 |

## 为什么是协作不是竞争

每层解决不同维度的问题，组合起来覆盖代码生命周期的三个关键节点：

1. **开发中（交互式）**：开发者主动请求审查 → Skill 提供标准，SubAgent 提供隔离
2. **写完后（自动）**：Hook 在工具执行前后自动触发检查 → 不依赖开发者记得运行审查
3. **提交后（无人值守）**：Headless + MCP → 7×24 自动化，多个工具并行给出反馈

## 相关笔记

- [[17｜海纳百川：MCP 协议与外部工具连接]] — MCP 连接外部审查工具
- [[18｜庖丁解牛：Tools 工具系统内核剖析]] — 工具系统与权限控制
- [[19｜无人值守：Headless 模式与 CICD 集成]] — CI/CD 中的 Headless 审查
