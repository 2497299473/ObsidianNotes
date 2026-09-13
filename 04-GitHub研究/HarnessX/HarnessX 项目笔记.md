---
lark_doc_token: KoCwd9MzooME2sxwyc9cD5pinqd
lark_doc_url: https://my.feishu.cn/docx/KoCwd9MzooME2sxwyc9cD5pinqd
---
# HarnessX 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/Darwin-Agent/HarnessX
> - **所属组织**：Darwin-Agent（org，非小米官方组织）
> - **协议**：MIT License
> - **主要语言**：Python 3.12+
> - **Stars / Forks**：442+ / 52
> - **创建 / 最近推送**：2026-04-30 / 2026-07-29（状态：Beta，v0.1.0）
> - **定位**：Agent harness 铸造厂——从可复用 processor 和 bundle 锻造任意数量的 agent harness，与任意模型配对，并通过训练使其进化
> - **记录日期**：2026-08-24

---

## 一句话定位

**让 Agent 的"外壳/脚手架"（harness）也能像模型一样：可组合（Compose）、自适应（Adapt）、通过训练自我进化（Evolve）——实现模型与 harness 的协同进化（co-evolution），让小模型也能大幅提升跑流程能力。** 核心一句话：`agent = model.agentic(harness)`。

> 📌 用户原话：小米有个开源 harnessX，就是 llm 和 agent 一起训练，可以让小模型跑流程提高挺多。
> ⚠️ 注意：仓库挂在 **Darwin-Agent** 组织下（org `Darwin-Agent`），不是 Xiaomi 官方 GitHub org。报告源自小米团队，但代码托管在该 org 下。别和小米自家的 MiMo-Hermes / MiMo Code 系列混淆——那是模型权重，HarnessX 是训练/编排框架。

---

## 🧩 核心思想：harness 也是可训练的"第一公民"

传统思路：模型是唯一可训练对象，harness（提示词、工具编排、记忆回路）是人工搭的固定脚手架。

**HarnessX 改变这一点**：把 harness 视为和模型对等的可进化组件。

```
agent = model.agentic(harness)
        └─────── 一条命令把模型 + harness 绑定 ───────┘
                 · ModelConfig   模型选择 / 路由 / 回退 / 按角色分配
                 · HarnessConfig 完整行为管线（工具、记忆、处理器、轨迹、沙箱）
```

- **eXtensible Behavior Composition**（X 的来源）：任何行为都是 **Processor**，用 `|` 组合成 9 维行为管线
- 每种可训练组件都可拆成：`Compose → Adapt → Evolve` 三步循环

| 阶段 | 机制 | 产出 |
|------|------|------|
| 🧩 **Compose** | 9 维行为管线，任意行为 = Processor，`\|` 组合 | 结构化、可复制的 harness 配置 |
| ⚙️ **Adapt** | harness 观察自身性能，自动搜索最优配置 | 自适应的最优 harness |
| 🚀 **Evolve** | 每次运行产生带奖励标注的轨迹，喂给 SFT / RL | 可训练的强 harness |

---

## 🚀 关键突破：Cross-harness GRPO

模型侧用的是 **GRPO**（DeepSeek-R1 同款强化学习算法，Group Relative Policy Optimization）。

- **Cross-harness GRPO**：harness 自进化过程中产生的执行数据，直接反向用于训练模型
- **"一鱼多吃"**：harness 进化跑出来的轨迹，天然就是带奖励标注的 RL 训练数据，无需额外采集
- 协同进化（模型 + harness 一起练）比单独练模型，**平均多 +4.7% 性能**

---

## 📊 效果数据（小模型受益最大）

| 配置 | 性能 | 说明 |
|------|------|------|
| 9B 小模型基线 | 33.97% | 未进化 |
| + harness 进化 | 41.67% | 只进化 harness（壳） |
| + harness 进化 + 模型进化 | **55.77%** | 协同进化，**相对提升 +64%** |

- 在所有组合里做了 **15 轮**自我迭代
- 联动过的模型：Claude 4.6 Sonnet、GPT-5.4、Qwen 3.5-9B 等
- 评估基准：GAIA、SWE-bench、WebShop 等
- （具体单基准分数待补充，README 主打 GAIA/SWE-bench/WebShop 三件套）

---

## 🔍 架构分层（README 摘要）

- **HarnessConfig + Processor 管线**：行为拆成可复用处理器，支持管线式组合（`|`）
- **ModelConfig**：模型路由、回退、按角色/任务分配不同模型
- **轨迹 / 沙箱层**：运行产生轨迹，沙箱托底
- **训练桥接**：轨迹 → SFT / RL（GRPO）数据管道
- **Lab UI**：可视化窗口（`hx lab`，localhost:8000）

---

## ⚙️ 快速上手

```bash
# 一键安装（Linux/macOS：curl；Windows：Powershell 脚本）
curl -sSf https://raw.githubusercontent.com/Darwin-Agent/HarnessX/main/scripts/install.sh | bash

# 使用
hx "Research 2026 AI agent trends and write a structured report"
hx lab   # 打开 Lab UI (localhost:8000)
```

---

## 🧭 同赛道参照坐标

HarnessX 处于 **Agent 训练/自进化框架** 这一档，和上下文/记忆基础设施不是一回事：

| 项目 | 层级 | 差异 |
|------|------|------|
| **HarnessX**（Darwin-Agent） | Agent 编排/训练框架 | 模型 + harness 协同进化，GRPO，让小模型强跑流程 |
| headroom（headroomlabs-ai） | 上下文压缩层 | CCR 可逆压缩砍 token，不在训练维度 |
| CodeGraph（colbymchenry） | 代码知识图谱 | 本地预索引，非训练框架 |
| mnemosyne（mnemosyne-oss） | Agent 记忆层 | 零云 SQLite 记忆，非训练框架 |

> 一句话：HarnessX 卡的是"**Agent 怎么被训练出来**"，记忆/压缩层卡的是"**Agent 跑起来怎么喂料**"，上下游关系，不冲突。

---

## 📝 备注 / 观察

- **Status：Beta / v0.1.0**，属于早期项目，442 stars 不算高，但有真实基准背书
- 代码托管组织名为 **Darwin-Agent**（"达尔文"暗合"进化/自然选择"之意，很贴合 harness 自进化叙事）
- **跨模型能力突出**：同一个 harness 逻辑可挂 Claude / GPT / Qwen，天然适配多模型路由场景
- 小米的 AI 开源矩阵里，HarnessX（训练框架）与 MiMo-Hermes（模型权重）是不同层次的东西，注意区分
- 潜在关注点：442 stars / 52 forks 体量尚小，需留意后续维护活跃度与 release 节奏

---

## 🔗 相关链接

- 仓库：https://github.com/Darwin-Agent/HarnessX
- 知乎解读（收录参考）：《Claude 和 Manus 还要人工搭框架？小米直接让 Agent 自我进化》
- 关联项目（小米系，注意区分）：MiMo-Hermes / MiMo Code（模型权重，非本框架）
