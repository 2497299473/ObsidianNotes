---
title: Socratopia 研究笔记
created: 2026-08-02
tags:
  - AI工具
  - 苏格拉底式学习
  - AI教育
  - 量化
  - 全栈
  - DevOps
description: Socratopia：AI 苏格拉底式学习平台研究 + 书库 527 本书四大领域推荐
lark_doc_url: https://my.feishu.cn/docx/J5eVdg7jXo0AkdxlEUlchWtEnmf
---

# Socratopia

**日期**: 2026-08-02
**来源**: https://www.socratopia.app/
**开发者**: SocraLab LLC（macOS · Windows 桌面应用，签名公证）

## 是什么

Socratopia 是一个 **AI 苏格拉底式学习平台**：不直接给答案，而是像苏格拉底一样不断提问、引导你自己推理出结论。核心由两部分组成：

1. **Socratopia Library（在线书库）**：300+ 题材的 AI 原生教材，浏览器免费阅读，无需注册。实测数据为 **527 本书、10415 章，全部免费（0 Credits）**，中英各半（EN 266 / ZH 261）。
2. **桌面 App（AI 伴读）**：把书载入 App 后，AI 以苏格拉底式对话教学；也支持上传自己的 PDF / ePub / Markdown——"PDF 进，课程出"。

## 核心机制

- **苏格拉底式对话**：AI 伴读问一个问题、再问一个，直到想法在你脑子里自己建立起来
- **本地优先**：数据不出设备
- **跨学科**：有教材就有课，App 自动记录进度、生成笔记和闪卡
- **主题合集**：如 "AI Science"（三卷：从感知机到 GPT 到生成式 AI）、"Time Traveler's Tech Tree"（穿越古代搞工业化）等叙事型课程包

## 定价

- App 免费下载，每月送 **100 万 tokens** 的 AI 教学额度
- 订阅（Live-In Tutor Plan）：App 内 Settings 查看定价并订阅，Stripe 支付，支持支付宝/微信
- 取消：App 内一键取消，本期结束前仍可用；退款一般不退，先体验再付费
- 营销话术："$0.4/小时 vs 私教 $40/小时"、"Aristotle taught one prince. Now he teaches anyone."

## 书库数据概览（2026-08-02 抓取）

| 分类 | 数量 | 分类 | 数量 |
|------|------|------|------|
| language | 82 | physics | 36 |
| cs | 62 | psychology | 32 |
| socsci | 59 | art | 28 |
| business | 58 | biology | 16 |
| humanities | 52 | health | 15 |
| math | 45 | engineering | 14 |
| chemistry | 12 | history | 11 |
| growth | 5 | | |

⚠️ 教材为 AI 生成，官方声明不保证准确性/完整性——**交易、金融类内容务必交叉验证**。

## 与我的匹配度

| 领域 | 相关度 | 说明 |
|------|--------|------|
| AI 研究 | ⭐⭐⭐⭐⭐ | AI Science 三卷从感知机推导到前沿架构；ML 书每个算法带推导；统计思维书把 p 值滥用讲透了 |
| 量化交易 | ⭐⭐⭐⭐⭐ | 有专门的 Quantitative Finance 实务教程 + 因子投资两卷 + 计量经济学，配套数学地基（概率/线代/微积分） |
| 全栈开发 | ⭐⭐⭐⭐ | Modern Web Development 从 HTTP 讲起、JS/TS、数据库、API 设计都有 |
| DevOps | ⭐⭐⭐⭐ | Cloud Native & Platform Engineering、分布式系统、计算机网络、SRE/SLO 数学 |

## 四大领域推荐书单

### 🤖 AI 研究

| 书 | 章数 | 语言 | 亮点 |
|----|------|------|------|
| **AI Science I: Neural Networks and the Transformer** | 26 | EN | 从感知机到 Transformer，每个架构亲手推导；含语言模型数学：自回归链式法则、交叉熵=MLE=KL、BPE 分词、解码策略 |
| **AI Science II: Large Language Models** | 24 | EN | LLM 专项：从第一性原理推导 MDP、Bellman 方程、REINFORCE、RLHF 相关基础 |
| **AI Science III: Generative AI and Frontier Architectures** | 21 | EN | 生成式 AI 与前沿架构收尾卷 |
| **Machine Learning: From Data to Intelligence** | 26 | EN | 每个算法都有可跟进的推导（如 Ridge 回归的贝叶斯先验视角）；两个贯穿项目：房价预测 RMSE 从线性模型到 XGBoost 降 38%，IMDb 情感分类从 BoW 85% 到 BERT 96.1%——十年 NLP 史七章讲完 |
| **Statistical Thinking in the Age of AI** | 27 | EN | 开篇就是 Semmelweis 手卫生被拒的故事；p 值章节引用 2016 ASA 声明批判科学界误用；把神经网络=最大似然估计、正则化=贝叶斯先验、LLM 幻觉=从不约束真理的分布采样；最难也最值的是因果推断章（辛普森悖论、DAG） |
| **Information Theory: From Shannon to AI** | 22 | EN | 信息论到 AI 的桥梁，量化研究的地基之一 |
| **How AI Actually Works — A Curious Reader's Guide to the Machinery** | 25 | EN | 面向好奇读者的 AI 机制科普，偏轻松 |
| **AI Engineering: Building Production Systems with LLMs** | 29 | EN | LLM 生产系统落地，工程向 |
| **写给经济学家的 AI 科学课（中文）** | 76 | ZH | 巨厚中文版，从经济学视角讲 AI 科学 |
| **Getting to Know AI** | 16 | EN | 最快入门，适合先试水 |

