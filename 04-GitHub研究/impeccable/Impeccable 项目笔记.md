---
lark_doc_token: OvuFd6Hl3o7vYzxojyWc6cs0nQb
lark_doc_url: https://my.feishu.cn/docx/OvuFd6Hl3o7vYzxojyWc6cs0nQb
---
# Impeccable 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/pbakaus/impeccable
> - **官网**：https://impeccable.style
> - **文档**：https://impeccable.style/docs/detector 、https://impeccable.style/docs/hooks
> - **协议**：Apache-2.0（API 核实）
> - **主要语言**：JavaScript
> - **Stars**：58.8k（2026-08-13 记录），Fork 3.6k，贡献者约 45 人
> - **作者**：Paul Bakaus（前 Google 开发者布道师）
> - **定位**：给 AI 编码工具装上"设计品味 + 确定性质检"的设计语言
> - **记录日期**：2026-08-13

---

## 一句话定位

**一套安装到 AI 编码工具里的"设计语言"：23 个命令覆盖从规划、构建到打磨的全流程，59 条确定性检测规则机械化揪出 AI 生成 UI 的"AI 味"，还能以 hook 形式在编辑时实时拦截坏设计。**

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **AI 生成的 UI 千人一面** | 所有模型都在同一批 SaaS 模板上训练：Inter 字体打天下、蓝紫渐变、卡片套卡片、彩色背景配灰字、标题上方必有圆角图标块 |
| **"改好看点"无法执行** | 品味类反馈主观模糊，agent 无法落地，也无法验证是否改好 |
| **设计质检靠人眼** | 对比度不足、触控目标过小、行宽过长这类问题没有自动化检查手段，进不了 CI |
| **缺乏项目级设计上下文** | agent 不知道产品的受众、品牌调性、配色约定，每次生成都在"裸奔" |

---

## 🏗️ 核心机制

### 整体架构

```
你的 AI 编码工具（Claude Code / Cursor / Codex / Copilot / Gemini CLI / Grok / …共 14 种）
    │  /impeccable <命令> <目标>
    ▼
┌─────────────────────────────────────────────────────────┐
│  Impeccable                                              │
│  ──────────────────────────────────────────────────────  │
│  ① 上下文层：init 写入 PRODUCT.md + DESIGN.md             │
│     （受众 / 品牌车道 / 语气 / 反参考 / 色彩 / 字体 / 组件） │
│                                                          │
│  ② 命令层：23 个命令（shape→craft→critique→audit→polish…）│
│                                                          │
│  ③ 检测层：59 条确定性规则（无需 LLM / 无需 API key）       │
│     ├─ CLI：npx impeccable detect <目录|文件|URL>         │
│     └─ Hook：编辑 UI 文件时实时拦截 / 事后审查             │
│                                                          │
│  ④ Live 模式：浏览器内可视化变体迭代                       │
└─────────────────────────────────────────────────────────┘
```

### 23 个命令速览（节选）

| 阶段 | 命令 | 作用 |
|------|------|------|
| 规划 | `init` / `shape` / `document` | 采集设计上下文、写 PRODUCT.md/DESIGN.md、先规划后编码 |
| 构建 | `craft` | 完整"先定形后构建 + 视觉迭代"流程 |
| 审查 | `critique` / `audit` | UX 设计评审（层级/清晰度/情感）；技术质检（a11y/性能/响应式） |
| 打磨 | `polish` / `bolder` / `quieter` / `distill` | 收尾对齐设计系统；放大/收敛/提炼 |
| 专项 | `animate` / `colorize` / `typeset` / `layout` / `harden` / `onboard` / `clarify` / `adapt` / `optimize` / `delight` / `overdrive` | 动效、色彩、字体、布局、边界情况、首启流程、文案、多端适配、性能等单项能力 |
| 迭代 | `live` | 浏览器内对具体元素做可视化变体迭代 |

`/impeccable pin audit` 可把常用命令固定为独立快捷方式（如 `/audit`）。

