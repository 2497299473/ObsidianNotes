---
title: 99-Transformer Phase 1 复习检查点
created: 2026-07-25
tags:
  - TensorFlow
  - Transformer
  - 复习检查点
description: Phase 1（架构理解 T1-T4）的复习检查点：17 项检查清单、10 道自测题、3 道代码填空、错误诊断、端到端 Transformer 文本分类实操、错题本、30 秒速查表。
lark_doc_url: https://my.feishu.cn/docx/ZVrhdkMd7oeOTIxmlnkcjV5PnSg
---

## 🎯 Phase 1 完成度自检

| # | 检查项 | 状态 |
|---|--------|:---:|
| 1 | 能写出 `softmax(QK^T/√d_k)V` 并解释每个符号 | ☐ |
| 2 | 知道为什么要除以 √d_k（防 softmax 饱和→梯度消失） | ☐ |
| 3 | 能实现单头 Self-Attention 并打印权重矩阵 | ☐ |
| 4 | 知道 Multi-Head 的参数量 ≈ 4×d_model²（不随头数增加） | ☐ |
| 5 | 能实现 split_heads 把 `(B,S,d)` 切成 `(B,H,S,d_k)` | ☐ |
| 6 | 知道不同头会自发学习语法/语义/共指等不同模式 | ☐ |
| 7 | 能实现 Causal Mask 并验证每行只能看到自己和之前 | ☐ |
| 8 | 能组合 Padding Mask + Causal Mask（tf.maximum） | ☐ |
| 9 | 知道 sin/cos PE 的置换不变性证明（无 PE = 词袋模型） | ☐ |
| 10 | 能实现可学习位置编码（trainable=True） | ☐ |
| 11 | 知道 RoPE 通过旋转 Q/K 编码相对位置 | ☐ |
| 12 | 能实现 FFN（Dense→ReLU→Dense，dff≈4×d_model） | ☐ |
| 13 | 能解释为什么 Transformer 需要残差连接（深层梯度消失） | ☐ |
| 14 | 知道 Pre-LN vs Post-LN 的区别（Pre-LN 更稳定，不需 warmup） | ☐ |
| 15 | 能写出 Encoder Block 和 Decoder Block 的完整结构 | ☐ |
| 16 | 知道 Cross-Attention 的 Q 来自 Decoder，K/V 来自 Encoder | ☐ |
| 17 | 能组装完整 Transformer（Encoder + Decoder + 最终投影） | ☐ |

**通过标准**：至少 14/17 ☑。少于 10 个，建议重读 T1-T4。

---

## 📝 快速自测（10 题）

### Q1：Self-Attention 的缩放因子是什么？为什么要缩放？
<details>
<summary>答案</summary>
缩放因子是 √d_k。不缩放时，Q·K^T 的方差≈d_k（维度越大值越大），softmax 会饱和（最大值≈1，其余≈0），导致梯度消失、训练不稳定。除以 √d_k 把方差拉回 1。
</details>

### Q2：Multi-Head 为什么不增加参数量？
<details>
<summary>答案</summary>
每个头的 Q/K/V 维度是 d_model/h，h 个头加起来仍是 d_model。总参数量 ≈ 4×d_model²，与单头相同。多头不是"复制"参数，而是"切分"到不同子空间。
</details>

### Q3：Causal Mask 在哪里用？为什么？
<details>
<summary>答案</summary>
在 Decoder 的 Self-Attention 中用。防止生成第 t 个 token 时看到 t+1 及以后的 token（否则就是作弊）。上三角设为 -∞，softmax 后权重为 0。
</details>

### Q4：sin/cos 位置编码和可学习位置编码谁更好？
<details>
<summary>答案</summary>
效果相当。sin/cos 可外推到更长序列（固定函数），可学习在固定长度任务上略好（BERT/GPT 用）。现代大模型（LLaMA）倾向用 RoPE（相对位置 + 可外推）。
</details>

### Q5：RoPE 的核心思想是什么？
<details>
<summary>答案</summary>
在 Q 和 K 上施加旋转矩阵 R(m) 和 R(n)，使得注意力分数 q_m·k_n 只依赖相对位置 m-n。不需要显式的位置 Embedding。
</details>

### Q6：FFN 的作用是什么？为什么 dff = 4×d_model？
<details>
<summary>答案</summary>
FFN 给每个位置独立的非线性变换（Attention 本身只有 softmax 一个非线性）。dff=4×d_model 让模型在高维空间做非线性变换再降回来，是原始论文的经验选择。
</details>

