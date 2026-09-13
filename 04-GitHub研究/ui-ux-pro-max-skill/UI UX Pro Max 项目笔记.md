---
lark_doc_token: WMfhd8hYsojhNLxcFIHcqLYhnMh
lark_doc_url: https://my.feishu.cn/docx/WMfhd8hYsojhNLxcFIHcqLYhnMh
---
# UI UX Pro Max 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/nextlevelbuilder/ui-ux-pro-max-skill
> - **官网**：https://uupm.cc
> - **协议**：MIT（API 核实）
> - **主要语言**：Python（检索/推理脚本，纯标准库）、TypeScript（CLI）
> - **Stars**：116.4k（2026-08-13 记录，当前 GitHub 上最热的 AI 设计 Skill），Fork 12.5k，贡献者约 74 人
> - **最新版本**：v2.14.2（2026-08-12）
> - **定位**：按行业/产品类型一键生成完整设计系统的 AI Skill
> - **记录日期**：2026-08-13

---

## 一句话定位

**把"设计常识"编译成可离线检索的结构化知识库（84 风格 / 192 色板 / 74 字体搭配 / 161 行业规则），用 BM25 检索 + 推理引擎，把一句需求变成包含版式、风格、配色、字体、反模式与交付清单的完整设计系统。**

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **开发者不懂设计** | 配色、字体、版式全靠拍脑袋，产出"能跑但不好看"的界面 |
| **AI 默认脸** | 不做约束时模型输出同质化模板（正是 Impeccable 批判的那批 SaaS 脸） |
| **设计决策缺乏依据** | 为什么美容 SPA 用柔粉金、银行忌讳霓虹紫？行业级设计知识散落在设计师脑中 |
| **设计约定无法沉淀** | 每次会话从零开始，颜色字体反复漂移，页面间不一致 |

---

## 🏗️ 核心机制

### 设计系统生成流水线（v2.0 旗舰能力）

```
用户请求："给我的美容 SPA 做个落地页"
    │
    ▼
多域并行检索（5 路，BM25 排序）
    ├─ 产品类型匹配（192 个品类）
    ├─ 风格推荐（84 种 UI 风格）
    ├─ 配色选择（192 套行业色板，与品类 1:1 对齐）
    ├─ 落地页模式（34 种版式模式）
    └─ 字体搭配（74 组字体组合，含 Google Fonts 导入）
    │
    ▼
推理引擎
    ├─ 产品 → UI 品类规则匹配
    ├─ 行业反模式过滤（如：银行业禁 AI 紫/粉渐变）
    └─ JSON 条件决策规则
    │
    ▼
完整设计系统输出
    版式 + 风格 + 配色（含色值）+ 字体 + 关键效果
    + 需避免的反模式 + 交付前检查清单
```

### 知识库规模

| 维度 | 数量 | 说明 |
|------|------|------|
| 行业推理规则 | 161 条 | 覆盖 SaaS/金融/医疗/电商/服务/创意/生活/Web3 等 8 大类 |
| UI 风格 | 84 种 | 玻璃拟态、黏土拟态、粗野主义、Bento、AI-Native UI、HUD 科幻等 |
| 行业色板 | 192 套 | 与 192 个产品类型一一对应 |
| 字体搭配 | 74 组 | 附情绪标签与适用场景 |
| 图表类型 | 25 种 | 仪表盘/分析场景推荐 |
| 技术栈指南 | 22 个 | React/Next/Vue/Nuxt/Svelte/Astro/Laravel，含桌面（WPF/WinUI/JavaFX/Avalonia）与移动（SwiftUI/Compose/Flutter/RN） |
| UX 指南 | 98 条 | 最佳实践、反模式、可访问性规则 |

> [!important] 核心理念
> 检索脚本是**纯 Python 标准库**，不发任何网络请求、不装任何依赖——知识库以 CSV 形式随 skill 落地，完全离线可跑。这对内网/敏感环境很关键。

### 设计系统持久化（Master + Overrides）

```bash
python3 .claude/skills/ui-ux-pro-max/scripts/search.py "SaaS dashboard" \
    --design-system --persist -p "MyApp" --page "dashboard"
```

生成 `design-system/MASTER.md`（全局唯一事实源）+ `design-system/pages/dashboard.md`（页面级覆盖）。构建具体页面时先查页面文件，其规则覆盖 Master——跨会话保持设计一致性。

---

## ✅ 优点

- **知识库规模断层领先**：161 条行业规则 + 192 色板 + 84 风格，是三者中唯一"行业级"的方案
- **完全离线**：纯 Python 标准库 + 本地 CSV，无网络依赖
- **平台覆盖最广**：20 个 AI 平台（Claude Code/Cursor/Windsurf/Codex/Qoder/Trae/Kiro/Roo Code/Continue…）+ 22 个技术栈（含桌面与移动端）
- **可沉淀**：Master+Overrides 模式让设计系统跨会话复用
- **社区最热**：116k stars、74 贡献者、语义化发布 + 自动化流水线，工程化程度高
- **MIT 协议**，有官方中文社区教程（bbylw/ui-ux-pro-max-skill-cn）

---

## ⚠️ 潜在局限

| 方面 | 说明 |
|------|------|
| **需要 Python 3.x** | 检索脚本依赖本机 Python（仅标准库，但环境得有） |
| **模板化正确 ≠ 有品味** | 生成结果"不会错"但未必出彩，个性化打磨需另配（如 Impeccable） |
| **商业化导向** | README 多处引流 Premium 付费版（品牌设计/AI 资产生成/企业 Token 架构） |
| **仓库体积大** | 直接传 ZIP 给 Claude.ai 会超 200 文件上限；旧版 symlink 问题曾导致 marketplace 安装失败（v2.5.1+ 修复，建议用 CLI 安装） |
| **CLI 版本碎片** | 旧包名 `uipro-cli` 已废弃，须用 `ui-ux-pro-max-cli`；CLI 过旧会缺 uninstall/update 命令 |

