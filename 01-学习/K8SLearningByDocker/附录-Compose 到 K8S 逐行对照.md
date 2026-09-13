---
title: 附录-Compose 到 K8S 逐行对照
created: 2026-07-25
tags:
  - K8S
  - Docker
  - Compose
  - 迁移
  - 逐行对照
  - 附录
description: 一个完整的 Docker Compose 应用（Nginx + Flask + Redis + PostgreSQL）逐段迁移到 K8S 的对照实战，左侧 Compose 右侧 K8S YAML，逐行解释每个映射。
lark_doc_url: https://my.feishu.cn/docx/UvHndZIVHog0uLxvXfZcPxRrnMf
---

> 📌 这是 Docker 背景学习者最需要的"看见迁移过程"的练习。将一个完整的 Compose 文件逐段迁移到 K8S。

---

## 原始 Docker Compose 应用

```yaml
# docker-compose.yaml —— Web + API + Redis + PostgreSQL
services:
  web:                                    # ① Nginx 前端
    image: nginx:alpine
    ports: ["80:80"]
    depends_on: [api]
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf

  api:                                    # ② Flask API
    build: ./api
    environment:
      DATABASE_URL: postgres://user:pass@db:5432/app
      REDIS_URL: redis://cache:6379
    depends_on: [db, cache]
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:5000/health"]
      interval: 10s

  db:                                     # ③ PostgreSQL
    image: postgres:15-alpine
    volumes: ["pgdata:/var/lib/postgresql/data"]
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: app

  cache:                                  # ④ Redis
    image: redis:7-alpine

volumes:
  pgdata:
```

---

## 逐段迁移对照

### ① web 服务 → Deployment + Service + ConfigMap

| Compose 行 | K8S 对应 | 迁移说明 |
|-----------|---------|---------|
| `image: nginx:alpine` | `image: nginx:alpine` | **不变** |
| `ports: ["80:80"]` | Service `type: NodePort` | 从容器端口映射升级为 Service 抽象 |
| `volumes: ./nginx.conf:...` | ConfigMap Volume 挂载 | 从本地文件升级为集群级配置资源 |
| `depends_on: [api]` | Init Container | K8S 用 Init Container 实现启动依赖 |
| 无 replicas | `replicas: 2` | Compose 默认 1 个，K8S 显式声明 |

```yaml
# ── K8S: ConfigMap（nginx.conf 外部化）──
apiVersion: v1
kind: ConfigMap
metadata:
  name: nginx-config
data:
  # 完整主配置：events{} + http{} 上下文必需，server{} 必须位于 http{} 内，否则 nginx 启动失败
  nginx.conf: |
    events {}
    http {
      server {
        listen 80;
        location / {
          proxy_pass http://api:5000;   # api 是 Flask 应用，监听 5000（与全课程一致）
        }
      }
    }
---
# ── K8S: Deployment ──
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web
spec:
  replicas: 2
  selector:
    matchLabels:
      app: web
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
      - name: nginx
        image: nginx:alpine
        volumeMounts:
        - name: config
          mountPath: /etc/nginx/nginx.conf
          subPath: nginx.conf      # ★ 用 ConfigMap 挂单个文件必须加 subPath（取键名），否则挂载的是目录，nginx 读不到配置
      volumes:
      - name: config
        configMap:
          name: nginx-config
---
# ── K8S: Service（端口映射）──
apiVersion: v1
kind: Service
metadata:
  name: web
spec:
  type: NodePort        # ≈ -p 80:80
  selector:
    app: web
  ports:
  - port: 80
    targetPort: 80
```

---

### ② api 服务 → Deployment + ConfigMap + Secret + Probe

| Compose 行 | K8S 对应 | 迁移说明 |
|-----------|---------|---------|
| `build: ./api` | `docker build` + `image: myrepo/api:1.0.0` | 构建不变，K8S 用仓库镜像 |
| `environment: DATABASE_URL` | Secret（敏感） | 密码/连接串 → Secret |
| `environment: REDIS_URL` | ConfigMap（非敏感） | 普通配置 → ConfigMap |
| `healthcheck:` | `livenessProbe` + `readinessProbe` | 从单一健康检查升级为双探针 |
| `depends_on: [db, cache]` | `initContainers` | 从简单依赖升级为精确等待逻辑 |

