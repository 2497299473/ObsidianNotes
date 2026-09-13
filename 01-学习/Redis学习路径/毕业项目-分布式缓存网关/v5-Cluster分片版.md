---
title: v5-Cluster分片版
created: 2026-07-20
tags:
  - Redis
  - 毕业项目
  - v5
  - Cluster
  - 分片
  - 方法论驱动
description: 搭建 3 主 3 从 Redis Cluster，实现数据分片与水平扩展。
lark_doc_url: https://my.feishu.cn/docx/ALyud6SRWo8qOVxkCTYcYP5Lnvc
---

# v5-Cluster分片版

> 📍 **前置**：[[v4-主从高可用版]] → 本版本 → [[v6-Lua脚本与事务版]]

## 🎯 本版本目标

搭建 3 主 3 从 Redis Cluster，实现：**数据分片 + 水平扩展**。

---

## 💻 核心实现

### 启动 6 节点并创建集群

```bash
# 启动 6 个 Redis 节点
for port in 7000 7001 7002 7003 7004 7005; do
  redis-server --port $port \
    --cluster-enabled yes \
    --cluster-config-file nodes-$port.conf \
    --cluster-node-timeout 5000 \
    --appendonly yes \
    --daemonize yes
done

# 创建集群（3 主 3 从）
redis-cli --cluster create \
  127.0.0.1:7000 127.0.0.1:7001 127.0.0.1:7002 \
  127.0.0.1:7003 127.0.0.1:7004 127.0.0.1:7005 \
  --cluster-replicas 1
# [OK] All 16384 slots covered
```

### 验证

```bash
redis-cli -p 7000 CLUSTER INFO
redis-cli -p 7000 CLUSTER NODES
redis-cli -p 7000 CLUSTER SLOTS

# 集群模式读写
redis-cli -c -p 7000 SET "user:1" "alice"  # 自动路由
redis-cli -c -p 7000 GET "user:1"

# Hash Tag 确保同槽
redis-cli -c -p 7000 MSET {user:1}:name alice {user:1}:age 30
```

### 应用层连接 Cluster

```python
from redis.cluster import RedisCluster

rc = RedisCluster(host='localhost', port=7000)

# 自动路由到正确节点
rc.set("user:1", "alice")
print(rc.get("user:1"))

# Hash Tag 确保同槽
rc.hset("{user:1}:profile", mapping={"name": "alice", "age": "30"})
rc.sadd("{user:1}:tags", "vip", "active")

# Pipeline（集群模式）
with rc.pipeline() as p:
    p.set("k1", "v1")
    p.set("k2", "v2")
    results = p.execute()
```

---

## ✅ 验收标准

- [ ] 6 节点 Cluster 运行正常
- [ ] 16384 个槽位全部分配
- [ ] 客户端自动路由到正确节点
- [ ] Hash Tag 实现多 Key 同槽

---

## 🔮 下一版本预告

当前无原子操作和事务。下一版本用 **Lua 脚本** 实现原子性。

---

## ✅ 自检清单

### 🟢 基础
- [ ] Cluster 搭建成功
- [ ] 自动路由正确

### 🟡 进阶
- [ ] 能执行扩容缩容
- [ ] 理解 Hash Tag 原理

### 🔴 挑战
- [ ] 能设计跨机房 Cluster 方案
- [ ] 能分析 Cluster 故障转移过程

---

*最后更新：2026-07-20*
