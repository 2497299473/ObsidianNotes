---
lark_doc_token: (未生成飞书文档)
lark_doc_url: https://my.feishu.cn/sheet/ (未生成飞书文档)
---
# Jev 决策范式与 TokenRhythm 项目笔记

> [!info] 项目信息
> - **记录来源**：[[Clippings/一文读懂 Jev 决策范式：不用逐 Token 生成，又快又准做决策打分]]（微信公众号「基元律动」@2026-10-14）
> - **仓库 / 主页**
>   - 代码与示例：https://github.com/TokenRhythm/NeoHorse （分支 `jev`）
>   - 模型主页：https://huggingface.co/TokenRhythm/NeoHorse-Jev-4B
>   - 部署说明：https://huggingface.co/TokenRhythm/NeoHorse-Jevalmdash]ev
> - **协议**：未公开说明（MIT 待确认）
> - **主要语言**：Python（推理源码）、LLM 权重（NeoHorse-1-4B 蒸馏）
> - **Stars**：~442（组织 TokenRhythm，2026-08-24 记录 HarnessX 时的组织状态，需复核）
> - **创建 / 测评公布**：2026-09-24（评测公布）~ 2026-10-14（文章发布）
> - **定位**：基于 LLM 的**结构化决策调用**（Jev）——完 input 后直接返回候选得分/选择/概率，省去逐 token 生成答案文本，用于频繁、答案范围明确的业务判断。
> - **记录日期**：2026-10-14

---

## 一句话定位

**把「理解类语义判断」做成可被程序调用的打分原语**：不用逐 token 生成答案文本，模型在输入计算后直接给出候选得分/选择/概率。适合「催发货=否进入订单查询流程」「这条工单属于哪类」「这条消息的紧急程度」等判断任务。

> 与生成式调用相比：生成式 = 理解 → 逐 token 生成文本（慢）；Jev = 理解 → 计算候选分 → 返回选择/概率（快）。

---

## 🎯 三种输出格式

| 格式 | 回答 | 适用场景 |
|------|------|----------|
| **Noul** | 「是不是」，返回 0~1 概率 | 是非判断，如「用户是否在催促发货？」 |
| **Choice** | 「选哪一个」，返回候选类别 + 概率分布 | 消息分流/工单分类，如「属于发货咨询/开票申请/退货诉求哪类？」 |
| **Score** | 「到什么程度」，返回有序等级 + 综合分值 | 程度评分，如「催促强度：普通/催促/加急」 |

---

## 🏗️ 三条技术实现路线

### 路线 1：读取语言模型的词表 logits（LLM2Jev）

保留词表输出层（LM head），对是非问题取输入末尾位置 `yes/no` 的 logits 并归一化。多选则拆成逐候选的是非判断，独立分数 `r_i` 再归一化为分布 `p_i = r_i / Σ r`。

- **优点**：可直接用现成权重，主要靠提示模板 + 语言理解能力；也可再任务微调。
- **代价**：K 个候选通常 = K 次评分输入；批处理能提高吞吐，但每个候选分支仍需计算。

### 路线 2：上下文表示 + 决策头直接评分（NeoHorse-Jev 采用）

保留模型主干，用**决策头**替代词表取分。输入按「当前信息 → 问题 → 各个候选 → 决策位置」组织，用完 forward 取决策位置状态 `hd` 与各候选结束位置状态 `hi`，用 pointer head 投影后计算缩放点积 `z_i = <fq(hd), fk(hi_i)> / dp`，再 softmax 得概率分布。

- **优点**：输出不再绑定词表 token；候选由请求动态给出，不必为每个新类增加固定输出；一次 forward 对同问题全部候选评分。
- **训练**：监督目标为候选分类交叉熵，可训练决策头，或结合 LoRA / 全量微调主干。

### 路线 3：双编码器 + 对比学习（CLM-v0.1-8B）

冻结 Qwen3-8B，信息侧与候选侧分别编码，经可训练投影头 `g_s`、`g_a` 映射到共同空间，L2 归一化后按点积算匹配分数，softmax 得分布。

- **优点**：固定候选的向量可提前计算并缓存；新消息只需编码信息侧即可与缓存候选匹配。
- **训练**：对比学习（InfoNCE 损失），匹配对拉高相似度，同 batch 其他候选作负例；双向 InfoNCE + 难负例训练。
- **代价**：候选与状态的交互主要发生在最终向量匹配；缓存需随候选描述更新。

---

## 🧪 评测表现（NeoHorse-Jev-4B）

基元律动基于 NeoHorse-1-4B 训练 NeoHorse-Jev-4B（路线 2 决策头），2026-09-24 评测结果：

| 对比维度 | 结果 |
|----------|------|
| **同规模基座对比**（Nimble + VitaminC + MASSIVE 本次子集，等权） | 平均 **71.76% → 83.26%**，提升 **11.50 个百分点** |
| **Nimble（结构化决策）** | **69.15% → 87.23%** |
| **综合均分（六组完整成绩开源权重模型对比）** | **77.70**，四轮成绩完整的四个开源权重模型中最高（JevBench、Kev、OpenJev 文本、Nimble、VitaminC、MASSIVE） |

- 闭环决策示例：NeoHorse-Jev 贪吃蛇回放中，模型每帧读取局面、在{上/下/左/右}中选出下一步，无需生成动作说明。

---

## 🛠️ 部署与落地

NanoHorse-Jev 提供模型权重、推理源码及调用示例，支持 vLLM、SGLang 及原生 Python、HTTP 调用。建议从一个高频、答案范围明确的业务节点切入，用真实样本检查判断质量后测部署延迟。

- **免费体验**：tokenrhythm.studio
- **模型下载**：https://huggingface.co/TokenRhythm/NeoHorse-Jev-4B
- **代码 & 示例**：https://github.com/TokenRhythm/NeoHorse （分支 `jev`）
- **部署说明**：https://huggingface.co/TokenRhythm/NeoHorse-Jev-4B/blob/main/DEPLOYMENT.md

---

## ⚠️ 注意事项

- 准确率 ≠ 概率校准：即使来自 logits / 相似度，也可能过度自信，需业务样本测试误判/漏判后再定自动执行、补充信息或人工复核的阈值。
- **候选覆盖**影响结果：若候选不包含正确选项（如用户问「改收货地址」而候选只有发货/开票/退货），最高分未必正确；可加「其他」选项或先判断任务适用性。
- 训练数据与决策结构各自贡献多少，需额外对照实验剥离。

---

## 🔗 参考资料

1. TypeSafe Jev 与结构化决策
2. Yinsongxu LLM2Jev 请求转换与评分机制
3. TypeSafe Noul、Choice 与 Score
4. Contrastive LM CLM v0.1 8B 模型卡
5. Contrastive LM 训练与推理实现
6. TokenRhythm NeoHorse Jev 决策结构与推理实现
7. TokenRhythm NeoHorse Jev 4B 模型卡与评测
8. Contrastive LM 投影头与相似度计算实现
9. TokenRhythm NeoHorse Jev 请求结构与返回字段

---

## 📌 下一步

- [ ] 按 DEPLOYMENT.md 实测 vLLM / SGLang 推理延迟与吞吐（与生成式调用对比）
- [ ] 对自家业务判断（如工单分流）跑小批量真实样本，验证 Noul/Choice/S core 准确率与校准
- [ ] 确认 TokenRhythm 具体协议（MIT）及组织 Star 数（需复核）
