"""
Redis 阶段一 · 字符串命令示例
环境: Python 3.10+ + redis-py + 本地 Redis 6379
对应笔记: [[01-Redis基础与字符串命令]]
"""
import redis

r = redis.Redis(host="localhost", port=6379, decode_responses=True)

# 基础读写
r.set("user:1:name", "alice")
print("GET:", r.get("user:1:name"))

# 计数器（原子操作）
r.set("article:1:views", 0)
r.incr("article:1:views")
print("VIEWS:", r.get("article:1:views"))

# 分布式锁（单机安全版：SET NX EX 一条命令保证原子性）
lock_key = "lock:order:100"
ok = r.set(lock_key, "owner", nx=True, ex=30)
print("LOCK:", ok)

# 清理
r.delete("user:1:name", "article:1:views", lock_key)
