---
title: 03-Cluster分片集群
created: 2026-07-20
tags:
  - Redis
  - Cluster
  - 分片
  - 方法论驱动
description: Redis Cluster 架构、哈希槽分片、节点管理与扩缩容。
lark_doc_url: https://my.feishu.cn/docx/AneRdzqY9oH5BQxAHcFcPdrUn1b
---

# 03-Cluster分片集群

> 📍 **前置**：[[02-主从复制与哨兵]] → 本笔记 → [[01-学习/Redis学习路径/阶段二-持久化与高可用/99-阶段2复习检查点|99-阶段2复习检查点]]

## 🎯 学习目标

1. 理解 Redis Cluster 的分片原理（哈希槽）
2. 能搭建 Redis Cluster（6 节点：3 主 3 从）
3. 能执行节点扩容与缩容
4. 理解 Cluster 故障转移机制

---

## 📖 核心内容

### Cluster 架构

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│ Master1     │  │ Master2     │  │ Master3     │
│ Slots 0-5460│  │Slots 5461-  │  │Slots 10923- │
│             │  │     10922   │  │     16383   │
│   ↑         │  │   ↑         │  │   ↑         │
│ Replica1    │  │ Replica2    │  │ Replica3    │
└─────────────┘  └─────────────┘  └─────────────┘
```

### 数据分片原理

Redis Cluster 将所有数据划分为 **16384 个槽位（Slot）**，每个 Master 负责一部分槽。

```
key → CRC16(key) % 16384 → Slot → Master
```

### 搭建 Cluster（6 节点：3 主 3 从）

```bash
# 启动 6 个 Redis 节点（端口 7000-7005）
for port in 7000 7001 7002 7003 7004 7005; do
  redis-server --port $port \
    --cluster-enabled yes \
    --cluster-config-file nodes-$port.conf \
    --cluster-node-timeout 5000 \
    --appendonly yes \
    --daemonize yes
done

# 创建集群
redis-cli --cluster create \
  127.0.0.1:7000 127.0.0.1:7001 127.0.0.1:7002 \
  127.0.0.1:7003 127.0.0.1:7004 127.0.0.1:7005 \
  --cluster-replicas 1
# [OK] All 16384 slots covered
```

### 集群操作

```bash
# 查看集群信息
redis-cli -p 7000 CLUSTER INFO
# cluster_state:ok
# cluster_slots_assigned:16384

# 查看节点
redis-cli -p 7000 CLUSTER NODES

# 查看槽位分配
redis-cli -p 7000 CLUSTER SLOTS

# 查看某个 key 属于哪个槽
redis-cli -p 7000 CLUSTER KEYSLOT mykey
```

### 集群模式读写

```bash
# 普通模式：key 不在当前节点会报 MOVED
redis-cli -p 7000 SET mykey hello
# (error) MOVED 14687 127.0.0.1:7002
# 槽号可用 CLUSTER KEYSLOT mykey 复核（→ 14687，落在 10923-16383 区间即 7002）

# 集群模式（自动重定向）
redis-cli -c -p 7000 SET mykey hello
# OK（自动跳到 7002）

# 多 key 操作需要同一槽
redis-cli -c -p 7000 MSET key1 v1 key2 v2
# (error) CROSSSLOT

# 用 Hash Tag 确保同槽
redis-cli -c -p 7000 MSET {user:1}:name alice {user:1}:age 30
# OK（{user:1} 决定槽位）
```

### 迁移中的重定向：MOVED vs ASK

扩容/缩容时槽处于“迁移中”，客户端可能连续收到两种重定向，含义完全不同：

| 响应 | 含义 | 客户端应做 |
|------|------|-----------|
| `-MOVED <slot> <host:port>` | 该槽已**永久**归属另一节点 | 更新本地槽→节点映射，后续直接发往新节点 |
| `-ASK <slot> <host:port>` | 该 key 已迁走，但**槽还没迁完** | **一次性**转向该节点，且必须先发 `ASKING`（只对紧随的那一条命令生效），否则被 `-MOVED` 打回 |

> `redis-cli -c` 和 redis-py/Jedis 等客户端库会自动处理 MOVED 与 ASK（含补发 `ASKING`），所以你只看得到 `OK`；自己写 RESP 客户端时必须手动补这一步。反差验证：迁槽前后各发一次同一 key，先得 `ASK`、槽迁完后得 `MOVED`。

### 扩容

```bash
# 启动新节点
redis-server --port 7006 --cluster-enabled yes ...