### Q7：Pre-LN vs Post-LN，哪个更稳定？
<details>
<summary>答案</summary>
Pre-LN（`x + Sublayer(LN(x))`）更稳定，训练初期梯度流更通畅，不需要 warmup。Post-LN（`LN(x + Sublayer(x))`）是原始论文方案，需要 warmup 防止训练崩溃。现代模型（GPT/LLaMA）基本用 Pre-LN。
</details>

### Q8：Cross-Attention 的 Q/K/V 分别来自哪里？
<details>
<summary>答案</summary>
Q 来自 Decoder 当前层，K 和 V 来自 Encoder 输出。作用是让 Decoder 查询 Encoder 编码的信息（如翻译时对齐源语言和目标语言）。
</details>

### Q9：Teacher Forcing 是什么？推理时用吗？
<details>
<summary>答案</summary>
训练时，Decoder 的输入是真实目标序列（右移一位），而不是模型自己的预测。这避免了生成错误累积，加速收敛。推理时不用——模型逐 token 自回归生成。
</details>

### Q10：BERT 和 GPT 的核心架构区别？
<details>
<summary>答案</summary>
BERT 只用 Encoder（双向 Attention，无 causal mask，适合理解任务）；GPT 只用 Decoder（因果 Attention，有 causal mask，适合生成任务）。
</details>

---

## 🔧 代码填空练习

### 填空 1：实现 Multi-Head Attention 的 split_heads

```python
def split_heads(self, x, batch_size):
    """把 (batch, seq, d_model) 拆成 (batch, heads, seq, d_k)"""
    x = tf.reshape(x, (batch_size, -1, __A__, __B__))
    return tf.transpose(x, perm=[__C__])
```

<details>
<summary>答案</summary>

```python
def split_heads(self, x, batch_size):
    x = tf.reshape(x, (batch_size, -1, self.num_heads, self.d_k))
    return tf.transpose(x, perm=[0, 2, 1, 3])  # (B, H, S, d_k)
```
</details>

### 填空 2：实现 Decoder Block 的 Cross-Attention

```python
ln_x = self.ln2(x)
# Q 来自 __A__，K/V 来自 __B__
attn2 = self.mha2(__C__, __D__, __E__)
x = x + attn2
```

<details>
<summary>答案</summary>

```python
ln_x = self.ln2(x)
# Q 来自 Decoder，K/V 来自 Encoder
attn2 = self.mha2(ln_x, enc_output, enc_output)
x = x + attn2
```
</details>

### 填空 3：Causal Mask 生成

```python
def create_causal_mask(seq_len):
    """返回 (1, 1, seq, seq)，上三角=1，下三角=0"""
    lower = tf.linalg.band_part(__A__, __B__, __C__)
    mask = 1 - lower
    return tf.reshape(mask, (__D__,))
```

<details>
<summary>答案</summary>

```python
def create_causal_mask(seq_len):
    lower = tf.linalg.band_part(tf.ones((seq_len, seq_len)), -1, 0)
    mask = 1 - lower
    return tf.reshape(mask, (1, 1, seq_len, seq_len))
```
</details>

---

## 🚨 常见错误诊断

### 错误 1：`ValueError: Input 0 is incompatible with layer multi_head_attention`

<details>
<summary>原因与修复</summary>

`key_dim` 必须等于 `d_model // num_heads`。如果 `key_dim=64` 但输入维度是 512 且 `num_heads=8`，需要 `key_dim=512//8=64`（正确）。检查输入 shape 和 key_dim 的匹配。
</details>

### 错误 2：训练 loss 不下降，注意力权重全是 1/n

<details>
<summary>排查步骤</summary>

1. 检查是否除以 √d_k（不缩放 → softmax 饱和 → 梯度消失）
2. 降低学习率（2e-5 或更小）
3. 检查初始化：用 `glorot_uniform` 而非 `zeros`
4. 打印中间层输出，看是否全为 0 或 NaN
</details>

### 错误 3：Transformer 输出与输入顺序无关

<details>
<summary>原因与修复</summary>

没有加位置编码！Self-Attention 本身是置换不变的。修复：在 Embedding 后加 PE（sin/cos、learnable 或 RoPE）。
</details>

---

## 🎯 端到端实操：完整 Transformer 文本分类器