```yaml
# ── K8S: ConfigMap（非敏感配置）──
apiVersion: v1
kind: ConfigMap
metadata:
  name: api-config
data:
  REDIS_URL: "redis://cache:6379"
---
# ── K8S: Secret（敏感配置）──
apiVersion: v1
kind: Secret
metadata:
  name: db-secret
type: Opaque
stringData:
  DATABASE_URL: "postgres://user:pass@db:5432/app"
  POSTGRES_PASSWORD: "pass"       # ③ 的 StatefulSet 也从这个 Secret 引用 POSTGRES_PASSWORD 键
---
# ── K8S: Deployment ──
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api
  template:
    metadata:
      labels:
        app: api
    spec:
      initContainers:                    # ≈ depends_on（精确等待）
      - name: wait-for-db
        image: busybox
        command: ["sh", "-c", "until nc -z db 5432; do sleep 1; done"]
      - name: wait-for-cache
        image: busybox
        command: ["sh", "-c", "until nc -z cache 6379; do sleep 1; done"]
      containers:
      - name: api
        image: myrepo/api:1.0.0
        envFrom:                         # ≈ environment（批量注入）
        - configMapRef:
            name: api-config
        env:                             # 单独注入 Secret
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: DATABASE_URL
        livenessProbe:                    # ≈ healthcheck
          httpGet:
            path: /health
            port: 5000
        readinessProbe:                   # K8S 独有：就绪探针
          httpGet:
            path: /ready
            port: 5000
---
# ── K8S: Service（★ 必须创建：web 的 proxy_pass http://api 靠它提供集群内 DNS 名 "api"）──
apiVersion: v1
kind: Service
metadata:
  name: api
spec:
  selector:
    app: api
  ports:
  - port: 5000
    targetPort: 5000
```

---

### ③ db 服务 → StatefulSet + PVC + Secret

| Compose 行 | K8S 对应 | 迁移说明 |
|-----------|---------|---------|
| `image: postgres:15-alpine` | `image: postgres:15-alpine` | **不变** |
| `volumes: pgdata:...` | `volumeClaimTemplates` | 从 Docker Volume 升级为动态 PVC |
| `environment: POSTGRES_PASSWORD` | Secret | 密码 → Secret |
| 无固定名称 | `StatefulSet`（有序编号 + 稳定 DNS） | 数据库需要稳定标识 |

```yaml
# ── K8S: Headless Service（StatefulSet 必需）──
apiVersion: v1
kind: Service
metadata:
  name: db-headless
spec:
  clusterIP: None          # ★ Headless
  selector:
    app: db
  ports:
  - port: 5432
---
# ── K8S: StatefulSet ──
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: db
spec:
  serviceName: db-headless
  replicas: 1
  selector:
    matchLabels:
      app: db
  template:
    metadata:
      labels:
        app: db
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        env:
        - name: POSTGRES_USER
          value: "user"
        - name: POSTGRES_PASSWORD      # ≈ environment（密码→Secret）
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: POSTGRES_PASSWORD
        - name: POSTGRES_DB
          value: "app"
        - name: PGDATA                 # ★ 避免挂载冲突
          value: /var/lib/postgresql/data/pgdata
        volumeMounts:                 # ≈ volumes: pgdata:...
        - name: data
          mountPath: /var/lib/postgresql/data
  volumeClaimTemplates:               # ≈ docker volume create pgdata
  - metadata:
      name: data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 10Gi
```

---

### ④ cache 服务 → Deployment + Service

| Compose 行 | K8S 对应 | 迁移说明 |
|-----------|---------|---------|
| `image: redis:7-alpine` | `image: redis:7-alpine` | **不变** |
| 无端口暴露 | Service `ClusterIP` | 集群内部访问 |
| 无持久化 | 可选 PVC（如需 Redis 持久化） | Redis 缓存通常无状态 |

```yaml
# ── K8S: Deployment + Service（Redis 无状态）──
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cache
spec:
  replicas: 1
  selector:
    matchLabels:
      app: cache
  template:
    metadata:
      labels:
        app: cache
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
---
apiVersion: v1
kind: Service
metadata:
  name: cache
spec:
  selector:
    app: cache
  ports:
  - port: 6379
    targetPort: 6379
```

---

## 迁移映射总结矩阵