**优先推荐**：AI Science 三卷（体系化）+ Machine Learning（推导型）。先拿《Getting to Know AI》16 章试水苏格拉底对话模式。

### 📈 量化交易

| 书 | 章数 | 语言 | 亮点 |
|----|------|------|------|
| **Quantitative Finance（量化金融：实务派教程）** | 31 | EN | Practitioner's Finance Series 收官卷，数学引擎室：随机微积分、Black-Scholes（Feynman-Kac）、波动率曲面、Heston/SABR/Bates、跳跃与 Lévy 过程、Vasicek→BGM 利率模型、copulas 与 2008 高斯 copula 失败、Monte Carlo、校准、最优执行、金融计量、EVT 风险度量、**López de Prado 的 ML in finance 纪律**；200+ 数值例子 + 五个真实失败复盘（1987 组合保险、1998 LTCM、2008 高斯 copula、2020 国债基差、2022 UK LDI）。核心主张：每个模型只是报价语言/信号生成器/优化工具，**绝不是真理** |
| **Factor Investing I: The Logic and Evidence of Factors** | 22 | EN | 把每个因子当"待审问的主张"而非"可买的产品"：三栏怀疑者账簿（经济故事/证据/反方证据），教你手算超额收益、Sharpe、多空价差、alpha/beta、t 统计量；复制危机是全书脊柱 |
| **Factor Investing II: Building and Testing Factor Portfolios** | 24 | EN | 卷一审问完的因子如何构建组合 + 实证陷阱 |
| **The Investment Analyst's Foundation 系列（7 卷）** | 16-21/卷 | EN | 卷一职业伦理 → 卷二定量方法 → 卷三经济学 → 卷四财报分析 → 卷五公司金融与股权 → 卷六固定收益与衍生品 → 卷七组合管理/另类/行为金融。CFA 体系化覆盖 |
| **Econometrics I: Regression and Statistical Inference** | 13 | EN | 回归与统计推断 |
| **Econometrics II: Causal Inference** | 12 | EN | 因果推断，量化策略验证的关键 |
| **Probability and Statistics: A Foundation for Reasoning Under Uncertainty** | 22 | EN | 不确定性推理的数学地基 |
| **Linear Algebra: From Foundations to the Frontiers of AI** | 25 | EN | 线代地基，AI 与量化的公共基础 |
| **The Cognitive Science of Investing** | 23 | EN | 投资认知科学，行为金融向 |
| **Calculus: From Cannonballs to Gradients** | 27 | EN | 微积分地基，从直观到梯度 |

**优先推荐**：Quantitative Finance（直接对口）+ Factor Investing 两卷（因子逻辑与实证纪律）+ Statistical Thinking in the Age of AI（统计思维底座）。

### 🌐 全栈开发

| 书 | 章数 | 语言 | 亮点 |
|----|------|------|------|
| **Modern Web Development: Full-Stack from HTTP Up** | 29 | EN | 从 HTTP 语义（动词/状态码/缓存/条件请求）讲起，到浏览器渲染管线（parser/layout/paint/compositor）、Fetch/FormData/URL，再到全栈 |
| **JavaScript & TypeScript: The Language of the Web** | 35 | EN | Web 语言专项，35 章很全 |
| **Python Programming in the AI Era** | 39 | EN | AI 时代的 Python，39 章巨制 |
| **SQL & Thinking in Data** | 32 | EN | SQL + 数据思维 |
| **Database Systems: Design, Tradeoffs, and Why They Work the Way They Do** | 23 | EN | 数据库设计权衡，不只是用法 |
| **Designing APIs and Distributed Communication** | 29 | EN | API 设计与分布式通信 |
| **Web Pages from Zero 系列（HTML/CSS/JS 三卷，中文）** | 15-16/卷 | ZH | 中文零基础前端三件套 |

