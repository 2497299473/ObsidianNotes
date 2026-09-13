"""
Redis 阶段三 · 缓存三大问题解决方案
环境: Python 3.10+ + redis-py + 本地 Redis 6379
对应笔记: [[03-缓存三大问题与分布式锁]]
"""
import redis, time, random, json

r = redis.Redis(host="localhost", port=6379, decode_responses=True)

# ===== 1. 缓存穿透：空值缓存 =====
def get_with_null_cache(key):
    cached = r.get(key)
    if cached is not None:
        return None if cached == "NULL" else json.loads(cached)
    # 模拟 DB 查询
    value = query_db(key)
    if value is None:
        r.setex(key, 60, "NULL")  # 穿透防护：缓存空值
    else:
        r.setex(key, 300 + random.randint(0, 60), json.dumps(value))  # 雪崩防护
    return value

def query_db(key):
    return {"id": 1, "name": "alice"} if "existing" in key else None

# ===== 2. 缓存击穿：互斥锁 =====
def get_with_mutex(key):
    cached = r.get(key)
    if cached:
        return cached
    lock_key = f"lock:{key}"
    if r.set(lock_key, "1", nx=True, ex=5):
        try:
            value = f"data-{random.randint(1, 100)}"
            r.setex(key, 60, value)
            return value
        finally:
            r.delete(lock_key)
    else:
        time.sleep(0.1)
        return r.get(key)

# ===== 测试 =====
print("穿透测试（不存在的key）:", get_with_null_cache("nonexistent:key"))
print("穿透测试（存在的key）:", get_with_null_cache("existing:1"))
print("击穿测试:", get_with_mutex("hotkey:1"))

# ===== 3. 分布式锁 =====
RELEASE_SCRIPT = """
if redis.call("GET", KEYS[1]) == ARGV[1] then
    return redis.call("DEL", KEYS[1])
else
    return 0
end
"""
lock_ok = r.set("lock:order:100", "owner-token", nx=True, ex=30)
print("分布式锁:", "成功" if lock_ok else "失败")

# 清理
r.delete("lock:order:100", "hotkey:1", "nonexistent:key", "existing:1")
print("Done!")
