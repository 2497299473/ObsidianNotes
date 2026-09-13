---
title: 01-RDB与AOF持久化
created: 2026-07-20
tags:
  - Redis
  - RDB
  - AOF
  - 持久化
  - 方法论驱动
description: RDB 与 AOF 持久化原理、混合持久化、恢复流程与最佳实践。
lark_doc_url: https://my.feishu.cn/docx/PdPddhuOHoITz2xHkmec66xznsf
---

# 01-RDB与AOF持久化

> 📍 **前置**：[[../阶段一-数据结构与命令/03-Pub-Sub与Stream消息]] → 本笔记 → [[02-主从复制与哨兵]]

## 🎯 学习目标

1. 理解 RDB 与 AOF 的原理、优缺点与适用场景
2. 掌握 AOF 的三种刷盘策略
3. 理解混合持久化（RDB+AOF）
4. 能根据业务恢复点目标选择持久化方案

---

## 📖 核心内容

### RDB（Redis Database）

RDB = 将内存数据以二进制快照写入磁盘文件。

| 特性 | 说明 |
|------|------|
| 触发方式 | 手动 `SAVE`/`BGSAVE` 或自动（定期） |
| 文件 | `dump.rdb` |
| 优点 | 文件小、恢复快、适合备份 |
| 缺点 | 可能丢失最后一次快照后的数据 |

```bash
# 手动触发
SAVE              # 前台阻塞（生产禁用）
BGSAVE            # 后台 fork 子进程（推荐）

# 配置自动快照条件
save 3600 1      # 1小时内至少1个key变化
save 300 100     # 5分钟内至少100个key变化
save 60 10000    # 1分钟内至少10000个key变化
```

### AOF（Append-Only File）

AOF = 将每条写命令追加到日志文件。

| 特性 | 说明 |
|------|------|
| 触发方式 | 每条写命令自动追加 |
| 文件 | `appendonly.aof` |
| 优点 | 数据丢失少（最多1秒） |
| 缺点 | 文件大、恢复慢 |

```bash
# redis.conf
appendonly yes
appendfilename "appendonly.aof"

# 三种刷盘策略
# appendfsync always    # 每条命令都刷盘（最安全，最慢）
appendfsync everysec    # 每秒刷盘（推荐，最多丢1秒）
# appendfsync no        # 由 OS 决定（最快，可能丢较多）
```

### AOF 重写

AOF 文件会越来越大，重写机制移除冗余命令：

```bash
# 手动触发重写
BGREWRITEAOF

# 自动重写条件
auto-aof-rewrite-percentage 100  # 文件比上次重写后大100%
auto-aof-rewrite-min-size 64mb   # 最小重写大小
```

### 混合持久化（Redis 4.0+）

```bash
aof-use-rdb-preamble yes
```

AOF 重写时先写 RDB 格式的全量数据，再追加增量 AOF 命令。兼具 RDB 的恢复速度和 AOF 的数据安全。

### 恢复流程

```
Redis 启动 → 检查 AOF 文件
  ├── 有 AOF → 用 AOF 恢复（含 RDB 前导则先加载 RDB）
  └── 无 AOF → 检查 RDB → 用 RDB 恢复
```

### 对比总结

| 维度 | RDB | AOF | 混合 |
|------|-----|-----|------|
| 数据丢失 | 多（快照间隔） | 少（1秒） | 少 |
| 恢复速度 | ⚡ 快 | 🐢 慢 | ⚡ 快 |
| 文件大小 | 小 | 大 | 中 |
| CPU/内存开销 | 低（fork） | 中（重写） | 中 |
| 推荐场景 | 冷备 | 生产主用 | 生产推荐 |

---

## 💻 代码示例

```bash
# 查看当前持久化配置
redis-cli CONFIG GET save
redis-cli CONFIG GET appendonly
redis-cli CONFIG GET appendfsync

# 生产推荐配置
redis-cli CONFIG SET appendonly yes
redis-cli CONFIG SET appendfsync everysec
redis-cli CONFIG SET aof-use-rdb-preamble yes

# 写入测试数据
for i in $(seq 1 1000); do redis-cli SET "key:$i" "value:$i"; done

# 触发 RDB 快照
redis-cli BGSAVE

# 触发 AOF 重写
redis-cli BGREWRITEAOF

# 查看持久化状态
redis-cli INFO persistence
```

### 恢复演练

```bash
# 1. 写入测试数据
redis-cli SET "test:data" "hello"

# 2. 关闭 Redis
redis-cli SHUTDOWN NOSAVE

# 3. 确认持久化文件
ls -la /var/lib/redis/
# dump.rdb  appendonly.aof

# 4. 重启 Redis（自动加载持久化文件）
redis-server /etc/redis/redis.conf

# 5. 验证数据
redis-cli GET "test:data"  # hello
```

---

## ⚠️ 常见陷阱

| 陷阱 | 后果 | 解决方案 |
|------|------|---------|
| 生产用 `appendfsync no` | 宕机丢大量数据 | 用 `everysec` |
| 只用 RDB 不做 AOF | 丢失快照间隔数据 | 开 AOF 或混合 |
| AOF 文件过大不重写 | 磁盘满 | 配置自动重写 |
| `SAVE` 而非 `BGSAVE` | 主线程阻塞 | 用 `BGSAVE` |
| fork 时内存翻倍 | OOM | 确保 `maxmemory` 配置合理 |

---

## 🔗 相关笔记

- [[../阶段一-数据结构与命令/03-Pub-Sub与Stream消息]] — 前置
- [[02-主从复制与哨兵]] — 后续
- [[01-学习/Redis学习路径/阶段二-持久化与高可用/99-阶段2复习检查点|99-阶段2复习检查点]] — 阶段验收
- [[../00-Redis学习路径总索引]] — 返回总索引
- [[阶段二-持久化与高可用/01-RDB与AOF持久化]] — 既有深度参考

---

## ✅ 自检清单

### 🟢 基础（必须掌握）

- [ ] 能解释 RDB 和 AOF 的原理与区别
- [ ] 能配置 AOF 的三种刷盘策略
- [ ] 能用 `BGSAVE` 和 `BGREWRITEAOF` 手动触发
- [ ] 能用 `INFO persistence` 查看持久化状态

### 🟡 进阶（综合运用）

- [ ] 能配置混合持久化并解释其优势
- [ ] 能设计备份与恢复演练流程
- [ ] 能根据 RPO 选择合适的持久化策略
- [ ] 能排查 AOF 文件损坏问题

### 🔴 挑战（独立判断）

- [ ] 能设计 Redis 数据备份与灾难恢复方案
- [ ] 能分析 fork 对内存的影响并给出优化建议
- [ ] 能对比 Redis 持久化与 MySQL binlog 的异同

---

*最后更新：2026-07-20*
