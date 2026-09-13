---
lark_doc_url: https://my.feishu.cn/docx/VgKedV7iUof6SsxuPGjc6YxMnIb
---
# Helm Chart — go-todo-api

> 对应 [[01-学习/DockerNew/10-业务场景实战合集]] + [[K8SLearningByDocker/00-K8S 总览索引（Docker 迁移版）]] 的迁移路径

## 依赖更新

```bash
cd helm/go-todo-api
helm dependency update
# 下载 Bitnami postgresql + redis 子 chart
```

## 安装

```bash
# 默认配置（适合测试）
helm install go-todo-api .

# 开发环境（单副本、无持久化）
helm install go-todo-api . -f values-dev.yaml

# 生产环境（三副本、HPA、TLS、ServiceMonitor）
helm install go-todo-api . -f values-prod.yaml
```

## 常用命令

```bash
# 查看 Helm 发布状态
helm status go-todo-api

# 查看 Pod
kubectl get pods -l app.kubernetes.io/name=go-todo-api

# 查看 HPA
kubectl get hpa

# 端口转发测试
kubectl port-forward svc/go-todo-api 8080:8080
curl http://localhost:8080/health

# 升级（修改 values 后）
helm upgrade go-todo-api . -f values-prod.yaml

# 卸载
helm uninstall go-todo-api
```

## 模板清单

| 文件 | 作用 | 对应 Docker Compose |
|------|------|---------------------|
| `deployment.yaml` | Deployment（非 root、只读、健康检查、资源限制） | `compose.yaml` + `compose.prod.yaml` |
| `service.yaml` | ClusterIP Service | Docker bridge 网络 |
| `hpa.yaml` | HorizontalPodAutoscaler（CPU + 内存双指标） | Docker 无对应（K8S 独有） |
| `ingress.yaml` | Ingress（Nginx 入口 + TLS + 限流注解） | `nginx/nginx.conf` |
| `configmap.yaml` | 非敏感环境变量 | `environment:` 字段 |
| `secret.yaml` | DB DSN / Redis URL | `environment:` 中的密码 |
| `serviceaccount.yaml` | ServiceAccount | Docker 无对应 |
| `servicemonitor.yaml` | Prometheus Operator 自动发现 | `prometheus.yml` 抓取配置 |

## Docker Compose → K8S 对照表

| Docker Compose | Kubernetes | 说明 |
|---------------|-----------|------|
| `compose.yaml` | `values.yaml` + `templates/` | 基础配置 |
| `compose.dev.yaml` | `values-dev.yaml` | 开发覆盖 |
| `compose.prod.yaml` | `values-prod.yaml` | 生产覆盖 |
| Nginx 反向代理 | Ingress + annotations | 端口收敛 → 域名入口 |
| Docker healthcheck | livenessProbe + readinessProbe | 健康检查探针 |
| `--memory` / `--cpus` | `resources.limits` | 资源限制 |
| `--read-only` | `readOnlyRootFilesystem` | 只读文件系统 |
| `--user 1000:1000` | `runAsUser` / `runAsGroup` | 非 root |
| `cap_drop: ALL` | `capabilities.drop: [ALL]` | capabilities 最小化 |
| 无 | HPA | K8S 独有的自动扩缩容 |
| Volume | PersistentVolumeClaim | 子 chart 管理 |
| `depends_on` | `initContainer` 或 readinessProbe | 依赖管理 |