```python
"""
完整 Transformer 文本分类器（从数据到推理）
目标：验证你能把 T1-T4 的零件组装成可用模型
"""
import tensorflow as tf
import numpy as np

# 1. 位置编码
def get_pos_encoding(seq_len, d_model):
    pe = np.zeros((seq_len, d_model))
    for pos in range(seq_len):
        for i in range(0, d_model, 2):
            pe[pos, i] = np.sin(pos / (10000 ** (2*i/d_model)))
            if i + 1 < d_model:
                pe[pos, i+1] = np.cos(pos / (10000 ** (2*i/d_model)))
    return tf.constant(pe, dtype=tf.float32)

# 2. Transformer Block
def transformer_block(x, d_model, num_heads, dff):
    attn = tf.keras.layers.MultiHeadAttention(num_heads, d_model//num_heads)(x, x)
    x = tf.keras.layers.LayerNormalization()(x + attn)
    ffn_out = tf.keras.layers.Dense(dff, activation='relu')(x)
    ffn_out = tf.keras.layers.Dense(d_model)(ffn_out)
    x = tf.keras.layers.LayerNormalization()(x + ffn_out)
    return x

# 3. 完整模型
def build_classifier(vocab_size, max_len=50, d_model=64, num_heads=4, dff=256):
    inputs = tf.keras.Input((max_len,))
    x = tf.keras.layers.Embedding(vocab_size, d_model)(inputs)
    x = x * tf.math.sqrt(tf.cast(d_model, tf.float32))
    x = x + get_pos_encoding(max_len, d_model)
    x = transformer_block(x, d_model, num_heads, dff)
    x = transformer_block(x, d_model, num_heads, dff)
    x = tf.keras.layers.GlobalAveragePooling1D()(x)
    outputs = tf.keras.layers.Dense(1, activation='sigmoid')(x)
    return tf.keras.Model(inputs, outputs)

# 4. 验证
model = build_classifier(1000)
print(f"参数量: {model.count_params():,}")

# 5. 验证位置编码生效（顺序不同 → 输出不同）
x1 = tf.constant([[1, 2, 3, 4, 5] + [0]*45])
x2 = tf.constant([[5, 4, 3, 2, 1] + [0]*45])
pred1 = model.predict(x1, verbose=0)
pred2 = model.predict(x2, verbose=0)
print(f"顺序不同 → 输出不同: {not np.allclose(pred1, pred2, atol=1e-4)}")
# 应该 True（有 PE，输出随顺序变化）
```

**验收标准**：
- 模型能构建并编译
- 顺序不同的输入产生不同输出（证明 PE 生效）

---

## 📋 错题本模板

| 题目 | 我的答案 | 正确答案 | 错在哪 |
|------|---------|---------|--------|
| | | | |
| | | | |

---

## 📝 30 秒速查表

| 概念 | 公式/关键点 |
|------|-----------|
| Self-Attention | `softmax(QK^T/√d_k)V` |
| Multi-Head | `Concat(head_1...head_h)·W_O`，h×d_k=d_model |
| Causal Mask | `mask = 1 - band_part(ones, -1, 0)` |
| sin/cos PE | `sin(pos/10000^(2i/d))`, `cos(pos/10000^(2i/d))` |
| RoPE | 旋转 Q 和 K，`q'=R_m·q`, `k'=R_n·k` |
| FFN | `W_2·relu(W_1·x+b_1)+b_2`，dff≈4×d_model |
| 残差连接 | `output = x + Sublayer(x)` |
| Pre-LN | `x + Sublayer(LayerNorm(x))`（更稳定） |
| Encoder Block | Self-Attn + FFN + 残差 + LN |
| Decoder Block | Masked Self-Attn + Cross-Attn + FFN |
| Cross-Attn | Q from Decoder, K/V from Encoder |
| Teacher Forcing | 训练时用真实目标序列，推理时自回归 |

---

## 🔜 Phase 2 预告：掌握模型

| 篇 | 内容 | 学时 |
|---|------|:---:|
| T5 | BERT 双向预训练 + 微调 | 4h |
| T6 | GPT 自回归生成 | 3.5h |
| T7 | HuggingFace + TF 实战 | 3.5h |

**目标**：能用 BERT/GPT 做实际 NLP 任务。

---

*前置：[[T4-Encoder-Decoder 完整架构]]*
*后续：[[T5-BERT 双向预训练与微调]]*