---

## 🧩 适合谁用

- 🚀 **独立开发者 / 全栈**：不懂设计也要快速产出专业感页面（落地页、MVP）
- 🏥 **做行业应用的人**：医疗、金融、电商等垂直领域，需要"行业正确"的配色与版式
- 📦 **需要沉淀设计系统的团队**：Master+Overrides 跨会话保持一致
- 🖥️ **桌面/移动开发者**：唯一覆盖 WPF、WinUI、JavaFX、SwiftUI、Compose、Flutter 的方案

---

## 🚀 如何使用（上手步骤）

> [!tip] 前置条件：Node.js（装 CLI）+ Python 3.x（跑检索脚本，仅标准库）。

### 第一步：安装 CLI

```bash
npm install -g ui-ux-pro-max-cli
# 注意：包名是 ui-ux-pro-max-cli（命令仍叫 uipro）；
# 旧包名 uipro-cli 已过时，不要用
```

### 第二步：为 AI 工具安装 Skill

```bash
cd /path/to/your/project

uipro init --ai claude      # Claude Code
uipro init --ai cursor      # Cursor
uipro init --ai windsurf    # Windsurf
uipro init --ai qoder       # Qoder
uipro init --ai codex       # Codex CLI
uipro init --ai copilot     # GitHub Copilot
uipro init --ai trae        # Trae（需切 SOLO 模式才会自动触发）
uipro init --ai universal   # 通用 .agents/skills/
uipro init --ai all         # 全部安装

# 装到用户全局（对所有项目生效）
uipro init --ai claude --global
```

Claude Code 也可走插件市场：

```
/plugin marketplace add nextlevelbuilder/ui-ux-pro-max-skill
/plugin install ui-ux-pro-max@ui-ux-pro-max-skill
```

### 第三步：自然语言触发（Skill 模式）

装好后直接说需求，skill 自动激活并先生成设计系统再写代码：

```
Build a landing page for my SaaS product
Create a dashboard for healthcare analytics
Build a fintech banking app with dark theme
```

Kiro / Copilot / Roo Code / KiloCode 用斜杠命令：`/ui-ux-pro-max Build a landing page for my SaaS product`

### 第四步（进阶）：手动调用设计系统生成器

```bash
# 生成完整设计系统（ASCII 输出）
python3 .claude/skills/ui-ux-pro-max/scripts/search.py "beauty spa wellness" --design-system -p "Serenity Spa"

# Markdown / JSON 输出（--json 不截断长字段）
python3 .../search.py "fintech banking" --design-system -f markdown
python3 .../search.py "SaaS" --domain style --json

# 单域检索
python3 .../search.py "glassmorphism" --domain style
python3 .../search.py "elegant serif" --domain typography
python3 .../search.py "dashboard" --domain chart

# 技术栈专项指南
python3 .../search.py "form validation" --stack react
python3 .../search.py "responsive layout" --stack html-tailwind
```

### 第五步（推荐）：持久化设计系统

```bash
python3 .../search.py "SaaS dashboard" --design-system --persist -p "MyApp"
python3 .../search.py "SaaS dashboard" --design-system --persist -p "MyApp" --page "dashboard"
```

之后每次构建页面前提示 agent：

```
我正在构建 [页面名] 页面。请先读 design-system/MASTER.md；
若 design-system/pages/[页面名].md 存在则优先其规则，否则只用 Master。
现在开始生成代码……
```

### 第六步：维护

```bash
uipro versions               # 查看可用版本
uipro update                 # 用当前 CLI 包刷新 skill 文件
uipro uninstall --ai claude  # 卸载指定平台
npm install -g ui-ux-pro-max-cli@latest   # CLI 报错先升级 CLI
```

---

## 🔗 相关资源

| 资源 | 地址 |
|------|------|
| GitHub 仓库 | https://github.com/nextlevelbuilder/ui-ux-pro-max-skill |
| 官网 | https://uupm.cc |
| 中文社区教程 | https://github.com/bbylw/ui-ux-pro-max-skill-cn |
| npm CLI | https://www.npmjs.com/package/ui-ux-pro-max-cli |

---

## 💡 个人思考

UI/UX Pro Max 的本质是把**设计师的行业经验数据结构化**：192 个产品类型各自配好色板、字体、版式与禁忌，再用 BM25 检索按需取出。161 条行业推理规则是它的护城河——"银行业别用霓虹紫"这类知识，大多数开发者根本不知道需要知道。116k stars 说明"不懂设计的开发者"是个巨大市场。

但它的上限也清晰：生成结果是"行业模板的最优解"，能保证不犯错，难保证出彩。这与 [[Impeccable 项目笔记]] 形成天然分工——**Pro Max 负责从 0 到 1 的正确，Impeccable 负责从 1 到 100 的品味**。两者都支持 Claude Code / Cursor / Qoder，共存没有冲突。

工程上值得学习：纯标准库离线脚本、Master+Overrides 沉淀模式、semantic-release 自动化发布，都是 skill 类项目的范本。需要留意它的商业化路线（Premium 版），核心检索能力保持开源即可放心用。

> [!quote] 一句话总结
> 不懂设计又想快速出专业感界面——让 UI/UX Pro Max 按你的行业生成设计系统，是当前覆盖面最广、最省心的选择。

> 状态：✅ 已分析，⏳ 待实践 · 关联：[[Impeccable 项目笔记]]、[[UI Skills 项目笔记]]、[[Impeccable vs UI UX Pro Max vs UI Skills]]
