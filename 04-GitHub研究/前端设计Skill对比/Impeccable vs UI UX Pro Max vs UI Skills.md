---
lark_doc_token: E6qtd39BCoFGoTxqTjZcyr44nig
lark_doc_url: https://my.feishu.cn/docx/E6qtd39BCoFGoTxqTjZcyr44nig
---
# Impeccable vs UI UX Pro Max vs UI Skills

> 记录日期：2026-08-13 · 关联笔记：[[Impeccable 项目笔记]]、[[UI UX Pro Max 项目笔记]]、[[UI Skills 项目笔记]]

三者都是给 AI 编码工具（Claude Code / Cursor / Codex 等）提供前端设计能力的 Skill，但定位完全不同：**Pro Max 管"生成"，Impeccable 管"质检"，UI Skills 管"选型与路由"**。

---

## 一句话定位

| 项目 | 仓库 | 一句话定位 | Stars | 协议 | 主语言 | 首发 | 最新版本 |
|------|------|-----------|-------|------|--------|------|----------|
| **Impeccable** | [pbakaus/impeccable](https://github.com/pbakaus/impeccable) | 23 命令 + 59 条确定性检测规则，给 AI 装上设计品味与质检器 | 58.8k | Apache-2.0 | JavaScript | 2025-11 | ext-v1.3.1（2026-07-30） |
| **UI/UX Pro Max** | [nextlevelbuilder/ui-ux-pro-max-skill](https://github.com/nextlevelbuilder/ui-ux-pro-max-skill) | 161 条行业规则 + BM25 检索，一句话生成完整设计系统 | 116.4k | MIT | Python | 2025-11 | v2.14.2（2026-08-12） |
| **UI Skills** | [ibelick/ui-skills](https://github.com/ibelick/ui-skills) | 技能路由器 + 聚合注册表，按任务挑"最小够用"的技能集 | 7.2k | MIT | TypeScript | 2026-01 | v0.2.3（2026-06-22） |

（数据均为 2026-08-13 经 GitHub API 核实；三者最近 push 均在记录日一周内，都在活跃维护）

---

## 核心机制对照

| 维度 | Impeccable | UI/UX Pro Max | UI Skills |
|------|-----------|---------------|-----------|
| **思路** | 流程 + 规则：命令覆盖规划→构建→审查→打磨 | 知识库 + 推理：离线检索行业设计资产生成设计系统 | 路由 + 聚合：按任务分发到一方/社区技能 |
| **资产规模** | 59 条检测规则 + PRODUCT.md/DESIGN.md 约定 | 84 风格 / 192 色板 / 74 字体 / 161 行业规则 / 22 技术栈 | 7 个一方技能 + 数十个社区技能注册表 |
| **质量把关** | ✅ 确定性检测器（无 LLM、可进 CI、hook 实时拦截） | ⚠️ 反模式清单 + 交付前检查清单 | ❌ 依赖各技能自身 |
| **调用方式** | `/impeccable <命令>` + live 浏览器迭代 | 自然语言自动触发 / 斜杠命令 / Python 脚本 | `npx ui-skills start` 路由 + `get` 拉取 |
| **运行时依赖** | Node | Node + Python 3（纯标准库，离线） | Node |
| **平台覆盖** | 14 个 harness | 20 个 AI 平台 + 22 技术栈 | 主流 agent（路由技能约定） |
| **持久化** | PRODUCT.md / DESIGN.md / .impeccable/ | design-system/MASTER.md + pages/ 覆盖 | 无（各技能自管） |
| **商业化** | 无 | 有 Premium 付费版（核心开源） | 无 |

---

## 优缺点速览

### Impeccable

- ✅ 确定性检测独一档：规则透明、零 token、可进 CI、Cursor 可前置拦截坏写入
- ✅ 命令词汇表完整，团队可共享设计语言；作者为前 Google 布道师，工程完成度高
- ⚠️ 概念多上手陡（init/hook/live/gitignore 一套）；检测器管"错"不管"美"；`.impeccable/` 侵入项目目录

### UI/UX Pro Max

- ✅ 行业知识库规模断层领先，完全离线（纯 Python 标准库 + CSV）
- ✅ 唯一覆盖桌面/移动技术栈；Master+Overrides 可跨会话沉淀；社区最热（116k stars）
- ⚠️ 生成偏模板化正确、缺个性打磨；有付费版引流；仓库大、CLI 版本碎片需注意

### UI Skills

- ✅ 最轻量（一条 npx）；"最小够用、≤3 技能"路由保护上下文预算；聚合入口可一键拉取前两者
- ⚠️ 体量最小（约 6 贡献者）、文档偏薄；一方技能仅 7 个，深度依赖社区收录质量

---

## 选型建议

| 场景 | 推荐 |
|------|------|
| 从零建落地页/行业应用，要快速专业感 | **UI/UX Pro Max** |
| 已有页面"AI 味"重，想系统打磨 | **Impeccable**（先 `audit`/`critique` 再 `polish`） |
| 想把设计质检进 CI / 编辑时拦截 | **Impeccable**（detect CLI + hook，唯一选项） |
| 桌面端（WPF/WinUI/JavaFX）或移动端（SwiftUI/Compose/Flutter） | **UI/UX Pro Max**（唯一覆盖） |
| 只想补单点能力（a11y/动效性能/SEO 元数据） | **UI Skills** 一方技能 |
| 技能太多不知选哪个 | **UI Skills** 做入口，`get` 按需拉 |

### 组合玩法（推荐）

```
UI Skills（入口/路由，按需加载）
    │
    ├─ get ui-ux-pro-max ──▶ 从 0 生成行业正确的设计系统并构建
    │                              │
    │                              ▼
    └─ get impeccable ────▶ audit 质检 → critique 品味评审 → polish 收尾
                               （detect CLI 进 CI，防设计倒退）
```

三者互补而非竞品：Pro Max 保证"从 0 到 1 不出错"，Impeccable 保证"从 1 到 100 有品味"，UI Skills 负责别把上下文塞爆。

---

## 上手成本对比

| 项目 | 前置 | 最短路径 | 预估上手时间 |
|------|------|---------|-------------|
| UI Skills | Node | `npx ui-skills start` | 2 分钟 |
| UI/UX Pro Max | Node + Python 3 | `npm i -g ui-ux-pro-max-cli` → `uipro init --ai claude` → 直接说需求 | 5–10 分钟 |
| Impeccable | Node | `npx impeccable install` → `/impeccable init` | 20–30 分钟（含理解上下文文件与 hook） |

---

## 相关笔记

- [[Impeccable 项目笔记]] — 设计品味 + 确定性质检（单项深挖）
- [[UI UX Pro Max 项目笔记]] — 行业级设计系统生成器（单项深挖）
- [[UI Skills 项目笔记]] — 技能路由器与聚合入口（单项深挖）
