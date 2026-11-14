---
title: v6-Docker 部署版
created: 2026-07-24
stage: 4
order: 6
difficulty: ⭐⭐⭐⭐
estimated_hours: 3
tags:
  - Go
  - 毕业项目
  - Docker
  - 部署
  - 多阶段构建
  - K8S
description: 毕业项目 v6：Docker 多阶段构建，将 Go 二进制打包为最小化镜像（<15MB），与 K8SLearningByDocker 路径联动，形成 Go→Docker→K8S 完整 DevOps 链路。
lark_doc_url: https://my.feishu.cn/docx/E3smdT1IJo058KxHqw1cHJsinzf
---

## 项目目标

**在 v5 基础上**：用 Docker 多阶段构建打包 Go 二进制，实现从 Python 脚本到容器化部署的完整迁移。

```mermaid
flowchart LR
    A["v5 二进制<br/>go build"] -->|Docker 化| B["v6 容器镜像<br/>多阶段构建"]
    B -.->|联动| C["K8SLearningByDocker<br/>K8S 部署"]
    style B fill:#4CAF50,color:#fff
```

---

## 改造要点

| v5 | v6 | 改造维度 |
|----|----|---------|
| `go build` 本地编译 | Docker 多阶段构建 | 容器化 |
| 手动部署二进制 | `docker run` 一键部署 | 自动化 |
| 无镜像优化 | 多阶段构建最小镜像 | 镜像优化 |
| 无 K8S 衔接 | 镜像可直接用于 K8S | DevOps 链路 |

---

## Dockerfile（多阶段构建）

```dockerfile
# ========== Stage 1: 编译阶段 ==========
FROM golang:1.22-alpine AS builder

WORKDIR /app

# 先拷贝依赖文件（利用 Docker 缓存层）
# 本项目无第三方依赖，只有 go.mod 没有 go.sum。
# 坑：COPY 的通配符匹配不到文件会直接失败（moby#13045），所以不能写 go.sum*；
# 日后真加了依赖时 go.sum 会自动出现，那时再改成 COPY go.mod go.sum ./ 或 COPY go.* ./
COPY go.mod ./
RUN go mod download

# 拷贝源码
COPY . .

# 静态编译（CGO_ENABLED=0 确保纯静态二进制）
# 对比 Python: PyInstaller 无法做到真正的静态二进制
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
    -ldflags="-w -s" \
    -o scheduler \
    ./main.go

# ========== Stage 2: 运行阶段 ==========
FROM alpine:3.19

# 安装 ca-certificates（HTTPS 请求需要）和 tzdata（时区）
RUN apk --no-cache add ca-certificates tzdata

WORKDIR /app

# 从 builder 阶段拷贝编译好的二进制
COPY --from=builder /app/scheduler .

# 创建非 root 用户（安全最佳实践）
RUN adduser -D -u 1001 appuser
USER appuser

# 本程序是批处理 CLI（跑完即退出），不监听端口，无需 EXPOSE

ENTRYPOINT ["./scheduler"]
```

**注释**：
- **多阶段构建**是 Go 的杀手级优势——Stage 1 用 `golang:1.22-alpine`（~350MB）编译，Stage 2 用 `alpine:3.19`（~7MB）运行。最终镜像 ~15MB。
- 对比 Python：即使打包成单一文件，Python 应用仍需基础镜像包含 Python 解释器（~150MB+）。
- `CGO_ENABLED=0` 确保纯静态二进制，可以在 `scratch` 镜像上运行。
- `-ldflags="-w -s"` 去掉调试信息，减小二进制体积约 30%。

---

## .dockerignore

```
.git
.gitignore
*.md
.env
.vscode/
.idea/
```

---

## docker-compose.yml

```yaml
version: '3.8'

services:
  scheduler:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: scheduler
    # 批处理容器：跑完就退出，没有端口可映射，也不需要 restart 策略
    # （用 restart: unless-stopped 会让容器反复重启、重跑同一批任务）
    restart: "no"
    environment:
      - SCHEDULER_MAX_WORKERS=10
      # 不要写死 GOMAXPROCS：Go 1.25+ 会按 cpus 配额（这里 0.5 核）自动设定并动态更新
    deploy:
      resources:
        limits:
          cpus: "0.5"
          memory: 128M
```

---

## Python vs Go Docker 部署对比

| 维度 | Python | Go |
|------|--------|-----|
| 基础镜像大小 | `python:3.12-slim` ~150MB | `golang:1.22-alpine` ~350MB（仅构建） |
| 运行镜像大小 | ~150-200MB | ~8-15MB |
| 多阶段构建 | 可行但仍需 Python 运行时 | 构建阶段用 Go，运行阶段只需二进制 |
| 静态编译 | 不可能 | `CGO_ENABLED=0` 实现纯静态 |
| 启动速度 | 2-5 秒（解释器初始化） | < 100ms（直接执行二进制） |
| 安全攻击面 | 较大（Python + 依赖） | 极小（仅静态二进制） |

