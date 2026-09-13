---
title: ego-lite 项目笔记
created: 2026-09-09
tags:
  - GitHub研究
  - AI工具
  - 浏览器自动化
  - Agent
description: citrolabs/ego-lite —— 为 AI Agent 而生的 Chromium 浏览器（Space 隔离 + 登录态迁移），含与 huashu-chrome 的正面对比与「3.45×」宣称的事实核查
aliases:
  - ego-lite
  - ego (lite)
---
# ego-lite 项目笔记

> [!info] 项目信息
> - **仓库**：https://github.com/citrolabs/ego-lite （开源部分 = `ego-browser` skill/harness + 文档，MIT）
> - **产品本体**：https://lite.ego.app —— 浏览器 App **独立免费下载、闭源**（⚠️ 「开源」只覆盖 harness，不覆盖浏览器）
> - **作者**：CitroLabs
> - **定位口号**：The fastest browser for AI agents to run browser automation
> - **热度**：约 15.1k stars（2026-09 初）
> - **平台**：⚠️ **仅 macOS**（Intel + Apple Silicon）；Windows / Linux 在 roadmap，**无发布日期**
> - **形态**：Chromium 浏览器（当前 0.5.0.28 staged rollout，Chromium 152）+ `ego-browser` Node harness（走 CDP）
> - **价格**：个人永久免费，无需账号；自带 model API key
> - **记录日期**：2026-09-09（Summer 要求与 huashu-chrome 做异同对比）

---

## 一句话定位

**把「浏览器」本身改造成 Agent 的一等公民运行环境**：它不是给你现有 Chrome 外挂自动化层，而是重做一个 Chromium——Agent 在你机器上开**独立 Space（工作空间）**跑任务，你的正常浏览不受打扰，登录态在首次启动时从 Chrome **一次性迁移**过来。

核心解决三件事：
1. **登录墙**（agent 拿不到 session 就干不了活）
2. **抢标签页**（人和 agent 共用一个窗口互相干扰）
3. **token 浪费**（喂原始 HTML 给模型）

---

## 🧠 关键机制

| 机制 | 说明 |
|---|---|
| **Space 隔离** | 每个 Agent 任务一个独立空间，**多任务并行**，只受机器性能限制；可 Pin、可键盘切换、可指定 Profile |
| **登录态迁移** | 首次启动从检测到的源浏览器拷贝 cookie / 扩展 / 书签；可开「定期同步浏览器数据」 |
| **Snapshot（语义树）** | 页面快照 **~200–400 tokens/页**，不是原始 DOM；配 `snapshotText` + `@ref` 元素定位 |
| **Code-based 执行** | `ego-browser` 吃 **Node.js heredoc 脚本**：Agent 一次写完整条 JS 流程，helper 预注入作用域，浏览器状态常驻 —— 省回合数 |
| **人机接管** | 任务中你可随时观察 / 停止 / 接管该 Space；Agent 控制期间屏蔽会打断它的快捷键与文件选择框等 UI |
| **数据本地** | 100% 本机，无账号无遥测；任务标签默认静音，不打扰你 |

---

## ⚡ 性能宣称（⚠️ 带水分，看这里）

> [!warning] 「3.45×」这个数字对不上
> - **官方首页/博客**：「比 Vercel 的 agent-browser 快 **3.45×**，4 个复杂任务」
> - **第三方核查（aibrew，2026-08-12）**：翻遍官方自己公布的基准图，四个任务 median-of-5 的实测差距是 **1.4× / 1.9× / 2.6×**，**「3.45」在图里根本不存在**，且未标注硬件与日期
> - **结论**：官方自测厂商基准，宣传口径高于自家图表。**采信「显著更快」，不采信「3.45×」**

> [!note] 另一组数字（同样来自官方，利益相关需打折）
> Real-World Bench（2026-08，31 个真实站点任务，同一模型 + 独立评审，gpt-5.6-sol max effort）完美完成率：
> - ego (lite) **93.5%**
> - Browser Harness（Browser Use 本地版）77.4%
> - playwright-cli 71.0%
> - agent-browser（Vercel）62.9%
> - chrome-devtools-cli 61.3%
>
> 注意：该榜单由 ego 自家文章发布且明确承认「我们是榜单第一名、也是被评产品」，属自证性证据。

- 官方还宣称「Skill 会把成功动作蒸馏成可复用工具，同类任务快至 5×」—— 标注为 **coming soon**，未落地

