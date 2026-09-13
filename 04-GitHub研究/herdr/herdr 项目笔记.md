---
lark_doc_token: Q2gJd8cfcoSM2ExPJ5tcTtGsnjc
lark_doc_url: https://my.feishu.cn/docx/Q2gJd8cfcoSM2ExPJ5tcTtGsnjc
---
# herdr 项目笔记

> 创建日期：2026-08-08
> 仓库：[herdrdev/herdr](https://github.com/herdrdev/herdr)
> 官网：[herdr.dev](https://herdr.dev)
> 状态：开源 Apache 2.0，已关注 🔍

---

## 项目概述

**herdr**（读作 "herd-er"，即"牧人"）是 **Rust** 编写的终端 **Agent 多路复用器**（terminal agent multiplexer），定位是 **"the runtime your coding agents live on"**——编码 AI Agent 的运行运行时。它不包装、不替换 Claude Code、Codex、Cursor、OpenCode、Grok 等任何 Agent，只接管它们的终端，让多个 Agent 在 workspace / tab / pane 里并行工作，并实时标注每个 pane 的状态。

本质上是 **tmux 的 AI-Agent 版**：一个常驻后台的 PTY 多路复用器 + Agent 状态感知 + 持久会话 + Socket API。

- **语言**：Rust
- **许可证**：Apache 2.0
- **形态**：单一 Rust 二进制，无 Electron，跑在任意已有终端里
- **热度**：2026 年 6 月底登顶 GitHub Trending（Rust 类 #1，全站日榜 #17），并上了 Hacker News 首页

---

## 核心指标

| 指标 | 数值 |
|------|------|
| Star | 25.0k |
| Fork | 1.8k |
| 最新版本 | v0.4.0 |
| 社区插件 | 500+（marketplace 自动发现 GitHub `herdr-plugin` topic） |
| 支持 Agent | Claude Code、Codex、Cursor、OpenCode、Grok CLI、Qoder CLI 等 |
| 中文支持 | 官方 README 提供简体中文版 |

---

## 安装方式

```bash
# 一行脚本（macOS / Linux）
curl -fsSL https://herdr.dev/install.sh | sh

# Homebrew
brew install herdr

# mise
mise use -g herdr

# Windows Beta（PowerShell）
powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"

# 源码构建
git clone https://github.com/herdrdev/herdr && cd herdr && cargo build --release
```

启动后：`ctrl+b q` 分离会话，重新运行 `herdr` 即可重连。

---

## 关键特性

- 🕒 **常驻运行（always running）**：herdr 是后台 server，终端存活在 server 内部。合盖、断网、重启机器，Agent 继续工作，会话可恢复；可从任意终端或 SSH 重连
- 🔔 **注意力队列（never hunt for the stuck one）**：每个 pane 标记 working / blocked / idle，Agent 停下来需要你回答时主动提示，不用逐个窗口找
- 🤖 **Agent 原生（agent-native）**：CLI 与 Socket API 是同一套接口，Agent 可直接驱动——spawn pane、Agent 之间互相 prompt、等待另一个 Agent 真正阻塞
- 🧩 **插件生态**：150+ 社区插件（现 500+），扩展 pane 与工作流；发布只需给仓库打 `herdr-plugin` topic + `herdr-plugin.toml` 清单，自动被市场索引
- 🖱️ **键盘鼠标双第一公民**：tmux 风格前缀键 + 点击、拖拽、分屏，按场景选择
- 🚀 **轻量**：一个 Rust 二进制，无 Electron，复用已有终端

## 典型用法（CLI / Socket API）

```bash
# 创建工作区结构
herdr --cwd ~/project --label api
herdr --label logs

# 分屏并运行任务
herdr 1-1 --direction right
herdr 1-2 "just test"

# 等待、检查、继续
herdr 1-1 --status done
herdr 1-2 --source recent-unwrapped
```

---

## 与你相关

| 领域 | 关联度 | 说明 |
|------|--------|------|
| AI 研究 | 🔥 强相关 | 多 Agent 并行编排、Agent 状态感知基础设施的代表作 |
| 全栈开发 | 🔥 强相关 | 同时跑多个编码 Agent（Claude Code / Codex / Qoder）监控协作 |
| DevOps | ⚡ 间接 | 后台守护进程模式、断线重连、SSH 远程管理思路可借鉴 |
| 量化交易 | ⚡ 间接 | 多策略/多任务并行执行与状态监控的场景可类比复用 |

---

## 社区与资源

- GitHub：[herdrdev/herdr](https://github.com/herdrdev/herdr)
- 官网：[herdr.dev](https://herdr.dev)
- 文档：[herdr.dev/docs](https://herdr.dev/docs/)（quick start / concepts / socket api / plugins）
- 插件市场：[herdr.dev/plugins](https://herdr.dev/plugins)
- 中文 README：[README.zh-CN.md](https://github.com/herdrdev/herdr/blob/master/README.zh-CN.md)
- 生态指南（第三方）：awesome-herdr 等
- 典型第三方插件：herdr-plus（Projects/Quick Actions）、herdr-file-viewer（git 文件树 + diff 侧栏）、herdr-reviewr（diff 评审回注给 Agent）、vim-herdr-navigation（vim 风格导航）

---

## 探索路线

1. 官网 quick start → 装一个跑起来
2. 用 Claude Code / Codex 各开一个 pane，体验 attention queue
3. 读 Socket API 文档 → 看 Agent 如何自驱动
4. 逛插件市场 → 装 1-2 个常用插件
5. 对比 tmux/Zellij 的使用体验差异

---

## 日志

- 2026-08-08：首次了解 herdr，建立笔记（v0.4.0，25k stars）
