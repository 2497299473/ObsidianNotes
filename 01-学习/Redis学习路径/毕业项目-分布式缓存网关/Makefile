# Redis 毕业项目·自动化验收脚本
# 对应笔记: 毕业项目-分布式缓存网关
# 用法: make <version>  或  make all

.DEFAULT_GOAL := help

.PHONY: help v1 v2 v3 v4 v5 v6 v7 all clean

help: ## 显示帮助
	@echo "Redis 毕业项目验收脚本"
	@echo "用法: make <version>"
	@echo "  v1  单机缓存服务"
	@echo "  v2  多数据结构组合"
	@echo "  v3  持久化与AOF"
	@echo "  v4  主从高可用"
	@echo "  v5  Cluster分片"
	@echo "  v6  Lua脚本与事务"
	@echo "  v7  监控与压测调优"

v1: ## v1: 单机缓存服务
	python app.py &
	sleep 2
	curl -X PUT http://localhost:5000/cache/test -H "Content-Type: application/json" -d '{"value":"ok"}'
	curl -sf http://localhost:5000/cache/test | grep -q "ok"
	curl -X DELETE http://localhost:5000/cache/test
	kill %1
	@echo "✅ v1 验收通过"

v2: ## v2: 多数据结构组合
	python -c "import redis; r=redis.Redis(); r.hset('t',{'a':'1'}); print('Hash:', r.hgetall('t')); r.zadd('z',{'x':1}); print('ZSet:', r.zrange('z',0,-1,withscores=True)); r.delete('t','z')"
	@echo "✅ v2 验收通过"

v3: ## v3: 持久化与AOF
	redis-cli CONFIG SET appendonly yes
	redis-cli SET "v3:test" "persisted"
	redis-cli BGREWRITEAOF
	sleep 1
	redis-cli SHUTDOWN NOSAVE
	redis-server --daemonize yes --appendonly yes
	sleep 1
	[ "$$(redis-cli GET 'v3:test')" = "persisted" ]
	redis-cli DEL "v3:test"
	@echo "✅ v3 验收通过"

v4: ## v4: 主从高可用
	redis-server --port 6379 --daemonize yes
	redis-server --port 6380 --daemonize yes --replicaof 127.0.0.1 6379
	sleep 1
	redis-cli -p 6379 INFO replication | grep -q "connected_slaves:1"
	redis-cli -p 6380 INFO replication | grep -q "role:slave"
	redis-cli -p 6380 SHUTDOWN NOSAVE
	redis-cli -p 6379 SHUTDOWN NOSAVE
	@echo "✅ v4 验收通过"

v5: ## v5: Cluster分片
	for p in 7000 7001 7002 7003 7004 7005; do redis-server --port $$p --cluster-enabled yes --cluster-node-timeout 5000 --daemonize yes; done
	sleep 1
	redis-cli --cluster create 127.0.0.1:7000 127.0.0.1:7001 127.0.0.1:7002 127.0.0.1:7003 127.0.0.1:7004 127.0.0.1:7005 --cluster-replicas 1 --cluster-yes
	redis-cli -p 7000 CLUSTER INFO | grep -q "cluster_state:ok"
	redis-cli -c -p 7000 SET "v5:test" "cluster"
	[ "$$(redis-cli -c -p 7000 GET 'v5:test')" = "cluster" ]
	for p in 7000 7001 7002 7003 7004 7005; do redis-cli -p $$p SHUTDOWN NOSAVE 2>/dev/null; done
	@echo "✅ v5 验收通过"

v6: ## v6: Lua脚本与事务
	python -c "import redis; r=redis.Redis(); r.set('stock','10'); s=r.register_script('local s=tonumber(redis.call(\"GET\",KEYS[1])) if s>=tonumber(ARGV[1]) then redis.call(\"DECRBY\",KEYS[1],ARGV[1]) return 1 else return 0 end'); assert(s(keys=['stock'],args=[3])==1); assert(s(keys=['stock'],args=[100])==0); print('Lua OK'); r.delete('stock')"
	@echo "✅ v6 验收通过"

v7: ## v7: 监控与压测调优
	redis-cli INFO memory | grep -q "used_memory"
	redis-cli CONFIG SET maxmemory 256mb
	redis-cli CONFIG SET maxmemory-policy allkeys-lru
	redis-benchmark -t set,get -n 10000 -c 50 -q
	redis-cli SLOWLOG RESET
	@echo "✅ v7 验收通过"

all: v1 v2 v3 v4 v5 v6 v7
	@echo "🎉 全部验收通过！"

clean:
	redis-cli FLUSHALL 2>/dev/null || true
	for p in 6380 6379 7000 7001 7002 7003 7004 7005; do redis-cli -p $$p SHUTDOWN NOSAVE 2>/dev/null; done
	pkill -f "python app.py" 2>/dev/null || true