---

## 🔌 生态

`ego-browser` skill 官方文档覆盖：Claude Code / Codex / Cursor / Kiro / Gemini CLI / Hermes Agent / OpenCode / DeepSeek Harness / OpenClaw，以及「任何会写代码的 agent」。

接入方式：装 macOS App → CLI 自动同步 skill 与 agent 集成（浏览器更新后自动同步版本）。

---

## ⚖️ 与 huashu-chrome 的异同

详见 [[04-GitHub研究/huashu-chrome/huashu-chrome 项目笔记#🆚 与 ego-lite 的对比（2026-09-09）|huashu-chrome 笔记内对比章节]]。要点：

| 维度 | ego-lite | huashu-chrome |
|---|---|---|
| **层级** | 重造浏览器（改内核） | 给现有 Chrome 外挂（扩展 + 本地桥 + MCP） |
| **平台** | ❌ 仅 macOS | ✅ 跨平台（Node ≥20） |
| **登录态** | 首次**一次性拷贝** profile | 直接操控**你正在用的** Chrome，实时零拷贝 |
| **是否抢焦点** | ✅ Space 隔离，不打扰你 | ❌ 在你的真实窗口开标签操作 |
| **token 效率** | ✅ 语义快照 200–400/页 | 常规文本读取 |
| **并行多任务** | ✅ 多 Space 并行 | ❌ 单浏览器实例串行 |
| **独门能力** | 并行 + 快照压缩 + Code-based 单次跑整流程 | **`network` 摸接口 + `fetch` 带 cookie 翻页落盘 JSONL** |
| **体量** | 15k stars，公司团队，完整文档站 | 个人项目，npm v1.2.0 |

**共同点**：都走 CDP/真实浏览器（不是 headless 模拟）、都吃真实登录态、都有**站点经验沉淀库**（ego 叫 experience accumulation，huashu 叫 `learnings/`）、都 MIT、都不解决 IP 层封禁。

---

## 🎯 裁决（针对本机 / 本场景）

1. **纯项目实力 → ego-lite 胜**，量级差异（内核改造 + Space 并行 + token 工程 + 团队化文档）
2. **今天可用性 → huashu-chrome 胜**：Summer 这台是 **Windows**，ego-lite **装不了**
3. **QuantV1 场景（摸接口 + 批量取数）→ huashu-chrome 的 `network` 无替代**；ego-lite 面向「通用网页操作代理」，无对等的接口侦察工具
4. **风控角度 ego-lite 反而更危险**：多 Space 并行很容易把目标站踩爆
5. **东财类 IP/接口级封禁两个都救不了** —— 共用宿主机出口 IP，换浏览器 ≠ 换 IP（已在 huashu-chrome 笔记实证）

---

## 👀 观察清单

- [ ] **Windows 版落地**是唯一的启用条件。当前进展：
  - Maintainer 在 discussion #49 明确回复：「正在积极做 Windows 版，**已专门购入 Windows 机器搭构建与测试环境**，in progress 不只是 planned」
  - 但官方 FAQ / 下载页仍写「macOS today，Windows & Linux on the roadmap，无确定日期」，站点提供 **"Get pinged for Windows"** 等候名单
  - issue #374（2026-09-08 开）请求「从 WSL2 agent 驱动 Windows 宿主机上的 ego lite」—— 侧面说明 Windows 本体仍未出
  - 0.4.5.5 / 0.5.0.28 changelog 已出现 Windows 相关修复项（开始菜单快捷方式命名、跨平台窗口圆角、软件渲染/虚拟机缩略图）—— **内测迹象，但无公开包**
  - 复核节奏：每季度看一次 lite.ego.app/download 是否出现 Windows 按钮
- [ ] 若将来有 Mac：优先实测 Space 并行 + snapshot token 消耗，并与 huashu-chrome 在同一登录态任务上对比 token 用量
- [ ] 关注「experience accumulation（同类任务快 5×）」从 coming soon 变为实际功能

---

## 🔗 相关笔记

- [[04-GitHub研究/huashu-chrome/huashu-chrome 项目笔记|huashu-chrome 项目笔记]]（本机可跑的那一个）
- [[量化交易工具/板块K线数据源-东财频控诊断与兜底通道-20260908|板块K线数据源-东财频控诊断与兜底通道-20260908]]（「换浏览器 ≠ 换 IP」的实证来源）
