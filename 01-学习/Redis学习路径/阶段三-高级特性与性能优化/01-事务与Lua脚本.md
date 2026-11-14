---
title: 01-事务与Lua脚本
created: 2026-07-20
tags:
  - Redis
  - 事务
  - Lua
  - 方法论驱动
description: Redis 事务（MULTI/EXEC/WATCH）、Lua 脚本原子操作与性能。
lark_doc_url: https://my.feishu.cn/docx/HD1Vd7Hfgo8t28xUbWLc2Dj3nle
---

# 01-事务与Lua脚本

> 📍 **前置**：[[../阶段二-持久化与高可用/03-Cluster分片集群]] → 本笔记 → [[02-内存管理与性能调优]]

## 🎯 学习目标

1. 理解 Redis 事务的原理与局限
2. 能用 MULTI/EXEC/WATCH 实现乐观锁
3. 掌握 Lua 脚本编写与 EVAL/EVALSHA
4. 能用 Lua 实现原子操作

---

## 📖 核心内容

### Redis 事务

```redis
MULTI              # 开启事务
SET key1 "v1"
INCR counter
SET key2 "v2"
EXEC               # 执行所有命令
# 或
DISCARD            # 取消事务
```

### 事务的局限

| 特性 | 说明 |
|------|------|
| 原子性 | ✅ 命令顺序执行，不可被打断 |
| 一致性 | ⚠️ 仅当全部命令都能执行时成立；运行期有命令报错时，已执行的不回滚，一致性需业务或 Lua 自保 |
| 隔离性 | ✅ 单线程保证 |
| 持久性 | 取决于持久化配置 |
| **回滚** | ❌ 不支持，某条失败后续仍执行 |

### WATCH 乐观锁

```redis
WATCH balance      # 监视 key
val = GET balance
MULTI
DECRBY balance 100
INCRBY savings 100
EXEC               # 如果 balance 在 WATCH 后被修改，EXEC 返回 nil
```

```python
def transfer(from_acct, to_acct, amount):
    with r.pipeline() as pipe:
        while True:
            try:
                pipe.watch(from_acct)
                balance = int(pipe.get(from_acct) or 0)
                if balance < amount:
                    pipe.unwatch()
                    raise ValueError("余额不足")
                pipe.multi()
                pipe.decrby(from_acct, amount)
                pipe.incrby(to_acct, amount)
                pipe.execute()
                break
            except redis.WatchError:
                continue  # 重试
```

### Lua 脚本

Lua 脚本在 Redis 中原子执行，不会被打断。

```bash
# EVAL 执行脚本
EVAL "return redis.call('set', KEYS[1], ARGV[1])" 1 mykey myvalue

# 条件判断脚本（库存扣减）
EVAL "
local stock = redis.call('GET', KEYS[1])
if stock and tonumber(stock) > 0 then
    redis.call('DECR', KEYS[1])
    return 1
else
    return 0
end
" 1 stock:1001
```

### SCRIPT LOAD 与 EVALSHA

```bash
# 加载脚本到 Redis 缓存，返回 SHA1
SCRIPT LOAD "return redis.call('set', KEYS[1], ARGV[1])"
# "a1b2c3d4..."

# 用 SHA1 调用（节省网络传输）
EVALSHA "a1b2c3d4..." 1 mykey myvalue

# 检查脚本是否存在
SCRIPT EXISTS "a1b2c3d4..."
```

### 事务 vs Lua

| 特性 | MULTI/EXEC | Lua 脚本 |
|------|-----------|---------|
| 原子性 | ✅ | ✅ |
| 条件判断 | ✅（WATCH） | ✅（脚本内判断） |
| 中间读取 | ❌ | ✅ |
| 失败回滚 | ❌ | ❌ |
| 复杂度 | 简单 | 灵活 |
| 性能 | 多命令网络往返 | 单次调用 |
| 推荐场景 | 简单批量操作 | 复杂条件+原子操作 |

---

## 💻 代码示例

Lua 实现原子库存扣减：

```python
import redis

r = redis.Redis(decode_responses=True)

# 原子扣减库存脚本
lua_script = """
local stock = redis.call('GET', KEYS[1])
if stock and tonumber(stock) > 0 then
    redis.call('DECR', KEYS[1])
    return 1
else
    return 0
end
"""
decr_stock = r.register_script(lua_script)
result = decr_stock(keys=['stock:1001'])
print(f"扣减结果: {result}")

# Lua 实现安全释放分布式锁
unlock_script = """
if redis.call('GET', KEYS[1]) == ARGV[1] then
    return redis.call('DEL', KEYS[1])
else
    return 0
end
"""
unlock = r.register_script(unlock_script)
```

---

## ⚠️ 常见陷阱

| 陷阱 | 后果 | 解决方案 |
|------|------|---------|
| 认为 Redis 事务支持回滚 | 错误数据 | 业务层做补偿，或用 Lua |
| 长 Lua 脚本 | 阻塞 Redis 主线程 | 脚本逻辑尽量短 |
| WATCH 后忘记 UNWATCH | 连接状态异常 | EXEC 失败后 UNWATCH |
| Cluster 下跨槽 Lua | 报错 | 用 Hash Tag 保证同槽 |
| Lua 每次传全文 | 浪费带宽 | 用 SCRIPT LOAD + EVALSHA |

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `MULTI` 里写 `SET k1 v1` + `INCR k1`（对字符串做 INCR）+ `SET k2 v2`，`EXEC` 后：`INCR` 那条报错，而 `k1`、`k2` **都已写入**（`GET` 能取到）——这就是“无回滚” |
| 🟡 进阶 | 两会话：A `WATCH k` → `MULTI` → `SET k 1`；期间 B 执行 `SET k 9`；A `EXEC` 返回 `(nil)`（`GET k` 为 9）；A 重试后返回非 nil 且 `GET k` 为 1 |
| 🔴 挑战 | `SCRIPT LOAD "return redis.call('GET',KEYS[1])"` 拿到 SHA1 → `EVALSHA <sha> 1 k` 能执行且 `SCRIPT EXISTS <sha>` 返回 1；再把一个扣库存脚本改成两个 KEYS 跨槽发到 Cluster 上，捕获 `CROSSSLOT`，改用 `{tag}` 修好并说明为何 Lua 不能替代 Cluster 的槽约束 |

---

## 🔗 相关笔记

- [[../阶段二-持久化与高可用/03-Cluster分片集群]] — 前置
- [[02-内存管理与性能调优]] — 后续
- [[01-学习/Redis学习路径/阶段三-高级特性与性能优化/99-阶段3复习检查点|99-阶段3复习检查点]] — 阶段验收
- [[../00-Redis学习路径总索引]] — 返回总索引
- [[../毕业项目-分布式缓存网关/v6-Lua脚本与事务版]] — 毕业项目应用

---

*最后更新：2026-07-20*
