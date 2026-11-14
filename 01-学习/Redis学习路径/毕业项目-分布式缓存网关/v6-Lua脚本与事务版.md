---
title: v6-Lua脚本与事务版
created: 2026-07-20
tags:
  - Redis
  - 毕业项目
  - v6
  - Lua
  - 事务
  - 方法论驱动
description: 用 Lua 脚本实现原子库存扣减与安全分布式锁释放。
lark_doc_url: https://my.feishu.cn/docx/YMXcdoRUQoiPlgxxuNvcPhpanI0
---

# v6-Lua脚本与事务版

> 📍 **前置**：[[v5-Cluster分片版]] → 本版本 → [[v7-监控与压测调优版]]

## 🎯 本版本目标

用 Lua 脚本实现原子库存扣减和安全分布式锁，实现：**业务一致性保障**。

---

## 💻 核心实现

### 原子库存扣减

```python
decr_stock = r.register_script("""
local stock = redis.call('GET', KEYS[1])
if stock and tonumber(stock) > 0 then
    redis.call('DECR', KEYS[1])
    return 1
else
    return 0
end
""")

# 扣减库存
result = decr_stock(keys=['stock:item:1001'])
if result:
    print("扣减成功")
else:
    print("库存不足")
```

### 安全分布式锁

```python
import uuid

# 获取锁（SET NX EX 一条命令保证原子性）
def acquire_lock(key, ttl=30):
    token = str(uuid.uuid4())
    ok = r.set(f"lock:{key}", token, nx=True, ex=ttl)
    return token if ok else None

# Lua 安全释放锁（比对 token + 删除 原子执行）
unlock = r.register_script("""
if redis.call('GET', KEYS[1]) == ARGV[1] then
    return redis.call('DEL', KEYS[1])
else
    return 0
end
""")

def release_lock(key, token):
    return unlock(keys=[f"lock:{key}"], args=[token])

# 使用
token = acquire_lock("order:1001")
if token:
    try:
        # 业务逻辑
        decr_stock(keys=['stock:item:1001'])
    finally:
        release_lock("order:1001", token)
```

### WATCH 乐观锁（转账）

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
                continue
```

---

## ✅ 验收标准

- [ ] 库存扣减原子性验证（并发无超卖）
- [ ] 分布式锁获取/释放正确
- [ ] 不会误删他人锁
- [ ] WATCH 乐观锁能处理冲突

---

## 🔮 下一版本预告

当前无性能监控和压测。下一版本集成 **监控与压测调优**。

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `SET stock:item:1001 10` 后调 `decr_stock(keys=['stock:item:1001'], args=[3])` 返回 `1`，`GET` 为 `7`；再扣 `100` 返回 `0` 且余额仍为 `7`（不会扣成负数） |
| 🟡 进阶 | 库存 10、100 个并发各扣 1：成功次数恰好 `10`、`GET` 为 `0`，无超卖也无少卖；拿一个错误 token 调 `unlock` 返回 `0` 且锁仍在（不误删他人锁） |
| 🔴 挑战 | 锁 TTL 设 1s + 业务 `sleep 2` 复现误删（B 的锁被 A 释放），换 token+Lua 后不再发生；再给 `decr_stock` 加一层“续期”方案：`expire/3` 时发 `PEXPIRE` 且先校验 token，用 2 行说明它与 fencing token 各自兑什么底 |

---

*最后更新：2026-07-20*
