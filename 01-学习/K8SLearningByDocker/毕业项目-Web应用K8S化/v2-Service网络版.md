---
title: v2-Service网络版
created: 2026-07-23
project: 毕业项目-Web应用K8S化
version: 2
difficulty: ⭐⭐⭐
estimated_hours: 3
tags:
  - 毕业项目
  - K8S
  - Docker
  - Service
  - Ingress
description: 毕业项目 v2：在 v1 基础上添加 Service 网络层和 Ingress 路由。从 Docker 端口映射和 Traefik 反向代理迁移到 K8S Service + Ingress。
lark_doc_url: https://my.feishu.cn/docx/S8CsdMWLfo1GoBxaBh9ceeomnqf
---

## 版本目标

在 v1（4 个 Deployment）基础上，**只改一个维度：添加网络层**，让应用在集群内可互访、集群外可访问。

## v1 → v2 改造

| 组件 | v1 | v2 | 说明 |
|------|----|-----|------|
| web | Deployment | Deployment + Service | 添加 ClusterIP Service |
| api | Deployment | Deployment + Service | 添加 ClusterIP Service |
| db | Deployment | Deployment + Service | 添加 Headless Service |
| cache | Deployment | Deployment + Service | 添加 ClusterIP Service |
| 外部访问 | 无 | Ingress | 基于域名路由 |

## Docker Compose 网络（回顾）

```yaml
# Docker Compose 自动创建网络，服务名 = DNS 名
services:
  web:
    ports: ["80:80"]          # 对外暴露 80 端口
  api:
    # 内部通过 api:5000 访问
  db:
    # 内部通过 db:5432 访问
```

## K8S YAML（新增 Service 和 Ingress）

```yaml
# v2-services-ingress.yaml

# ===== web Service (ClusterIP) =====
apiVersion: v1
kind: Service
metadata: { name: web-svc, labels: { app: web } }
spec:
  type: ClusterIP
  selector: { app: web }
  ports: [{ name: http, port: 80, targetPort: 80 }]
---
# ===== api Service (ClusterIP) =====
apiVersion: v1
kind: Service
metadata: { name: api-svc, labels: { app: api } }
spec:
  type: ClusterIP
  selector: { app: api }
  ports: [{ name: http, port: 5000, targetPort: 5000 }]
---
# ===== db Service (Headless，为 v3 StatefulSet 准备) =====
apiVersion: v1
kind: Service
metadata: { name: db-svc, labels: { app: db } }
spec:
  clusterIP: None           # Headless Service
  selector: { app: db }
  ports: [{ name: postgres, port: 5432, targetPort: 5432 }]
---
# ===== cache Service (ClusterIP) =====
apiVersion: v1
kind: Service
metadata: { name: cache-svc, labels: { app: cache } }
spec:
  type: ClusterIP
  selector: { app: cache }
  ports: [{ name: redis, port: 6379, targetPort: 6379 }]
---
# ===== Ingress (HTTP 路由，对标 Traefik；不设 rewrite-target，URI 直通) =====
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: app-ingress
spec:
  ingressClassName: nginx
  rules:
  - host: app.local
    http:
      paths:
      - path: /
        pathType: Prefix
        backend: { service: { name: web-svc, port: { number: 80 } } }
      - path: /api             # Prefix 直通，/api/* 原样转发到 api-svc
        pathType: Prefix
        backend: { service: { name: api-svc, port: { number: 5000 } } }
```

## 更新 api Deployment 的环境变量

```yaml
# 将硬编码的连接地址改为 Service DNS 名
env:
- name: DATABASE_URL
  value: "postgres://user:pass@db-svc:5432/app"     # ★ 用 Service 名
- name: REDIS_URL
  value: "redis://cache-svc:6379"                     # ★ 用 Service 名
```

## 验证步骤

```bash
# 1. 部署 v1 Deployment（如果还没部署）
kubectl apply -f v1-all-deployments.yaml

# 2. 部署 Service 和 Ingress
kubectl apply -f v2-services-ingress.yaml

# 3. 验证 Service
kubectl get svc
# NAME         TYPE        CLUSTER-IP      PORT(S)
# api-svc      ClusterIP   10.96.100.2     5000/TCP
# cache-svc    ClusterIP   10.96.100.3     6379/TCP
# db-svc       ClusterIP   None            5432/TCP
# web-svc      ClusterIP   10.96.100.1     80/TCP

# 4. 验证 CoreDNS 服务发现
kubectl run test --rm -it --image=alpine -- sh
# 在 Pod 内
nslookup web-svc        # 应解析到 ClusterIP
nslookup api-svc        # 应解析到 ClusterIP
nslookup db-svc         # 应解析到 Pod IP（Headless）
wget -qO- http://web-svc  # 应返回 Nginx 欢迎页
exit

# 5. 启用 Ingress 并访问
minikube addons enable ingress
echo "$(minikube ip) app.local" | sudo tee -a /etc/hosts
curl http://app.local       # → web 服务
curl http://app.local/api   # → api 服务

# 6. 验证服务间通信
kubectl exec -it deployment/api -- sh -c "nc -z db-svc 5432 && echo 'DB OK'"
kubectl exec -it deployment/api -- sh -c "nc -z cache-svc 6379 && echo 'Redis OK'"

# 7. 清理
kubectl delete -f v2-services-ingress.yaml
kubectl delete -f v1-all-deployments.yaml
```

## Docker vs K8S 对比

| 功能 | Docker Compose | K8S v2 | 差异 |
|------|---------------|--------|------|
| 服务发现 | 服务名 = DNS | Service 名 + CoreDNS FQDN | K8S 支持完整 FQDN 和跨命名空间解析 |
| 端口映射 | `ports: ["80:80"]` | Service + Ingress | K8S 将端口映射和路由分离 |
| 路由规则 | Traefik 标签 | Ingress YAML | K8S 声明式路由 |
| 负载均衡 | Docker 内置 | Service 自动分发（iptables/ipvs） | K8S 更灵活 |

## 🎯 本版自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 4 个 Service 全部创建，CoreDNS 解析正常，服务间可通信 |
| 🟡 进阶 | Ingress 路由生效，`http://app.local` → web，`/api` → api |
| 🔴 挑战 | 解释为什么 db 用 Headless Service（clusterIP: None），而 web/api 用 ClusterIP |

## 下一版本

→ [[v3-存储持久化版]] — 用 PVC 持久化 PostgreSQL 数据，db 改为 StatefulSet

---

*最后更新：2026-07-23*