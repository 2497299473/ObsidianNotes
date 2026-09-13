---
lark_doc_url: https://my.feishu.cn/docx/E0qIdP99UoXcM4xLC4hcc0OXnFd
lark_doc_token: E0qIdP99UoXcM4xLC4hcc0OXnFd
---
# Kronos 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/shiyu-coder/Kronos
> - **论文**：[Kronos: A Foundation Model for the Language of Financial Markets](https://arxiv.org/abs/2508.02739)（AAAI 2026 录用）
> - **Live Demo**：https://shiyu-coder.github.io/Kronos-demo/ （BTC/USDT 未来 24 小时预测可视化）
> - **模型下载**：Hugging Face `NeoQuasar`（Kronos-mini / small / base + 对应 Tokenizer）
> - **协议**：MIT
> - **主要语言**：Python
> - **Stars**：36.0k+（Fork 6.0k+，Open Issues 253）
> - **团队**：清华大学团队（Yu Shi, Zongliang Fu, Shuo Chen, Bohan Zhao, Wei Xu, Changshui Zhang, Jian Li）
> - **定位**：首个面向金融 K 线（Candlestick）的开源基础模型
> - **记录日期**：2026-08-04

> [!note] 同名项目区分
> GitHub 上还有几个叫 Kronos 的项目，与本文无关：MobileNativeFoundation/Kronos（Swift NTP 时间同步库）、lyft/Kronos-Android（Kotlin SNTP 库）、jgorset/django-kronos（Django cron 定时任务）。本文记录的是 star 数最高（36k+）的金融 K 线基础模型。

---

## 一句话定位

**把金融市场的 K 线（OHLCV）当作一种"语言"，用「K 线分词器 + 自回归 Transformer」两阶段架构做预训练的基础模型——输入历史 K 线，自回归地"续写"出未来 K 线，是首个开源的金融 K 线基础模型。**

---

## 🎯 解决的核心痛点

| 痛点 | 说明 |
|------|------|
| **金融数据噪音极高** | 通用时序基础模型（TSFM）主要针对平稳、低频噪音的时序数据，直接搬到高噪音、低信噪比的金融 K 线上效果差 |
| **K 线不是"文本"** | K 线是连续、多维（开高低收 + 成交量额）的数值序列，不符合 LLM 的离散 token 范式，没法直接喂给 Transformer 做语言建模 |
| **缺少统一预训练底座** | 传统量化建模往往一个任务、一个市场、一个频率单独训练，缺少跨市场、跨资产的统一预训练表征 |
| **跨市场泛化难** | 不同交易所、不同资产、不同频率的 K 线分布差异大，单市场训练的模型迁移能力弱 |

---

## 🏗️ 核心机制

### 两阶段框架

```
历史 K 线序列（OHLCV，连续多维数值）
    │
    ▼
┌────────────────────────────────────────┐
│ ① Kronos-Tokenizer（K 线分词器）          │
│    把连续 K 线量化为分层离散 token          │
│    （先粗后细的分层量化，保留 OHLCV 结构）  │
└────────────────────────────────────────┘
    │  K 线 token 序列（"金融市场的语言"）
    ▼
┌────────────────────────────────────────┐
│ ② Kronos（decoder-only Transformer）    │
│    在 45+ 全球交易所的海量 K 线上           │
│    做自回归预训练（next-token prediction）│
└────────────────────────────────────────┘
    │  生成的未来 K 线 token
    ▼
经 Tokenizer 反量化 → 预测的未来 OHLCV 序列
```

> [!important] 核心思想
> 类比 NLP：先把 K 线"分词"（tokenizer 量化成离散 token），再像 GPT 一样自回归地"续写"下一个 token。这样就能用一个统一的预训练模型承接预测、微调等多种量化任务，而不需要为每个任务单独设计架构。

### 模型家族（Model Zoo）

| 模型 | 配套 Tokenizer | 上下文长度 | 参数量 | 是否开源 |
|------|---------------|-----------|--------|---------|
| Kronos-mini | Kronos-Tokenizer-2k | 2048 | 4.1M | ✅ |
| Kronos-small | Kronos-Tokenizer-base | 512 | 24.7M | ✅ |
| Kronos-base | Kronos-Tokenizer-base | 512 | 102.3M | ✅ |
| Kronos-large | Kronos-Tokenizer-base | 512 | 499.2M | ❌ 未开源 |

### 推理能力

- `KronosPredictor` 封装了预处理、归一化、预测、反归一化全流程，几行代码出结果
- 支持温度采样（`T`）、nucleus sampling（`top_p`）、多路径采样平均（`sample_count`）做**概率性预测**
- `predict_batch` 支持多条序列 GPU 并行批量预测（要求相同 lookback 和 pred_len）
- 输入 DataFrame 需含 `open/high/low/close`，`volume`、`amount` 可选

### 微调流水线（以 A 股为例）

官方提供基于微软 [Qlib](https://github.com/microsoft/qlib) 的完整四步微调示例：

1. **配置**（`finetune/config.py`）：数据路径、股票池、训练区间、超参数
2. **数据准备**（`qlib_data_preprocess.py`）：切分训练/验证/测试集
3. **微调**：先 `train_tokenizer.py` 微调分词器，再 `train_predictor.py` 微调主模型（均支持 `torchrun` 多卡）
4. **回测**（`qlib_test.py`）：生成预测信号，跑简单的 top-K 策略回测，输出累计收益曲线

> [!warning] 官方免责声明
> 官方明确说明该流水线是**教学演示级别**，不是生产级量化系统。真实策略还需要组合优化、风险因子中性化、交易成本/滑点建模等。

---

## ✅ 优点

- **开创性**：首个开源金融 K 线基础模型，AAAI 2026 录用，学术可信度高
- **预训练数据广**：覆盖 45+ 全球交易所，跨市场泛化能力有基础
- **架构优雅**：两阶段设计把 K 线自然映射进 LLM 范式，统一模型承接多种量化任务
- **开箱即用**：mini/small/base 三档模型全部在 Hugging Face 开放，`from_pretrained` 一行加载
- **链路完整**：推理 → 批量预测 → 微调 → 回测全都有示例代码，还带 webui 和 live demo
- **MIT 协议**：商用友好，无锁定
- **社区热度高**：36k+ stars，衍生项目已出现（如 facecat-kronos 量化工具）

---

## ⚠️ 潜在局限

| 方面 | 说明 |
|------|------|
| **不是摇钱树** | 金融市场本质高噪音，模型输出是概率性参考信号，直接拿去交易大概率亏钱 |
| **最大模型未开源** | 效果最好的 Kronos-large（499M）不开放，开源版能力有上限 |
| **流水线仅演示级** | 微调/回测示例明确标注非生产级，风险管理、组合优化都要自己做 |
| **A 股需自行微调** | 预训练以全球市场为主，直接用于 A 股效果有限，需按官方示例用 Qlib 数据微调 |
| **算力门槛** | 微调推荐多 GPU（torchrun），纯 CPU 环境基本只能跑小模型推理 |
| **维护节奏放缓** | 最近一次 push 在 2026-04，且积压 253 个 open issues |
| **代码注释含 AI 生成** | finetune 目录部分注释由 AI 生成，官方提示可能有不准确之处，以代码为准 |

---

## 🧩 适合谁用

- 📈 **量化研究员**：把预测信号作为 alpha 特征/研究起点，接入自己的策略与风控体系
- 🔬 **AI 研究者**：研究 LLM 范式（tokenization + 自回归预训练）向金融时序迁移的前沿方向
- 🎓 **学生 / 学习者**：学习时序数据分词、基础模型预训练、微调与回测的完整范例
- 🏗️ **量化工具开发者**：基于开源模型做二次开发（已有 facecat-kronos 这类衍生工具）
- ❌ **不适合**：期望"加载模型直接预测赚钱"的投资者——官方自己都在劝退

---

## 🚀 如何使用（上手步骤）

> [!tip] 建议按以下顺序逐步进行

### 第一步：安装

```bash
git clone https://github.com/shiyu-coder/Kronos.git
cd Kronos
# 需要 Python 3.10+
pip install -r requirements.txt
```

### 第二步：加载模型并预测

```python
from model import Kronos, KronosTokenizer, KronosPredictor

# 从 Hugging Face 加载（首次会自动下载）
tokenizer = KronosTokenizer.from_pretrained("NeoQuasar/Kronos-Tokenizer-base")
model = Kronos.from_pretrained("NeoQuasar/Kronos-small")
predictor = KronosPredictor(model, tokenizer, max_context=512)

# df 为含 open/high/low/close（volume/amount 可选）的 DataFrame
pred_df = predictor.predict(
    df=x_df,
    x_timestamp=x_timestamp,   # 历史数据时间戳
    y_timestamp=y_timestamp,   # 待预测时间戳
    pred_len=120,
    T=1.0, top_p=0.9, sample_count=1
)
```

### 第三步：跑通官方示例

```bash
# 完整可运行脚本：加载数据 → 预测 → 画图对比真实值与预测值
python examples/prediction_example.py
```

### 第四步（可选）：用自己的数据微调

以 A 股为例，按 `finetune/` 目录四步走：改 `config.py` → `qlib_data_preprocess.py` → `torchrun` 跑 `train_tokenizer.py` 和 `train_predictor.py` → `qlib_test.py` 回测。需先安装 `pyqlib` 并准备好 Qlib 数据。

---

## 🔗 相关资源

| 资源 | 地址 |
|------|------|
| GitHub 仓库 | https://github.com/shiyu-coder/Kronos |
| 论文（arXiv） | https://arxiv.org/abs/2508.02739 |
| Hugging Face 模型 | https://huggingface.co/NeoQuasar |
| Live Demo | https://shiyu-coder.github.io/Kronos-demo/ |
| 配套数据工具（Qlib） | https://github.com/microsoft/qlib |

---

## 💡 个人思考

Kronos 最大的价值不在于"预测股价"，而在于它验证了一个范式迁移：**LLM 的「分词 + 自回归预训练」套路可以搬到金融 K 线上。** K 线是连续多维数值，传统上跟语言模型完全不搭界；Kronos 用一个专门的分词器把 OHLCV 量化成分层离散 token，硬是把"市场行为"变成了一门可以 next-token prediction 的"语言"。这个思路本身比模型权重更有启发性，对任何想做时序基础模型的人都值得细读。

但要清醒：官方连 README 里都在反复强调这只是研究工具，回测流水线是演示级的。金融市场的信噪比决定了任何模型输出的都只是概率信号，真正能落地的策略还需要组合优化、风险因子中性化、成本滑点建模这些"脏活"。36k 的 star 数里，有多少是冲着"AI 炒股"来的热度，多少是真正的研究价值，需要区分看待。

如果后续想实践，合理的路径是：先用小模型（small/base）在熟悉的标的上跑推理，感受输出分布；再按 Qlib 流程在 A 股数据上微调；最后只把预测当作策略里的一个特征，而不是交易指令。

> [!quote] 一句话总结
> Kronos 证明了"K 线是一种可建模的语言"，是金融时序基础模型的开创性开源工作——但请把它当研究工具，别当交易水晶球。

> 状态：✅ 已分析，⏳ 待实践
