---
lark_doc_url: https://my.feishu.cn/docx/AYR0dE561oBGC5xR3oKczPaQnHb
---
# pdf-inspector

**日期**: 2026-08-05
**来源**: https://github.com/firecrawl/pdf-inspector
**所属**: Firecrawl 官方开源项目（与 [[Firecrawl 研究笔记]] 同一家公司 / 同一生态）

## 是什么

pdf-inspector 是 Firecrawl 开源的 **本地 PDF 解析引擎**（Rust 编写，MIT 协议，~9.6k stars），也是他们新发布的 **Fire-PDF 引擎**的核心组件。

一句话总结：**本地 PDF 分类 + 文本提取 + 转 Markdown 的库，全程不需要 OCR**。

它解决的核心问题：约 54% 的 PDF 是纯文本型（财报、论文、发票、法律文书），本地处理只要 **<200ms**，根本不需要送昂贵的 OCR 服务。它做的是「**先分类、再路由**」——文本页本地秒提取，只有扫描页才升级到 OCR。

## 三大核心能力

| 能力 | 作用 |
|------|------|
| 🧠 智能分类 | 10~50ms 判断 PDF 是「文本型 / 扫描型 / 图片型 / 混合型」，带置信度，并给出哪些页需要 OCR（`pages_needing_ocr`） |
| 📄 文本提取 | 带字体、XY 坐标的位置感知提取，支持多栏排版、CID 字体、RTL |
| 📝 转 Markdown | 标题层级、列表、代码块、表格（矩形+启发式双模式）、粗斜体、链接，自动过滤页码、合并断词 |

## 性能基准（opendataloader-bench，200 份 PDF）

| 引擎 | 综合分 | 表格(TEDS) | 200份耗时 |
|------|--------|-----------|----------|
| **pdf-inspector** | **0.875** | **0.814** | **0.47s** |
| liteparse | 0.873 | 0.693 | 0.75s |
| pymupdf4llm | 0.735 | 0.401 | 17.1s |
| markitdown (微软) | 0.589 | 0.273 | 16.2s |

表格识别和速度都是第一梯队，尤其**表格**这一项比 pymupdf4llm 和 markitdown 强一个档次——对处理财务数据很关键。

## 与 Firecrawl 的关系

Firecrawl 云 API = 网页搜索 + 爬虫 + 浏览器操作（云端）；
pdf-inspector = 它家 **PDF 解析能力的开源本地版**——把最常用的一块能力免费开源，还支持浏览器 WebAssembly（纯前端就能跑）。

## 与我的匹配度

| 领域 | 相关度 | 说明 |
|------|--------|------|
| 量化交易 | ⭐⭐⭐⭐⭐ | 财报、公告、研报 PDF 批量转 Markdown → 喂 RAG / 结构化数据提取，本地免费跑 |
| AI 研究 | ⭐⭐⭐⭐ | 论文 PDF 批量转 Markdown，作为知识库/检索管道数据源，不用等云端 OCR |
| 全栈开发 | ⭐⭐⭐⭐ | Python / Node.js 一行集成进自己的服务；CLI 一条命令 `pdf2md document.pdf` |
| DevOps/成本 | ⭐⭐⭐ | 和 Firecrawl API 配合做智能路由：文本 PDF 本地处理，只有扫描页才走 OCR，省钱省延迟 |

## 快速上手

```bash
# Python
pip install pdf-inspector
# Node
npm install @firecrawl/pdf-inspector
# 或直接用 CLI（装 Rust 工具链后）
pdf2md document.pdf
```

## 探索路径（行动清单）

1. **装 CLI** → 装 Rust 工具链，跑 `pdf2md` 处理一份研报/财报 PDF，看效果
2. **对比** → 同一份 PDF 用 markitdown / pymupdf4llm 跑一遍，对比表格和速度
3. **验证分类能力** → 拿一份扫描版 PDF，看它能否正确标记 `pages_needing_ocr`
4. **集成** → 在自己的 Python 项目里 pip 安装，写个批量转 Markdown 的小脚本
5. **串起来** → 和 Firecrawl 笔记的路线合并：云端爬网页 + 本地解析 PDF，构建完整的数据管道

## 结论

对量化交易方向价值最高：财报/研报的**表格**提取是目前开源方案里最强的，而且本地免费、速度毫秒级。建议先拿一份真实的财报 PDF 跑 `pdf2md` 验证效果，再决定是否集成进数据管道。
