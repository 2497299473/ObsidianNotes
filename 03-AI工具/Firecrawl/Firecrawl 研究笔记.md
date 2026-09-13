---
lark_doc_url: https://my.feishu.cn/docx/Y4DvdZjbGoqG0OxsonKc2YLRn9c
---
# Firecrawl

**日期**: 2026-08-02
**来源**: https://www.firecrawl.dev/
**文档**: https://docs.firecrawl.dev
**GitHub**: https://github.com/firecrawl/firecrawl（开源，150K+ stars，GitHub Top 100）

## 是什么

Firecrawl 是 **"上下文 API"（The context API）**：一个把网页变成 **LLM-ready 数据** 的基础设施服务。口号是帮 AI "找到、读取、操作"实时网页。

一句话总结：**给 AI Agent / LLM 用的"搜索 + 爬虫 + 浏览器操作"三合一 API**，输出干净的 Markdown / JSON / 截图，而不是原始 HTML。

## 三大核心能力

| 能力 | 作用 | 典型场景 |
|------|------|----------|
| 🔍 Search | 搜索网页并直接返回结果全文（Markdown） | AI 深度研究、RAG 知识库 |
| 🧹 Scrape | 输入 URL → 输出干净 Markdown/JSON/截图，自动处理 JS 渲染 | 数据采集、页面结构化 |
| 🖱️ Interact | 让 AI 操作网页（点击、滚动、填表、登录后抓取） | 动态页面、多步骤流程 |

另外还有 /crawl（整站爬取）、/map（站点结构）、/monitor（监控）等端点。

## 关键特点

- **开源**：GitHub 上最大的同类开源仓库（150K+ stars），可自部署；托管版（Fire-engine）有专有代理/渲染基础设施
- **免费额度**：每月 **1000 credits**（约 1000 页），无需付费即可试用
- **token 友好**：输出比原网页少 93% 的输入 token（去掉了导航、页脚、广告）
- **可靠性**：声称覆盖 96% 网页（含 JS 重页面），P95 延迟 3.4s
- **生态**：官方 SDK（Python / Node / Go / Rust / Java / Elixir）+ CLI + REST API
- **MCP 支持**：官方 MCP server，Cursor、Claude Code、Windsurf 一键接入（已安装 40 万+ MCP server）
- **一键接入 AI 编码工具**：`npx -y firecrawl-cli@latest init --all --browser`

## 计费（2026-08 官网信息）

| 项目 | 成本 |
|------|------|
| Scrape / Crawl / Map / Monitor | 1 credit / 页 |
| Search | 2 credits / 10 条结果 |
| Interact | 2 credits / 浏览器分钟 |
| Agent（预览） | 每天 5 次免费运行 |

免费档每月 1000 credits；Hobby / Standard / Growth 计划按 credits 升级。credits 当月不结转（Scale/Enterprise 可结转）。

## 与我的匹配度

| 领域 | 相关度 | 说明 |
|------|--------|------|
| AI 研究 | ⭐⭐⭐⭐⭐ | 给 AI Agent 接入实时网络数据，深度研究、RAG 管道的核心基建 |
| 全栈开发 | ⭐⭐⭐⭐ | 替代自建爬虫；把网页数据喂给应用；MCP 一键接入 Claude Code/Cursor |
| 量化交易 | ⭐⭐⭐ | 定期抓取新闻/公告/行情页做数据源（Monitor 端点可做定时监控） |
| DevOps | ⭐⭐⭐ | 开源版可 Docker 自部署，省 API 费用 |

## 探索路径（明天行动清单）

1. **注册** → firecrawl.dev 注册免费账号，拿 API key（每月 1000 credits）
2. **看文档** → docs.firecrawl.dev 快速开始，试一次 /scrape 和 /search
3. **接入 Claude Code** → `npx -y firecrawl-cli@latest init --all --browser`，一键获得搜索+抓取工具
4. **试用例** → 抓一个 JS 重页面（如动态加载的网站）转 Markdown，对比效果
5. **评估自部署** → 如果免费额度不够用，再看 GitHub 仓库的 Docker 自部署方案

## 结论

对 AI 研究方向价值很高：它解决的是 **"AI 拿不到干净实时网页数据"** 这个痛点，而且免费额度够先试水。建议先注册用免费档接入 Claude Code 体验，跑通一个真实用例（比如抓取某行业新闻做分析），再决定是否深入或自部署。

## 相关笔记

- [[pdf-inspector 研究笔记]] — Firecrawl 开源的本地 PDF 解析引擎（2026-08-05 记录）
