---
title: 14｜星火燎原：从 Claude Code 到行业开放标准
source: https://time.geekbang.org/column/article/952321
author: 黄佳
published: 2026-03-13
created: 2026-06-23
tags:
  - clippings
  - Claude Code
  - Skills
  - 开放标准
  - AAIF
  - AGENTS.md
description: Agent 生态中的 Skills 从产品特性发展成行业开放标准，揭示了声明式、自包含和知识本位三个本质属性，以及从内部 SOP 到行业标准协议的升维。
lark_doc_url: https://my.feishu.cn/docx/EYO1d3jtNoypyDxR1XRcVSoGnCe
---

> 黄佳 · Claude Code 工程化实战

释题：星火燎原。一个 Markdown 文件格式，用百天时间从一个产品特性变成行业开放标准。这不是偶然——它揭示了 AI Agent 生态中，什么样的机制能活下来，什么样的知识能跨越边界。

你好，我是黄佳。前面几讲，我们从结构设计、渐进式披露到高级模式，把 Skills 的工程能力掰开揉碎讲了个透。

但如果我们只停留在"怎么写好一个 SKILL.md"，就会错过一个更大的故事。

2025 年 10 月，Anthropic 在 Claude Code 里上线了 Skills。那时候没人觉得这是什么大事——不过是一个 Markdown 文件里写写指令嘛。

2026 年 2 月，27+ Agent 平台原生支持这个格式。52,000+ Skills 被注册。Top Skills 单个安装量突破 180,000。

从一个产品特性到行业开放标准，Skills 只用了不到百天。

这一讲，我们来回答三个问题：第一，发生了什么；第二，为什么偏偏是 Skills 能出圈；第三，这对你作为 AI 工程师意味着什么。

我们先用一张图来完整复盘这些天里发生的事情。

