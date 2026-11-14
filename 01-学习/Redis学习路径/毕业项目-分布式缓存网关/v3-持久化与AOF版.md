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

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `SET test:persist hello-world` → `redis-cli SHUTDOWN`（非 NOSAVE）→ 重启 → `GET test:persist` 仍为 `hello-world`，`INFO persistence` 中 `loading:0`、`aof_enabled:1` |
| 🟡 进阶 | `ls appendonlydir/` 能同时看到 `*.base.rdb`、`*.incr.aof`、`*.manifest`；`BGREWRITEAOF` 后 base 文件编号递增且 mtime 更新，`INFO persistence` 的 `aof_rewrite_in_progress:0` |
| 🔴 挑战 | 把最后写入的 `.incr.aof` 尾部截掉半条命令 → 重启报错 → `redis-check-aof --fix <该文件>` 后能启动；记录“丢了哪一条写”（前后 `GET` 对比），并用 2 行说清为何只靠 RDB 做不到这个粒度 |

---

*最后更新：2026-07-20*
