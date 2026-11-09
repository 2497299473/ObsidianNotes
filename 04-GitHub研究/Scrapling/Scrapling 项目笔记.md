---
title: Scrapling 项目笔记
created: 2026-11-09
tags:
  - GitHub研究
  - 采集
  - MCP
  - 已启用
description: D4Vinci/Scrapling 抓取框架研究与 MCP 接入实录：17 工具已进 OpenSquilla，gateway 冷启动 95 秒特性与踩坑全记录
---

# Scrapling 项目笔记

> 一句话：Python 抓取框架（解析/抓取/爬虫/AI 四层一体），已以 MCP stdio 方式接入 OpenSquilla，17 个工具可被模型直接调用。

- 仓库：https://github.com/D4Vinci/Scrapling
- 基本盘（2026-11-09 API 实测）：**99,921★ / 10,449 fork / BSD-3-Clause**，2024-10-13 创建，持续活跃（当日仍有 push）
- 版本：0.5.1，Python ≥3.10；文档 https://scrapling.readthedocs.io
- 状态：**已启用**（MCP 接入，2026-11-09 验收通过）

## 一、能力速览

| 层 | 内容 |
|---|---|
| 解析 | CSS/XPath/正则/文本搜索 + **自适应选择器**（改版后按相似度重定位元素，`adaptive=True`） |
| 抓取 | `Fetcher`（HTTP+TLS 指纹伪装+HTTP/3）→ 会话态 → `StealthyFetcher`（Playwright/Chrome，Cloudflare Turnstile） |
| 爬虫 | Scrapy 风格 Spider、并发、代理轮换、AutoThrottle、`crawldir` 断点续爬、流式产出、JSON/CSV/XML 导出 |
| AI | MCP server（17 工具）、Agent Skill、CLI + 交互 shell |

## 二、本机安装（隔离环境）

- venv：`D:\PythonProject\QuantV1\.venv-scrapling`（uv 建，Python 3.12.13，51 包）
- 安装：`uv pip install "scrapling[fetchers,ai]"` + `scrapling install`（浏览器二进制，约 66s）
- 与主 `.venv`、全局 playwright 完全隔离，互不影响

## 三、MCP 接入实录（2026-11-09）

**配置**（`C:\Users\Turn-\.opensquilla\config.toml`，改前备份 `config.toml.bak-mcp-20261109`）：

```toml
[mcp]
enabled = true
connect_timeout_seconds = 300.0   # ← 关键！勿调回默认

[[mcp.servers]]
name = "scrapling"
transport = "stdio"
command = 'D:\PythonProject\QuantV1\.venv-scrapling\Scripts\scrapling-mcp.exe'
tool_timeout_seconds = 60.0
```

**验收链**（全部实测）：
1. 生产代码路径冒烟（`MCPStdioClient → discover_and_register → ToolRegistry` handler）：17 工具注册 + `make_request` 真调 200 ✅
2. gateway 重启后 boot 日志：`tools=17, failures=0`，耗时 **95.5 秒** ✅
3. 模型身份 `tool_search` 可见 `mcp__scrapling` 命名空间 + 真调 `make_request` 返回 CSS 抽取结果 ✅

**踩坑（重要）**：
- **gateway 上下文里 stdio 子进程启动极慢**：同样代码在普通会话 16.9s 完成，gateway（owned-launch 进程树）里要 90–97s。`connect_timeout_seconds` 默认 30 → 90 都超时，**300 才稳过**。每次重启 gateway 后约 95 秒工具才可用，属已知特性，不是故障。
- `config.reload` RPC **不会**重连 MCP；MCP discovery 只在 boot 跑。改 `[mcp]` 段必须重启 gateway。
- scrapling 的 `--http` 模式是 streamable-http，而 opensquilla 0.5.8 MCP 客户端只支持 stdio + legacy SSE（`/sse` 404），HTTP 模式接不进配置层，只能当 Python 侧通道（已实测 1.1s 连通 + 17 工具，备用方案，现未启用）。

**授权边界**：配置 description 已写明只读 demo 已批、**禁止发东财请求**（铁律 7：Scrapling 解决反爬识别，解决不了东财 IP 级频控）。

## 四、对 QuantV1 的价值判断

- 可用：TLS 指纹伪装降低脚本识别概率；解析性能（本机实测 5000 次 CSS 查询 829ms）；断点续爬思路
- 不能指望：东财 IP 频控（与 huashu-chrome 结论一致，换指纹不换 IP 无效）
- 未接入主链路，仅 MCP 工具形态待命

## 五、关联

- [[04-GitHub研究/ziwei-doushu/ziwei-doushu 项目笔记]] — 命理 Agent 原型的另一半（领域知识）
- [[04-GitHub研究/huashu-chrome/huashu-chrome 项目笔记]] — 同槽对照（都救不了东财 IP 频控）
- GitHub 抓取口径（本机网络）：`github.com`/`raw.githubusercontent.com` 直连被断（超时/RST），走系统代理 `127.0.0.1:7890`（Invoke-WebRequest 自动读）；`api.github.com`/`codeload.github.com` 直连通。**抓 GitHub 一律优先 API**；web_fetch 工具不读系统代理，对前两域失败属预期。
