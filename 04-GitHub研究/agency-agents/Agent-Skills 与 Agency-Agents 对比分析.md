---
lark_doc_url: https://my.feishu.cn/docx/GlHddip9Kotq3RxGeN2c8fsMnSf
lark_doc_token: GlHddip9Kotq3RxGeN2c8fsMnSf
---
# Agent-Skills 与 Agency-Agents 对比分析

> 两套 AI Agent 增强体系，一个解决"**以谁的身份思考**"，一个解决"**按什么流程执行**"。
> 已部署到 OpenSquilla 工作区 + Claude Code。

---

## 1. 一句话区分

| 项目 | 本质 | 解决的问题 |
|------|------|-----------|
| **Agency-Agents** | 角色人格库 | "AI 应该**以谁的身份**思考" |
| **Agent-Skills** | 工程工作流剧本 | "AI 应该**按什么流程**把代码做对" |

打比方：
- `agency-agents` = 请了一个 281 人的**专家团队**，随时让 AI 变身其中一位
- `agent-skills` = 给 AI 发了一本 Google 工程部的**作业 SOP 手册**，让它按规矩干活

---

## 2. 核心对比

| 维度 | Agency-Agents | Agent-Skills |
|------|--------------|--------------|
| **作者** | Mike Sitarzewski（社区项目） | **Addy Osmani**（Google Chrome 工程负责人） |
| **规模** | 281 个 Agent，18 个部门 | 24 个 Skill + 4 个 Persona + 8 个 Slash 命令 |
| **覆盖域** | 全领域：工程 / 营销 / 销售 / 设计 / 安全 / 财务 / 医疗 / 游戏 / GIS… | **单一领域：软件工程全生命周期** |
| **核心格式** | YAML frontmatter（name / emoji / vibe）+ 人格描述散文 | SKILL.md（Overview / Process / Rationalizations / Red Flags / Verification） |
| **内容类型** | **人格卡**：身份设定、记忆模型、核心使命、交付物 | **工作流剧本**：分步骤流程、质量门禁、反偷懒表 |
| **设计理念** | "让 AI 换一套脑子" | "让 AI 按高级工程师的纪律干活" |
| **Slash 命令** | ❌ 无 | ✅ `/spec /plan /build /test /review /ship /webperf /code-simplify` |
| **反偷懒机制** | ❌ 无 | ✅ 每个 skill 都有"常见借口 + 反驳"表 |
| **验证门** | 弱（靠成功指标描述） | **强**（"seems right" 不算，必须有测试通过/构建输出等证据） |
| **工程方法论** | 无特定绑定 | Google 文化：Hyrum's Law、Beyonce Rule、trunk-based dev、Shift Left |
| **Eval 测试** | ❌ 无 | ✅ `evals/` 目录，每个 skill 有 JSON 测试用例 + fixture 代码 |
| **安装方式** | Bash 脚本批量转换 | `npx skills add` CLI / Claude Code 原生 Plugin Marketplace |
| **协议** | MIT | MIT |

---

## 3. Agent-Skills 的 24 个 Skill 速查

按软件开发生命周期排列：

| 阶段 | Skill | 用途 |
|------|-------|------|
| 🔍 发现 | `using-agent-skills` | 元技能：自动路由到正确的 skill |
| 💡 构思 | `idea-refine` | 粗略想法 → 多个精炼方案 |
| 💡 构思 | `interview-me` | "不知道要什么" → 结构化访谈澄清需求 |
| 📝 规范 | `spec-driven-development` | 写代码前先生成 SPEC.md |
| 📝 规范 | `source-driven-development` | 代码验证对照文档/源码 |
| 📋 计划 | `planning-and-task-breakdown` | 把 spec 拆成有序任务列表 |
| 🔨 构建 | `incremental-implementation` | 一次一个任务，RED→GREEN→COMMIT |
| 🔨 构建 | `test-driven-development` | TDD：先写失败测试，再写代码 |
| 🔨 构建 | `frontend-ui-engineering` | UI 工程特定模式 |
| 🔨 构建 | `api-and-interface-design` | API 设计模式 |
| 🔨 构建 | `context-engineering` | 长会话上下文管理 |
| 🔨 构建 | `doubt-driven-development` | 高风险/不熟悉代码：质疑→验证 |
| 🧪 测试 | `browser-testing-with-devtools` | 浏览器运行时验证 |
| 🧪 测试 | `debugging-and-error-recovery` | 系统性调试与错误恢复 |
| 🧪 测试 | `code-review-and-quality` | 五轴代码审查 |
| 🧪 测试 | `code-simplification` | 降低复杂度 |
| 🧪 测试 | `security-and-hardening` | 输入验证、密钥、认证 |
| 🧪 测试 | `performance-optimization` | 性能审计 |
| 🚀 发布 | `git-workflow-and-versioning` | 分支策略、提交规范 |
| 🚀 发布 | `ci-cd-and-automation` | 流水线、Shift Left |
| 🚀 发布 | `shipping-and-launch` | 发布检查清单 |
| 🔧 维护 | `deprecation-and-migration` | 安全废弃旧 API |
| 🔧 维护 | `documentation-and-adrs` | ADR、活文档 |
| 🔧 维护 | `observability-and-instrumentation` | 日志、指标、链路追踪 |