---

## 构建和运行

```bash
# 构建镜像
docker build -t scheduler:latest .

# 查看镜像大小（对比 Python 镜像）
docker images scheduler
# REPOSITORY   TAG       SIZE
# scheduler    latest    15.2MB
# 对比：Python Flask 镜像通常 100-200MB

# 运行容器（前台运行才能看到输出；批处理不需要 -d 和 -p）
docker run --rm scheduler:latest

# 查看日志
docker logs scheduler

# 停止和清理
docker stop scheduler && docker rm scheduler
```

---

## K8S 部署（联动 K8SLearningByDocker）

```yaml
# k8s/job.yaml
# ★ 本程序是批处理任务（建 4 个任务→跑完打印统计→退出），不是常驻服务！
# 用 Job 而非 Deployment：无端口、无 /health 探针、无 replicas（跑完即退出，配探针会 CrashLoopBackOff）
apiVersion: batch/v1
kind: Job
metadata:
  name: scheduler
  labels:
    app: scheduler
spec:
  template:
    metadata:
      labels:
        app: scheduler
    spec:
      restartPolicy: Never   # 批处理任务失败不原地重启
      securityContext:
        runAsNonRoot: true   # 与镜像里的 USER appuser（uid 1001）配套
        runAsUser: 1001
      containers:
        - name: scheduler
          image: yourname/scheduler:v6   # 与下面的 docker push 一致，否则 ImagePullBackOff
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true   # 本程序不写盘；以后要写文件就挂 emptyDir
            capabilities:
              drop: ["ALL"]
          resources:
            limits:
              cpu: "500m"
              memory: "128Mi"
            requests:
              cpu: "100m"
              memory: "64Mi"
```

> [!important] CLI → 常驻服务是 v5→v6 缺失的一步
> 上面的 scheduler 跑完就退出，所以用 Job。若要当服务部署（Deployment + 探针 + 多副本），需先给程序加一个最小 `http.Server`（`/health` 返回 200、`/run` 触发一次 RunAll）让它常驻监听，再配 EXPOSE/livenessProbe/replicas——这一步本版本未做，列为进阶练习。

```bash
# 推送到镜像仓库
docker tag scheduler:latest yourname/scheduler:v6
docker push yourname/scheduler:v6

# K8S 部署（K8SLearningByDocker 路径会详细讲解）
kubectl apply -f k8s/job.yaml
kubectl get pods -l app=scheduler
# 等 Job 跑完再看日志：kubectl wait --for=condition=complete job/scheduler
```

> [!tip] DevOps 链路
> 本镜像可直接用于 [[../../K8SLearningByDocker/00-K8S 总览索引（Docker 迁移版）|K8S 学习路径]] 中的 K8S 部署练习，形成 Go→Docker→K8S 完整链路。

---

## 🎯 本版本自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能用 `docker build` 构建多阶段镜像 |
| 🟡 进阶 | 能解释多阶段构建如何减小镜像体积 |
| 🔴 挑战 | 能对比 Python 和 Go 的 Docker 部署差异，部署到 K8S |

---

## 迁移完成总结

```mermaid
flowchart LR
    A["Python 脚本"] --> B["v1 语法迁移"]
    B --> C["v2 struct+method"]
    C --> D["v3 接口+错误"]
    D --> E["v4 并发+channel"]
    E --> F["v5 测试+部署"]
    F --> G["v6 Docker+K8S<br/>生产就绪"]
    style G fill:#4CAF50,color:#fff
```

| 版本 | 改造维度 | Python 概念 → Go 概念 | 预计学时 |
|------|---------|----------------------|---------|
| v1 | 基础语法 | Python 脚本 → Go 语法 | 2h |
| v2 | struct+method | Python class → Go struct | 3h |
| v3 | 接口+错误 | try/except → error + interface | 3h |
| v4 | 并发+channel | asyncio → goroutine | 4h |
| v5 | 测试+部署 | pytest → go test + 交叉编译 | 4h |
| v6 | Docker+K8S | 容器化 → 多阶段构建 + K8S | 3h |

---

## 相关笔记

- ⬅️ 前置：[[v5-测试与部署版]] — 二进制编译基础
- 🔗 关联：[[00-Go 总览索引（Python 迁移版）]] — 路径总览
- 🔗 关联：[[../../K8SLearningByDocker/00-K8S 总览索引（Docker 迁移版）|K8S 学习路径]] — K8S 部署路径联动

---

*最后更新：2026-07-24*
