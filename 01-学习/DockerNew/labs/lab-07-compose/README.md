---
lark_doc_url: https://my.feishu.cn/docx/AZREdgtF5orCTWxhbsncmNDTnOb
---
# Lab 07：Docker Compose 多容器编排

> 📖 对应章节：[[07-Docker Compose 多容器编排]]
> ⏱️ 预计用时：60 分钟
> 🎯 目标：用 Compose 一键启动 Web + PostgreSQL + Redis 三服务，体验健康检查、依赖管理和数据持久化

---

## 前置条件

- 完成 Lab 03（理解 Dockerfile 和镜像构建）
- Docker Compose V2 已安装（`docker compose version`）

---

## 🚀 快速开始

```bash
cd lab-07-compose
docker compose up -d --build
```

等待 10-15 秒（healthcheck 就绪），然后：

```bash
# 查看服务状态——三个服务全部 Up (healthy)
docker compose ps

# 浏览器访问
# http://localhost:5000/health          → {"status":"ok"}
# http://localhost:5000/api/todos       → []
# http://localhost:5000/api/cache/test  → {"visit_count":1}
```

---

## 📋 服务说明

| 服务 | 镜像 | 端口 | 作用 |
|------|------|------|------|
| **web** | Python 3.12 + Flask | 5000:5000 | TODO REST API + Redis 缓存演示 |
| **db** | PostgreSQL 16 Alpine | 5432 | 数据存储（Volume 持久化） |
| **cache** | Redis 7 Alpine | 6379 | 缓存（visit counter） |

---

## 🎯 实践任务

### 1. 一键启动并验证三服务

```bash
docker compose up -d --build
docker compose ps
# 三个服务全部 Up (healthy)

# 观察启动顺序：db 和 cache 先启动，web 等待 healthcheck 通过
docker compose logs --tail 30
```

### 2. 测试 CRUD API

```bash
# 创建 TODO
curl -X POST http://localhost:5000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"学习 Docker Compose","done":false}'
# {"id":1,"title":"学习 Docker Compose","done":false}

# 列出所有 TODO
curl http://localhost:5000/api/todos

# 更新 TODO
curl -X PUT http://localhost:5000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"学完 Compose","done":true}'

# 删除 TODO
curl -X DELETE http://localhost:5000/api/todos/1
```

### 3. 测试 Redis 缓存

```bash
# 多次访问，观察 visit_count 递增
curl http://localhost:5000/api/cache/test
curl http://localhost:5000/api/cache/test
curl http://localhost:5000/api/cache/test
# {"visit_count":3}
```

### 4. 验证容器名 DNS 互访

```bash
# 进入 web 容器，用容器名 ping 数据库
docker compose exec web sh
# 在容器内执行：
  pip install psycopg2-binary 2>/dev/null
  python -c "import socket; print(socket.gethostbyname('db'))"
  # 输出 IP 地址，证明 DNS 解析生效
  exit
```

### 5. 验证数据库持久化（Volume）

```bash
# 创建一条 TODO
curl -X POST http://localhost:5000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"持久化测试","done":false}'

# 停止并删除 web 容器
docker compose stop web && docker compose rm -f web

# 重新启动
docker compose up -d web

# 数据还在！
curl http://localhost:5000/api/todos
# [{"id":1,"title":"持久化测试","done":false}]
```

### 6. 体验健康检查和依赖

```bash
# 模拟 DB 故障
docker compose stop db

# 查看 web 状态
docker compose ps
# web 的 healthcheck 可能变成 unhealthy

# 恢复 DB
docker compose start db

# 等待恢复
docker compose ps
# 全部恢复 healthy
```

### 7. 环境变量管理

```bash
# 查看解析后的最终配置（.env 变量插值）
docker compose config

# 修改端口测试
WEB_PORT=6000 docker compose up -d
# web 现在监听 6000
curl http://localhost:6000/health
```

### 8. 清理

```bash
docker compose down       # 删容器+网络，保留 Volume
docker compose down -v    # ⚠️ 删容器+Volume（数据丢失！）
```

---

## ✅ 通关标准

- [ ] 三个服务全部 `Up (healthy)`
- [ ] 能用 curl 完成 TODO 的 CRUD 全流程
- [ ] 删除 web 容器后重新启动，PostgreSQL 数据不丢失
- [ ] 能用容器名 `db` 在 web 容器内做 DNS 解析
- [ ] 能解释 `depends_on` + `condition: service_healthy` 的作用
- [ ] 能区分 `docker compose down` 和 `down -v`

---

## 🔗 下一步

→ [[08-镜像优化：多阶段构建与瘦身]] | → Lab 08

---

*最后更新：2026-07-28*
