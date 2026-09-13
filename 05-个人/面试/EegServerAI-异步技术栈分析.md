---
lark_doc_url: https://my.feishu.cn/docx/CC1UdtXQBoAZ8Cxtw9AcTBgDn1e
---
# EegServerAI 异步技术栈分析：asyncio / aioredis / aiomysql

> 项目：数眠 AI 计算服务（EegServerAI）
> 场景：实时脑电波（EEG）数据流监听 → AI 睡眠分析 → 结果持久化
> 🔗 本项目是 [[AppEegServer 项目面试 QA|AppEegServer（Web/API 层）]] 的独立 AI 计算后端，通过 RPC 被调用。两个项目的关系详见 [[AppEegServer 项目面试 QA#2.2 为什么把 Celery 改成 RPC？改造过程中遇到什么坑？|QA 2.2：Celery → RPC 改造]]。

---

## 一、asyncio — 异步事件循环引擎

### 应用场景

在 **3 个文件**中直接使用：

| 文件 | 使用方式 |
|------|---------|
| `realeeg.py:54-55` | `run(start_listen(func, id))` — 启动顶层事件循环 |
| `realeeg.py:174` | `create_task(reader(pubsub))` — 创建并发读取任务 |
| `ai_workflow.py:59` | `asyncio.run(self.ai_data_dispose(q))` — 每个子进程独立运行事件循环 |
| `aiosql.py:159-161` | `create_task(fn)` — `nowait()` 不等待写入 |
| `svd_task.py:376-377` | `asyncio.get_event_loop().run_until_complete()` — 音乐推荐入口 |

### 核心流程

```
ai_workflow.py 主进程
  │
  ├── 启动10个 Process (multiprocessing)
  │     └── 每个 Process 内部: asyncio.run(Consumer().ai_data_dispose(q))
  │           └── 独立的事件循环, 轮询 Queue 中的计算任务
  │
  └── 主进程: listen(process_eeg) 
        └── asyncio.run(start_listen(func, id))
              └── create_task(reader(pubsub))  ← 异步读 Redis
              └── while True: await sleep(1)   ← 保持事件循环存活
```

### 解决了什么问题？

**单线程并发 I/O 多路复用**。一个事件循环同时管理：
- Redis Pub/Sub 消息接收
- 多个用户的 EEG 数据回调处理
- MySQL 异步写入

整个系统只需要 **1个主事件循环 + 10个子进程事件循环**，就能处理成百上千用户的实时数据流。

### 没有 asyncio 会怎样？

必须用**多线程**替代。假设同时有 100 个在线用户：

| 维度 | asyncio | 多线程方案 |
|------|---------|-----------|
| 线程/协程数 | ~12个协程 | 100+ 线程 |
| 上下文切换 | 用户态协程切换，开销极小 | 内核态线程切换，每次 ~1-10μs |
| GIL 影响 | 不受 GIL 限制（I/O 在 C 层释放 GIL） | 受 GIL 影响，CPU 密集型会阻塞其他线程 |
| 连接管理 | 连接池 + await，天然支持 | 需要线程安全的连接池，加锁 |
| 代码复杂度 | `await` 线性书写 | 回调地狱或复杂的 Future/Promise |

对于这个**高并发 I/O 密集型**场景（接收 Redis 消息 + 写 MySQL），asyncio 是最合适的选择。

---

## 二、aioredis 2.x — 异步 Redis 客户端

### 应用场景

| 文件 | 使用方式 |
|------|---------|
| `realeeg.py:143-152` | `aioredis.from_url()` → `pubsub.psubscribe("eegstream:*")` 订阅脑电数据 |
| `realeeg.py:89-104` | `channel.get_message()` 非阻塞读取 Pub/Sub 消息 |
| `aio_redis.py:30-33` | `ConnectionPool.from_url()` 创建连接池（最大500连接） |
| `svd_task.py:185-186,219-221` | `aioredis.Redis()` 存储/读取 SVD 模型（pickle 序列化） |

### 核心数据链路

```
IoT 设备
  │
  ▼
上游服务 ──► Redis Pub/Sub ──► aioredis 异步订阅 ──► realeeg.py
             channel:                    │
             eegstream:{uid}            │ get_message() 非阻塞
                                        ▼
                                   decode_data()
                                   gzip解压 → base64解码 → int列表
                                        │
                                        ▼
                                   UserInfoData[uid].append()
                                        │
                                        ▼
                                   callfn(userinfo, eegdata)
                                        │
                                        ▼
                                   ai_workflow.process_eeg()
```

### 解决了什么问题？

**实时性**。脑电数据每秒上传 250 个采样点，数据流是**连续的、实时的**。Redis Pub/Sub 作为消息中间件：

- 生产者（上游 IoT 网关）只管 `PUBLISH eegstream:2714 <gzip压缩数据>`
- 消费者（aioredis 订阅）`get_message()` 非阻塞拉取，不会因为一条消息处理慢而阻塞后续消息

### 没有 aioredis 会怎样？

| 维度 | aioredis 异步 | 同步 redis-py |
|------|-------------|-------------|
| 消息接收 | `await get_message()` 非阻塞，1个协程处理所有用户 | `get_message()` 阻塞，需要每个用户一个线程 |
| 连接数 | 1条连接 + Pub/Sub 订阅所有频道 | 可能需要连接池轮询 |
| 模型存储 | 异步 `set()`，不阻塞数据流 | 同步写入，数据流暂停等待 |
| 集成 | 与 asyncio 事件循环天然融合 | 需要 `run_in_executor()` 桥接，增加复杂度 |

在这个实时 EEG 监测系统中，**Redis 是数据入口**。如果 Redis 读取是同步阻塞的，那整个系统的实时性就崩溃了——每秒要处理 250 个数据点，任何阻塞都会导致数据积压。

---

## 三、aiomysql — 异步 MySQL 驱动

### 应用场景

全部集中在 `utils/aiosql.py`，被以下文件调用：

| 调用方 | 场景 |
|--------|------|
| `result_storage.py:94-96` | `nowait(execute(...))` — 3秒放松度/睁闭眼结果入库 |
| `result_storage.py:144-148` | `nowait(execute(...))` + `execute(...)` — 30秒分期结果入库（两张表） |
| `result_storage.py:44-49` | `select()` + `select_one()` — 查询用户手机号 |
| `svd_task.py` | `select()` — 批量查询音乐库、标签库、用户标签、治疗反馈 |

### 关键设计：双层容错

```python
# aiosql.py — 核心执行逻辑
async def _db_execute(statement, params, ...):
    while True:                          # ← 第一层：无限重试
        try:
            async with _pool_acquire() as db:    # ← 第二层：连接池复用
                async with db.cursor(DictCursor) as cursor:
                    await cursor.execute(statement, params)
                    ...
        except ProgrammingError as ex:   # SQL语法错误 → 直接抛出
            raise ex
        except DatabaseError as ex:      # 数据库异常 → 重试
            await sleep(1)
        except RuntimeError as ex:       # 运行时异常 → 重试
            await sleep(1)
```

连接池实现也很有意思——复用成功连接的实例：

```python
_db_pool = []  # 全局连接缓存

class _pool_acquire:
    async def __aenter__(self):
        self._conn = await db_new()      # 优先从池中取
        return self._conn

    async def __aexit__(self, exc_type, ...):
        if exc_type == None:
            _db_close(self._conn)         # 成功 → 放回池中复用
        else:
            self._conn.close()            # 失败 → 丢弃此连接
```

### `nowait()` — "发射后不管"模式

```python
def nowait(fn):
    task = create_task(fn)      # 创建独立 Task，不 await
    task.add_done_callback(print)
```

这是生产环境的关键优化：**睡眠分期/放松度结果写入 MySQL 不应该阻塞数据流的持续接收**。`nowait()` 让写入操作"点火即走"——在当前事件循环中启动一个 Task 去写库，主流程立刻返回去处理下一个 3 秒/30 秒的数据。

### 解决了什么问题？

**写入不阻塞数据流**。考虑这个时间线：

```
时间轴 →
│ 30s  │  31s  │  32s  │ ... │  60s  │
│ 收到  │  收到  │  收到  │     │  下一批 │
│ 7500点│  新数据│  新数据│     │  7500点 │
│       │       │       │     │         │
│ 触发分期计算 ←── 如果同步写库阻塞 2秒 ──→ 33s才返回
│       ↑ 这3秒的新数据全部积压！
```

用 `nowait()` 后：

```
│ 30s  │  31s  │  32s  │ ... │  60s  │
│ 触发分期 │       │       │     │  下一批 │
│ nowait(insert)──→ MySQL 后台写入（不阻塞）│
│ 立即返回处理31s数据                        │
```

### 没有 aiomysql 会怎样？

| 维度 | aiomysql | 同步 PyMySQL |
|------|---------|------------|
| 写入延迟 | `nowait()` 后 0ms 返回 | 等待 MySQL 响应，1-200ms |
| 数据丢失风险 | 仅丢失崩溃时刻的飞行中 Task | 事务内保证一致性 |
| 连接复用 | `_pool_acquire` 异步上下文管理 | 需要线程安全连接池或每次新建 |
| 与事件循环集成 | 原生 `await`，不阻塞事件循环 | 同步调用阻塞整个事件循环 |

> ⚠️ `nowait()` 是 **fire-and-forget**，如果进程崩溃，飞行中的写入会丢失。对于睡眠数据场景（有一定容错容忍度），这是可接受的权衡。

> 🔗 在 Web 层（AppEegServer）中同样使用了 `nowait()` 模式，但面试中可能被追问风险——详见 [[AppEegServer 项目面试 QA#2.6 Django 3.2 的 async view 有什么坑？跟同步中间件混用有什么问题？|QA 2.6：async view 的坑]]。

---

## 四、三者协作全景图

```
                    asyncio 事件循环 (主进程)
                    ════════════════════════════
    aioredis ──► Redis Pub/Sub 订阅
      │             │ get_message() 非阻塞
      │             ▼
      │         EEG 数据解码 + 缓冲
      │             │
      │             ▼
      │         3秒/30秒 计数器触发
      │             │
      │      ┌──────┴──────┐
      │      ▼              ▼
      │   Queue[0]      Queue[1~9]
      │      │              │
      │      ▼              ▼
      │   Process 0     Process 1~9
      │   asyncio.run() asyncio.run()
      │      │              │
      │      ▼              ▼
      │   three_data_   thirty_data_
      │   manage()      manage()
      │      │              │
      │      └──────┬───────┘
      │             ▼
      └──────── aiomysql ──► MySQL
              nowait(execute(...))
              (不阻塞事件循环)

三者层级关系:
  asyncio     ← 调度框架 (I/O 多路复用)
  aioredis    ← 数据入口 (实时 EEG 流)
  aiomysql    ← 数据出口 (AI 结果持久化)
```

---

## 五、总结

| 技术 | 角色 | 核心价值 | 没有它的话 |
|------|------|----------|-----------|
| **asyncio** | 异步调度引擎 | 单线程管理所有 I/O，~12个协程替代100+线程 | 需大量线程，上下文切换开销大，代码变回调地狱 |
| **aioredis** | 实时数据入口 | 非阻塞订阅 Redis Pub/Sub，保证 250Hz 数据不积压 | 读取阻塞，数据流实时性崩溃，每秒丢点 |
| **aiomysql** | 结果持久化出口 | `nowait()` 写入不阻塞主循环 + 双层容错自动重连 | 每次写入阻塞 1-200ms，累计导致数据积压和丢帧 |

三者共同支撑了「每秒 250 点 → 每 3 秒睁闭眼/放松度 → 每 30 秒睡眠分期」这条实时 AI 计算管线，是高并发 I/O 密集型系统中 asyncio 生态的一个典型工程实践案例。

---

## 六、关联阅读：[[AppEegServer 项目面试 QA]]

本笔记聚焦 **AI 服务的异步技术栈**，另一篇笔记覆盖 **Web/API 层的面试问答**。两篇的交叉点：

| 本笔记章节 | 对应 QA 章节 | 关系 |
|-----------|-------------|------|
| [[EegServerAI-异步技术栈分析#一、asyncio — 异步事件循环引擎\|一、asyncio]] | [[AppEegServer 项目面试 QA#2.2 为什么把 Celery 改成 RPC？改造过程中遇到什么坑？\|QA 2.2: Celery→RPC]] | RPC 改造的背景和动机 |
| [[EegServerAI-异步技术栈分析#三、aiomysql — 异步 MySQL 驱动\|三、aiomysql/nowait]] | [[AppEegServer 项目面试 QA#2.6 Django 3.2 的 async view 有什么坑？跟同步中间件混用有什么问题？\|QA 2.6: async view 的坑]] | nowait 模式在两层的不同风险 |
| [[EegServerAI-异步技术栈分析#二、aioredis 2.x — 异步 Redis 客户端\|二、aioredis]] | [[AppEegServer 项目面试 QA#1. 高可用是怎么保证的？集群部署还是其他手段？\|QA 1: 高可用]] + [[AppEegServer 项目面试 QA#4. 缓存穿透、缓存雪崩、缓存击穿怎么解决？\|QA 4: 缓存策略]] | Redis 在两层承担不同角色 |
| EegServerAI 两阶段分期算法 | [[AppEegServer 项目面试 QA#2.3 两阶段分期具体怎么做？决策树的 PSD 阈值怎么调的？\|QA 2.3: 两阶段分期]] | 同一算法，本笔记侧重工程调度，QA 侧重算法细节 |
| EegServerAI 睁闭眼检测 | [[AppEegServer 项目面试 QA#2.11 睁闭眼检测的全流程是怎样的？\|QA 2.11: 睁闭眼检测]] | 同一流程，本笔记侧重 STFT+AutoGluon 实现 |

> 💡 本笔记中的 RPC 模型部署模式 + 异步数据管线经验，也是 [[05-个人/面试/AI视觉检测-JD映射与面试预测|AI 视觉检测岗位]] 的核心竞争优势。详见 [[05-个人/面试/AI视觉检测-JD映射与面试预测#🔴🔴🔴 职责 4a：模型管理|JD 分析 § 模型管理]]。
