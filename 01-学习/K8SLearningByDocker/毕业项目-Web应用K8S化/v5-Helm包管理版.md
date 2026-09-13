---
title: v5-Helm包管理版
created: 2026-07-23
project: 毕业项目-Web应用K8S化
version: 5
difficulty: ⭐⭐⭐⭐
estimated_hours: 4
tags:
  - 毕业项目
  - K8S
  - Docker
  - Helm
  - 包管理
description: 毕业项目 v5：将 v4 的所有 YAML 打包为 Helm Chart，实现参数化、版本管理、一键部署。这是 Docker Compose 无法做到的工程化能力，对标 docker stack deploy。
lark_doc_url: https://my.feishu.cn/docx/TfwydJx9goOCuixbwkOc7bponZf
---

## 版本目标

在 v4（配置外部化）基础上，**只改一个维度：打包为 Helm Chart**，实现一键部署、参数化、版本管理。对标 Docker Compose 的"一个文件启动"体验。

## v4 → v5 改造

| 维度 | v4 | v5 | 说明 |
|------|----|-----|------|
| 部署方式 | 多个 YAML 手动 apply | `helm install webapp ./chart` | 一条命令 |
| 配置管理 | 写死在各 YAML | `values.yaml` 参数化 | 一次修改，全局生效 |
| 版本管理 | 无 | `helm upgrade` / `helm rollback` | 版本历史 |
| 模板化 | 无 | `{{ .Values.xxx }}` Go 模板 | 复用性 |
| Docker 对标 | — | Docker Compose 单文件体验 | 最接近 |

## Chart 目录结构

```
webapp-chart/
├── Chart.yaml            # Chart 元信息
├── values.yaml          # 默认配置值（对标 Docker Compose 变量）
├── values-dev.yaml      # 开发环境覆盖
├── values-prod.yaml     # 生产环境覆盖
├── templates/
│   ├── _helpers.tpl     # 模板辅助函数
│   ├── configmap.yaml
│   ├── secret.yaml
│   ├── web-deployment.yaml
│   ├── web-service.yaml
│   ├── api-deployment.yaml
│   ├── api-service.yaml
│   ├── db-statefulset.yaml
│   ├── db-service.yaml
│   ├── cache-deployment.yaml
│   ├── cache-service.yaml
│   └── ingress.yaml
└── .helmignore
```

## Chart.yaml

```yaml
apiVersion: v2
name: webapp
description: "Web Application K8S Chart - 从 Docker Compose 迁移而来"
type: application
version: 1.0.0
appVersion: "1.0.0"
```

## values.yaml（参数化配置）

```yaml
# values.yaml —— 所有可配置参数，类似 Docker Compose 的环境变量
global:
  namespace: default

# Nginx Web 服务
web:
  image: { repository: nginx, tag: alpine }
  replicas: 2
  resources:
    requests: { cpu: "100m", memory: "64Mi" }
    limits:   { cpu: "200m", memory: "128Mi" }

# Flask API 服务
api:
  image: { repository: flask-api, tag: latest }
  replicas: 2
  resources:
    requests: { cpu: "100m", memory: "128Mi" }
    limits:   { cpu: "200m", memory: "256Mi" }

# PostgreSQL 数据库
db:
  image: { repository: postgres, tag: "15-alpine" }
  storage: { size: "10Gi", className: standard }
  credentials:
    user: user
    password: super-secret-password  # 生产环境用 --set 覆盖
    database: app

# Redis 缓存
cache:
  image: { repository: redis, tag: "7-alpine" }
  replicas: 1

# Ingress 配置（不设 rewrite-target 注解，URI 直通，/api/* 原样转发）
ingress:
  enabled: true
  className: nginx
  host: app.local
```

## templates/_helpers.tpl（辅助函数）

```yaml
{{- define "webapp.labels" -}}
app.kubernetes.io/name: {{ .Chart.Name }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/version: {{ .Chart.AppVersion }}
{{- end -}}

{{- define "webapp.fullname" -}}
{{- printf "%s-%s" .Release.Name .Chart.Name -}}
{{- end -}}
```

## templates/web-deployment.yaml（模板化示例）

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ .Release.Name }}-web
  labels:
    {{- include "webapp.labels" . | nindent 4 }}
spec:
  replicas: {{ .Values.web.replicas }}
  selector:
    matchLabels:
      app: {{ .Release.Name }}-web
  template:
    metadata:
      labels:
        app: {{ .Release.Name }}-web
    spec:
      containers:
      - name: nginx
        image: "{{ .Values.web.image.repository }}:{{ .Values.web.image.tag }}"
        ports: [{ containerPort: 80 }]
        resources:
          {{- toYaml .Values.web.resources | nindent 10 }}
---
apiVersion: v1
kind: Service
metadata:
  name: {{ .Release.Name }}-web-svc
spec:
  type: ClusterIP
  selector:
    app: {{ .Release.Name }}-web
  ports: [{ port: 80, targetPort: 80 }]