![](https://static001.geekbang.org/resource/image/88/b7/88dd7ddyy69ef588d01b8e32404775b7.jpg?wh=4615x2405)

整个故事的转折点是 12 月 18 日——不是技术突破，而是一个**战略决策**：Anthropic 选择把 Skills 开放。

在此之前，Skills 是 Claude Code 的竞争优势。开放意味着竞争对手可以免费使用它、甚至用它来对抗你。为什么 Anthropic 愿意这样做？

因为标准的价值大于独占的价值。

独占 Skills，只有 Claude Code 的用户可以用；开放 Skills，所有 Agent 平台上创建的 Skills 都兼容 Claude Code。这是典型的平台经济学——当你的格式成为行业标准，每个人创建的内容都在增强你的生态。就像 USB 标准，Intel 发明了它，但开放后全世界都在用，Intel 反而获益最大。

## 谁在用：四层采纳矩阵

知道了时间线，接下来看采纳范围。截至 2026 年为止的 27+ 平台并不是简单的堆砌数字，它们呈现出清晰的分层结构。

![](https://static001.geekbang.org/resource/image/12/cf/12c71052yy0d9279846d0cfcbd1018cf.jpg?wh=3639x1396)

当 OpenAI 的 Codex CLI 和 Google 的 Antigravity 都支持你的格式时，这已经不是"一家公司的功能"了——它是事实标准。

**Agent Skills 采纳全景图：**

| 层级 | 代表平台 |
|------|----------|
| **创始者** | Anthropic: Claude Code ★、Claude.ai |
| **第一梯队** | OpenAI: Codex CLI、ChatGPT（测试中）；Google: Antigravity、Gemini CLI、Gemini |
| **第二梯队** | Microsoft: VS Code、GitHub Copilot；Block: Goose（原生支持） |
| **第三梯队** | Cursor、Windsurf、Trae（原生支持） |
| **其他** | Manus、Amp、Roo Code、Letta、OpenCode、Kiro CLI、Droid、Kilo |
| **总计** | 27+ 平台 |

与此同时，Anthropic 还在 claude.com/connectors 上线了 Skills 目录，第三方合作伙伴发布了官方 Skill 包：

![](https://static001.geekbang.org/resource/image/10/b1/1000a8e1461yy74e4b98f19916167db1.jpg?wh=3010x1757)

企业级管理方面，Anthropic 的 Team/Enterprise 计划支持管理员集中配置 Skills——控制哪些 Skills 可用，同时让员工自定义自己的工作流。

## Skills 出圈的三个本质属性

刚刚我们了解了"发生了什么"，更重要的问题是"为什么"。为什么是 Skills 出圈，而不是 SubAgents、不是 Hooks、不是 Plugins？

答案藏在 Skills 的三个本质属性里。

**第一，声明式（Declarative）。** Skills 的载体是纯 Markdown 文件——YAML frontmatter 加 Markdown 正文。没有编程语言，没有 import/require，没有编译和构建步骤。这意味着什么？任何能读 Markdown 的系统都能理解一个 Skill。不需要 Python 运行时，不需要 Node.js 环境。Claude 能读，GPT 能读，Gemini 也能读——因为它们都能读 Markdown。

如果 Skills 是用 Python 类定义的（像 LangChain 的 Tool），那每个平台都需要一个 Python 运行时、兼容的 SDK、和特定的加载逻辑。切换平台就等于重写代码。

**第二，自包含（Self-contained）。** 一个 Skill 就是一个文件夹。它不依赖任何外部注册中心，不需要在某个平台注册，不需要安装特定的 runtime，不需要配置 API key，不需要连接外部服务。复制这个文件夹到任何支持 Skills 的 Agent 环境，它就能工作。这就是为什么 Git 是 Skills 的天然分发渠道——`git clone` 就是"安装"。

**第三，知识本位（Knowledge-centric）。** Skills 的价值不在格式——格式只是 Markdown；不在工具——工具是 Agent 自带的；不在运行时——运行时是 Agent 平台提供的。价值在内容本身，在"怎么做某件事"的知识，在可操作的领域智慧。一份好的"如何做代码审查"的 SOP，不管是 Claude 读还是 GPT 读还是 Gemini 读，知识本身都是有价值的。

这三个属性合在一起，天然具备跨平台复用的属性：

- 声明式 → 任何 LLM 都能读
- 自包含 → 任何文件系统都能存
- 知识本位 → 知识的价值不绑定平台

![](https://static001.geekbang.org/resource/image/9e/9d/9e7f59e8007f183ff83e1853e387d89d.jpg?wh=1597x874)

## 为什么 SubAgents 不能出圈

理解了 Skills 为什么能出圈，再看 SubAgents 为什么不能——两者的对比揭示了一个深层的架构原理。

![](https://static001.geekbang.org/resource/image/bc/f9/bc2bd32c8fceba4200aae4cbf9d0b9f9.jpg?wh=3490x1523)

根本原因在于：Skills 封装的是**知识（Knowledge）**，SubAgents 封装的是**行为（Behavior）**。

![](https://static001.geekbang.org/resource/image/55/8f/55848f07b8d6cc2aa951f419ed0cd38f.jpg?wh=3688x1422)

用一个简单的类比来说：Skills 像一本烹饪书，不管你用什么厨房（Claude / GPT / Gemini），只要能读懂食谱，就能做菜，换厨房带上书就行；SubAgents 像一位厨师，厨师的技能绑定在人身上，你不能把"厨师"复制到另一家餐厅的系统里，你能做的是给新厨师一本食谱。

![](https://static001.geekbang.org/resource/image/2f/71/2f8bdef00a03bac25a6dae8645e08371.jpg?wh=3368x1405)

越接近"纯知识"的机制越容易跨平台，越接近"运行时行为"的机制越绑定平台。Skills 是最纯粹的知识封装，所以它出圈了。

## Agentic AI Foundation——AI 时代的 W3C

Skills 出圈的背后，是一个更大的行业趋势：Agent 标准化运动。

2025 年 12 月 9 日，Anthropic、OpenAI 和 Block（原 Square）在 Linux Foundation 下联合成立了 **Agentic AI Foundation (AAIF)**。如果你熟悉 Web 标准的历史，这就是 AI 时代的 W3C——让不同厂商的 Agent 能用同一套标准互操作。

AAIF 有三大创始项目，每个解决 Agent 生态的一个核心问题：

| 项目 | 贡献方 | 解决问题 | 定位 |
|------|--------|----------|------|
| **MCP** | Anthropic | Agent 如何连接工具？ | 工具接口标准 |
| **goose** | Block | Agent 如何落地执行？ | 执行框架参考实现 |
| **AGENTS.md** | OpenAI | Agent 如何理解项目？ | 项目上下文标准 |

**会员层级：**

- **Platinum**: AWS, Anthropic, Block, Bloomberg, Cloudflare, Google, Microsoft, OpenAI
- **Gold**: Cisco, Datadog, Docker, IBM, JetBrains, Oracle, Salesforce, SAP, Shopify, ...
- **共 50+ 企业会员**

Agent Skills 不是 AAIF 的三大创始项目之一——它是 Anthropic 在 AAIF 成立 9 天后独立发布的开放标准。但它与 AAIF 生态密切关联。

MCP 提供工具能力，Skills 提供使用知识——Agent 同时需要两者。goose 原生支持加载 Agent Skills。AGENTS.md 是项目的通用规则和指南，Skills 是特定领域的专业知识——前者是常驻全局上下文（push），后者是按需加载领域知识（pull）。

这里有一个关键发现：AGENTS.md 与 Skills 的关系，完美映射到我们讲过的 CLAUDE.md 与 Skills 的关系（参考[[09｜触类旁通：SKILL.md 结构与触发机制|第 9 讲]]）。

![](https://static001.geekbang.org/resource/image/7d/19/7de335f201d02633dd156c4bc5477419.jpg?wh=3548x1260)

这不是巧合。Claude Code 的架构设计就是这套标准的原型。你学的不是一个产品的用法，而是行业标准的第一手实践。

## skills.sh——Skills 的 npm

有了标准，自然就需要分发机制。2026 年 1 月 20 日，Vercel 推出了 skills.sh——Agent Skills 的包管理器和目录服务。如果 Agent Skills 是"代码包"，那 skills.sh 就是"npm"。

```bash
# 安装一个 Skill
npx skills add vercel-labs/next-app-creation

# 交互式搜索和发现 Skills
npx skills find

# 更新本地已安装的 Skills
npx skills update
```

生态数据如下表所示：

![](https://static001.geekbang.org/resource/image/c1/37/c14303cd3214f6aaed7853f965dcd037.jpg?wh=2500x1252)

skills.sh 的核心理念是把 Agent 推理与执行分离——给 Agent 一组预定义的、可审计的命令，而不是让它动态生成 shell 逻辑。

**传统方式（Agent 自由执行）：**

```
Agent → 推理 "我需要构建这个项目" → 自己生成 npm build 命令 → 执行
问题：不可预测、不可审计
```

**Skills 方式（Agent 调用预定义 Skill）：**

```
Agent → 触发 "build-project" Skill → Skill 内置标准构建流程 → 执行
优势：可预测、可审计、可复用
```

## Push vs Pull 大辩论：AGENTS.md vs Skills

有了行业标准和分发机制，一个自然的问题浮出水面：既然 AGENTS.md（Push）和 Skills（Pull）是互补的，那什么时候该用哪个？

2026 年 2 月 2 日，Vercel 发布了一篇引发广泛讨论的博文——**AGENTS.md outperforms skills in our agent evals**。他们对 Build、Lint、Test 三类任务做了严格的对照实验。

![](https://static001.geekbang.org/resource/image/06/25/06a721d281360f37525yy2a85e0b2d25.jpg?wh=3464x1872)

表面结论是，一个静态 Markdown 文件打败了按需检索的 Skill 系统。但这个结论需要更深层的分析。

![](https://static001.geekbang.org/resource/image/e4/e1/e4bce88fbf88c97c0yy2fd90c0c0bbe1.jpg?wh=2191x1132)

这个权衡，你在[[09｜触类旁通：SKILL.md 结构与触发机制|第 9 讲]]就学过了——CLAUDE.md vs Skills 就是同一个 Push vs Pull 的设计决策。

![](https://static001.geekbang.org/resource/image/dd/b8/dd76331e66a5f2e2b6eb739a968d52b8.jpg?wh=1374x1275)

Vercel 的测试场景是 Build / Lint / Test——这些是每个项目都需要的高频操作。对于高频操作，Push 模型当然更好：把构建指南放在 AGENTS.md 里，Agent 每次都能看到。

但如果你的场景是 15 个不同领域的 Skills（安全审查、API 文档、数据分析、性能优化……），每次只需要其中 1-2 个，全部 Push 进上下文等于 120KB+ 的 token 消耗。那 Pull 模型就是唯一可行的选择。

**场景矩阵：**

| | 知识量少（<8KB） | 知识量大（>50KB） |
|---|---|---|
| **每次都需要** | Push (AGENTS.md) | Push + 压缩 |
| **偶尔才需要** | 都行 | Pull (Skills) |

正确的工程决策不是二选一，而是组合使用。AGENTS.md / CLAUDE.md 放每次都需要的、少量的项目规则（构建命令、测试命令、代码风格）；Skills 放特定场景才需要的、详细的领域知识（安全审查清单、API 文档模板、财务分析 SOP）。行业共识正在形成——AGENTS.md 和 Skills 是互补的，不是竞争的。

![](https://static001.geekbang.org/resource/image/9d/ce/9dfa6f632d066e00a594747d76022bce.jpg?wh=2339x1013)

## 企业本体论升维：从内部 SOP 到行业标准协议

在[[09｜触类旁通：SKILL.md 结构与触发机制|第 9 讲]]中，我们建立了一个企业本体论映射：Skills 等于企业的 SOP / 操作手册。这个映射在企业内部是准确的——Skills 就是一家企业的标准操作程序，指导内部员工（Agent）如何执行特定任务。

但 Skills 出圈后，这个映射需要升维。

![](https://static001.geekbang.org/resource/image/ba/f8/ba105de4586ff4aa21eaa2cb996da8f8.jpg?wh=2740x1250)

如果把这个升维后的映射完整展开，会形成一幅更完整的对照图。

![](https://static001.geekbang.org/resource/image/c2/5d/c2a7dfbdc6a419420267d37e7265ab5d.jpg?wh=2306x1658)

这个升维对你作为 AI 工程师有三个实际意义。

**第一，学一次，到处用。** 你在 Claude Code 中精心设计的 Skill——精准的 description、合理的分层、强约束措辞——可以直接用在 Cursor、Copilot、Codex CLI 中。因为它就是一个符合行业标准的 Markdown 文件。

**第二，你的 Skill 工程能力是行业级能力。** 学会如何设计好的 SKILL.md 不再是"如何用好 Claude Code"的必要条件，而是"如何为 AI Agent 生态编写标准化知识包"的问题。这就像 2015 年学 REST API 设计——不是学某个框架，而是学行业通用技能。

**第三，Skills 是 AI 时代的 package.json。** 就像 package.json 定义了一个 Node.js 项目的依赖和行为，SKILL.md 正在成为定义 AI Agent 能力的标准格式。不同的是，package.json 是给机器读的 JSON，而 SKILL.md 是给 AI 读的 Markdown——因为 AI 的"运行时"就是自然语言。

## 从 Skills 出圈看 AI 工程的趋势

最后，让我们从 Skills 出圈这个事件中，提取三个更大的趋势。

**趋势一：从命令式到声明式。** 

```
传统软件：代码（命令式） → 编译器 → 机器执行
AI Agent：知识（声明式） → LLM    → Agent 执行
```

**趋势二：标准化大于平台锁定。** AAIF 的成立标志着 AI Agent 行业从平台战争走向标准协作。Anthropic 开放 MCP 和 Skills，OpenAI 开放 AGENTS.md，Block 开放 goose——竞争对手在工具层竞争，在标准层合作。

**趋势三：知识复用成为新重心。** 当知识可以脱离具体系统而存在，并通过标准协议在不同 Agent 之间流通时，软件生态的重心便开始从代码复用转向知识复用。

![](https://static001.geekbang.org/resource/image/47/ea/474d46d2b4bc376931d271234cc807ea.jpg?wh=2388x1127)

## 总结

如果我们把这段历史抽离时间线来看，它揭示的其实不是一个产品成功案例，而是一种"知识形态的进化"。在传统软件时代，能力的边界由代码决定，代码必须依附于某种语言、某种运行时、某种平台生态，因此能力天然带有锁定属性。

而在 Agent 时代，运行时变成了大模型，执行逻辑不再需要通过编译器翻译成机器指令，而是通过语言理解直接转化为行动。于是，知识第一次具备了"可执行性"。当一段结构化的自然语言可以稳定地驱动行为，它就不再只是说明书，而成为一种可以迁移、可以复用、可以标准化的能力单元。

Skills 的出圈并非因为 Anthropic 推广得足够快，而是因为它踩中了这一结构性转折点——它把"领域经验"从平台实现中剥离出来，使其成为独立于运行时的资产。当知识可以脱离具体系统而存在，并通过标准协议在不同 Agent 之间流通时，软件生态的重心便开始从代码复用转向知识复用。

这种变化意味着，未来竞争的焦点不再只是模型参数规模或工具调用效率，而是谁能将复杂领域智慧提炼为可迁移的知识结构。这一百天的进展只是一个缩影，它标志着 **AI 工程正在从"构建系统"转向"组织知识"，从编写逻辑转向设计认知。**

## 思考题

1. **可移植性自测**：拿你在本课程中创建的任何一个 Skill（比如 api-generating 的 SKILL.md），假设你要把它迁移到 Cursor 或 GitHub Copilot 中使用——哪些部分可以直接复用？哪些需要修改？`allowed-tools`、`context: fork` 这些 Claude Code 特有的 frontmatter 字段在其他平台怎么处理？

2. **Push vs Pull 设计决策**：你的项目有 8 个 Skills，总知识量约 80KB。根据 Vercel 的实验数据和课程讲的"500 行法则"，你如何设计 AGENTS.md（或 CLAUDE.md）与 Skills 的分工？哪些放 Push？哪些放 Pull？画出你的知识分配方案。

3. **标准化价值分析**：假设你是一家 200 人技术团队的架构师。你的团队在 Claude Code、Cursor 和 GitHub Copilot 三个平台上都有开发者。用这一讲的"企业本体论升维"框架，设计一套跨平台的 Skill 资产管理策略。

4. **批判性思考**：Vercel 实验发现 Skills "56% 的时间不被触发"。结合[[09｜触类旁通：SKILL.md 结构与触发机制|第 9 讲]]关于 description 设计的知识和强约束措辞的经验，你认为这个问题的根因是什么？是 Skills 机制本身的缺陷，还是 Skill 设计的问题？提出你的改进方案。

## 我的思考

### 1. 可移植性

相关资源：

- [anthropics/skills - skill-creator](https://github.com/anthropics/skills/tree/main/skills/skill-creator)
- [Agent Skills 规范](https://agentskills.io/specification)
- [Claude Code Skills 文档](https://code.claude.com/docs/en/skills#extend-claude-with-skills)

### 2. Push vs Pull

我的看法是：**大部分东西已经没必要放在 Push 了。** Pull 把主动触发拿掉，几乎所有上下文都应该用按需加载的方式加载。分项目的 CLAUDE.md 本质上不也是按项目加载吗？它本身就是一种 Pull——只是粒度在项目级别。

### 3. 跨平台 Skill 策略

Skill 要有**基础版**和**定制版**。基础版只包含标准内容，定制版围绕不同 Code 平台 + 模型做适配。原因有两个：

- **Frontmatter 差异**：Skill 的某些头部信息只有 Claude Code 能识别。其他平台未来大概率也会出现类似的定制扩展——这是行业趋势。
- **模型理解差异**：同样的 Markdown 描述，不同模型的理解和执行差异很大，所以必然得定制。

我的实践经验是：**直接上最好的模型（Opus），先把一个 Skill 的闭环效果做出来**，拿到一个最简洁、能体现核心价值的版本。然后用国产模型（如 GLM）大量实验——拿 Opus 跑出来的 case + skill，让 GLM 自己不断迭代，直到新的 skill 能趋近期望效果。

> ⚠️ 不建议拿国产模型做探索性任务，这是在浪费生命。

### 4. 自主加载是设计缺陷吗？

**在当下，这就是设计本身的缺陷。** 这个设计太超前了。

当前的使用场景，有按需加载就好——指定 skill 调用没有任何问题。举一个实际例子：我们在一个仓库装了 superpowers，但实际上不是每次开发需求都很大、都要走一整套流程。大多数时候用 Vibecoding 就够了。但你装了之后就没办法，它几乎每次都进头脑风暴，导致不得不在 CLAUDE.md 里追加区分场景使用 superpowers 的逻辑。而这个问题，在需要的时候直接 `/superpowers-xx` 其实就好了。

更不用说用"傻模型"时，superpowers 可能没走 worktree，直接在 main 上提交代码之类的风险。**明明写死调用、渐进加载就好。**

自主加载这件事是在未来的——当我们完全不介入、全部交给 AI 来，生成的代码日几十万、上百万。到那时候人其实介入不了，由人来按需判断会严重影响 AI 发挥的速度。这时就不得不交给 AI 来自主决策。

但**我们还没到这个阶段**。现在这么玩，就是吃着全自动的亏、做着半自动的事。

### 5. Skill 依赖管理：会不会出现 Skill 版的 package.json？

有了 SkillHub 之后，自然会想：是不是需要一个类似 Maven 的 pom.xml 或 npm 的 package.json 来统一管理 Skill？多个项目按需引用，而不是每次都要拷贝。

Skills 生态目前还没走到那一步——而且**有可能永远不会完全复制 npm/Maven 的模式**。原因是 Skills 的"自包含 + 纯文本"特性，让它天然适合一种比 npm 更轻量的依赖管理方式。

**npm 的包管理三件套 vs Skills：**

| | npm | Skills |
|---|---|---|
| 声明文件 | `package.json` ← "我依赖哪些包" | 不需要同等级别 |
| 仓库 | npm registry ← 中心化包仓库 | Git 仓库天然就是 registry |
| 本地安装 | `node_modules/` ← 实际代码 | 文件夹拷贝 / submodule |

**关键区别：Skills 之间没有代码依赖关系。** npm 的 `package.json` 之所以复杂，是因为要解决**依赖地狱**（A 依赖 B@1.0，C 依赖 B@2.0）。Skills 没有依赖地狱——一个 Skill 不 import 另一个 Skill。所以不需要同等复杂度的机制。

**"每次都要拷贝"的痛点是真实的，但 Git 已经解决了。** 不需要发明新的包管理器：

- **Git Submodule**：`git submodule add <skill-repo-url> .claude/skills/<name>`
- **符号链接（Monorepo）**：多个项目共享同一个 skill 目录
- **Claude Code Plugins 机制**：如果内置支持 `claude plugin add`，就类似 `go get`

**三阶段演进预测：**

```
阶段 1（当前）：
  git clone / 手动拷贝 / submodule
  → 够用，但原始

阶段 2（近期可能）：
  claude plugin add github:author/skill-name
  → 类似 "go get"，基于 Git 仓库
  → 不需要中心化 registry

阶段 3（中远期可能）：
  skills.yaml 声明依赖 + claude install
  → 类似 package.json 但更轻量
  → 没有依赖树，不需要 lock 文件
```

**阶段 3 可能永远不会出现**，因为 Git submodule + Plugin 机制可能已经足够好。npm 模式是为了解决代码级别的编译时依赖——Skills 是纯文本 prompt，不需要编译、不需要版本锁定、不需要 tree-shaking。**过度工程化反而会破坏 Skills 的简洁性。**

## 下一讲预告

Skills 专题到此结束。从第 9 讲的基础结构到第 14 讲的行业出圈，我们完成了 Skills 系统的完整旅程。

下一讲，我们将进入一个全新的领域——**Hooks（事件驱动钩子）**。如果说 Skills 是"什么时候用什么知识"，那 Hooks 就是"什么事件触发什么动作"。它是 Claude Code 自动化能力的最后一块拼图。

欢迎你在留言区和我交流讨论。如果这一讲对你有启发，别忘了分享给身边更多朋友。
