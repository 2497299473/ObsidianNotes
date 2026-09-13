#!/bin/bash
# Redis 阶段二 · Cluster 搭建
# 对应笔记: [[03-Cluster分片集群]]
set -e
CONF_DIR="."
echo "=== 启动 6 个节点 ==="
for port in 7000 7001 7002 7003 7004 7005; do
  sed "s/7000/$port/g" "$CONF_DIR/redis-7000.conf" > "$CONF_DIR/redis-$port.conf"
  redis-server "$CONF_DIR/redis-$port.conf"
done
sleep 1
echo "=== 创建集群（3 主 3 从）==="
redis-cli --cluster create \
  127.0.0.1:7000 127.0.0.1:7001 127.0.0.1:7002 \
  127.0.0.1:7003 127.0.0.1:7004 127.0.0.1:7005 \
  --cluster-replicas 1 --cluster-yes
echo "=== 集群状态 ==="
redis-cli -p 7000 CLUSTER INFO
redis-cli -p 7000 CLUSTER NODES
echo "=== 验证自动路由 ==="
redis-cli -c -p 7000 SET "user:1" "alice"
redis-cli -c -p 7000 GET "user:1"
echo "=== Hash Tag ==="
redis-cli -c -p 7000 MSET "{user:1}:name" "alice" "{user:1}:age" "30"
echo "=== 清理 ==="
for p in 7000 7001 7002 7003 7004 7005; do
  redis-cli -p $p SHUTDOWN NOSAVE 2>/dev/null || true
done
