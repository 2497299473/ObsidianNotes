---
lark_doc_token: WSDpdSTdMoLbAbx7xvMcg20snv8
lark_doc_url: https://my.feishu.cn/docx/WSDpdSTdMoLbAbx7xvMcg20snv8
---
# book-to-skill 项目笔记

> 创建日期：2026-08-08
> 仓库：[virgiliojr94/book-to-skill](https://github.com/virgiliojr94/book-to-skill)
> 状态：开源 MIT，热门项目 🔥 已关注

---

## 项目概述

**book-to-skill** 是一个把**技术书籍 / 文档文件夹 / 任意结构化文本**转化为 **Agent Skill**（`SKILL.md` 格式）的转换工具。核心定位一句话：

> *Turn any technical book, document folder, or collection of sources into a unified agent skill — ready to study, reference, and use while you work.*

也就是：你有一本技术书，读完就忘；与其反复翻 PDF 或把整本书塞进 LLM 上下文（贵且会忘），不如**一次性把书蒸馏成结构化的 Skill**，让 Claude Code / Copilot CLI / Amp 等 Agent 在需要时按需加载对应章节，直接基于书里的真实内容回答，不幻觉、不烧 token。

解决的真实痛点（README 原话）：
- 📄 搜 PDF → 得到的是页码列表，不是答案
- 🧠 直接问 Agent → 要么幻觉，要么说"我没有这本书的内容"
- 📝 自己记笔记 → 记完 200 行就再也没打开过

- **语言**：Python
- **许可证**：MIT（仅指转换器本身，不含任何书籍内容）
- **形态**：Agent Skill（克隆进 skills 目录即用）+ 本地 Python 提取器
- **热度**：18.6k stars / 2.0k forks（2026-08-08），登上 Trendshift 榜单，更新非常活跃（最后 push 2026-08-07）

---

## 核心指标

| 指标 | 数值 |
|------|------|
| Star | 18.6k |
| Fork | 2.0k |
| 创建时间 | 2026-05-01 |
| 主语言 | Python |
| 协议 | MIT |
| 支持宿主 | GitHub Copilot CLI、Amp、Claude Code（开放 Agent Skills 标准） |
| 支持格式 | PDF、EPUB、DOCX、MD、HTML、RTF、MOBI/AZW/AZW3、TXT、reST、AsciiDoc |
| Token 节省 | 相比整书塞上下文 **24×–51×**（实测） |

---

## 工作原理（三步）

1. **指向**：`/book-to-skill ./my-book.pdf`（支持单文件、文件夹、glob、多文件列表）
2. **蒸馏**：把书转成 Skill —— 提炼框架、决策规则、反模式、逐章文件。产出的是**结构**，不是摘要
3. **按需加载**：之后问 `/my-book replication`，Agent 读对应章节，从真实内容回答，不幻觉

架构上分两半：
- **确定性 Python 提取器**（`extract.py`）：文档 → 干净文本 + 元数据 + 章节检测，纯机械工作
- **规范驱动的生成器**：Agent 遵循仓库里的 `SKILL.md`（即"生成器规格书"）把提取出的文本蒸馏成结构化 Skill —— 所有"理解"工作由 LLM 完成

提取器按书籍类型自动选工具：
- **技术书**（代码/表格/公式多）→ `docling`（保留 markdown 表格和代码块，~1.5s/页）
- **纯文字书** → `pdftotext`（poppler，瞬时）
- EPUB → `ebooklib + beautifulsoup4`；其余格式各有对应解析器，纯文本类零依赖
- 一条命令检查环境：`python3 scripts/extract.py --check`

---

## 生成物结构（两层 Skill）

| 文件 | 作用 | 大小 |
|------|------|------|
| `SKILL.md` | 核心心智模型 + 章节索引 | ~4,000 tokens |
| `chapters/ch01-*.md` | 每章一个文件，**按需加载**（不占常驻预算） | ~1,000 tokens/章 |
| `glossary.md` | 全部关键术语，字母序 + 章节引用 | ~1,500 tokens |
| `patterns.md` | 所有技术、算法、设计模式 | ~2,000 tokens |
| `cheatsheet.md` | 决策表 + 速查规则 | ~1,000 tokens |

关键设计：**章节文件按需加载** —— 没问到的话题不占 Skill 预算。这就是"600 页的书变成几千 token 就能查"的秘密。

---

## 不止是书：适用输入

- 内部文档：ADR、runbook、入职指南，把整个 `docs/` 折成一个 Skill
- 品牌与设计系统：voice guidelines、组件规范
- 研究资料簇：一摞论文 + 自己的笔记，合并成统一 Skill，可增量更新（update/fold-in 模式）
- 规范与标准：RFC、API 契约、合规文档

> *"If you re-open a document often enough to wish you'd memorized it, it's a candidate."*

---

## 隐私与合规立场

- **本地处理**：提取和分析都在本机跑，工具本身不上传任何文件
- **自带书籍**：只处理你已购买/有权阅读的内容
- **产出即笔记**：生成的 Skill 是结构化综合（框架名、定义、要点），不是原文复制（质量规则明确禁止照抄原文段落），性质等同手写学习笔记
- **禁止再分发**：第三方版权书的 Skill 仅供个人使用，不要公开分享

---

## 相关项目对比

| 项目 | Stars | 定位 | 差异 |
|------|-------|------|------|
| **virgiliojr94/book-to-skill** | 18.6k | 书 → Skill 转换器（本笔记主角） | 专注转换，轻量，MIT |
| [vitalysim/the-knowledge-guy](https://github.com/vitalysim/the-knowledge-guy) | 19 | 含 `/book-to-skill` 摄取管道 + `/the-knowledge-guy` 路由/教学（13 种模式：ask/walk/course/compare 等） | 功能全家桶：跨书提问、互动课程网站、测验；但依赖 uv + PyMuPDF，重量级 |
| [simbajigege/book2skills](https://github.com/simbajigege/book2skills) | 128 | 社区协作把经典书手工蒸馏成 Agent Skills | 是"成品 Skills 库"而非转换工具，偏投资/分析类书籍 |
| Abdulrahman0Khaled/B00K2SKILLS | 4 | 书/文档 → Hermes Skills 的 AI 管道 | 目标平台不同（Hermes），星少 |

---

## 与我的关联度分析

**关联度：高** 🔗

| 维度 | 分析 |
|------|------|
| AI 研究 / Agent 生态 | 核心关联。我已在跟踪 herdr（Agent 多路复用）、EigenFlux（Agent 通信）、headroom（上下文压缩）——book-to-skill 属于同一波 **Agent Skills 生态**浪潮，且是其中热度最高的项目之一。它用"按需加载的两层结构"解决上下文预算问题，思路和 headroom 的上下文压缩同源 |
| 全栈开发 | 直接可用：把技术书、RFC、内部文档折成 Skill，开发时随问随答。OpenSquilla 自身就是 SKILL.md 生态，理念可直接借鉴/移植 |
| 量化交易 | 有想象空间：把《海龟交易法则》《金融炼金术》等交易经典转成 Skill，让 Agent 做策略参考时引用书中真实框架（the-knowledge-guy 甚至内置 financial genre profile）。但注意版权：第三方书的 Skill 只能自用 |
| DevOps | runbook、架构文档、合规规范 → Skill，运维问答不再翻文档 |
| 笔记习惯 | 输出结构（glossary/patterns/cheatsheet）本身就是高质量知识管理形态，和 Obsidian 双链笔记理念互补 |

**结论**：不是"看看就好"的项目，而是可以直接进入我 Agent 工作流工具箱的工具。它解决的问题（长文档知识无法被 Agent 高效复用）正是我日常高频遇到的——值得实操验证。

---

## 行动建议

1. **试用**：克隆到 skills 目录（`git clone ... ~/.claude/skills/book-to-skill`），拿一本技术书 PDF 跑一遍，对比直接塞上下文的 token 开销
2. **迁移思路到 OpenSquilla**：OpenSquilla 的 skill 机制也是 SKILL.md 标准，可以参照其"两层结构"设计自己的长文档技能（如把 Obsidian 笔记库的核心框架做成常驻索引 + 按需加载）
3. **跟踪**：项目更新活跃（semver 发布 + changelog），值得持续关注 release
4. **版权注意**：自己买的书转换后自用 OK，勿再分发

---

## 参考链接

- 仓库：<https://github.com/virgiliojr94/book-to-skill>
- 工作原理：<https://github.com/virgiliojr94/book-to-skill/blob/master/docs/how-it-works.md>
- 性能基准：<https://github.com/virgiliojr94/book-to-skill/blob/master/docs/performance.md>
- 安装文档：<https://github.com/virgiliojr94/book-to-skill/blob/master/docs/install.md>
- 相关：<https://github.com/vitalysim/the-knowledge-guy>
