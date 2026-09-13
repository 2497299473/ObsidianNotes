---
title: 深度学习实践：使用 RNN 预测 App 激活率走势
date: 2026-06-02
tags:
  - 深度学习
  - RNN
  - LSTM
  - 时间序列
  - 实战笔记
aliases:
  - RNN 预测 App 激活率
  - 时间序列预测实践
status: 📝 待处理
lark_doc_url: https://my.feishu.cn/docx/W9gMdjgjsoibEZxUPkIcEDYLn7Y
---

> [!note] 定位：个人加工笔记（2026-08-23 审阅标注）
> 本篇含原文没有的补充分析；课程原文见 [[12｜深度学习（中）：如何用RNN预测激活率走势？]]。

> [!ABSTRACT] 快速概览
> 使用 LSTM 循环神经网络对易速鲜花 App 日激活数进行时间序列预测，涵盖数据预处理（归一化、60 天时间窗口构造）、LSTM 模型搭建、训练评估全流程实战总结。

---

## 📝 正文内容

### 项目背景

本项目旨在使用循环神经网络（RNN）预测易速鲜花App未来的激活率走势。通过分析过去两年的每日激活数据，构建时间序列预测模型，为运营团队提供未来趋势参考。

## 1. 问题定义

### 业务场景

- **目标**：预测未来App激活率走势
- **数据来源**：2019年至今的每日激活数记录
- **任务类型**：时间序列回归问题

### 数据特点

- **数据集结构**：仅包含日期和激活数两列
- **特征识别**：日期本身不是有效特征，历史激活数序列才是关键特征
- **问题转化**：将时间序列预测问题转化为监督学习问题

## 2. 数据预处理

### 2.1 数据导入与可视化

```python
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

# 导入数据
df_app = pd.read_csv('app.csv', index_col='Date', parse_dates=['Date'])

# 数据可视化
plt.style.use('fivethirtyeight')
df_app["Activation"].plot(figsize=(12,4),legend=True)
plt.title('App Activation Count')
plt.show()
```

### 2.2 数据清洗

```python
# 检查缺失值
df_app.isna().sum()

# 检查负值
(df_app.Activation < 0).values.any()
```

### 2.3 训练集与测试集拆分

```python
# 按时间拆分数据集
Train = df_app[:'2020-09-30'].iloc[:,0:1].values
Test = df_app['2020-10-01':].iloc[:,0:1].values

# 显示数据形状
print('训练集的形状是：', Train.shape)
print('测试集的形状是：', Test.shape)
```

### 2.4 特征工程

```python
from sklearn.preprocessing import MinMaxScaler

# 数据归一化
Scaler = MinMaxScaler(feature_range=(0,1))
Train = Scaler.fit_transform(Train)
```

### 2.5 构建特征集和标签集

```python
# 构建训练集特征和标签
X_train = []
y_train = []
for i in range(60, Train.size):
    X_train.append(Train[i-60:i, 0])
    y_train.append(Train[i, 0])

X_train, y_train = np.array(X_train), np.array(y_train)
X_train = np.reshape(X_train, (X_train.shape[0], X_train.shape[1], 1))

# 构建测试集特征和标签
TrainTest = df_app["Activation"][:]
inputs = TrainTest[len(TrainTest)-len(Test) - 60:].values
inputs = inputs.reshape(-1,1)
inputs = Scaler.transform(inputs)

X_test = []
y_test = []
for i in range(60, inputs.size):
    X_test.append(inputs[i-60:i, 0])
    y_test.append(inputs[i, 0])

X_test = np.array(X_test)
X_test = np.reshape(X_test, (X_test.shape[0], X_test.shape[1], 1))
```

## 3. 模型选择与构建

### 3.1 算法选择依据

**为什么选择RNN？**

- 时间序列数据具有历史依赖性
- RNN具备记忆功能，能捕捉时间关联
- 适合处理序列数据的预测问题

**RNN变体比较：**

- **Simple RNN**：存在短期记忆问题
- **LSTM**：解决长期依赖问题，适合长周期数据
- **GRU**：Simple RNN和LSTM的折中方案

