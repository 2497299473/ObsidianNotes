---
title: 阶段二·持久化与高可用
created: 2026-07-20
tags:
  - Redis
  - 阶段入口
  - 方法论驱动
description: Redis 学习路径阶段二入口页，覆盖持久化与高可用。
lark_doc_url: https://my.feishu.cn/docx/QNTwdObQuo05NexTXr6cTRnUn6g
---

# 🔴 阶段二：持久化与高可用

> 📍 **预计学时**：12-16h | **难度**：⭐⭐⭐ | **前置**：[[../阶段一-数据结构与命令/README]]
> 📚 **深度参考**：[[阶段二-持久化与高可用/01-RDB与AOF持久化]]

## 🧱 Redis 高可用演进路线

```mermaid
flowchart LR
    A["单机 Redis"] --> B["RDB 持久化"]
    B --> C["AOF 持久化"]
    C --> D["主从复制"]
    D --> E["哨兵 Sentinel"]
    E --> F["Cluster 分片"]

    A -.->|"数据丢失风险"| B
    D -.->|"手动故障转移"| E
    E -.->|"自动故障转移"| F
    F -.->|"水平扩展"| F

    style A fill:#F44336,color:#fff
    style D fill:#FF9800,color:#fff
    style F fill:#4CAF50,color:#fff
```

## 📚 笔记列表

| 序号 | 笔记 | 核心内容 | 预计学时 |
|------|------|---------|---------|
| 01 | [[01-RDB与AOF持久化]] | RDB 快照、AOF 追加、混合持久化、BGREWRITEAOF | 3-4h |
| 02 | [[02-主从复制与哨兵]] | replicaof、Sentinel 选举、故障转移 | 4-5h |
| 03 | [[03-Cluster分片集群]] | 16384 槽位、Hash Tag、扩容缩容 | 4-5h |
| 99 | [[01-学习/Redis学习路径/阶段二-持久化与高可用/99-阶段2复习检查点|99-阶段2复习检查点]] | 🟢/🟡/🔴 三级验收 | 1-2h |

## 🎯 阶段目标

学完本阶段后，你应该能：

1. 根据 RPO 选择 RDB/AOF/混合持久化
2. 搭建主从 + 哨兵并验证故障转移
3. 部署 Redis Cluster 并理解槽位路由
4. 处理节点扩容缩容

## 🛠️ 配套代码

| 代码 | 对应笔记 | 状态 |
|------|---------|------|
| `../code/04-persistence/` | 01 | 待补充 |
| `../code/05-replication/` | 02 | 待补充 |
| `../code/06-cluster/` | 03 | 待补充 |

## 🔗 导航

- ⬅️ [[../阶段一-数据结构与命令/README]] — 上一阶段
- ➡️ [[../阶段三-高级特性与性能优化/README]] — 下一阶段

---

*最后更新：2026-07-20*