| Docker Compose 概念 | K8S 资源 | 迁移难度 | 关键差异 |
|---------------------|---------|---------|---------|
| `services:` 整体 | 多个独立 YAML | ⭐⭐ | Compose 一个文件，K8S 多资源 |
| `image:` | `image:` | ⭐ | **不变** |
| `build:` | `docker build`（外部） | ⭐ | K8S 不负责构建 |
| `ports:` | Service | ⭐⭐ | 从端口映射升级为网络抽象 |
| `environment:`（非敏感） | ConfigMap | ⭐⭐ | 配置外部化 |
| `environment:`（敏感） | Secret | ⭐⭐ | 密钥外部化 |
| `volumes:` | PV/PVC | ⭐⭐⭐ | 从单机 Volume 到集群存储 |
| `depends_on:` | Init Container | ⭐⭐⭐ | 从简单依赖到精确等待 |
| `healthcheck:` | liveness + readiness Probe | ⭐⭐ | 从单一检查到双探针 |
| `restart: always` | Deployment 控制器 | ⭐ | 从进程重启到控制器自愈 |
| `deploy.replicas` | `replicas:` | ⭐ | 直接对应 |
| `deploy.resources` | `resources.requests/limits` | ⭐⭐ | 从限制到请求+限制 |
| `networks:` | CNI 自动管理 | ⭐⭐⭐ | 无需手动创建网络 |
| `volumes:` 顶层声明 | StorageClass 动态供给 | ⭐⭐⭐ | 从手动创建到自动供给 |

---

## 一键迁移工具：Kompose

```bash
# Kompose 可以自动转换 Compose → K8S YAML（但不完美）
# 安装
curl -L https://raw.githubusercontent.com/kubernetes/kompose/master/scripts/get/setup.sh | bash

# 转换
kompose convert -f docker-compose.yaml -o k8s/
# 生成 deployment.yaml / service.yaml / pvc.yaml 等

# 注意：Kompose 转换结果需要手动调整
# - db 应改为 StatefulSet（Kompose 默认生成 Deployment）
# - 环境变量应拆分为 ConfigMap/Secret
# - depends_on 应改为 Init Container
```

> [!warning] Kompose 是起点不是终点
> Kompose 能完成 60-70% 的自动转换，但复杂场景（StatefulSet、Init Container、探针配置）需要手动调整。把它当作迁移的起点，而非一键完成。

---

## 🧪 实践练习

### 🟢 基础练习：手动迁移一个服务

```bash
# 1. 选 web 服务，手动写出对应的 K8S Deployment + Service + ConfigMap
# 2. 在 minikube 上部署
kubectl apply -f web-deployment.yaml -f web-service.yaml -f nginx-configmap.yaml
# 3. 验证：kubectl get pods,svc
# 4. 用 minikube service web --url 访问
```

### 🟡 进阶练习：用 Kompose 转换并修正

```bash
# 1. 用 Kompose 转换整个 Compose 文件
kompose convert -f docker-compose.yaml -o k8s/
# 2. 检查生成的 YAML，找出需要修正的地方
# 3. 手动修正：db 改为 StatefulSet，环境变量拆分为 ConfigMap/Secret
# 4. 部署修正后的 YAML
kubectl apply -f k8s/
# 5. 验证所有服务正常运行
```

### 🔴 挑战练习：完整迁移 + 验证

```bash
# 1. 完整迁移 4 个服务到 K8S（含 StatefulSet、Init Container、Probe）
# 2. 验证服务间通信
kubectl exec -it deployment/api -- sh -c "nc -z db 5432 && echo DB_OK"
kubectl exec -it deployment/api -- sh -c "nc -z cache 6379 && echo CACHE_OK"
# 3. 通过 Ingress 访问 web → web 调 api → api 调 db/cache
# 4. 模拟 Pod 故障，验证自愈
kubectl delete pod -l app=api
kubectl get pods -l app=api -w  # 观察自动重建
```

---

## 相关笔记

- ⬅️ 前置：[[01-环境搭建与核心概念映射]] ~ [[06-Docker有K8S无与K8S有Docker无]] — 全部核心笔记
- 🔗 关联：[[毕业项目-Web应用K8S化/v1-Deployment基础版]] — 毕业项目就是完整的 Compose→K8S 迁移
- 🔗 关联：[[01-学习/K8SLearningByDocker/07-业务场景实战合集|07-业务场景实战合集]] — 场景 1 是更完整的微服务迁移
- 🔗 关联：[[附录-Docker 到 kubectl 速查卡片]] — 命令级对照

---

*最后更新：2026-07-25*