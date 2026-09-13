"""
Redis 阶段一 · 五大基础数据结构示例
环境: Python 3.10+ + redis-py + 本地 Redis 6379
对应笔记: [[02-五大基础数据结构]]
"""
import redis

r = redis.Redis(host="localhost", port=6379, decode_responses=True)

# ===== Hash：用户信息 =====
r.hset("user:100", mapping={"name": "alice", "age": "30", "city": "Beijing"})
print("Hash:", r.hgetall("user:100"))

# ===== List：消息队列 =====
r.lpush("queue:tasks", "task1", "task2", "task3")
print("List:", r.lrange("queue:tasks", 0, -1))
print("Queue pop:", r.rpop("queue:tasks"))

# ===== Set：标签去重 =====
r.sadd("tags:article", "python", "redis", "python")
print("Set members:", r.smembers("tags:article"))
print("Tags count:", r.scard("tags:article"))

# ===== Sorted Set：排行榜 =====
r.zadd("rank:game", {"alice": 100, "bob": 85, "charlie": 95})
print("Top 3:", r.zrevrange("rank:game", 0, 2, withscores=True))
print("Rank of bob:", r.zrevrank("rank:game", "bob"))

# ===== 清理 =====
for k in ["user:100", "queue:tasks", "tags:article", "rank:game"]:
    r.delete(k)
print("Done!")
