#!/bin/bash
# Redis 阶段二 · 主从 + 哨兵搭建
# 对应笔记: [[02-主从复制与哨兵]]
set -e
echo "=== 启动主节点 ==="
redis-server --port 6379 --daemonize yes
echo "=== 启动从节点 ==="
redis-server replica.conf --port 6380 --daemonize yes
redis-server replica.conf --port 6381 --daemonize yes
sleep 1
echo "=== 查看主从状态 ==="
redis-cli -p 6379 INFO replication | grep -E "role|connected_slaves"
echo "=== 启动哨兵 ==="
redis-sentinel sentinel.conf --daemonize yes
sleep 1
redis-cli -p 26379 SENTINEL masters
echo "=== 模拟故障转移 ==="
echo "手动停止主节点: redis-cli -p 6379 SHUTDOWN"
echo "单哨兵约 5 秒后标记 SDOWN（主观下线）；注意：单哨兵无法完成故障转移，"
echo "要观察自动切换请起 3 个哨兵（quorum 2），见笔记 02-主从复制与哨兵"
echo "=== 清理 ==="
echo "redis-cli -p 6380 SHUTDOWN; redis-cli -p 6381 SHUTDOWN; redis-cli -p 26379 SHUTDOWN"