### 3.2 LSTM模型构建

```python
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, LSTM

# 构建LSTM网络
RNN_LSTM = Sequential()
RNN_LSTM.add(LSTM(units=50, return_sequences=True, input_shape=(X_train.shape[1],1)))
RNN_LSTM.add(LSTM(units=50, return_sequences=True))
RNN_LSTM.add(LSTM(units=50, return_sequences=True))
RNN_LSTM.add(LSTM(units=50))
RNN_LSTM.add(Dense(units=1))

# 编译模型
RNN_LSTM.compile(loss='mean_squared_error',
                 optimizer='rmsprop',
                 metrics=['mae'])

RNN_LSTM.summary()
```

## 4. 模型训练与评估

### 4.1 模型训练

```python
# 训练模型
history = RNN_LSTM.fit(X_train, y_train, epochs=50, validation_split=0.2)
```

### 4.2 训练过程分析

- **训练轮次**：50次
- **损失变化**：训练集损失逐渐减小
- **过拟合迹象**：验证集损失出现振荡上升

### 4.3 模型预测

```python
# 进行预测
predicted_stock_price = RNN_LSTM.predict(X_test)
predicted_stock_price = Scaler.inverse_transform(predicted_stock_price)
```

## 5. 结果分析与优化方向

### 5.1 模型表现

- **预测效果**：回归曲线与实际走势接近
- **局限性**：时间序列预测存在固有难度
- **改进空间**：可结合更多特征信息

### 5.2 优化建议

1. **特征丰富化**：加入季节性、节假日等特征
2. **模型调参**：调整LSTM层数和单元数
3. **集成方法**：结合多种模型进行预测
4. **外部数据**：引入市场环境等外部信息

## 6. 总结

### 核心收获

- **时间序列处理**：掌握了时序数据的预处理方法
- **RNN应用**：理解了循环神经网络在序列预测中的优势
- **LSTM原理**：学习了长短期记忆网络解决长期依赖的机制
- **实战经验**：完成了从数据预处理到模型预测的完整流程

### 注意事项

- **预测局限性**：历史数据预测未来存在固有挑战
- **模型选择**：根据数据特点选择合适的RNN变体
- **过拟合防范**：注意验证集表现，避免过度拟合
- **特征工程**：合理构建时间窗口特征

## 7. 扩展思考

### 7.1 不同RNN变体比较

可以尝试使用SimpleRNN和GRU进行对比实验，分析不同网络结构的预测效果差异。

### 7.2 多变量时间序列

考虑将单变量预测扩展为多变量预测，加入更多相关特征提升预测准确性。

### 7.3 实时预测系统

构建实时预测系统，定期更新模型以适应数据变化。

*本项目完整代码可在Kaggle Notebook中获取，欢迎参考学习。*


## ✅ 待办事项

- [ ] 尝试用 SimpleRNN 和 GRU 替换 LSTM 对比预测效果
- [ ] 调整时间窗口大小（30 天 / 60 天 / 90 天）
- [ ] 加入节假日、季节性等外部特征做多变量预测


## 🔗 关联笔记

- [[12｜深度学习（中）：如何用RNN预测激活率走势？]] | 原始课程文章
- [[01-学习/深度学习/深度学习CNN图像分类实战与原理全解析]] | CNN 实战笔记
- [[提升神经网络预测准确率全解析]] | CNN 课程笔记


## 📎 参考资料

- 极客时间《零基础实战机器学习》第 12 讲 — 黄佳
- [Keras LSTM 官方文档](https://keras.io/api/layers/recurrent_layers/lstm/)
- [时间序列预测指南](https://www.tensorflow.org/tutorials/structured_data/time_series)


## 📊 元数据

| 字段 | 值 |
|------|-----|
| 创建日期 | 2026-06-02 |
| 更新时间 | 2026-06-02 |
| 类型 | `实战笔记` |
| 状态 | 📝 待处理 |

---

> [!TIP]
> LSTM 层数并非越多越好，3~4 层是常见配置。训练时注意监控验证集损失，防止过拟合。

