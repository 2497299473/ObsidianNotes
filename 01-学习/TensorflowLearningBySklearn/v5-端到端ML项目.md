---
title: v5-端到端ML项目
stage: 毕业项目
order: v5
difficulty: ⭐⭐⭐⭐⭐
created: 2026-07-25
tags:
  - TensorFlow
  - 毕业项目
  - 端到端
  - 部署
  - TFLite
  - TF Serving
description: 毕业项目 v5：从数据到部署的完整 ML 系统——数据管道、预处理层嵌入、模型训练、SavedModel 导出、TFLite 量化、推理测试。这是三周学习的集大成项目。
lark_doc_url: https://my.feishu.cn/docx/OgnqdUHtLoozK0xARzucsucVndp
---

## 项目目标

构建一个从数据到部署的完整 ML 系统，整合三周学习成果。

**整合维度**：数据管道（02）+ 预处理嵌入（02/04）+ 模型训练（03/08/09）+ 评估调优（05）+ 部署（10）。

## 前置知识

- [[02-数据预处理与Pipeline对比]] · [[09-CNN与RNN-TF独有领域]] · [[01-学习/TensorflowLearningBySklearn/10-业务场景实战合集]]

---

## 项目：端到端图像分类系统

### 1. 数据管道

```python
import tensorflow as tf
import numpy as np

(x_train, y_train), (x_test, y_test) = tf.keras.datasets.cifar10.load_data()
x_train = x_train.astype(np.float32) / 255.0
x_test = x_test.astype(np.float32) / 255.0
y_train = y_train.flatten()
y_test = y_test.flatten()

# tf.data 管道
train_ds = tf.data.Dataset.from_tensor_slices((x_train, y_train)).shuffle(50000).batch(64).prefetch(tf.data.AUTOTUNE)
test_ds = tf.data.Dataset.from_tensor_slices((x_test, y_test)).batch(64).prefetch(tf.data.AUTOTUNE)
```

### 2. 模型（含预处理嵌入）

```python
data_aug = tf.keras.Sequential([
    tf.keras.layers.RandomFlip('horizontal'),
    tf.keras.layers.RandomRotation(0.1),
])

model = tf.keras.Sequential([
    tf.keras.Input(shape=(32, 32, 3)),
    data_aug,  # ← 数据增强嵌入模型（训练时激活，推理时自动关闭）
    tf.keras.layers.Conv2D(32, 3, padding='same', activation='relu'),
    tf.keras.layers.BatchNormalization(),
    tf.keras.layers.Conv2D(32, 3, padding='same', activation='relu'),
    tf.keras.layers.MaxPooling2D(),
    tf.keras.layers.Dropout(0.25),
    tf.keras.layers.Conv2D(64, 3, padding='same', activation='relu'),
    tf.keras.layers.BatchNormalization(),
    tf.keras.layers.Conv2D(64, 3, padding='same', activation='relu'),
    tf.keras.layers.MaxPooling2D(),
    tf.keras.layers.Dropout(0.25),
    tf.keras.layers.GlobalAveragePooling2D(),
    tf.keras.layers.Dense(128, activation='relu'),
    tf.keras.layers.Dropout(0.5),
    tf.keras.layers.Dense(10, activation='softmax')
])
```

### 3. 训练（回调函数）

```python
model.compile(optimizer='adam', loss='sparse_categorical_crossentropy', metrics=['accuracy'])
model.fit(train_ds, epochs=50, validation_data=test_ds,
    callbacks=[
        tf.keras.callbacks.EarlyStopping(monitor='val_loss', patience=10, restore_best_weights=True),
        tf.keras.callbacks.ReduceLROnPlateau(monitor='val_loss', factor=0.5, patience=3, min_lr=1e-6),
        tf.keras.callbacks.ModelCheckpoint('v5_best.keras', save_best_only=True, monitor='val_accuracy', mode='max'),
        tf.keras.callbacks.TensorBoard(log_dir='./logs/v5')
    ])
_, acc = model.evaluate(test_ds, verbose=0)
print(f"测试集准确率: {acc:.4f}")
```

