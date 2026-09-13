---
lark_doc_url: https://my.feishu.cn/docx/BteYdbWLzo04Bix5GT4cJ03Zn2f
lark_doc_token: BteYdbWLzo04Bix5GT4cJ03Zn2f
---
# Agency Agents 使用手册

> 本手册对应已部署在 OpenSquilla 工作区中的 `agency-agents` 技能。
> 总计 **281 个专业 Agent**，覆盖 **18 个部门**。

---

## 1. 什么是 Agency Agents？

**Agency Agents** 是一套开源的 AI 角色人格库，由 `msitarzewski/agency-agents` 仓库维护。每个 Agent 本质上是一张 Markdown 格式的 **角色卡**，包含：

- 角色名称、图标、颜色、氛围描述
- 身份设定与记忆模型
- 核心使命与工作流
- 可交付物与成功指标

它不是可执行代码，而是给 AI 助手用的 **人格化 prompt**。你可以把它理解为"给 AI 临时换一套脑子"。

---

## 2. 已部署环境

| 项目 | 路径 | 说明 |
|------|------|------|
| 角色源文件 | `C:\Users\Turn-\.opensquilla\workspace\agency-agents\references\agents\` | 按部门目录分类存放 |
| 完整索引 | `references\AGENTS-INDEX.md` | 281 个 Agent 速查表 |
| 调度器 | `SKILL.md` | 根据用户请求匹配并加载对应角色 |

---

## 3. 18 个部门一览

| 部门 | 数量 | 常见使用场景 |
|------|------|-------------|
| 🏗️ Engineering | 55 | 前端、后端、架构、DevOps、AI、代码审查、RAG、SRE |
| 📢 Marketing | 36 | 小红书、抖音/TikTok、SEO、知乎、公众号、私域运营 |
| ✨ Specialized | 57 | 商业、法律、医疗、招聘、MCP、Salesforce、文档生成 |
| 🎮 Game Development | 20 | Unity、Unreal、Godot、Roblox、Blender |
| ♟️ Strategy | 16 | 战略、咨询、项目阶段规划 |
| 🗺️ GIS | 13 | 地图、空间数据、Web GIS |
| 🛡️ Security | 12 | 渗透测试、AppSec、合规、威胁情报 |
| 📈 Sales | 9 | B2B 销售、方案、客户成功 |
| 🎨 Design | 9 | UI/UX、品牌、视觉、Image Prompt |
| 📋 Project Management | 7 | 敏捷、Jira、会议纪要与交付 |
| 💰 Paid Media | 7 | 广告投放、PPC、转化跟踪 |
| 📦 Product | 5 | 产品经理、优先级、反馈分析 |
| 💰 Finance | 5 | 财务建模、税务、投资 |
| 🆘 Support | 6 | 客服、合规、基础设施维护 |
| 🧪 Testing | 9 | 可访问性、性能、自动化、API 测试 |
| 🎓 Academic | 6 | 人类学、心理学、统计学 |
| 🏥 Healthcare | 3 | 临床证据、医疗创新 |
| 🥽 Spatial Computing | 6 | visionOS、WebXR、空间交互 |

---

## 4. 在 OpenSquilla 中如何使用

### 4.1 直接激活某个角色

> 💬 **说法示例**
> - "用前端开发专家的视角帮我重构这个组件"
> - "以渗透测试师的身份审计这段代码"
> - "切换到 SEO 专家"
> - "用产品经理的视角写个 PRD"

系统会从 `agency-agents/references/agents/` 中加载对应角色卡，并让我以该角色的视角继续对话。

### 4.2 查询有哪些角色

> 💬 **说法示例**
> - "有哪些可用的 agent？"
> - "marketing 部门有哪些角色？"
> - "给我 security 部门的所有 agent"
> - "有没有适合写小红书文案的 agent？"

我会列出对应部门或匹配描述的 Agent 清单。

### 4.3 多角色协作

你可以在同一次任务中切换角色：

> "先用产品经理写需求，再让前端开发评审，最后用 Reality Checker 挑刺。"

---

## 5. 常用角色速查表

### 5.1 技术开发类（Engineering）

| Agent 文件 | 名称 | 用途 |
|-----------|------|------|
| `engineering-frontend-developer.md` | Frontend Developer | 前端实现、组件重构、UI 优化 |
| `engineering-backend-architect.md` | Backend Architect | 后端架构、数据库、API 设计 |
| `engineering-code-reviewer.md` | Code Reviewer | 代码审查、可维护性、性能 |
| `engineering-software-architect.md` | Software Architect | 系统设计、DDD、架构决策 |
| `engineering-devops-automator.md` | DevOps Automator | CI/CD、基础设施自动化 |
| `engineering-sre.md` | SRE | SLO、可观测性、混沌工程 |
| `engineering-ai-engineer.md` | AI Engineer | ML 模型开发、部署、集成 |
| `engineering-rag-pipeline-engineer.md` | RAG Pipeline Engineer | RAG 检索、Chunk、重排、评估 |
| `engineering-prompt-engineer.md` | Prompt Engineer | Prompt 设计与优化 |
| `engineering-technical-writer.md` | Technical Writer | 技术文档、API 文档 |

### 5.2 中国市场营销类（Marketing）

| Agent 文件 | 名称 | 用途 |
|-----------|------|------|
| `marketing-xiaohongshu-specialist.md` | 小红书 Specialist | 小红书种草文案、话题、视觉 |
| `marketing-douyin-strategist.md` | 抖音 Strategist | 抖音短视频策略、算法、变现 |
| `marketing-bilibili-content-strategist.md` | B站内容策略 | UP 主增长、弹幕文化 |
| `marketing-zhihu-strategist.md` | 知乎 Strategist | 知乎问答、知识营销 |
| `marketing-wechat-official-account.md` | 公众号 Manager | 公众号内容、订阅转化 |
| `marketing-baidu-seo-specialist.md` | 百度 SEO | 中文 SEO、ICP、熊掌号 |
| `marketing-kuaishou-strategist.md` | 快手 Strategist | 下沉市场、直播电商 |
| `marketing-private-domain-operator.md` | 私域 Operator | 企业微信、SCRM、社群运营 |
| `marketing-china-ecommerce-operator.md` | 电商 Operator | 淘宝/天猫/拼多多/京东运营 |
| `marketing-livestream-commerce-coach.md` | 直播电商 Coach | 主播培训、直播间运营 |

### 5.3 安全与测试类

| Agent 文件 | 名称 | 用途 |
|-----------|------|------|
| `security-penetration-tester.md` | Penetration Tester | 渗透测试、红队 |
| `security-appsec-engineer.md` | AppSec Engineer | 安全开发生命周期 |
| `security-ai-generated-code-auditor.md` | AI 生成代码审计师 | 审计 AI/vibe 代码的安全问题 |
| `testing-reality-checker.md` | Reality Checker | 证据本位，挑刺找茬 |
| `testing-accessibility-auditor.md` | Accessibility Auditor | WCAG、无障碍测试 |
| `testing-api-tester.md` | API Tester | API 测试、边界条件 |

### 5.4 产品与项目管理类

| Agent 文件 | 名称 | 用途 |
|-----------|------|------|
| `product-manager.md` | Product Manager | 产品全生命周期 |
| `product-sprint-prioritizer.md` | Sprint Prioritizer | 敏捷优先级 |
| `project-management-meeting-notes-specialist.md` | Meeting Notes Specialist | 会议纪要与待办 |
| `project-management-jira-workflow-steward.md` | Jira Workflow Steward | Jira + Git 工作流 |

---

## 6. 角色卡文件结构

每个 Agent 文件遵循统一格式：

```markdown
---
name: Frontend Developer
description: Expert frontend developer...
color: cyan
emoji: 🖥️
vibe: Builds responsive, accessible web apps with pixel-perfect precision.
---

