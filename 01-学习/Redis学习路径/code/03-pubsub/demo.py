"""
Redis 阶段一 · Pub/Sub + Stream 消息示例
环境: Python 3.10+ + redis-py + 本地 Redis 6379
对应笔记: [[03-Pub-Sub与Stream消息]]
"""
import redis, threading, time

r = redis.Redis(host="localhost", port=6379, decode_responses=True)

# ===== Pub/Sub =====
def subscriber():
    pubsub = r.pubsub()
    pubsub.subscribe("channel:news")
    for msg in pubsub.listen():
        if msg["type"] == "message":
            print(f"Subscriber received: {msg['data']}")
            break

t = threading.Thread(target=subscriber, daemon=True)
t.start()
time.sleep(0.5)
r.publish("channel:news", "Breaking News!")
t.join(timeout=2)

# ===== Stream =====
r.xadd("stream:orders", {"order_id": "1001", "amount": "99.99"})
r.xadd("stream:orders", {"order_id": "1002", "amount": "199.99"})
msgs = r.xread({"stream:orders": "0"}, count=10, block=1000)
for stream, messages in msgs:
    for msg_id, fields in messages:
        print(f"Stream [{msg_id}]: {fields}")

# ===== 清理 =====
r.delete("stream:orders")
print("Done!")
