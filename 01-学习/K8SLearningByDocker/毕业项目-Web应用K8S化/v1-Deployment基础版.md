---
title: v1-Deployment基础版
created: 2026-07-23
project: 毕业项目-Web应用K8S化
version: 1
difficulty: ⭐⭐
estimated_hours: 2
tags:
  - 毕业项目
  - K8S
  - Docker
  - Deployment
description: 毕业项目 v1：将 Docker Compose 多容器应用（Nginx + Flask API + Redis + PostgreSQL）的每个容器转换为独立的 K8S Deployment。只保证跑起来，不考虑网络、存储、配置外部化。
lark_doc_url: https://my.feishu.cn/docx/F4fEdEAWZoY71WxrEmbcssm9nsh
---

## 版本目标

将 Docker Compose 多容器应用中的**每个容器转换为独立的 Deployment**，验证 Pod 正常运行。这是迁移的第一步——从 `docker run` 到 `kubectl apply`。

> [!warning] v1 的 db 暂用 Deployment（无持久化）
> 本版本 db（PostgreSQL）使用 Deployment 部署，**没有持久化存储**——Pod 重建后数据会丢失。这是有意为之：v1 只验证"能跑起来"，v3 才引入 PVC + StatefulSet 实现持久化。请按版本顺序学习，不要跳过 v3。

## 原始 Docker Compose 应用

```yaml
# docker-compose.yaml
services:
  web:
    image: nginx:alpine
    ports: ["80:80"]
    depends_on: [api]
  api:
    image: flask-api:1.0.0
    environment:
      DATABASE_URL: postgres://user:pass@db:5432/app
      REDIS_URL: redis://cache:6379
    depends_on: [db, cache]
  db:
    image: postgres:15-alpine
    volumes: ["pgdata:/var/lib/postgresql/data"]
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: app
  cache:
    image: redis:7-alpine
volumes:
  pgdata:
```

## v1 改造：容器 → Deployment

**只改一个维度**：将 Compose 服务的容器改为 K8S Deployment，保持最小可用。

| Docker Compose | K8S v1 | 说明 |
|---------------|--------|------|
| web (nginx) | Deployment web | 无状态 Deployment |
| api (Flask) | Deployment api | 无状态 Deployment |
| db (PostgreSQL) | Deployment db | 临时用 Deployment（v3 改 StatefulSet） |
| cache (Redis) | Deployment cache | 无状态 Deployment |
| ports: ["80:80"] | （暂不处理） | v2 加 Service |
| volumes: pgdata | （暂不处理） | v3 加 PVC |
| environment | 直接写在 YAML 中 | v4 改 ConfigMap/Secret |

## 镜像从哪来：先构建、再加载

Deployment 里写 `flask-api:1.0.0`，集群里就必须真的存在这个 tag。v1 用的是本地构建的镜像，标准三步：

```bash
# 1. 构建（在 Flask 项目目录执行）
docker build -t flask-api:1.0.0 .

# 2. 把镜像加载进 minikube 的容器运行时（关键一步！不 load 会 ImagePullBackOff）
minikube image load flask-api:1.0.0
minikube image ls | grep flask-api      # 应看到 docker.io/library/flask-api:1.0.0

# 3. 容器 spec 里显式写 imagePullPolicy: IfNotPresent（见下方 api Deployment）
```

> [!warning] 为什么不用 `flask-api:latest`
> ① `:latest` 的默认拉取策略是 `Always`，镜像只在本地时反而要去远端拉、直接 `ImagePullBackOff`；
> ② 每次 build 都覆盖同一个 tag，回滚时分不清"哪一版"——02 篇「坑 2」讲的就是这件事。
> 显式写 `imagePullPolicy: IfNotPresent` 后，行为不再随 tag 变化，换镜像用 `kubectl rollout restart` 即可。

## K8S YAML