```

## templates/db-statefulset.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: {{ .Release.Name }}-db-svc
spec:
  clusterIP: None
  selector:
    app: {{ .Release.Name }}-db
  ports: [{ port: 5432 }]
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: {{ .Release.Name }}-db
spec:
  serviceName: {{ .Release.Name }}-db-svc
  replicas: 1
  selector:
    matchLabels:
      app: {{ .Release.Name }}-db
  template:
    metadata:
      labels:
        app: {{ .Release.Name }}-db
    spec:
      containers:
      - name: postgres
        image: "{{ .Values.db.image.repository }}:{{ .Values.db.image.tag }}"
        env:
        - { name: POSTGRES_USER, value: {{ .Values.db.credentials.user | quote }} }
        - { name: POSTGRES_PASSWORD, value: {{ .Values.db.credentials.password | quote }} }
        - { name: POSTGRES_DB, value: {{ .Values.db.credentials.database | quote }} }
        - { name: PGDATA, value: "/var/lib/postgresql/data/pgdata" }
        volumeMounts:
        - { name: data, mountPath: /var/lib/postgresql/data }
  volumeClaimTemplates:
  - metadata: { name: data }
    spec:
      accessModes: [ReadWriteOnce]
      storageClassName: {{ .Values.db.storage.className }}
      resources:
        requests: { storage: {{ .Values.db.storage.size }} }
```

## 部署与验证

```bash
# 1. 安装 Helm（如果还没有）
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash

# 2. 创建 Chart 骨架
helm create webapp-chart
# 然后用上面的 Chart.yaml、values.yaml 和 templates/ 替换默认内容

# 3. 验证模板（不实际部署）
helm template myapp ./webapp-chart
# 检查渲染输出是否正确

# 4. 一键部署！
helm install myapp ./webapp-chart

# 5. 验证
helm list
helm status myapp
kubectl get pods,svc,ingress

# 6. 升级（修改参数后）
helm upgrade myapp ./webapp-chart --set api.replicas=3
kubectl get pods -l app=myapp-api  # 应看到 3 个 Pod

# 7. 回滚
helm rollback myapp 1
helm history myapp

# 8. 卸载（一键清理所有资源）
helm uninstall myapp
```

## 多环境部署

```bash
# values-dev.yaml
web: { replicas: 1 }
api: { replicas: 1 }
ingress: { host: dev.app.local }

# values-prod.yaml
web: { replicas: 3 }
api: { replicas: 3 }
db:
  storage: { size: "50Gi" }
ingress: { host: app.example.com }

# dev 环境
helm install myapp ./webapp-chart -f values-dev.yaml -n dev --create-namespace

# prod 环境
helm install myapp ./webapp-chart -f values-prod.yaml -n prod --create-namespace

# 密码用 --set 覆盖，不写入 values 文件
helm install myapp ./webapp-chart \
  --set db.credentials.password=$(kubectl get secret prod-db-pass -o jsonpath='{.data.password}' | base64 -d) \
  -n prod
```

## Docker vs K8S 对比

| 维度 | Docker Compose/Stack | Helm Chart |
|------|---------------------|------------|
| 部署方式 | `docker compose up` / `docker stack deploy` | `helm install` |
| 参数化 | 不支持 | `values.yaml` + `--set` |
| 模板化 | 不支持 | Go template `{{ .Values.xxx }}` |
| 版本管理 | 不支持 | `helm history` / `helm rollback` |
| 多环境 | 多 Compose 文件 | 不同 values 文件 + `--set` |
| 一键卸载 | `docker compose down` | `helm uninstall` |

## 🎓 毕业总结

完成 v1→v5 后，你已经完成了从 Docker Compose 到 K8S Helm Chart 的完整迁移路径：

| 版本 | 改造维度 | Docker 锚点 | 核心收获 |
|------|---------|------------|---------|
| v1 | Deployment 基础 | `docker run` | Pod + Deployment + 自愈 |
| v2 | Service 网络 | `-p` + Traefik | Service + Ingress + CoreDNS |
| v3 | 存储持久化 | Volume | PV/PVC + StatefulSet + 稳定标识 |
| v4 | 配置管理 | `-e` | ConfigMap + Secret + envFrom |
| v5 | Helm 打包 | Compose/Stack | Helm Chart + values + 多环境 |

> [!important] 迁移式学习的完整闭环
> 从 Docker Compose 的单文件编排 → K8S 的多资源声明式 → Helm Chart 的模板化打包。
> 这就是从 Docker 用户到 K8S 工程师的迁移路径。

## 相关笔记

- ⬅️ 前置版本：[[v4-ConfigMap与Secret版]]
- ⬅️ 理论基础：[[06-Docker有K8S无与K8S有Docker无]] — Docker 有 K8S 无，K8S 有 Docker 无
- ⬅️ 理论基础：[[01-学习/K8SLearningByDocker/07-业务场景实战合集|07-业务场景实战合集]] — 场景 8 CI/CD + 场景 10 多环境管理
- 🏆 回到总览：[[00-K8S 总览索引（Docker 迁移版）]]
- 🔗 关联：[[05b-Helm 与 Chart 模板]] — 深入学习 Helm Chart 模板

---

*最后更新：2026-07-23*