### 确定性检测器（最大差异点）

> [!important] 核心理念
> 审美是主观的，但"AI 味"和基础质量问题可以写成**确定性规则**。59 条规则由代码判断，不消耗 LLM token、结果可复现、可进 CI，与 LLM 的主观 critique 形成互补闭环。

检测覆盖两类问题：

| 类别 | 例子 |
|------|------|
| AI slop（AI 味） | 侧边 tab 边框、紫色渐变、bounce 缓动、暗色发光 |
| 通用设计质量 | 行宽过长、内边距局促、触控目标过小、标题层级跳跃等 |

内置反模式清单（明确禁止）：禁用滥俗字体（Arial/Inter/系统默认）、禁彩色背景配灰字、禁纯黑纯灰（必须带色调）、禁卡片套卡片、禁 bounce/弹性缓动。

支持豁免机制：`detector.ignoreRules/ignoreFiles/ignoreValues` 配置，或文件内联注释 `<!-- impeccable-disable overused-font: 说明 -->`（支持行级 disable-line）。

### Design Hook

`npx impeccable install` 会同时安装各 harness 原生 hook：Claude Code / Copilot / Codex / Grok Build 在编辑后（部分支持 Stop 时深度扫描）反馈问题；**Cursor 可在坏写入落盘前直接拦截**。

---

## ✅ 优点

- **确定性检测是独一档能力**：规则透明、可复现、零 token 成本、能进 CI，把"设计质检"工程化
- **命令词汇表完整**：从 init 规划到 polish 收尾 23 个命令，和 agent 建立共同设计语言
- **Hook 实时拦截**：不是事后审查，而是编辑时就把坏设计挡下来（Cursor 前置拦截）
- **harness 覆盖最广**：14 种工具（Cursor/Claude Code/Copilot/Gemini CLI/Codex/Grok/OpenCode/Pi/Kiro/Trae/Rovo Dev/Qoder/Vibe/Antigravity）
- **Live 模式**：浏览器内直接对元素做可视化变体迭代
- **Apache-2.0**，作者背景强（前 Google DevRel），迭代活跃（最近一次 push 就在当天）

---

## ⚠️ 潜在局限

| 方面 | 说明 |
|------|------|
| **概念多、上手陡** | PRODUCT.md/DESIGN.md/hook/.impeccable 目录/gitignore 配置，安装方式就有 5 种，需要花时间消化 |
| **检测器管"错"不管"美"** | 59 条规则负责纠偏，从零生成好看页面仍依赖 LLM + 上下文质量 |
| **工作目录侵入项目** | `.impeccable/` 会写入截图、缓存、会话状态，需按文档维护 gitignore 块 |
| **hook 各平台差异** | Codex 更新 hook 后需重新批准；Grok 需信任项目目录；跨平台心智负担存在 |
| **项目较新** | 2025-11 首发，快速迭代期，规则与命令可能变动 |

---

## 🧩 适合谁用

- 🎨 **有页面想"去 AI 味"的人**：已有项目生成感太重，需要系统性打磨
- 🏗️ **想把设计质量工程化的团队**：检测器 + CI + hook，把品味问题变成可检查项
- 📐 **有设计系统/品牌规范的团队**：init 生成的 DESIGN.md 让 agent 按你的规范干活
- 🔁 **高频用 Claude Code / Cursor 写前端的人**：hook 实时拦截收益最大

---

## 🚀 如何使用（上手步骤）

> [!tip] 前置条件：Node.js（npx 可用）。推荐走 CLI 安装器，其他方式仅作备选。

### 第一步：安装 Skill

```bash
# 在项目根目录执行（推荐）
npx impeccable install

# 脚本化场景可跳过交互：
npx impeccable install --providers=claude,cursor --scope=project
```

安装器会自动探测 harness 目录（`~/.claude`、`~/.codex`、`.cursor` 等），询问安装到当前项目还是全局，并顺带装上对应平台的 design hook。

