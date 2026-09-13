---
lark_doc_url: https://my.feishu.cn/docx/EomAdNCudo5cGSxiI6actxnKnfc
---
# AppEegServer（数眠）项目面试 QA

> 项目定位：脑电（EEG）驱动的睡眠监测、分期与音乐辅助治疗平台
> 技术栈：Python 3.9.6 + Django 3.2.8 + Channels 4.0.0 + Celery 5.2.7 + MySQL + Redis + AutoGluon
> 架构：Web/API 层（本项目） → RPC 调用 → [[EegServerAI-异步技术栈分析|AI 计算服务（EegServerAI）]]（独立部署）

---

## 一、核心 10 问

### 1. 高可用是怎么保证的？集群部署还是其他手段？

**项目实际情况**：Docker Compose 单机部署（nginx:9055 + api:9057），非 K8s 多节点集群。

**面试话术**：
> 当前阶段采用 Docker Compose 容器化部署，保证环境一致性和快速恢复——容器挂了 Docker daemon 自动拉起。Redis 做 Celery 的 Broker，任务不丢。MySQL 用阿里云 RDS，自带主从热备 + 自动故障切换。

**💡 话术拆解——这三个具体是什么？**

**① Docker daemon 自动拉起**

`dockerd` 是 Docker 的后台守护进程，管理所有容器的生命周期。所谓"自动拉起"，靠的是容器重启策略（restart policy）：

| 策略 | 行为 |
|---|---|
| `no` | 挂了不重启（默认） |
| `always` | 无论什么原因退出都重启；daemon 重启后也会把容器拉起来 |
| `on-failure[:N]` | 仅非 0 退出码才重启，可限制重试次数 |
| `unless-stopped` | 类似 always，但手动 `docker stop` 后不再自动重启 |

项目 `docker-compose.yml` 中配 `restart: always`，容器崩溃时 daemon 立刻拉新实例。注意：这解决的是**进程级故障**（OOM、异常退出），不是物理机宕机——单机部署做不到真正的跨主机高可用。

**② Redis 做 Celery Broker**

Celery 的三层模型：

```
[Producer: Django]  →  [Broker: Redis]  →  [Worker: Celery Worker]
 task.delay(args)      消息队列(存任务)     拉任务 → 执行 → 存结果
```

- Producer 把任务序列化成 JSON，push 到 Redis **List**
- Worker 用 **BRPOP** 阻塞式 pop 任务来执行
- 结果通过 `result_backend` 也存回 Redis

为什么用 Redis 而不是 RabbitMQ？部署简单、内存极快、支持 Pub/Sub（广播/撤销任务）。但 Redis 做 Broker 是"尽力而为"——如果 Redis 挂了且没开持久化，队列中的任务会丢。真正"任务不丢"的场景（金融、订单）应该用 RabbitMQ 的 ACK 确认机制。

**💡 Broker 到底是什么？**

类比理解：

```
你(Producer) → 写字楼前台(Broker/Redis) → 保洁阿姨(Worker)
  "打扫301"       把任务记在本子上           "哦，301 要打扫"
```

Broker 就是那个**消息中转站**——Producer 和 Worker 不直接通信，都跟 Broker 打交道。收益是：
- **解耦**：Producer 不需要知道 Worker 在哪、有几个、是否在线
- **缓冲**：任务多的时候在队列里排队，Worker 一个一个消化
- **可靠传递**：任务消息存在 Redis 里，Worker 取走后确认，不会重复消费

技术上（Redis 做 Broker 时）：
- Celery 把任务消息（函数名 + 参数）序列化成 JSON，用 **LPUSH** 推进 Redis List
- Worker 用 **BRPOP** 阻塞等待——有任务就取走，没任务就睡着，不浪费 CPU
- 结果写回 Redis（`result_backend`），Producer 通过 `AsyncResult(task_id).get()` 拿返回值

**⚠️ 面试陷阱：Celery 在本项目的真实情况**

面试话术中提了「Redis 做 Celery 的 Broker，任务不丢」，但**实际上项目的 Celery 任务在当前版本已被废弃**：

| 阶段 | Celery 用途 | 状态 |
|---|---|---|
| **V1.0** | `eeg_judge_task.delay(data)` —— ML 睡眠分期异步推理 | 核心链路 |
| **V2.0（当前）** | 全部改为 RPC 调用 EegServerAI | **已注释，不再使用** |

