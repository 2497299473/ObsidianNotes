---
title: 01-Redis基础与字符串命令
created: 2026-07-20
tags:
  - Redis
  - 字符串
  - 键值
  - 阶段一
description: Redis 学习起点：数据模型、字符串命令、键管理与过期机制。
lark_doc_url: https://my.feishu.cn/docx/QdBQdQROboCTKpxzeF5cOiHZnre
---

# 01-Redis基础与字符串命令

> 📍 **阶段一·第一篇** | ⬅️ 前置：[[../00-Redis学习路径总索引]] | ➡️ 后续：[[02-五大基础数据结构]]
>
> 📚 **深度补充阅读**：[[阶段一-数据结构与命令/02-五大基础数据结构]]

## 🎯 本篇目标

学完本篇你应当能够：
1. 理解 Redis 的单线程模型与内存数据特性
2. 掌握字符串 (String) 类型的核心命令与典型用法
3. 用键过期 (EXPIRE) 实现缓存/限流场景
4. 区分 `SET` / `SETNX` / `SETEX` / `MSET` 的适用场景

---

## 一、Redis 是什么

Redis = **Re**mote **Di**ctionary **S**erver，基于内存的 KV 数据库。

| 特性 | 说明 |
|------|------|
| 基于内存 | 读写极快（单机 10万+ QPS） |
| 单线程命令处理 | 避免锁竞争，简化并发模型（6.0+ 仅 IO 多线程，命令仍单线程） |
| 丰富数据结构 | String/List/Hash/Set/ZSet/Stream/Bitmap 等 |
| 持久化 | RDB 快照 + AOF 日志双轨制 |
| 高可用 | 主从、哨兵、Cluster 分片 |

> 💡 单线程为何快？纯内存操作 + IO 多路复用（epoll）+ 避免锁竞争切换开销。

---

## 二、环境准备

```bash
# 方式一：本地安装
redis-server              # 默认 6379
redis-cli                 # 进入交互式 CLI

# 方式二：Docker（推荐学习用）
docker run -d --name redis -p 6379:6379 redis:7.2

# 测试连接
redis-cli ping            # 预期：PONG
```

---

## 三、字符串命令

### 基础读写
```redis
SET user:1:name "alice"      # 设置
GET user:1:name              # 读取 → "alice"
DEL user:1:name              # 删除
```

### 带条件的设置（缓存防击穿必备）
```redis
SETNX lock:order:100 "owner1"    # 不存在才设置 → 1（成功）/ 0（失败）
SETEX token:abc 3600 "user1"      # 带过期设置（秒）
```

### 计数器（原子操作）
```redis
SET article:1:views 0
INCR article:1:views            # +1 → 1
INCR article:1:views            # +1 → 2
INCRBY article:1:views 10        # +10 → 12
DECR article:1:views            # -1 → 11
```

### 批量操作（减少 RTT）
```redis
MSET k1 v1 k2 v2 k3 v3
MGET k1 k2 k3                   # 一次返回多值
```

### 旧值返回设置
```redis
GETSET k1 "newv"               # 返回旧值，同时设置新值
```

---

## 四、键管理与过期

```redis
EXISTS user:1:name              # 1 存在 / 0 不存在
EXPIRE user:1:name 60           # 60 秒后过期
TTL user:1:name                 # 剩余秒数（-1 永久 / -2 已过期/不存在）
PERSIST user:1:name             # 移除过期，变永久
KEYS user:*                     # ⚠️ 阻塞，生产禁用，用 SCAN 替代
TYPE user:1:name                # 数据类型
```

### 过期策略
- **惰性删除**：访问时检查过期才删
- **定期删除**：后台周期性随机抽样删除
- 两者结合，平衡性能与内存

---

## 五、典型应用场景

| 场景 | 命令组合 |
|------|---------|
| 缓存对象 | `SET` + `EXPIRE` 或 `SETEX` |
| 分布式锁（单机） | `SETNX` + `EXPIRE`（原子性问题→用 `SET key val NX EX 30`） |
| 计数器/排行榜分数 | `INCR` / `INCRBY` |
| 限流（固定窗口） | `INCR` + `EXPIRE` 1 秒，判 ≥N |
| Session 存储 | `SETEX session:<id> 1800 <payload>` |

---

## 六、常见陷阱

| 陷阱 | 表现 | 对策 |
|------|------|------|
| `KEYS *` 阻塞 | 大库导致 Redis 卡顿 | 用 `SCAN` 迭代 |
| SETNX 后未原子加过期 | 持有者崩溃致锁永驻 | 用 `SET key val NX EX 30` 一条命令 |
| 大 key（单值 MB 级） | 网络与内存抖动 | 单值控制在 10KB 以内 |
| 持久化误以为安全 | 内存数据库宕机仍丢数据 | 关键数据落 DB，Redis 仅做缓存 |

---

## 🔗 相关笔记

- ⬅️ 前置：[[../00-Redis学习路径总索引]]
- ➡️ 后续：[[02-五大基础数据结构]]
- 🔗 关联：[[阶段一-数据结构与命令/02-五大基础数据结构]]（深度补充）
- 🔗 关联：[[../../../03-AI工具/AI协作方法论/技术学习路径审查与优化方法论]]

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能用 `SET/GET/DEL` 完成键值读写，用 `redis-cli ping` 验证连接 |
| 🟡 进阶 | 能用 `INCR` 实现计数器，并用 `SET ... NX EX` 实现单机分布式锁，解释为何 `SETNX` + `EXPIRE` 非原子 |
| 🔴 挑战 | 能用 `EXPIRE` + `TTL` 实现固定窗口限流，并解释惰性删除与定期删除的协作机制 |

---

*最后更新：2026-07-20*