其他安装方式（按需）：

```bash
# Claude Code 插件市场
/plugin marketplace add pbakaus/impeccable

# Git submodule（团队 vendor 管理）
git submodule add https://github.com/pbakaus/impeccable .impeccable
npx impeccable link --source=.impeccable --providers=claude,cursor
```

### 第二步：初始化项目设计上下文

在 AI 工具里执行：

```
/impeccable init
```

回答品牌型（营销/落地页/作品集）还是产品型（App/仪表盘/工具），它会写出 `PRODUCT.md` 和 `DESIGN.md`，之后所有命令都会读这份上下文。已有项目可用 `/impeccable document` 从代码反推 DESIGN.md。

### 第三步：日常命令

```
/impeccable audit landing        # 审计落地页（a11y/性能/响应式）
/impeccable critique the header  # 对页头做 UX 设计评审
/impeccable polish settings      # 设置页发布前收尾
/impeccable bolder this hero     # 太平淡？放大张力
/impeccable redo this hero section  # 也可以直接用自然语言描述
```

常用命令固定成快捷方式：`/impeccable pin audit` → 之后直接 `/audit`。

### 第四步：独立 CLI 检测（无需 AI 工具）

```bash
npx impeccable detect src/                 # 扫描目录
npx impeccable detect index.html           # 扫描单文件
npx impeccable detect https://example.com  # 扫描线上 URL（Puppeteer）
npx impeccable detect --json .             # CI 友好的 JSON 输出
npx impeccable ignores add-file "src/legacy/**"        # 配置忽略
npx impeccable ignores add-value overused-font Inter --reason "品牌字体"
```

### 第五步：维护与收尾

```bash
npx impeccable update    # 升级 skill 与 hook
```

- 把 README 中的 `impeccable-ignore-start ... impeccable-ignore-end` gitignore 块复制到项目 `.gitignore`（截图、缓存等临时产物不入库；`config.json`、`design.json`、`critique/*.md` 需保留跟踪）
- Codex 用户安装/更新后打开 `/hooks` 重新批准一次项目 hook

---

## 🔗 相关资源

| 资源 | 地址 |
|------|------|
| GitHub 仓库 | https://github.com/pbakaus/impeccable |
| 官网 / 文档 | https://impeccable.style |
| npm 包 | https://www.npmjs.com/package/impeccable |
| 前后对比案例 | https://impeccable.style/cases/neo-mirai |
| Claude 官方前端技能（它的前身参照） | https://github.com/anthropics/skills/tree/main/skills/frontend-design |

---

## 💡 个人思考

Impeccable 最锋利的洞察是：**"AI 味"不是玄学，而是可以枚举的规则。** 紫渐变、bounce 缓动、卡片套卡片——这些之所以成为 AI 生成页面的通病，是因为训练数据高度同质化；而把它们写成 59 条确定性检测规则后，"设计质检"第一次可以脱离 LLM、零成本地跑进 CI 和编辑 hook。这与"靠 prompt 祈祷模型有品味"是完全不同的工程范式。

它和 [[UI UX Pro Max 项目笔记]] 恰好互补：后者负责"从 0 生成正确的设计系统"，Impeccable 负责"把已有的设计审出品味问题并拦截倒退"。组合使用（Pro Max 生成 → Impeccable 验收打磨）可能是当前 AI 前端工作流里完成度最高的方案。

上手成本是它的主要门槛——init/hook/live/gitignore 一套概念下来需要半小时学习，但换来的是整个团队共享的设计词汇表，适合长期项目而非一次性页面。

> [!quote] 一句话总结
> 如果你受够了 AI 生成页面千篇一律的"AI 味"，Impeccable 是目前唯一把"品味质检"做成确定性工程能力的方案。

> 状态：✅ 已分析，⏳ 待实践 · 关联：[[UI UX Pro Max 项目笔记]]、[[UI Skills 项目笔记]]、[[Impeccable vs UI UX Pro Max vs UI Skills]]
