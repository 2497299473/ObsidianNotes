---
lark_doc_url: https://my.feishu.cn/docx/V3MLdlQlOoGd9vxunmdcuNCOncW
lark_doc_token: V3MLdlQlOoGd9vxunmdcuNCOncW
---
# Astryx 项目笔记

> 创建日期：2026-08-02
> 仓库：[facebook/astryx](https://github.com/facebook/astryx)
> 官网：[astryx.atmeta.com](https://astryx.atmeta.com)
> 状态：开源 MIT，已关注 🔍

---

## 项目概述

**Astryx** 是 **Meta** 内部用了 8 年、支撑了 13,000+ 个应用的设计系统，2026 年 6 月以 MIT 协议开源。

- **语言**：TypeScript（React 19 + StyleX）
- **许可证**：MIT
- **定位**：零依赖、AI-Fluent 的 React 组件库，150+ 无障碍组件，7 套主题

---

## 核心指标

| 指标 | 数值 |
|------|------|
| 组件数量 | 150+ |
| 预置主题 | 7 套（neutral / butter / chocolate / matcha / stone / gothic / y2k） |
| 内部使用年限 | 8 年 |
| 支撑应用数 | 13,000+ |
| 无障碍 | 内置完整键盘导航 + 屏幕阅读器支持 |
| 依赖 | 零（预编译 CSS，无需 PostCSS / Babel 配置） |

---

## 安装方式

```bash
# 核心
npm install @astryxdesign/core @astryxdesign/theme-neutral

# CLI（选装）
npm install -D @astryxdesign/cli
```

支持 Next.js、Vite、Tailwind、CDN 等多种集成方式。

---

## 关键特性

- 🤖 **AI-Fluent**：API、文档、CLI 三位一体，自带 MCP 服务器，人类和 AI 用同一套工具构建
- 🎨 **行为与外观分离**：组件控制行为和无障碍，主题（CSS token 级别）控制外观
- 🔧 **渐进式定制**：token 调色 → className 覆盖 → CSS 深入 → `swizzle` 弹出源码完全接管
- 🧪 **Vibe Tests**：内置评估框架，用数据而非意见来裁决设计决策
- 📦 **零依赖**：预编译 CSS，直接 import 就能用
- 🧭 **CLI 工具**：浏览文档、脚手架、生成主题、自动升级 codemods
- 📊 **实验性图表**：`@astryxdesign/charts`（实验性，对量化可视化有潜力）

---

## 与你相关

| 领域 | 关联度 | 说明 |
|------|--------|------|
| 全栈开发 | 🔥 强相关 | React 项目直接使用，150+ 组件覆盖大多数 UI 需求 |
| AI 研究 | 🔥 强相关 | 首个为 AI Agent 设计的组件库，MCP 服务器让 AI 工具直接读取文档和 API |
| 量化交易 | ⚡ 间接 | Dashboard、回测面板、数据监控页面可用 |
| DevOps | ⚡ 间接 | 内部运维面板、监控大盘可用模板快速搭建 |

---

## 社区与资源

- GitHub：[facebook/astryx](https://github.com/facebook/astryx)
- 官网文档：[astryx.atmeta.com](https://astryx.atmeta.com)
- Storybook：[facebook.github.io/astryx/storybook/](https://facebook.github.io/astryx/storybook/)
- 在线 Sandbox：[facebook.github.io/astryx/sandbox/](https://facebook.github.io/astryx/sandbox/)
- 介绍博客：[astryx.atmeta.com/blog/introducing-astryx](https://astryx.atmeta.com/blog/introducing-astryx)
- 技术博客：[astryx.atmeta.com/blog/how-astryx-works](https://astryx.atmeta.com/blog/how-astryx-works)

---

## 探索路线

1. 官网文档站 → `/components` 浏览组件
2. Storybook → 交互式 demo
3. 在线 Sandbox → 浏览器直接写代码
4. GitHub → 看 `packages/core` 源码
5. 本地 `npm install` 试手

---

## 日志

- 2026-08-02：首次了解 Astryx，建立笔记