# Frontend Developer Agent Personality

## 🧠 Your Identity & Memory
## 🎯 Your Core Mission
## 📋 Your Workflow
## ✅ Deliverables
## 🗣️ Communication Style
```

---

## 7. 进阶用法

### 7.1 多 Agent 流水线

适合复杂任务的分阶段处理：

```text
1. Strategy 部门 → 定方向
2. Product 部门 → 出需求
3. Engineering 部门 → 写代码
4. Testing 部门 → 挑刺
5. Marketing 部门 → 写文案
```

### 7.2 与 Claude Code / Cursor 联动

OpenSquilla 的 `sub-agent` 技能可派生 Claude Code 子进程，带上指定 Agent 人格卡执行：

```bash
claude --permission-mode bypassPermissions --print "[前端开发者人格卡] 重构 /path/to/component"
```

### 7.3 自定义新角色

你可以基于现有模板创建新角色：

1. 复制一个 `.md` 文件
2. 修改 `name`、`description`、`emoji`、`vibe`
3. 替换正文中的身份、使命、工作流
4. 放到 `references/agents/<department>/` 下
5. 文件名格式：`department-role-name.md`

---

## 8. 常见问题

**Q: 一个请求只能用一个角色吗？**
A: 不是。你可以让我切换多个角色，或在一次任务中按步骤使用多个角色。

**Q: 角色会改变我的系统设定吗？**
A: 不会。角色卡只在当前任务/会话上下文中生效，不会影响 OpenSquilla 的核心行为。

**Q: 可以只加载部分部门吗？**
A: 可以。所有角色文件都是按需读取的，未激活的角色不会占用上下文。

**Q: 281 个角色都记不住怎么办？**
A: 直接问"有哪些…"，或查看完整索引：`agency-agents/references/AGENTS-INDEX.md`。

---

## 9. 更新与维护

- 上游仓库：`https://github.com/msitarzewski/agency-agents`
- 当前版本：仓库 `main` 分支全量快照
- 更新方式：重新下载 zip 并替换 `references/agents/` 目录

---

## 10. 快速开始示例

> **你**："用前端开发专家的视角帮我优化这个 React 组件"
>
> **我**：（加载 `engineering-frontend-developer.md`，进入前端开发专家角色）
> "我会从性能、可维护性、可访问性和响应式四个维度审查。请把组件贴出来。"

> **你**："让 Reality Checker 看看这个方案"
>
> **我**：（加载 `testing-reality-checker.md`）
> "在批准之前，我需要看到可验证的证据。请提供…"

---

*手册生成时间：2026-07-23*
*对应 Agent 库版本：msitarzewski/agency-agents main 全量快照*
