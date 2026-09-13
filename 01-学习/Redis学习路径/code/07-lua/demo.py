"""
Redis 阶段三 · Lua 脚本调用示例
环境: Python 3.10+ + redis-py + 本地 Redis 6379
对应笔记: [[01-事务与Lua脚本]]
"""
import redis

r = redis.Redis(host="localhost", port=6379, decode_responses=True)

# 读取 Lua 脚本
with open("stock.lua", "r") as f:
    script = f.read()

stock_decr = r.register_script(script)

# 初始化库存
r.set("stock:item:100", 10)

# 原子扣减 3 件（应成功）
result = stock_decr(keys=["stock:item:100"], args=[3])
print(f"扣减3件: {'成功' if result == 1 else '失败'}, 剩余: {r.get('stock:item:100')}")

# 超库存扣减 100 件（应失败）
result = stock_decr(keys=["stock:item:100"], args=[100])
print(f"扣减100件: {'成功' if result == 1 else '失败'}, 剩余: {r.get('stock:item:100')}")

# 清理
r.delete("stock:item:100")
print("Done!")