### 4. 部署

```python
# --- SavedModel 格式（TF Serving 用） ---
model.save('v5_saved_model/')

# --- TFLite 量化（移动端用） ---
converter = tf.lite.TFLiteConverter.from_saved_model('v5_saved_model/')
converter.optimizations = [tf.lite.Optimize.DEFAULT]  # 动态量化
tflite_model = converter.convert()
with open('v5_model.tflite', 'wb') as f:
    f.write(tflite_model)
print(f"TFLite 模型大小: {len(tflite_model) / 1024:.1f} KB")
```

### 5. 推理验证

```python
# 加载 SavedModel 验证
loaded = tf.keras.models.load_model('v5_saved_model/')
sample = x_test[:5]
pred = loaded.predict(sample, verbose=0).argmax(axis=1)
print(f"SavedModel 预测: {pred}")
print(f"真实标签: {y_test[:5]}")

# TFLite 推理验证
interpreter = tf.lite.Interpreter(model_path='v5_model.tflite')
interpreter.allocate_tensors()
input_detail = interpreter.get_input_details()[0]
output_detail = interpreter.get_output_details()[0]
interpreter.set_tensor(input_detail['index'], sample[:1].astype(np.float32))
interpreter.invoke()
tflite_pred = interpreter.get_tensor(output_detail['index']).argmax()
print(f"TFLite 预测: {tflite_pred}, 真实: {y_test[0]}")
```

### 验收标准

- [ ] 模型准确率 > 70%
- [ ] SavedModel 保存成功
- [ ] TFLite 量化后大小 < 原始模型 50%
- [ ] 加载 SavedModel 后推理正常
- [ ] TFLite 推理结果与原模型一致
- [ ] 数据增强层推理时自动关闭

---

## 项目总结

| 阶段 | 使用技术 | 对应笔记 |
|------|---------|---------|
| 数据管道 | `tf.data.Dataset` | 02 |
| 数据增强 | `RandomFlip/Rotation` 层 | 09 |
| 模型 | `Conv2D + BatchNorm + Dropout` | 08/09 |
| 训练控制 | `EarlyStopping + ReduceLROnPlateau` | 05 |
| 保存 | `model.save()` SavedModel | 10 |
| 部署 | `TFLiteConverter` 量化 | 10 |
| 推理验证 | `load_model` + TFLite Interpreter | 10 |

---

## 🎉 恭喜完成全部毕业项目！

| 版本 | 任务 | 难度 | 状态 |
|------|------|:---:|:---:|
| v1 | 基础回归分类迁移 | ⭐⭐ | ⬜ |
| v2 | 数据管道与特征工程 | ⭐⭐⭐ | ⬜ |
| v3 | 模型评估与超参调优 | ⭐⭐⭐ | ⬜ |
| v4 | 深度学习入门 | ⭐⭐⭐⭐ | ⬜ |
| v5 | 端到端 ML 项目 | ⭐⭐⭐⭐⭐ | ⬜ |

> [!important] 能力评估
> - 完成 **v1+v2+v3** = **中级**（能用 TF 独立完成 ML 任务）
> - 完成 **v4** = **高级**（能处理深度学习任务）
> - 完成 **v5** = **专家级**（能设计端到端 ML 系统并部署）

### 后续学习建议

1. **Transformer / Attention**：学习 BERT/GPT，进入 NLP 前沿
2. **GAN / Diffusion**：生成式模型
3. **RL（强化学习）**：决策系统
4. **TFX**：端到端 ML 流水线（数据→训练→部署→监控）
5. **分布式训练**：`MultiWorkerMirroredStrategy`
6. **HuggingFace + TensorFlow**：结合预训练 NLP 模型

---
*创建时间：2026-07-25*