```yaml
# v1-all-deployments.yaml
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web
  labels: { app: web, version: v1 }
spec:
  replicas: 2
  selector: { matchLabels: { app: web } }
  template:
    metadata: { labels: { app: web, version: v1 } }
    spec:
      containers:
      - name: nginx
        image: nginx:alpine
        ports: [{ containerPort: 80 }]
        resources:
          requests: { cpu: "100m", memory: "64Mi" }
          limits:   { cpu: "200m", memory: "128Mi" }
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api
  labels: { app: api, version: v1 }
spec:
  replicas: 2
  selector: { matchLabels: { app: api } }
  template:
    metadata: { labels: { app: api, version: v1 } }
    spec:
      containers:
      - name: api
        image: flask-api:1.0.0
        imagePullPolicy: IfNotPresent    # 镜像由 minikube image load 预置（见上一节）
        env:
        - { name: DATABASE_URL, value: "postgres://user:pass@db:5432/app" }
        - { name: REDIS_URL, value: "redis://cache:6379" }
        ports: [{ containerPort: 5000 }]
        resources:
          requests: { cpu: "100m", memory: "128Mi" }
          limits:   { cpu: "200m", memory: "256Mi" }
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: db
  labels: { app: db, version: v1 }
spec:
  replicas: 1
  selector: { matchLabels: { app: db } }
  template:
    metadata: { labels: { app: db, version: v1 } }
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        env:
        - { name: POSTGRES_USER, value: "user" }
        - { name: POSTGRES_PASSWORD, value: "pass" }
        - { name: POSTGRES_DB, value: "app" }
        - { name: PGDATA, value: "/var/lib/postgresql/data/pgdata" }
        ports: [{ containerPort: 5432 }]
        resources:
          requests: { cpu: "200m", memory: "256Mi" }
          limits:   { cpu: "500m", memory: "512Mi" }
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cache
  labels: { app: cache, version: v1 }
spec:
  replicas: 1
  selector: { matchLabels: { app: cache } }
  template:
    metadata: { labels: { app: cache, version: v1 } }
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports: [{ containerPort: 6379 }]
        resources:
          requests: { cpu: "100m", memory: "128Mi" }
          limits:   { cpu: "200m", memory: "256Mi" }
```

## 验证步骤

```bash
# 1. 部署
kubectl apply -f v1-all-deployments.yaml

# 2. 验证所有 Deployment
kubectl get deployments
# NAME    READY   UP-TO-DATE   AVAILABLE   AGE
# api     2/2     2            2           30s
# cache   1/1     1            1           30s
# db      1/1     1            1           30s
# web     2/2     2            2           30s

# 3. 验证所有 Pod Running
kubectl get pods

# 4. 查看 Pod 日志
kubectl logs deployment/web
kubectl logs deployment/db

# 5. 扩缩容验证
kubectl scale deployment api --replicas=3
kubectl get pods -l app=api  # 应看到 3 个 Pod

# 6. 自愈验证：删除一个 Pod
# 注意：不能写 `kubectl delete pod -l app=web | head -1`——删除请求在管道前就已全部发出，
# head -1 只是截断输出，实际会删掉所有匹配的 Pod
kubectl delete pod $(kubectl get pods -l app=web -o jsonpath='{.items[0].metadata.name}')
kubectl get pods -l app=web -w  # 观察新 Pod 自动创建（自愈！）

# 7. 清理
kubectl delete -f v1-all-deployments.yaml
```

## Docker vs K8S 对比

| 维度 | Docker Compose | K8S v1 |
|------|---------------|--------|
| 配置文件 | 1 个 compose.yaml | 1 个 YAML（含 4 个 Deployment） |
| 扩缩容 | `docker compose scale web=3` | `kubectl scale deployment web --replicas=3` |
| 自愈 | `restart: always` | Deployment 控制器自动重建 |
| 资源限制 | `deploy.resources.limits` | `resources.requests/limits` |
| 网络 | Compose 内置 DNS | 暂无（v2 加 Service） |
| 持久化 | Volume 简单 | 暂无（v3 加 PVC） |

## 🎯 本版自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 4 个 Deployment 全部 Running，`kubectl scale` 扩缩容正常 |
| 🟡 进阶 | 删除一个 Pod 后观察自动重建（自愈），理解 `kubectl logs` 查看日志 |
| 🔴 挑战 | 解释为什么 v1 版本只能"跑起来"但不能对外提供服务（缺少 Service） |

## 下一版本

→ [[v2-Service网络版]] — 添加 Service 和 Ingress，让应用对外可访问

---

*最后更新：2026-07-23*