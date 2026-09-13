---
title: 03-Pub-Sub与Stream消息
created: 2026-07-20
tags:
  - Redis
  - Pub-Sub
  - Stream
  - 消息队列
  - 方法论驱动
description: Redis 发布订阅、Stream 消息队列、消费者组与消息确认。
lark_doc_url: https://my.feishu.cn/docx/U0DDdjyrjo7BCvxTUvEc8FbjnKh
---

# 03-Pub-Sub与Stream消息

> 📍 **前置**：[[02-五大基础数据结构]] → 本笔记 → [[01-学习/Redis学习路径/阶段一-数据结构与命令/99-阶段1复习检查点|99-阶段1复习检查点]]

## 🎯 学习目标

1. 理解 Pub/Sub 发布订阅模型与局限
2. 掌握 Stream 消息队列与消费者组
3. 能对比 List、Pub/Sub、Stream 的适用场景
4. 能为异步任务设计消息方案

---

## 📖 核心内容

### Pub/Sub 发布订阅

```
Publisher → Channel → Subscriber1, Subscriber2, Subscriber3
```

```redis
# 订阅频道
SUBSCRIBE news
# 发布消息
PUBLISH news "hello world"
# 模式订阅（通配符）
PSUBSCRIBE news.*
# 查看活跃频道
PUBSUB CHANNELS
PUBSUB NUMSUB news  # 订阅者数量
```

### Pub/Sub 的局限

| 局限 | 说明 |
|------|------|
| 消息不持久化 | 订阅者离线时消息丢失 |
| 无消费者确认 | 不知道谁消费了 |
| 无法回溯 | 消息发完即消失 |
| 离线无缓冲 | 重连后收不到历史消息 |

### Stream（推荐的消息队列方案）

Stream = 持久化消息队列 + 消费者组 + 消息确认（ACK）

```redis
# 生产消息
XADD mystream * name alice age 30
XADD mystream * name bob age 25

# 读取消息
XRANGE mystream - +           # 全部消息
XREAD COUNT 2 STREAMS mystream 0  # 从头读2条
XLEN mystream                  # 消息数量
```

### 消费者组

```redis
# 创建消费者组
XGROUP CREATE mystream group1 $  # $ = 从最新开始
XGROUP CREATE mystream group1 0 MKSTREAM  # 从头开始

# 消费者读取消息
XREADGROUP GROUP group1 consumer1 COUNT 1 STREAMS mystream >

# 确认消息
XACK mystream group1 <message-id>

# 查看待确认消息
XPENDING mystream group1

# 消费者信息
XINFO GROUPS mystream
XINFO CONSUMERS mystream group1
```

### 三种消息模型对比

| 特性 | Pub/Sub | List（LPUSH+BRPOP） | Stream |
|------|---------|---------------------|--------|
| 持久化 | ❌ | ✅ | ✅ |
| 消费者组 | ❌ | ❌ | ✅ |
| 消息确认 | ❌ | ❌ | ✅ |
| 消息回溯 | ❌ | ❌ | ✅ |
| 多消费者广播 | ✅ | ❌（竞争消费） | ✅ |
| 适用场景 | 实时通知 | 简单队列 | 可靠消息队列 |

### Stream 消息处理流程

```
1. XADD → 生产消息
2. XREADGROUP → 消费者读取
3. 处理消息
4. XACK → 确认消费
5. XPENDING → 检查未确认
6. XCLAIM → 转移超时消息给其他消费者
```

---

## 💻 代码示例

Python + redis-py 实现 Stream 消费者：

```python
import redis
import time

r = redis.Redis(host='localhost', port=6379, decode_responses=True)

# === 生产者 ===
def produce():
    msg_id = r.xadd("orders", {"order_id": "1001", "amount": "99.9"})
    print(f"Produced: {msg_id}")

# === 消费者 ===
def consume():
    try:
        r.xgroup_create("orders", "workers", id="$", mkstream=True)
    except redis.exceptions.ResponseError:
        pass  # 已存在

    while True:
        messages = r.xreadgroup(
            groupname="workers",
            consumername="worker-1",
            streams={"orders": ">"},
            count=1,
            block=5000  # 阻塞5秒
        )
        for stream, msg_list in messages:
            for msg_id, data in msg_list:
                print(f"Processing: {data}")
                # 处理业务逻辑...
                r.xack("orders", "workers", msg_id)  # 确认
                print(f"Acked: {msg_id}")

# 死信处理
def claim_stale_messages():
    pending = r.xpending_range("orders", "workers", min="-", max="+", count=10)
    for item in pending:
        msg_id = item["message_id"]
        idle_time = item["time_since_delivered"]
        if idle_time > 30000:  # 超过30秒
            r.xclaim("orders", "workers", "worker-2",
                     min_idle_time=30000, message_ids=[msg_id])

# 限制 Stream 长度
r.xadd("orders", {"k": "v"}, maxlen=10000, approximate=True)
```

---

## ⚠️ 常见陷阱

| 陷阱 | 后果 | 解决方案 |
|------|------|---------|
| 用 Pub-Sub 做可靠队列 | 离线消息丢失 | 用 Stream |
| 忘记 XACK | XPENDING 无限增长 | 消费后立即 ACK |
| 不处理死信 | 消费者宕机消息卡住 | 用 XCLAIM 转移 |
| Stream 无限增长 | 内存爆满 | 设置 MAXLEN 或定期 XTRIM |
| XREADGROUP 不用 `>` | 重复消费已处理消息 | 新消息用 `>`，未确认用 `0` |

---

## 🔗 相关笔记

- [[01-Redis基础与字符串命令]] — 前置
- [[02-五大基础数据结构]] — 前置
- [[01-学习/Redis学习路径/阶段一-数据结构与命令/99-阶段1复习检查点|99-阶段1复习检查点]] — 阶段验收
- [[../00-Redis学习路径总索引]] — 返回总索引
- [[阶段一-数据结构与命令/03-Pub-Sub与Stream消息]] — 既有深度参考

---

## ✅ 自检清单

### 🟢 基础（必须掌握）

- [ ] 能用 `SUBSCRIBE` 和 `PUBLISH` 实现实时消息
- [ ] 能用 `XADD` 和 `XREAD` 实现基本消息队列
- [ ] 能创建消费者组并读取消息
- [ ] 能用 `XACK` 确认消息消费

### 🟡 进阶（综合运用）

- [ ] 能用 `XPENDING` 和 `XCLAIM` 处理死信
- [ ] 能用 `XTRIM` 或 `MAXLEN` 控制 Stream 长度
- [ ] 能用 Python 实现完整的消费者组流程
- [ ] 能对比 Pub-Sub、List、Stream 的优劣

### 🔴 挑战（独立判断）

- [ ] 能设计一个支持 Exactly-Once 的消息方案
- [ ] 能分析 Stream 消费者组的可靠性与 Kafka 的对比
- [ ] 能设计 Stream 的监控与告警方案

---

*最后更新：2026-07-20*
