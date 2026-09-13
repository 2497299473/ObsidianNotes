#!/bin/bash
# Redis 阶段二 · 持久化验证
# 对应笔记: [[01-RDB与AOF持久化]]
set -e
mkdir -p data
echo "=== 启动 Redis（启用 AOF）==="
redis-server redis.conf --daemonize yes
sleep 1
echo "=== 写入数据 ==="
redis-cli SET "test:persist" "hello-world"
redis-cli ZADD "rank:game" 100 "alice"
redis-cli SAVE  # 触发 RDB
echo "=== 持久化状态 ==="
redis-cli INFO persistence | grep -E "aof_enabled|rdb_last_save"
echo "=== 重启 Redis ==="
redis-cli SHUTDOWN NOSAVE
redis-server redis.conf --daemonize yes
sleep 1
echo "=== 验证数据恢复 ==="
echo "String: $(redis-cli GET 'test:persist')"
echo "ZSet: $(redis-cli ZRANGE 'rank:game' 0 -1 WITHSCORES)"
echo "=== 清理 ==="
redis-cli FLUSHALL
redis-cli SHUTDOWN NOSAVE
