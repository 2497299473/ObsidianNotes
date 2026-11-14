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
SET lock:order:100 "owner1" NX   # 不存在才设置 → OK（成功）/ (nil)（已存在）
SETEX token:abc 3600 "user1"      # 带过期设置（秒）
SETNX lock:order:100 "owner1"    # ⚠️ 历史写法：自 2.6.12 起 deprecated，等价于 SET ... NX
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
SET k1 "newv" GET              # 返回旧值，同时设置新值（6.2+ 推荐）
GETSET k1 "newv"               # ⚠️ 历史写法：自 6.2 起 deprecated，等价于 SET ... GET
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
| 分布式锁（单机） | `SET key val NX EX 30`（`SETNX`+`EXPIRE` 两步非原子，且 `SETNX` 已 deprecated） |
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

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `redis-cli ping` 返回 `PONG`；`SET user:1:name alice` → `GET` 返回 `alice`、`TYPE` 返回 `string`、`DEL` 后 `EXISTS` 返回 `0` |
| 🟡 进阶 | 连发两次 `SET lock:o1 t1 NX EX 30`：第一次 `OK`、第二次 `(nil)`，且 `TTL lock:o1` 落在 25–30；把 `PERSIST` 后 `TTL` 返回 `-1`；一句话说清 `SETNX`+`EXPIRE` 的两步窗口为何会漏锁 |
| 🔴 挑战 | 用 `INCR` + `EXPIRE` 写固定窗口限流（10 QPS）：① 先 `redis-cli DEL rl:u1` 归零（否则计数只增不减，第二次跑永远 >10）；② 用一行 Lua 保证「自增 + 首次置过期」原子：`redis-cli EVAL "local c=redis.call('INCR',KEYS[1]) if c==1 then redis.call('EXPIRE',KEYS[1],1) end return c" 1 rl:u1`（★ 只在第一次 `INCR` 时设 `EXPIRE`：计数器刚创建才开 1s 窗口，之后自增不续期；两步写法 `c=$(redis-cli INCR rl:u1); [ "$c" = 1 ] && redis-cli EXPIRE rl:u1 1` 有中间窗口、不原子，仅作对照）。判据：连发 12 次（`seq 1 12 \| xargs -P12 -I{}` 包住上面的 EVAL），返回 1–10 的 10 次放行、11–12 的 2 次拒绝；并说明固定窗口在跨秒边界会瞬时放过约 2 倍流量 |

---

## 🔗 相关笔记

- ⬅️ 前置：[[../00-Redis学习路径总索引]]
- ➡️ 后续：[[02-五大基础数据结构]]
- 🔗 关联：[[阶段一-数据结构与命令/02-五大基础数据结构]]（深度补充）
- 🔗 关联：[[../../../03-AI工具/AI协作方法论/技术学习路径审查与优化方法论]]

---

*最后更新：2026-07-20*
