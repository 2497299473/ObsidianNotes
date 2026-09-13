---
title: v3-持久化与AOF版
created: 2026-07-20
tags:
  - Redis
  - 毕业项目
  - v3
  - 持久化
  - AOF
  - 方法论驱动
description: 启用 AOF 持久化，实现重启后数据不丢失与恢复验证。
lark_doc_url: https://my.feishu.cn/docx/Pel4dX6ySo0m3QxOdgTcjMZRn9k
---

# v3-持久化与AOF版

> 📍 **前置**：[[v2-多数据结构组合版]] → 本版本 → [[v4-主从高可用版]]

## 🎯 本版本目标

启用 AOF 持久化，实现：**重启后数据不丢失 + 恢复流程验证**。

---

## 💻 核心实现

### redis.conf

```bash
appendonly yes
appendfsync everysec
aof-use-rdb-preamble yes
```

### 启动与验证

```bash
redis-server redis.conf
redis-cli CONFIG GET appendonly
redis-cli CONFIG GET appendfsync

# 写入数据
redis-cli SET "test:persist" "hello-world"
redis-cli ZADD "rank:game" 100 "alice"

# 重启 Redis
redis-cli SHUTDOWN
redis-server redis.conf

# 验证数据恢复
redis-cli GET "test:persist"        # hello-world
redis-cli ZRANGE "rank:game" 0 -1 WITHSCORES  # alice 100

# 查看持久化状态
redis-cli INFO persistence
```

### 应用层恢复验证

```python
# 写入数据后模拟重启
r.set("cache:data", "before-restart")
r.zadd("rank:game", {"alice": 100})

# 重启 Redis...

# 验证数据恢复
assert r.get("cache:data") == "before-restart"
assert r.zscore("rank:game", "alice") == 100.0
```

---

## ✅ 验收标准

- [ ] AOF 配置生效
- [ ] 重启后数据不丢失
- [ ] `INFO persistence` 显示 AOF 正常
- [ ] BGREWRITEAOF 可手动触发

---

## 🔮 下一版本预告

当前单机无高可用。下一版本搭建 **主从复制 + 哨兵**。

---

## ✅ 自检清单

### 🟢 基础
- [ ] AOF 持久化配置正确
- [ ] 重启后数据恢复

### 🟡 进阶
- [ ] 理解 `appendfsync everysec`
- [ ] 能执行 BGREWRITEAOF

### 🔴 挑战
- [ ] 能设计备份与恢复演练流程
- [ ] 能对比 RDB/AOF/混合持久化的 RPO

---

*最后更新：2026-07-20*