# 加入集群（作为 Master）
redis-cli --cluster add-node 127.0.0.1:7006 127.0.0.1:7000

# 迁移槽位到新节点
redis-cli --cluster reshard 127.0.0.1:7000 \
  --cluster-from <node-id-7000> \
  --cluster-to <node-id-7006> \
  --cluster-slots 1000 \
  --cluster-yes

# 添加 Replica
redis-cli --cluster add-node 127.0.0.1:7007 127.0.0.1:7000 \
  --cluster-slave --cluster-master-id <node-id-7006>
```

### 缩容

```bash
# 迁移槽位回其他节点
redis-cli --cluster reshard 127.0.0.1:7000 \
  --cluster-from <node-id-7006> \
  --cluster-to <node-id-7000> \
  --cluster-slots 1000

# 删除节点
redis-cli --cluster del-node 127.0.0.1:7006 <node-id-7006>
```

---

## 💻 代码示例

```python
from redis.cluster import RedisCluster

# 连接集群
rc = RedisCluster(host='localhost', port=7000)

# 读写（自动路由到正确节点）
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
    print(results)
```

---

## ⚠️ 常见陷阱

| 陷阱 | 后果 | 解决方案 |
|------|------|---------|
| 不用 `-c` 集群模式 | 频繁 MOVED 报错 | 用 `redis-cli -c` |
| 多 key 操作跨槽 | CROSSSLOT 错误 | 用 Hash Tag `{}` |
| 扩容不迁移槽 | 新节点不承载数据 | 用 `--cluster reshard` |
| 节点数少于 3 主 | 集群不可用 | 至少 3 主 3 从 |
| 不配置 replicas | Master 宕机无备份 | 每主至少 1 Replica |

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `redis-cli --cluster create 127.0.0.1:7000 … :7005 --cluster-replicas 1` 后，`CLUSTER INFO` 显示 `cluster_state:ok` 与 `cluster_slots_assigned:16384`；`CLUSTER NODES` 里 3 个 `master` + 3 个 `slave` |
| 🟡 进阶 | `CLUSTER KEYSLOT mykey` → **14687**；`redis-cli -p 7000 SET mykey v` 得 `(error) MOVED 14687 127.0.0.1:7002`；`MSET {u1}:a 1 {u1}:b 2` 成功，而 `MSET u1:a u2:b` 报 `CROSSSLOT Keys in request don't hash to the same slot` |
| 🔴 挑战 | 手工 `CLUSTER SETSLOT <slot> IMPORTING/MIGRATING` + `MIGRATE` 迁 1 个槽，迁移中访问命中该槽的 key，拓下一条 `ASK` 并补发 `ASKING` 使其成功，用 2 行写清 ASK（一次性）与 MOVED（永久改路由）的区别 |

---

## 🔗 相关笔记

- [[01-RDB与AOF持久化]] — 前置
- [[02-主从复制与哨兵]] — 前置
- [[01-学习/Redis学习路径/阶段二-持久化与高可用/99-阶段2复习检查点|99-阶段2复习检查点]] — 阶段验收
- [[../00-Redis学习路径总索引]] — 返回总索引
- [[阶段二-持久化与高可用/01-RDB与AOF持久化]] — 既有深度参考
- [[../毕业项目-分布式缓存网关/v5-Cluster分片版]] — 毕业项目应用

---

*最后更新：2026-07-20*