**优先推荐**：Modern Web Development（从 HTTP 到全栈的完整链路）+ JS/TS（与你的 React/TS 栈直接相关）。

### 🛠 DevOps

| 书 | 章数 | 语言 | 亮点 |
|----|------|------|------|
| **Cloud Native & Platform Engineering** | 30 | EN | 不是工具目录而是学科思考：平台团队"拥有什么/暴露什么/拒绝拥有什么"；三个工程师贯穿全书（50 人 fintech 的 IDP、B2B SaaS 的 SRE、多云顾问）；URL 缩短器用 5 种方式部署做对比（裸 VM→Docker Compose→K8s→Lambda→Workers）；**SLO 数学是真的**：多窗口多燃烧率代数 B = target_pct × 720 / W 推导出 14.4/6/1 燃烧率阈值；贯穿全书 "The Six Platform Lies LLMs Tell"（AI 生成平台代码的六大谎言）第 30 章+附录 D 汇成检查清单；2026 内容：GPU 调度/MIG 切片、多租户 LLM 推理经济学、AI agent 管基础设施、FinOps |
| **Distributed Systems** | 30 | EN | 分布式系统全景 |
| **Computer Networks: From Cable to Cloud to Edge** | 31 | EN | 网络从物理层到边缘 |
| **Operating Systems: The Abstraction Design Approach** | 26 | EN | 操作系统抽象设计 |
| **Security Engineering in the AI Era** | 31 | EN | AI 时代安全工程 |
| **System Design Thinking: How Large Software Systems Work** | 28 | EN | 系统设计思维，面试友好 |
| **Concurrent and Parallel Programming** | 32 | EN | 并发与并行编程 |
| **Six Superpowers of the AI Era 系列（6 卷）** | 14-16/卷 | EN | 卷I Git/GitHub、卷II 开源 LLM 与 Ollama、卷III 命令行、卷IV 服务器与部署、卷V API 与自动化、卷VI 第二大脑——DevOps 快速补齐 |

**优先推荐**：Cloud Native & Platform Engineering（直接对口 DevOps，且 SLO 数学 + LLM 平台谎言清单很实用）。

## 探索路径

1. **浏览器逛书库**（零成本）→ socratopia.app/library，挑一本熟主题翻几章感受文风
2. **下载桌面 App**，用每月 100 万 tokens 试核心体验 → 建议拿一本**已懂 60%** 的书开课，检验是真加深理解还是换个方式考你
3. **试上传自己的资料** → 如量化策略笔记或论文 PDF，看"PDF 进，课程出"的效果
4. **满意再订阅** → 先刷完免费额度，App 内 Settings 订阅（支持支付宝/微信）

## 免费使用策略（2026-08-02 追加）

> 目标：抛开每月 100 万 tokens，靠官方机制把免费额度拉长。

### 官方确认的免费途径

1. **推荐机制（Referrals）** — 条款原文："Additional tokens can be earned through referrals and activities"。App 内 Settings/Account 生成专属邀请链接，朋友注册后双方得奖励 tokens。⚠️ 官方禁止多账号刷推荐，会被封。
2. **应用内活动（Activities）** — 签到/完成章节/分享等任务，App 内 Account/奖励中心查看明细。
3. **奖励 tokens 不过期** — FAQ 原文："Reward tokens earned through referrals and activities are independent of your subscription and do not expire when a subscription ends"。推荐+活动攒的 token 永久累积，不受订阅状态影响 → **长期白嫖的核心是持续攒奖励 tokens**。

### 零成本使用技巧

- **浏览器免费读全书**：书库所有书浏览器免账号阅读，只有 AI 对话消耗 tokens → 第一遍阅读走浏览器，AI 对话只留给难点章节
- **100 万 tokens 用在刀刃上**：只对卡住的推导/想深挖的概念开苏格拉底对话
- **本地导入资料**：PDF/ePub/Markdown 导入不额外收费（对话按套餐计费），量化笔记可转学习材料

### 注意事项

- 旧版 FAQ 提到过 "Library Credit（载入 App 需 1 积分，可任务/礼物获取）"，当前版已改口为"书免费载入、无单独书费"——机制可能调整过，以 App 内实际为准；若需 Credits，优先做任务攒积分
- 推荐/活动攒的 token 是"免费弹药库"，配合浏览器阅读可长期免费用

## 结论

对 AI 研究 + 量化双主线价值最高（AI Science 三卷 + Quantitative Finance + Factor Investing 都是直接对口的高质量内容，且推导型教学风格适合吃透原理）；全栈/DevOps 是加分项。全部书免费读、每月 100 万 tokens 免费额度，**先零成本试水，重点验证苏格拉底对话是否真的比自读/视频更能加深理解**。交易类知识务必交叉验证。
