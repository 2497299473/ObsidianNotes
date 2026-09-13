---
title: v4-ConfigMap与Secret版
created: 2026-07-23
project: 毕业项目-Web应用K8S化
version: 4
difficulty: ⭐⭐⭐⭐
estimated_hours: 3
tags:
  - 毕业项目
  - K8S
  - Docker
  - ConfigMap
  - Secret
  - 配置管理
description: 毕业项目 v4：在 v3 基础上将硬编码的环境变量和密码提取为 ConfigMap 和 Secret，实现配置外部化。从 Docker -e 环境变量迁移到 K8S ConfigMap/Secret。
lark_doc_url: https://my.feishu.cn/docx/OUVvdroiWos5lCxBmk3cNQCHnOc
---

## 版本目标

在 v3（持久化存储）基础上，**只改一个维度：配置外部化**，将硬编码的环境变量和密码提取为 ConfigMap 和 Secret。

## v3 → v4 改造

| 配置项 | v3 | v4 | 说明 |
|--------|----|-----|------|
| DATABASE_URL | 写死在 YAML | ConfigMap | 可独立更新 |
| REDIS_URL | 写死在 YAML | ConfigMap | 可独立更新 |
| POSTGRES_USER | 写死在 YAML | ConfigMap | 非敏感信息 |
| POSTGRES_PASSWORD | 写死在 YAML | Secret | 敏感信息 |
| POSTGRES_DB | 写死在 YAML | ConfigMap | 非敏感信息 |
| LOG_LEVEL | 无 | ConfigMap | 新增配置项 |

## Docker Compose 配置（回顾）

```yaml
services:
  api:
    environment:
      DATABASE_URL: postgres://user:pass@db:5432/app
      REDIS_URL: redis://cache:6379
  db:
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: app
```

## K8S YAML（新增 ConfigMap 和 Secret）

```yaml
# v4-config-secret.yaml

# ===== ConfigMap: 非敏感配置 =====
apiVersion: v1
kind: ConfigMap
metadata: { name: app-config, labels: { app: webapp, version: v4 } }
data:
  DATABASE_URL: "postgres://user:$(POSTGRES_PASSWORD)@db-0.db-svc:5432/app"   # $(POSTGRES_PASSWORD) 由 K8S 变量替换注入
  REDIS_URL: "redis://cache-svc:6379"
  POSTGRES_USER: "user"
  POSTGRES_DB: "app"
  LOG_LEVEL: "info"
  API_PORT: "5000"
---
# ===== Secret: 敏感信息 =====
apiVersion: v1
kind: Secret
metadata: { name: app-secret, labels: { app: webapp, version: v4 } }
type: Opaque
stringData:            # 明文写入，kubectl 自动 base64 编码
  POSTGRES_PASSWORD: "super-secret-password-v4"
  # 生产环境不要将密码写在这里，用 kubectl create secret 或 Vault
---
# ===== api Deployment（引用 ConfigMap + Secret）=====
apiVersion: apps/v1
kind: Deployment
metadata: { name: api, labels: { app: api, version: v4 } }
spec:
  replicas: 2
  selector: { matchLabels: { app: api } }
  template:
    metadata: { labels: { app: api, version: v4 } }
    spec:
      containers:
      - name: api
        image: flask-api:latest
        envFrom:                       # ★ 批量注入 ConfigMap
        - configMapRef: { name: app-config }
        env:                           # ★ 单独注入 Secret 与依赖变量
        - name: POSTGRES_PASSWORD      # 被 $(...) 引用的变量必须定义在前
          valueFrom:
            secretKeyRef: { name: app-secret, key: POSTGRES_PASSWORD }
        - name: DATABASE_URL           # 值里的 $(POSTGRES_PASSWORD) 在此被替换展开
          valueFrom:                   # （env 项覆盖 envFrom 注入的未展开值）
            configMapKeyRef: { name: app-config, key: DATABASE_URL }
        ports: [{ containerPort: 5000 }]
        readinessProbe:
          httpGet: { path: /health, port: 5000 }
        resources:
          requests: { cpu: "100m", memory: "128Mi" }
          limits:   { cpu: "200m", memory: "256Mi" }
---
# ===== db StatefulSet（引用 ConfigMap + Secret）=====
# 在 v3 基础上修改 env 段
apiVersion: apps/v1
kind: StatefulSet
metadata: { name: db, labels: { app: db, version: v4 } }
spec:
  serviceName: db-svc
  replicas: 1
  selector: { matchLabels: { app: db } }
  template:
    metadata: { labels: { app: db, version: v4 } }
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        envFrom:
        - configMapRef: { name: app-config }
        env:
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef: { name: app-secret, key: POSTGRES_PASSWORD }
        - name: PGDATA
          value: "/var/lib/postgresql/data/pgdata"
        ports: [{ containerPort: 5432 }]
        volumeMounts:
        - { name: data, mountPath: /var/lib/postgresql/data }
        readinessProbe:
          exec: { command: ["pg_isready", "-U", "user"] }
  volumeClaimTemplates:
  - metadata: { name: data }
    spec:
      accessModes: [ReadWriteOnce]
      resources: { requests: { storage: 10Gi } }
```

## 验证步骤

```bash
# 1. 创建 ConfigMap 和 Secret
kubectl apply -f v4-config-secret.yaml

# 2. 验证 ConfigMap
kubectl get configmap app-config -o yaml

# 3. 验证 Secret（base64 编码）
kubectl get secret app-secret -o jsonpath='{.data.POSTGRES_PASSWORD}' | base64 -d
# 应看到 super-secret-password-v4

# 4. 验证环境变量注入
kubectl exec -it deployment/api -- env | grep -E "DATABASE_URL|REDIS_URL|POSTGRES"
# 应看到 ConfigMap 和 Secret 中的值

kubectl exec -it db-0 -- env | grep POSTGRES_PASSWORD
# 应看到 Secret 中的值

# 5. 更新 ConfigMap 后重启生效（envFrom 不支持热更新）
kubectl edit configmap app-config
# 修改 LOG_LEVEL: "debug"
kubectl rollout restart deployment/api
kubectl rollout restart statefulset/db

# 6. 验证 Secret 安全性
kubectl describe secret app-secret
# 只显示名称和大小，不显示内容

# 7. 清理
kubectl delete configmap app-config
kubectl delete secret app-secret
kubectl delete -f v4-config-secret.yaml
kubectl delete pvc data-db-0
```

## Docker vs K8S 对比

| 维度 | Docker Compose | K8S v4 |
|------|---------------|--------|
| 配置存储 | 写死在 compose.yaml 或 `.env` | ConfigMap 独立资源 |
| 密钥存储 | 写死或 `docker secret` | Secret 独立资源 |
| 配置更新 | 修改 compose.yaml + `docker compose up -d` | `kubectl edit configmap` + `rollout restart` |
| 密钥加密 | Swarm Secret 加密存储 | K8S Secret 默认 base64（需 etcd 加密） |
| Git 安全 | 密码容易泄露到 git | Secret 不应提交到 git |
| 批量注入 | 多个 `-e` | `envFrom` 一行搞定 |

## 🎯 本版自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | ConfigMap 和 Secret 创建成功，Pod 环境变量正确注入 |
| 🟡 进阶 | 修改 ConfigMap 后 `rollout restart` 生效，验证 Secret 不在 `describe` 中暴露 |
| 🔴 挑战 | 解释为什么 Secret 默认只 base64 编码不是加密，如何启用 etcd 静态加密 |

## 下一版本

→ [[v5-Helm包管理版]] — 将整个应用打包为 Helm Chart，一键部署

---

*最后更新：2026-07-23*