---
title: TFX 路径复习检查点
created: 2026-07-25
tags:
  - TFX
  - MLOps
  - 复习
  - 检查点
description: TFX 路径学习成果检验：10 题概念自测、组件速查表、7 项实操检查清单、后续迭代建议。
lark_doc_url: https://my.feishu.cn/docx/W2gudZmcCotrCDx2whBcztcvnLf
---

## 一、概念自测（10 题）

1. TFX 的核心设计模式是什么？
2. 组件之间通过什么传递数据？
3. TFDV 的三个组件分别是什么？
4. Schema 的两个环境是什么？
5. TFT 解决的核心问题是什么？
6. TFT 的两阶段是什么？
7. TFMA 比 cross_val_score 多了什么？
8. Evaluator 输出什么决定 Pusher 是否推送？
9. TF Serving 的两个端口分别是什么？
10. 模型衰减的两个原因是什么？

<details>
<summary>🔑 答案</summary>

1. DAG（有向无环图）组件化流水线
2. Artifact（文件路径 + 元数据）
3. StatisticsGen、SchemaGen、ExampleValidator
4. TRAINING（有 label）和 SERVING（无 label）
5. 训练时用 Python 预处理，服务时 TF graph 不含预处理 → 不一致
6. Analyze（全量统计）和 Transform（用统计常量转换）
7. 切片分析（按群体）、公平性检查、新旧模型对比
8. blessing（True = 通过，Pusher 推送；False = 不通过，不推送）
9. 8501 = REST API，8500 = gRPC API
10. 数据漂移（分布变了）和概念漂移（关系变了）

</details>

---

## 二、组件速查表

| 组件 | 输入 | 输出 | 对应 Sklearn |
|------|------|------|-------------|
| ExampleGen | CSV/TFRecord | examples | `pd.read_csv()` |
| StatisticsGen | examples | statistics | `df.describe()` |
| SchemaGen | statistics | schema | 手动验证逻辑 |
| ExampleValidator | statistics + schema | anomalies | `assert` 语句 |
| Transform | examples + schema | transform_graph + transformed_examples | `Pipeline.fit_transform()` |
| Tuner | examples + schema | best_hyperparameters | `GridSearchCV` |
| Trainer | transformed_examples + transform_graph | model | `model.fit()` |
| Evaluator | model + examples | evaluation + blessing | `cross_val_score` |
| InfraValidator | model | blessing | 无对应 |
| Pusher | model + blessing | pushed_model | `model.save()` |

---

## 三、实操检查清单

- [ ] 能安装 TFX 并运行 LocalDagRunner
- [ ] 能用 TFDV 生成统计 + Schema + 检测异常
- [ ] 能手写 `preprocessing_fn`（TFT 模块）
- [ ] 能配置 EvalConfig（指标 + 切片 + 阈值）
- [ ] 能用 Pusher 部署到 TF Serving
- [ ] 能跑通完整 Pipeline（X6 代码，10 个组件）
- [ ] 能用 TFDV 检测数据漂移

---

## 四、流水线设计检查

- [ ] Pipeline 定义中包含 enable_cache=True
- [ ] metadata_connection_config 已配置（SQLite/MySQL）
- [ ] transform_module.py 和 trainer_module.py 路径正确
- [ ] EvalConfig 包含 slicing_specs（至少整体 + 一个切片维度）
- [ ] Pusher 的 push_destination 配置正确
- [ ] InfraValidator 配置了 max_loading_time_seconds

---

## 五、后续迭代建议

1. **短期（1-2 周）**：对已有项目用 TFDV 验证数据质量
2. **中期（1-3 个月）**：构建完整 TFX Pipeline，从数据到部署全自动
3. **长期（3-6 个月）**：部署到 Kubeflow，搭建监控闭环
4. **进阶**：学习 MLflow（实验追踪）、W&B（可视化）、Grafana（指标看板）

---

*回总览：[[X0-TFX 路径总览]]*
*回主路径：[[00-TensorFlow 总览索引（Sklearn 迁移版）]]*
*并行路径：[[T0-Transformer 路径总览]] / [[D0-分布式训练 路径总览]]*