证据在笔记 [[AppEegServer 项目面试 QA#2.12 为什么注释了这么多代码？项目维护状态如何？|QA 2.12]]：`views.py` 中 Celery 调用全被注释，`consumers.py` 标注「此文件代码已废弃」。

**面试时如果被追问"现在有哪些 Celery 任务在跑？"，安全回答**：

> V1.0 我们用 Celery + Redis Broker 做 ML 推理的异步任务队列——30 秒一批 EEG 数据来了，`delay()` 推到 Redis 队列，Worker 取走跑模型然后返回。V2.0 把 AI 推理拆成独立服务改走 RPC，核心链路现在不用 Celery 了。但 **Redis 本身还在用**——做 EEG 实时数据缓存（Sorted Set，10 分钟窗口）、WebSocket channel layer（Pub/Sub）、用户 Token 存储。

这样自然地把话题从"Celery 被废弃"转到"Redis 在项目里的多重角色"，展示对架构演变的完整理解。

**③ MySQL 阿里云 RDS——主从热备 + 自动故障切换**

**RDS（Relational Database Service）**：阿里云托管 MySQL，省去安装、升级、备份、监控。你拿到一个连接地址，后面的运维全自动。

**主从热备**：

```
[主库 Master]  ← App 写这里，产生 binlog
     |
     | binlog 实时同步（半同步复制）
     |
[从库 Slave]  ← 随时在线：分担读流量 + 随时可顶上
```

**自动故障切换流程**：

1. Master 宕机 → RDS 管控系统 10~30s 内检测到
2. 自动把 Slave 提升为新 Master
3. RDS 连接地址的 DNS/VIP 自动指向新 Master
4. App 重连即可（连接池自带重试），对应用层几乎透明
5. 原 Master 恢复后自动变成新 Slave

代码里不需要任何额外逻辑，连的始终是同一个地址。

**④ Redis Pub/Sub —— 贯穿项目的消息广播机制**

前面提到 Redis 在本项目中承担 Celery Broker、EEG 缓存、channel layer、Token 存储四个角色。其中 channel layer 和 EegServerAI 的实时数据接收都依赖同一个底层机制：**Redis Pub/Sub**。面试中很可能被追问「Pub/Sub 和消息队列有什么区别？」

**Pub/Sub 是什么？**

```
Publisher(发布者)                Subscriber(订阅者)
     │                                │
     │  PUBLISH channel msg           │  SUBSCRIBE channel
     ▼                                ▼
  ┌─────────────────────────────────────────┐
  │              Redis Pub/Sub               │
  │  消息来了 → 立即广播给所有订阅者 → 消息消失   │
  │  没人订阅？→ 消息直接丢弃，不存            │
  └─────────────────────────────────────────┘
```

三个关键特征：
- **即时投递**：消息不存盘、不排队，有订阅者就立即推送
- **一对多广播**：同一个 channel 的所有订阅者都收到相同消息（fan-out）
- **无持久化**：消息发出即忘——没有订阅者在线就永久丢失，事后连不上也拿不到

**Pub/Sub vs 消息队列（List/BRPOP）—— 面试高频对比**

| 维度 | Pub/Sub | 消息队列（Celery Broker） |
|---|---|---|
| **Redis 实现** | `PUBLISH` + `SUBSCRIBE` | `LPUSH` + `BRPOP` |
| **消费模式** | 一对多广播（所有订阅者都收到） | 一点一消费（一个 worker 取走，其他 worker 看不到） |
| **消息持久化** | ❌ 不存储，发完即丢 | ✅ 存在 List 中，worker 取走前一直保留 |
| **离线重连** | ❌ 断开期间的消息永久丢失 | ✅ 消息在队列里等着，worker 上线就能取 |
| **典型场景** | 实时推送、通知广播、跨进程通信 | 异步任务队列、削峰填谷 |
| **本项目中** | WebSocket channel layer + EEG 实时流 | Celery 异步任务（V1.0，V2.0 已废弃） |

一句话区分：**Pub/Sub 像广播电台**（没开机就听不到），**消息队列像快递柜**（放进去等你来取）。

**本项目两处用到 Pub/Sub**

**场景一：Django Channels 的 channel layer**

```
服务器实例 A                    服务器实例 B
     │                               │
Consumer A1 ←──WebSocket──→ 用户甲   Consumer B2 ←──WebSocket──→ 用户乙
     │                               │
     └──── 通过 Redis Pub/Sub ────────┘
              channel: "通知推送"

管理员 → HTTP → 服务器A → PUBLISH "通知推送" msg
                          → 服务器B 的 Consumer B2 收到
                          → push 给用户乙的 WebSocket
```

Channels 把每个 Consumer 实例注册为 Redis Pub/Sub 的订阅者。当需要跨实例发消息时，Redis 通过 Pub/Sub 广播到所有实例，目标 Consumer 收到后推给客户端。

**场景二：EegServerAI 的实时 EEG 数据接收**（详见 [[EegServerAI-异步技术栈分析]]）

```
IoT 网关                               EegServerAI
   │                                      │
   │  PUBLISH eegstream:2714             │  PSUBSCRIBE eegstream:*
   │  (gzip 压缩的 250Hz 脑电数据)         │  (通配符匹配所有用户频道)
   ▼                                      ▼
         ┌──────────────────────────┐
         │       Redis Pub/Sub       │
         └──────────────────────────┘
```

这里用 `PSUBSCRIBE eegstream:*` 做模式匹配——一条订阅命令就能接收所有用户的 EEG 数据（`eegstream:2714`、`eegstream:3001`……），每个用户一个 channel。数据到达后通过 asyncio 非阻塞读取，进入 AI 计算管线。

Pub/Sub 在这里特别合适：EEG 数据是实时流，每条数据只消费一次且时效性极强——0.5 秒前的数据已经没有分析价值了。如果 IoT 网关发了数据但 AI 服务没在线监听，说明系统已经故障了，丢失这些数据也是可以接受的。

**Pub/Sub 不适合的场景**：金融交易（必须 ACK）、订单处理（必须持久化）、离线消息（可能几小时后才消费）。这些场景应该用 RabbitMQ 或 Kafka。

**延伸可说的**：
- 如果上集群，会用 K8s + HPA 做水平自动扩缩
- WebSocket 长连接场景下的高可用需要 IP 哈希 / session affinity，否则 Channels 的连接状态会丢
- 独立 AI 服务器可以多实例 + 负载均衡

---

### 2. 响应速度控制在 200ms 以内，具体做了什么？

| 手段 | 具体做法 |
|---|---|
| **Redis 缓存热点数据** | 用户 EEG 数据用 Redis Sorted Set 缓存，10 分钟滑动窗口 |
| **异步非阻塞 I/O** | Django ASGI + Channels + aioredis，Web 请求不阻塞 |
| **AI 计算走 RPC** | ML 推理（睡眠分期、放松度）从本地 Celery 拆到独立 AI 服务器，Web 层只负责收发 |
| **数据库写入异步化** | `nowait(execute(...))` 模式——写操作不等待返回，直接返回响应 |
| **数据压缩** | 原始 EEG 数据 gzip+base64 压缩后存储，减少网络传输和磁盘 I/O |
| **连接池** | Redis 使用连接池复用（`max_connections=500`），避免频繁建连 |

关键代码证据：`app/views.py` 中 AI 判断走 `rpc_res(ai_host + "...")` 而非本地 Celery，`app/utils.py` 中大量 `nowait(execute(...))` 即发即弃。

---

### 3. Redis 和 MySQL 数据一致性怎么保证？

**数据流**：
```
EEG数据(每秒250个采样点) → Redis ZADD(10分钟TTL) → 同时 nowait 写 MySQL
```

**一致性策略**：

| 模式 | 说明 |
|---|---|
| **双写 + 异步持久化** | Redis 写入同时 `nowait` 写 MySQL，Redis 只保留最近 10 分钟数据（通过 `zremrangebyscore` 清理） |
| **Redis 是热数据窗口** | Redis 不是 MySQL 的缓存副本，而是时间窗口内的实时数据暂存区 |
| **最终一致性** | 查询实时 EEG 数据走 Redis（毫秒级），查询历史报告走 MySQL，两者数据时间维度不重叠 |

**面试话术**：
> 这里不是传统"缓存 DB 双写"场景，Redis 承担的是"最近 10 分钟实时流数据"的角色，历史数据才落 MySQL，两者按时间维度隔离，不存在强一致性冲突。

---

### 4. 缓存穿透、缓存雪崩、缓存击穿怎么解决？

| 问题 | 定义 | 解决方案 |
|---|---|---|
| **穿透** | 查一个数据库根本不存在的数据，缓存没有，每次都打到 DB | 布隆过滤器提前拦截；对不存在的数据也缓存一个空值（短 TTL） |
| **雪崩** | 大量缓存在同一时刻集体过期，所有请求打到 DB | 过期时间加随机值（±5 分钟）；多级缓存；限流降级 |
| **击穿** | 某个热点 key 过期瞬间，大量并发请求同时打到 DB | 互斥锁（Redis SETNX）只让一个请求去查 DB 并重建缓存；或永不过期 + 异步刷新 |

---

### 5.（追问）缓存击穿总会出现吧，具体怎么办？

> 确实很难完全避免，但可以通过**互斥锁 + 逻辑过期时间**来兜底。查不到缓存时，用 SETNX 抢锁，抢到的去查 DB 重建缓存，抢不到的返回旧值或稍等重试。核心是保证同一时刻只有一个请求落到 DB。

**伪代码**：
```python
def get_data(key):
    value = redis.get(key)
    if value is not None:
        return value
    # 抢锁
    if redis.setnx(key + ":lock", 1):
        try:
            value = db.query(...)
            redis.set(key, value, ex=3600 + random.randint(0, 300))
        finally:
            redis.delete(key + ":lock")
    else:
        time.sleep(0.1)  # 或返回旧值
        return get_data(key)
    return value
```

---

### 6. 异步代码里调用第三方同步阻塞函数，怎么防止卡死事件循环？

**标准答案**：

- **`run_in_executor`**：把同步函数丢到线程池执行，不阻塞事件循环
  ```python
  loop = asyncio.get_running_loop()
  result = await loop.run_in_executor(None, sync_blocking_func, arg)
  ```
- **`asyncio.to_thread`**（Python 3.9+）：更简洁的封装
  ```python
  result = await asyncio.to_thread(sync_blocking_func, arg)
  ```
- 如果 CPU 密集型（如 ML 推理），用 **`ProcessPoolExecutor`** 代替 `ThreadPoolExecutor`（绕过 GIL）
- 设置 **超时**：`asyncio.wait_for(task, timeout=5.0)` 避免无限等待

**项目中的实际做法**：把 ML 推理从本地同步调用改成了**远程 RPC**（`ai/utils.py` 中的 `rpc_res` 函数），将 CPU 阻塞风险转移到了独立 AI 服务，Web 层只做异步网络 I/O。

---

### 7. Python 3.6+ 字典有序的底层重构

**核心改动**：Python 3.6 用 **compact dict（紧凑字典）** 替代了旧的稀疏哈希表。

**旧实现**（Python 3.5 之前）：
- 一个 entries 数组，存 `[hash, key, value]` 三元组
- 哈希冲突靠开放寻址
- 内存碎片多，遍历慢，无序

**新实现**（Python 3.6 CPython 实现，3.7 成为语言规范）：

```
indices = [None, 0, None, 1, None, 2, None, None]  # 稀疏索引表
entries = [
    [hash0, key0, value0],  # index 0
    [hash1, key1, value1],  # index 1
    [hash2, key2, value2],  # index 2
]
```

**为什么省内存**：
- `indices` 是 `int8/16/32/64` 动态选择（取决于 dict 大小），比原来存完整 entry 小很多
- `entries` 紧密排列不浪费空间
- 删除时不立刻缩容

**为什么保序**：
- `entries` 按插入顺序 append，遍历时直接顺序扫描 entries
- Python 3.7 正式将「插入有序」写入语言规范

---

### 8. LEFT JOIN vs RIGHT JOIN

```
LEFT JOIN  → 以左表为准，右表匹配不上的填 NULL
RIGHT JOIN → 以右表为准，左表匹配不上的填 NULL
```

**示例**（查询所有用户的睡眠报告，不管有没有 EEG 数据）：
```sql
SELECT u.name, r.sleep_duration
FROM tb_user u
LEFT JOIN tb_sleepdata_report r ON u.id = r.user_id
```

**面试技巧**：补充一句——实际工作中不太用 RIGHT JOIN，因为把主表放左边用 LEFT JOIN 可读性更好，效果等价。

---

### 9. MySQL 索引有哪几种？

| 分类维度 | 类型 | 说明 |
|---|---|---|
| 物理存储 | **聚簇索引** (Clustered) | 数据按索引顺序物理存储，一张表只有一个（InnoDB 主键） |
| 物理存储 | **非聚簇索引** (Secondary) | 叶子节点存主键值，需要回表 |
| 逻辑 | **主键索引** (Primary Key) | 唯一 + 非空 |
| 逻辑 | **唯一索引** (Unique) | 唯一可为空 |
| 逻辑 | **普通索引** (Normal / B+Tree) | 加速查询 |
| 逻辑 | **全文索引** (Fulltext) | 大文本搜索 |
| 逻辑 | **联合索引** (Composite) | 多列组合，遵循最左前缀原则 |
| 数据结构 | **B+Tree** | InnoDB 默认 |
| 数据结构 | **Hash** | Memory 引擎，等值查询快 |

**常见追问**：
- 覆盖索引：查询列都在索引中，不需要回表
- 索引下推（ICP）：MySQL 5.6+，在索引层就过滤掉不符合条件的记录
- 联合索引的最左前缀：`(a, b, c)` 索引，查 `a` 或 `a,b` 或 `a,b,c` 都能用到，但单独查 `b` 或 `c` 不行

---

### 10. 数据库事务隔离级别（4 种）

| 级别 | 脏读 | 不可重复读 | 幻读 | InnoDB 默认 |
|---|---|---|---|---|
| **READ UNCOMMITTED** | ✅ 会发生 | ✅ | ✅ | |
| **READ COMMITTED** | ❌ | ✅ | ✅ | |
| **REPEATABLE READ** | ❌ | ❌ | ❌(MVCC 解决大部分) | ✅ MySQL InnoDB |
| **SERIALIZABLE** | ❌ | ❌ | ❌ | |

**InnoDB 的实现细节**：
- REPEATABLE READ 下用 **MVCC（多版本并发控制）** + **间隙锁（Gap Lock）** 解决了大部分幻读
- 每个事务看到的是一个一致性视图（Read View）
- 快照读（普通 SELECT）用 MVCC，当前读（SELECT ... FOR UPDATE）用 Next-Key Lock

---

## 二、延伸 12 问

### 2.1 大量 WebSocket 长连接怎么管理？Channels 的 channel layer 用的什么 backend？

**项目实现**（`app/routing.py`）：
```python
# 路由配置
websocket_urlpatterns = [
    re_path(r'ws/eeg/$', EegConsumer.as_asgi())
]
```

**Consumer 生命周期**（`app/consumers.py`）：
- `websocket_connect`：Token 鉴权 → 通过则保持连接
- `websocket_receive`：根据 type 分发 → `second`（秒级数据）/ `stage`（分期）/ `eyes`（睁闭眼）
- `websocket_disconnect`：主动断开

**连接管理策略**：

| 层面 | 方案 |
|---|---|
| **连接数限制** | ASGI 服务器（Uvicorn/Daphne）配置 worker 数和最大连接数 |
| **心跳保活** | 客户端定时发送数据本身就是心跳，超时未收到数据判为断连 |
| **水平扩展** | Channels 的 channel layer 用 **Redis Pub/Sub**，多实例间可以广播消息 |
| **故障恢复** | 客户端重连 + `sleep_id` 继续追加数据 |

**项目中的 channel layer 配置**：使用 Redis 作为 backend（默认 Pub/Sub 模式），生产环境建议用 `channels_redis` 包。

**💡 追问拆解：Channel Layer / WebSocket / HTTP / 普通 Socket 到底有什么区别？**

这是一个常见的概念混淆点，四个东西处于不同抽象层级：

**① Channel Layer 是什么？**

Channel layer 不是连接协议，而是 Django Channels 的**内部消息路由系统**。先把它跟 WebSocket 分清：

```
手机App ←──WebSocket 连接──→ Django Consumer 实例
                                  │
                                  │ 通过 channel layer 通信
                                  ▼
                            Redis Pub/Sub
                                  │
                                  ▼
                          另一个 Consumer 实例（可能在不同服务器上）
```

它的作用是让**不同进程/不同机器的 Consumer 之间能互发消息**。比如：
- 管理员在 Web 后台点「推送通知」→ HTTP 请求到 Server A → Server A 通过 channel layer 广播 → Server B 上的 WebSocket Consumer 收到 → 推给连在 Server B 的用户
- 单机部署时 channel layer 作用不大（都在一个进程里），多实例水平扩展时才是关键

**② Channel Layer ≠ WebSocket 连接**

很多人把这俩混为一谈。简单说：**WebSocket 是对外的（客户端↔服务器），Channel Layer 是对内的（服务器内部组件之间）。**

```
对外：客户端 ←────WebSocket────→ 服务器 (Channels Consumer)
对内：Consumer A ←──Channel Layer (Redis)──→ Consumer B
```

**③ WebSocket vs HTTP**

| 维度 | HTTP | WebSocket |
|---|---|---|
| **通信模式** | 请求-响应（一问一答） | 全双工（双方随时可以发） |
| **谁发起** | 永远是客户端先请求 | 连接建立后，服务器也可以主动推 |
| **连接生命周期** | 一次请求→一次响应→断开 | 一次握手→持久连接→显式关闭 |
| **状态** | 无状态（每个请求独立） | 有状态（连接上下文保持） |
| **头部开销** | 每次请求带完整 HTTP 头（~500-800 字节） | 建立后每条消息仅 2-14 字节帧头 |
| **典型场景** | 页面加载、REST API | 实时推送、聊天、EEG 数据流 |

关键差异用一句话：**HTTP 是你问我才答，WebSocket 是我可以主动告诉你。**

在这个项目里为什么必须用 WebSocket？
- 手机 App 每秒上传 250 个 EEG 采样点，是**持续的实时数据流**
- 如果用 HTTP，每秒要发 250 个 POST 请求——HTTP 头部开销远大于实际数据，服务器也会被连接风暴打死
- WebSocket 建立一条连接后，数据变成轻量帧，持续收发，效率高几个数量级

**④ WebSocket vs 普通 Socket（原始 TCP Socket）**

| 维度 | 原始 TCP Socket | WebSocket |
|---|---|---|
| **OSI 层级** | 传输层（L4） | 应用层（L7），**跑在 TCP 之上** |
| **浏览器支持** | ❌ 浏览器不暴露原始 TCP | ✅ `new WebSocket("wss://...")` 原生 API |
| **消息边界** | 无（就是一串字节流，要自己拆包粘包） | 有（自带消息帧，每个 frame 有 opcode + payload） |
| **握手** | 三次握手后直接发字节 | 先走 HTTP Upgrade 握手，再切协议 |
| **安全** | 需要自己实现加密 | WSS = WebSocket over TLS，一键开启 |
| **穿透代理/防火墙** | 容易被拦截 | 因为用 HTTP 端口（80/443）+ HTTP Upgrade，通常能穿透 |

一句话：**WebSocket 是封装在 TCP 之上的高级协议**。你不需要处理"这条消息从哪到哪、有没有收完整"——浏览器和库都帮你做好了。原始 Socket 你得手动解决拆包、心跳、协议设计、加密等所有问题。

**⑤ 四者关系总结**

```
┌─────────────────────────────────────────────┐
│  应用层    WebSocket 协议 (有帧、有opcode)     │
│             ↑ 基于 HTTP Upgrade 建立           │
│             │                                 │
│  中间层    Channel Layer (Django内部消息路由)   │
│             ↑ 用 Redis Pub/Sub 实现            │
│             │                                 │
│  传输层    TCP Socket (原始字节流)              │
│             ↑ 三次握手、可靠传输                │
│             │                                 │
│  对比      HTTP (请求-响应，一问一答)            │
└─────────────────────────────────────────────┘
```

面试时如果被问"Channel Layer 和 WebSocket 是一回事吗？"，一句话回应：
> 不是。WebSocket 是客户端和服务器之间的持久连接协议，Channel Layer 是 Django Channels 内部 Consumer 之间的消息路由机制。一个对外，一个对内。

---

### 2.2 为什么把 Celery 改成 RPC？改造过程中遇到什么坑？

> 🔗 RPC 调用方的内部实现详见 [[EegServerAI-异步技术栈分析]]——asyncio + aioredis + aiomysql 实时数据处理管线。

**为什么改**：
1. **解耦**：ML 模型更新不影响 Web 服务，独立部署独立扩缩容
2. **Celery 在 Windows 上不稳定**（gevent pool 兼容问题）
3. **调试困难**：Celery Worker 的错误日志与 Web 层分离，排查链路长
4. **同步任务不适用**：Celery task 定义为同步函数，在 async Django 中需要额外适配

**改造过程**：

```
[旧] 手机App → WebSocket → Django → eeg_judge_task.delay(data) → Celery Worker → ML模型 → 返回结果
[新] 手机App → WebSocket → Django → rpc_res(ai_host + "/app/ai/data_judge/") → 独立AI服务器 → 返回结果
```

**关键代码**（`ai/utils.py`）：
```python
async def rpc_res(url, data):
    async with aiohttp.ClientSession() as client:
        async with client.post(url, data=json.dumps(data)) as res:
            if res.status == 200:
                return json.loads(await res.text())
            return "task error"
```

**遇到的坑**：
- RPC 超时控制：AI 推理有时超过 5 秒，需要加 timeout
- 网络抖动重试：RPC 调用失败时需要 fallback 到备选模型路径
- 灰度切换：在 `views.py` 中用 try/except 兜底——先调 AG 模型，失败降级到旧模型

**💡 追问：如果远程 RPC 迟迟不回应怎么办？**

这是分布式系统中经典的「下游不可靠」问题。当前代码的 `rpc_res()` 其实**没有设 timeout**——一旦 AI 服务卡住，调用方会无限等待。生产环境需要**多层防御**：

**第一层：客户端超时（最基本）**

```python
async def rpc_res(url, data, timeout=5.0):
    timeout_obj = aiohttp.ClientTimeout(total=timeout)
    async with aiohttp.ClientSession(timeout=timeout_obj) as client:
        async with client.post(url, data=json.dumps(data)) as res:
            ...
```

不加 timeout 的后果：一个慢请求可以阻塞整个事件循环中的协程，导致**所有用户的 EEG 数据处理都卡住**。在 asyncio 中，`await` 一个永不返回的协程 = 当前 Task 永久挂起。

**第二层：重试 + 退避（应对网络抖动）**

```
第 1 次调用 → 超时 → 等 1s → 第 2 次 → 超时 → 等 2s → 第 3 次 → 成功 ✓
```

```python
async def rpc_res_with_retry(url, data, max_retries=3, base_timeout=5.0):
    for attempt in range(max_retries):
        try:
            return await rpc_res(url, data, timeout=base_timeout * (2 ** attempt))
        except (asyncio.TimeoutError, aiohttp.ClientError):
            if attempt == max_retries - 1:
                raise  # 最后一次也失败了，向上抛
            await asyncio.sleep(2 ** attempt)  # 指数退避：1s, 2s, 4s
```

注意：**重试要有上限**，且每次超时时间递增。否则一个服务挂了，所有请求线程都在疯狂重试，把自己也拖垮。

**第三层：熔断（断路器 / Circuit Breaker）**

如果 AI 服务连续失败 N 次，说明它大概率挂了。继续重试不仅无意义，还会让 Web 服务也堆积大量等待中的协程，造成**雪崩**。

```
状态流转：
  [Closed 正常] ──连续5次失败──→ [Open 熔断] ──30秒后──→ [Half-Open 半开]
       ↑                                                       │
       └──────────── 调用成功 ←────────── 放行一个试探请求 ──────┘
                   (恢复到Closed)           失败 → 重新Open
```

```python
class CircuitBreaker:
    def __init__(self, failure_threshold=5, recovery_timeout=30):
        self.failure_count = 0
        self.last_failure_time = None
        self.state = "closed"  # closed | open | half_open

    async def call(self, fn, *args):
        if self.state == "open":
            if time.time() - self.last_failure_time > self.recovery_timeout:
                self.state = "half_open"
            else:
                raise Exception("熔断中，拒绝请求")

        try:
            result = await fn(*args)
            self.failure_count = 0
            self.state = "closed"  # 成功了，恢复正常
            return result
        except Exception:
            self.failure_count += 1
            self.last_failure_time = time.time()
            if self.failure_count >= self.failure_threshold:
                self.state = "open"
            raise
```

**第四层：降级策略（兜底）**

RPC 彻底不通时，不能给用户一个 500 错误——需要有**有损但可用的**兜底方案。项目里实际上有三种思路：

| 降级方式 | 做法 | 项目中的体现 |
|---|---|---|
| **模型降级** | 从 AG 模型（AutoGluon，精度高但慢）降级到旧模型（简单规则，快但精度略低） | `views.py` 中 try/except：先调 AG，失败走旧模型 |
| **结果缓存** | 返回该用户最近一次成功的分析结果，标注"基于上次数据" | 可利用 Redis 中已有的 10 分钟 EEG 缓存 |
| **优雅降级响应** | 返回 `{"status": "pending", "msg": "分析中，稍后重试"}`，不阻塞客户端 | WebSocket 天然支持异步推送，结果好了再推 |

**第五层：对用户的影响最小化**

本项目独有的优势：EEG 数据通过 **WebSocket 双向通信**。这比 HTTP 的请求-响应模式更容易处理慢 RPC：

```
HTTP 模式（糟糕）:
  客户端请求 → 服务器等 RPC(5s) → 用户盯着转圈 → 超时 → 再试

WebSocket 模式（本项目）:
  客户端持续发 EEG 数据 →
  服务器收到 30 秒数据 → RPC 异步调用 → 不阻塞后续数据接收
  结果回来后 → WebSocket push 给客户端
```

关键：**不要让用户的 EEG 数据流被 RPC 的响应时间阻塞**。这就是为什么项目里 EEG 数据接收和 AI 分析是解耦的——数据照样收、照样缓存，分析结果异步返回。

**六层防御全景图**：

```
RPC 请求发起
    │
    ▼
[1. 客户端超时] ──超时──→ [2. 重试+退避] ──耗尽──→ [3. 熔断器检查]
    │                        │                        │
   成功                      成功                   Circuit Open?
    │                        │                   ┌────┴────┐
    ▼                        ▼                  是          否
  返回结果                 返回结果          [4. 降级兜底]  放行（可能再次超时）
                                            │
                                     ┌──────┼──────┐
                                   模型降级 结果缓存 优雅降级
```

**面试时的回答思路**：

> RPC 超时不回应，我有四层防御。第一层是 `aiohttp.ClientTimeout` 设 5 秒超时，不无限阻塞事件循环。第二层是指数退避重试，最多 3 次。第三层是熔断器——连续失败 5 次后直接断路 30 秒，避免雪崩。第四层是业务降级——本项目在 `views.py` 里用 try/except 兜底，AG 模型挂了自动切到旧模型。另外由于 EEG 数据走 WebSocket 而非 HTTP，RPC 慢不会阻塞数据流——结果好了再异步推给客户端。

> 🔗 多服务同时调 RPC 时，怎么知道哪个响应对应哪个请求？见 [[AppEegServer 项目面试 QA#2.13 多个服务同时调 RPC，AI 服务怎么知道响应对应哪个请求？|QA 2.13：RPC 请求-响应匹配]]。

---

### 2.3 两阶段分期具体怎么做？决策树的 PSD 阈值怎么调的？

> 🔗 此算法在 AI 服务（EegServerAI）的 `ai/eeg.py` → `EEGJudge.eeg_judge()` 中执行。该服务的异步调度管线见 [[EegServerAI-异步技术栈分析]]。

**第一层：决策树（PSD 功率谱密度阈值）**，核心代码 `ai/eeg.py` 中的 `EEGJudge.eeg_judge()`：

```
输入：30秒脑电数据（7500个采样点，250Hz采样率）
    ↓
1. MNE 带通滤波（1~20Hz）
    ↓
2. Multitaper 计算 2/3/4/12/16 Hz 的 PSD 功率值
    ↓
3. 决策树判断：
   ├─ P16 > 3000（L1） → 清醒（0）
   ├─ P12 > 3000（L2） → 清醒（0）
   ├─ P2+P3+P4 < 30000（L3）→ REM（1）
   └─ 以上都不满足 → 进入 ML 模型（-1）
    ↓
4. ML 模型（Random Forest）判断：浅睡（2） or 深睡（3）
```

**阈值调优**：
- L1、L2、L3 来自已有数据的归纳统计（注释中写"根据已有数据归纳得到"）
- 本质是经验参数，调整依据是正样本/负样本的分布边界
- 如果重新调优，可以用网格搜索 / ROC 曲线找最优截断点

---

### 2.4 14 维标签向量是什么？SVD+标签混合推荐的冷启动怎么处理？

**14 维标签向量（脑纹）本质**：

每个音乐的 `character_tag` 表中有 14 个标签（如：舒缓、轻快、低沉……），每个标签有一个权重值。用户听音乐时分析其脑电反应，匹配到对应的标签维度，形成该用户的**脑纹画像**。

**SVD 推荐流程**（`app/Music/music_utils.py`）：

```
用户听音乐日志 → 用户-音乐评分矩阵 → SVD 矩阵分解（K=min(rows, cols)-1）
    → 计算预估评分矩阵 → 对每个用户推荐 Top-10 音乐
```

**混合推荐公式**：
```
最终推荐 = SVD 协同过滤结果 × 权重 + 标签匹配结果 × 权重
```

**冷启动处理**：
- 新用户：先用标签匹配（脑纹标签）推荐，积累听歌数据后切换到 SVD
- 新音乐：用 `add_tag` / `minus_tag` 的标签权重作为初始分数
- 音乐分数计算公式：`score = Σ(add_tag权重) + Σ(minus_tag权重)`

---

### 2.5 EEG 数据属于医疗敏感数据，加密和隐私合规怎么做的？

**传输层**：HTTPS + WebSocket Secure (WSS)

**存储层**（`utils/tool.py`）：
```python
def encode_data(data):
    bytes_data = gzip.compress(str(data).encode("utf-8"))
    base64_data = base64.b64encode(bytes_data)
    return str(base64_data.decode())
```
原始脑电数据 **gzip 压缩 + base64 编码** 后存入 MySQL（目的是压缩体积 + 避免二进制存储问题，非加密）

**业务层**：
- Token 鉴权（80 位 token，每次请求验证）
- 用户手机号加密存储（`decrypt_headers(e_user_id, 4)`）
- 中间件层统一拦截未授权请求（`VerifyMiddleware`）

**合规方面**（面试时可补充）：
- 医疗数据需遵循《个人信息保护法》和《健康医疗大数据标准》
- 生产环境数据库访问需审计日志
- 用户有权导出/删除自己的数据

---

### 2.6 Django 3.2 的 async view 有什么坑？跟同步中间件混用有什么问题？

**项目中的混合同步/异步设计**：
- ASGI 模式下 HTTP 走 `get_asgi_application()`
- 但同步中间件（Django 内置的 SessionMiddleware、CsrfViewMiddleware 等）在 async 上下文中运行时会自动用 `sync_to_async` 包装

**常见坑**：

| 坑 | 说明 |
|---|---|
| **同步 ORM 调用** | Django 3.2 的 ORM 仍是同步的，在 async view 中直接调用会阻塞事件循环，需要用 `database_sync_to_async` 包装；项目中使用自研的 `utils/aiosql.py`（aiomysql 异步适配层）解决 |
| **中间件混用** | 项目中 `VerifyMiddleware.process_view` 是 `async def`，但 `RequestMiddleware.process_request` 是 `def`——Django 会自动用 `sync_to_async` 包装同步中间件，但性能有损耗 |
| **`nowait` 模式** | 项目大量使用 `nowait(execute(...))` —— 创建新的事件循环来执行 DB 写入，避免了 await 阻塞，但丢失了错误处理 |

**`nowait` 的风险**：写入失败不会反馈给用户，适合 EEG 数据的"尽力写入"场景，但不适合关键业务（如订单支付）。

> 🔗 `nowait()` 在 AI 服务（EegServerAI）中的实际应用和权衡分析见 [[EegServerAI-异步技术栈分析#三、aiomysql — 异步 MySQL 驱动]]。

---

### 2.7 睡眠报告表为什么存 stage_list 的 JSON？平滑算法具体逻辑是什么？

**为什么存 JSON**：睡眠报告中的 `stage_list` 是这样的结构：
```json
[
  {"stage": 0, "start_time": "2022-01-01 23:00:00", "end_time": "2022-01-01 23:05:00", "seconds": 300},
  {"stage": 2, "start_time": "2022-01-01 23:05:00", "end_time": "2022-01-01 23:30:00", "seconds": 1500},
  ...
]
```
这是**变长数组**——每段睡眠的分期数量不同（取决于睡眠时长），无法用固定列表达。JSON 字段（MySQL 5.7+ 原生支持 JSON 类型）是最合适的存储方式。

**平滑算法 4 步流程**（`ai/eeg.py` 中的 `EEGResult`）：

```
原始分期序列（每30秒一个标签）:
  [0,0,1,0,2,2,3,2,-1,3,3,2,1,1...]

Step 1: 清醒期调整
  → 前 60 分钟每 5 分钟判断：5 分钟内出现一次清醒 → 全部改为清醒

Step 2: 滑动窗口平滑（窗口宽度=5）
  → 取 5 个切片的众数，替换末尾元素
  → 窗口右移 1 格，重复
  → -1/-2（异常状态）不参与众数计算

Step 3: 深睡期调整
  → 第一个"浅睡"之前如果出现"深睡"，改为"浅睡"

Step 4: REM期调整
  → 第一个"深睡"之前如果出现"REM"，改为"浅睡"
```

---

### 2.8 Redis 内存满了怎么办？用什么淘汰策略？

**项目中的 Redis 使用场景和对应策略**：

| 场景 | Key 模式 | 内存管理 |
|---|---|---|
| EEG 实时数据流 | `{user_id}_eeg_data` (Sorted Set) | **主动清理**：`zremrangebyscore` 只保留 10 分钟数据 |
| 用户 Token | `token_{uid}` | 设置 TTL 过期 |
| Celery Broker | 任务队列 | 结果 24 小时过期（`CELERY_TASK_RESULT_EXPIRES = 60*60*24`） |
| WebSocket Channel Layer | 消息通道 | 瞬时消息，不持久化 |

**通用策略**：
- 淘汰策略建议选 **allkeys-lru**（最近最少使用）或 **volatile-lru**（仅对设了过期时间的 key）
- 项目中最危险的是 EEG 数据 Sorted Set——如果 `zremrangebyscore` 执行失败，内存会持续增长，需要监控 + 兜底
- 连接池设置了 `max_connections=500`，防止连接数撑爆

---

### 2.9 如果要支持 100 万用户同时睡眠，架构怎么演进？

**当前瓶颈分析**：

| 瓶颈点 | 当前方案 | 100 万用户挑战 |
|---|---|---|
| WebSocket 长连接 | 单机 ASGI | 100 万 TCP 连接 = 大量内存 + 文件描述符 |
| EEG 数据写入 | Redis ZADD + MySQL | 100 万 × 250 Hz = 每秒 2.5 亿写入 |
| AI 推理 | 独立 AI 服务器 | 每 30 秒一次推理 = 每秒 3.3 万次推理请求 |
| 睡眠报告生成 | 同步平滑算法 | 100 万份报告的 CPU 消耗 |

**演进方案**：

```
                         [全局负载均衡]
                              |
              +---------------+---------------+
              |               |               |
         [API 集群]      [WS 集群]      [AI 推理集群]
         (K8s HPA)    (按 user_id 分片)   (GPU 节点池)
              |               |               |
              +-------+-------+-------+-------+
                      |               |
                  [Redis 集群]    [MySQL 读写分离]
                  (分片+哨兵)     (分库分表)
                      |
                  [Kafka/Pulsar]
                  (EEG 数据异步写入)
```

关键改造点：
1. WebSocket 按 `user_id` hash 分片到不同节点
2. EEG 写入从直写 Redis → 改为 Kafka 异步削峰
3. AI 推理按模型分池（睁闭眼检测轻量、睡眠分期重量），混合部署
4. MySQL 按 `user_id` 分表（如 1024 张表），报告查询走 ES
5. Redis 集群模式（Codis / Redis Cluster）

---

### 2.10 项目中的决策树阈值怎么调优？

**当前阈值**：
```python
L1 = 3000   # P16 > L1 → 清醒
L2 = 3000   # P12 > L2 → 清醒
L3 = 30000  # P2+P3+P4 < L3 → REM
```

**调优思路**：
1. **数据标注**：医生/专家标记真实睡眠阶段（金标准用 PSG 多导睡眠图）
2. **网格搜索**：对 (L1, L2, L3) 做三维网格扫描，最大化 F1-score
3. **ROC 分析**：对每个阈值单独画 ROC 曲线，取 Youden 指数最大点
4. **分段优化**：清醒判断（L1/L2）和 REM 判断（L3）本质是两个二分类问题，可以独立调参
5. **在线 A/B**：新阈值小流量验证后再全量上线

---

### 2.11 睁闭眼检测的全流程是怎样的？

> 🔗 此逻辑在 AI 服务（EegServerAI）的 `ai/eye_judge.py` 中独立实现，详见 [[EegServerAI-异步技术栈分析]]。

**完整流程**（`app/Music/music_utils.py` 中的 `predict_data`）：

```
3 秒 EEG 数据（750 个采样点）
    ↓
1. 带通滤波（1~36Hz）+ 陷波滤波（49~51Hz）
    ↓
2. 短时傅里叶变换（STFT，Hamming 窗）
    ↓
3. 计算 6 个频段的 PSD 功率（min/mean/median）
   → 生成 19 维特征向量
    ↓
4. 归一化（Min-Max，用预计算的 vector_max/vector_min）
    ↓
5. 距离比较（4 种距离 vs 标准睁眼向量 vector0）：
   ├─ 曼哈顿距离 > 8.6 → 睁眼
   ├─ 切比雪夫距离 > 0.54 → 睁眼
   ├─ 欧式距离 > 1.98 → 睁眼
   └─ 皮尔逊相关系数 < 0.915 → 睁眼
    ↓
6. 不满足距离阈值 → 进入 AutoGluon 模型做最终判断
```

**特点**：距离比较是硬规则（快速过滤明显睁眼），模型兜底不明确的样本——典型的「规则 + ML」架构。

---

### 2.12 为什么注释了这么多代码？项目维护状态如何？

**观察到的现象**：
- `consumers.py` 顶部标注 `# TODO 此consumers文件代码 已废弃，相关函数不再维护`
- `views.py` 中 Celery 调用全被注释
- 多处标注 `# TODO 此函数目前废弃使用，停止维护`
- `unused/` 目录专门存放弃用代码

**演变过程**：
1. **V1.0**：Celery 本地 ML 推理 + WebSocket 直连
2. **V2.0**：AI 服务独立部署 + RPC 调用 → 旧的 Celery 任务注释保留
3. **当前**：混合状态——核心链路已迁移，旧代码保留但注释废弃

**面试建议**：坦诚说明这是一次真实的架构演进，保留旧代码是为了可回滚（灰度期的安全余量），体现了工程上的务实决策。

---

### 2.13 多个服务同时调 RPC，AI 服务怎么知道响应对应哪个请求？

> 本质是分布式系统中**请求-响应匹配（Request-Response Correlation）**的问题。解法取决于通信模式——同步还是异步。

**场景还原**

```
服务 A ──► ┐
服务 B ──► ├── EegServerAI（AI 计算服务）── 计算结果
服务 C ──► ┘
```

多个调用方同时发请求，AI 服务处理完后，怎么把结果准确送回对应的调用方？

**① 同步 RPC（本项目当前模式）——HTTP 协议天然解决**

```python
# ai/utils.py — 项目的实际 RPC 调用
async def rpc_res(url, data):
    async with aiohttp.ClientSession() as client:
        async with client.post(url, data=json.dumps(data)) as res:
            return json.loads(await res.text())
```

HTTP 是**请求-响应协议**：每个请求发出去，对应的响应在同一个交互中回来。即使连接池里同时有 100 个请求在飞，每个 `await client.post()` 返回的 `res` 对象天然属于那个请求——协议栈（TCP 连接 + HTTP 消息边界）帮你做好了配对。

类比：你微信同时给 10 个人发消息，每个人回复你的时候，聊天框里自然显示了谁回了什么——不会把张三的回复当成李四的。

> ⚠️ 前提：**调用方是同步等待响应**的。也就是说 `await rpc_res(...)` 这段代码会一直等到 AI 服务返回结果才继续往下走。

**② 异步回调模式——需要 Correlation ID（请求 ID）**

如果换成这种场景：

```
服务 A 发请求 ──→ AI 服务 ──→ "收到了，结果好了我回调你"
                                    │
                               （几分钟后）
                                    │
                              callback："结果算好了"
```

回调返回的结果跟原始请求已经**脱钩了**（不在同一个 HTTP 交互里），就需要一个唯一 ID 来配对：

```
┌────────── 请求方 ──────────┐        ┌────────── AI 服务 ──────────┐
│                            │        │                             │
│ 1. 生成 request_id:        │        │                             │
│    "abc-123"               │        │                             │
│           │                │        │                             │
│ 2. 发送请求（带 ID）────────┼────────→ 3. 收到请求，提取 request_id   │
│           │                │        │           │                  │
│ 4. 记录到 pending_requests │        │ 5. 丢进任务队列（带 ID）       │
│    {"abc-123": Future}     │        │           │                  │
│           │                │        │ 6. Worker 算完，结果也带 ID    │
│           │                │        │           │                  │
│ 7. 收到回调 ◀──────────────┼──────── 8. 回调结果带 request_id        │
│           │                │        │                             │
│ 9. 根据 ID 找到 Future     │        │                             │
│    future.set_result()     │        │                             │
│           │                │        │                             │
│ 10. await future 解除阻塞  │        │                             │
│     拿到结果 ✓              │        │                             │
└────────────────────────────┘        └─────────────────────────────┘
```

**实现代码**：

```python
import uuid
import asyncio

pending_requests: dict[str, asyncio.Future] = {}

# ===== 请求方 =====
async def send_and_wait(data: dict) -> dict:
    req_id = str(uuid.uuid4())
    future = asyncio.get_event_loop().create_future()
    pending_requests[req_id] = future
    
    # Header + Body 都带上 request_id
    headers = {"X-Request-ID": req_id}
    data["request_id"] = req_id
    
    async with aiohttp.ClientSession() as client:
        await client.post("http://ai-service/submit", json=data, headers=headers)
    
    try:
        return await asyncio.wait_for(future, timeout=60.0)
    finally:
        pending_requests.pop(req_id, None)


# ===== 回调接收方（比如一个 HTTP endpoint 或 WebSocket 消息处理） =====
def on_callback_received(callback_data: dict):
    req_id = callback_data.get("request_id")
    future = pending_requests.get(req_id)
    if future and not future.done():
        future.set_result(callback_data["result"])
```

**③ 消息队列模式——Celery / Kafka 的任务 ID**

这是另一种常见的异步模式：

```
Producer ──task_id: "t-001"──→ [Broker: Redis List] ──→ Worker 取走执行
                                                              │
                                              结果写回: {"task_id": "t-001", "result": ...}
                                                              │
Producer ◀──── AsyncResult("t-001").get() ────────────────────┘
```

在你项目的 V1.0 中，Celery 就是这么做的——`eeg_judge_task.delay(data)` 返回一个 `AsyncResult` 对象，内部靠 `task_id` 匹配。V2.0 改 RPC 后不需要这个机制了，因为 HTTP 协议层已经做了配对。

**④ 全链路追踪——Trace ID（进阶）**

生产环境中不止需要「匹配请求和响应」，还需要追踪一个请求在**整个系统**中的完整路径：

```
用户请求 → Nginx → Django → RPC → EegServerAI → MySQL
                                              → Redis
```

每个环节都用同一个 **Trace ID** 串联，丢到 Jaeger / Zipkin 里一查就能看到瓶颈在哪：

| 做法 | 说明 |
|------|------|
| **Header 注入** | 上游把 `X-Trace-ID` 塞进 HTTP Header，下游提取并透传 |
| **自动埋点** | OpenTelemetry 在 `aiohttp`、`aiomysql` 等库里自动注入/提取 |
| **日志关联** | 所有日志带 `[trace_id=xxx]`，出问题时 grep 一条线 |

```
[2024-01-15 10:23:01] [trace_id=7f3a] Nginx: 收到请求
[2024-01-15 10:23:01] [trace_id=7f3a] Django: 鉴权通过, uid=2714
[2024-01-15 10:23:01] [trace_id=7f3a] RPC: POST /data_judge → EegServerAI
[2024-01-15 10:23:04] [trace_id=7f3a] EegServerAI: 分期计算完成, stage=2
[2024-01-15 10:23:04] [trace_id=7f3a] Django: 结果写库完成
[2024-01-15 10:23:04] [trace_id=7f3a] Nginx: 200 OK (总耗时 3.2s)
```

**总结对比**

| 通信模式 | 匹配机制 | 本项目中 |
|---------|---------|---------|
| **同步 RPC**（HTTP 请求-响应） | TCP 连接 + HTTP 协议栈天然对应 | ✅ V2.0 的 `rpc_res()` |
| **异步回调**（发完不管，好了通知） | Correlation ID + pending request map | ❌ 未使用 |
| **消息队列**（Producer-Broker-Consumer） | Celery `task_id` / Kafka message key | V1.0 Celery（已废弃） |
| **全链路追踪**（排查问题用） | Trace ID 跨服务透传 | 可扩展方向 |

**面试回答模板**

> 这个问题本质是请求-响应匹配。我们项目当前用的是同步 RPC——通过 aiohttp 发 HTTP 请求然后 `await` 等响应，HTTP 协议栈天然保证了请求和响应的一一对应。如果换成异步回调模式，就要引入 Correlation ID——请求方生成 UUID 塞进 Header，AI 服务回调时原样带回，调用方用一个 `pending_requests` 字典根据 ID 找到对应的 Future 来解除阻塞。更深一层是全链路追踪，用 OpenTelemetry 在每个服务间透传 Trace ID，一条链串起来方便排查问题。

> 🔗 本节与 [[AppEegServer 项目面试 QA#2.2 为什么把 Celery 改成 RPC？改造过程中遇到什么坑？|QA 2.2：Celery → RPC 改造]] 直接关联——RPC 改造后请求-响应匹配从 Celery 的 task_id 模式变成了 HTTP 协议层模式。熔断/超时/降级见 QA 2.2 的六层防御。

---

## 三、面试高频追问速查表

| 方向 | 可能的追问 |
|---|---|
| **WebSocket** | "Uvicorn 和 Daphne 怎么选的？"、"Channels 的 consumer 是同步还是异步？" |
| **Python 异步** | "asyncio 的事件循环原理？"、"协程和线程的区别？" |
| **MySQL** | "B+Tree 为什么比 B-Tree 适合做索引？"、"覆盖索引是什么？" |
| **Redis** | "Redis 持久化 RDB vs AOF？"、"Redis 集群模式有哪些？" |
| **RPC/分布式** | "多个服务同时调 RPC，怎么区分响应对应哪个请求？"、"分布式链路追踪怎么做？" |
| **ML** | "AutoGluon 和其他 AutoML 框架的对比？"、"Multitaper 和 Welch 的 PSD 估计区别？" |