---

## 4. Agent-Skills 独有的三大核心设计

### ① 反合理化表

预判 AI 会找什么借口跳步骤，逐条反驳：

| AI 的借口 | 反驳 |
|-----------|------|
| "I'll add tests later" | Untested code is broken code you don't know about |
| "This is a simple change" | Simple changes still need verification |
| "The spec is obvious" | Obvious to whom? Write it down |

Agency-Agents 没有这种机制——它相信人格设定就够了。

### ② 强制验证门

每个 skill 结束必须出示证据，"seems right" 永远不够。

### ③ `/build auto` 模式

审批一次计划后自动执行全部任务，每个任务仍 TDD 驱动 + 单独提交，失败时暂停。

---

## 5. 互补关系

```
agency-agents  →  身份层（WHO）
"我是前端专家"          agent-skills  →  流程层（HOW）
                         "不管我是谁，按 spec→build→test→ship 走"
```

实际效果：你用 agency-agents 激活"前端专家"角色后，agent-skills 的 TDD、Code Review、Incremental Implementation 流程会约束这个"专家"不要偷懒、不要跳测试、不要写 500 行不提交。

| 场景 | 用哪个 |
|------|--------|
| "用前端专家视角帮我重构组件" | agency-agents → `frontend-developer.md` |
| "我要从头开发新功能，需要规范流程" | agent-skills → `/spec` → `/plan` → `/build` → `/test` |
| "以渗透测试师身份审计代码" | agency-agents → `penetration-tester.md` |
| "代码合并前做五轴 review" | agent-skills → `/review` |
| "帮我写小红书运营方案" | agency-agents → `xiaohongshu-strategist.md` |
| "确保 CI/CD 符合 Shift Left" | agent-skills → `ci-cd-and-automation` |

---

## 6. 部署信息

### Agency-Agents（已部署）

| 项目 | 路径 |
|------|------|
| 角色源文件 | `~/.opensquilla/workspace/agency-agents/references/agents/` |
| 完整索引 | `references/AGENTS-INDEX.md` |
| 调度器 | `SKILL.md` |
| 规模 | 281 个 Agent，18 个部门 |

### Agent-Skills（已部署）

| 项目 | 路径 |
|------|------|
| OpenSquilla Skill 文件 | `~/.opensquilla/workspace/agent-skills/references/skills/` |
| OpenSquilla Agent 文件 | `~/.opensquilla/workspace/agent-skills/references/agents/` |
| 调度器 | `SKILL.md` |
| Claude Code 命令 | `~/.claude/commands/`（8 个 slash 命令） |
| Claude Code Agent | `~/.claude/agents/`（4 个 sub-agent） |
| Claude Code 插件市场 | `~/.claude/plugins/marketplaces/addy-agent-skills/` |
| 规模 | 24 个 Skill，4 个 Persona，8 个 Slash 命令 |

---

## 7. 使用方式

### 在 OpenSquilla 中

| 你说 | 效果 |
|------|------|
| "用前端专家视角帮我重构" | → 加载 `agency-agents/frontend-developer.md` |
| "帮我做代码审查" | → 加载 `agent-skills/code-review-and-quality/SKILL.md` |
| "/spec 帮我写规范文档" | → 加载 `agent-skills/spec-driven-development/SKILL.md` |
| "/review 审查这次改动" | → 加载 `agent-skills/code-review-and-quality/SKILL.md` |
| "有哪些可用的 agent？" | → 展示 agency-agents 部门索引 |
| "有哪些可用的 skill？" | → 展示 agent-skills 技能目录 |

### 在 Claude Code 中

直接输入 slash 命令：`/spec`、`/plan`、`/build`、`/test`、`/review`、`/ship`、`/webperf`、`/code-simplify`

---

## 8. 更新与维护

| 项目 | 上游 | 更新方式 |
|------|------|---------|
| Agency-Agents | `github.com/msitarzewski/agency-agents` | 重新下载 zip → 覆盖 `references/agents/` |
| Agent-Skills | `github.com/addyosmani/agent-skills` | 重新下载 zip → 覆盖 `references/skills/` + Claude Code 插件目录 |

> ⚠️ 更新时建议先备份 `SKILL.md` 调度器，避免被覆盖。
