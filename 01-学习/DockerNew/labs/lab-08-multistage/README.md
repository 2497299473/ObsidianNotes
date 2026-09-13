---
lark_doc_url: https://my.feishu.cn/docx/UtCpdHR4qowE6AxDRtzcX1Jjnhx
---
# Lab 08：镜像优化 — 多阶段构建与瘦身

> 📖 对应章节：[[08-镜像优化：多阶段构建与瘦身]]
> ⏱️ 预计用时：45 分钟
> 🎯 目标：亲眼见证同一份 Go 代码，单阶段 ~850MB vs 多阶段 ~15MB 的震撼对比

---

## 前置条件

- 完成 Lab 03（理解 Dockerfile 基础）
- 理解镜像分层原理（[[04-镜像分层原理：为什么构建这么快]]）
- 本目录已包含 `main.go`、`go.mod`、`Dockerfile.single`、`Dockerfile.multi`、`Dockerfile.scratch`、`.dockerignore`

---

## 📋 文件说明

| 文件 | 作用 | 预期镜像大小 |
|------|------|-------------|
| `main.go` | Go HTTP 服务器（`/` + `/health` 端点） | — |
| `go.mod` | Go 模块定义 | — |
| `Dockerfile.single` | ❌ 单阶段构建（含编译器+源码） | ~850MB |
| `Dockerfile.multi` | ✅ 多阶段构建（只含二进制+非root） | ~15MB |
| `Dockerfile.scratch` | 🚀 极致瘦身（0MB 基础镜像） | ~8MB |
| `.dockerignore` | 排除无关文件 | — |

---

## 🚀 快速开始

### 1. 构建单阶段镜像（反面教材）

```bash
cd lab-08-multistage
docker build -t go-single:v1 -f Dockerfile.single .
# 构建后镜像大小：~850MB
```

### 2. 构建多阶段镜像（正确做法）

```bash
docker build -t go-multi:v1 -f Dockerfile.multi .
# 构建后镜像大小：~15MB
```

### 3. 对比镜像大小

```bash
docker images | grep go-
# REPOSITORY  TAG  SIZE
# go-single   v1   ~850MB    ← 包含整个 Go 工具链 + 源码
# go-multi    v1   ~15MB     ← 只有编译产物 + alpine 基础层
```

**差异 98%！** 这就是多阶段构建的威力。

### 4. 运行并验证

```bash
# 两个都能跑，但大小差 98%
docker run -d -p 8080:8080 --name ms-test go-multi:v1

curl http://localhost:8080
# Hello from multi-stage build!

curl http://localhost:8080/health
# {"status":"ok","uptime":"...","version":"1.0.0"}

# 进入容器验证非 root 用户
docker exec ms-test whoami
# app    ← 非 root！

docker exec ms-test cat /etc/os-release
# 看到 alpine
```

### 5. 查看分层差异

```bash
# 单阶段：层数多，包含 Go 工具链
docker image history go-single:v1

# 多阶段：层数少，只保留运行时必需
docker image history go-multi:v1
```

### 6. 清理

```bash
docker rm -f ms-test
docker rmi go-single:v1 go-multi:v1
```

---

## 📊 对比表

| 维度 | 单阶段 | 多阶段 |
|------|--------|--------|
| 镜像大小 | ~850MB | ~15MB |
| 包含 Go 编译器 | 是 | 否 |
| 包含源码 | 是 | 否 |
| 非 root 运行 | 否 | 是 |
| 安全性 | 低 | 高 |
| 拉取/推送速度 | 慢 | 快 |

---

## 🎯 实践任务

### 1. 构建对比

- [ ] 构建单阶段镜像，记录大小
- [ ] 构建多阶段镜像，记录大小
- [ ] 计算瘦身比例（应 > 98%）

### 2. 理解多阶段 Dockerfile

阅读 `Dockerfile.multi`，回答：

- 构建阶段从哪里开始，到哪里结束？
- `COPY --from=builder` 是什么意思？
- `-ldflags="-s -w"` 的作用是什么？
- 为什么最终阶段用 `alpine:3.20` 而不是 `golang:1.22`？

### 3. 验证安全

```bash
# 多阶段镜像使用非 root 用户
docker run --rm go-multi:v1 id
# uid=1000(app) gid=1000(app)  ← 非 root！

# 单阶段镜像默认 root
docker run --rm go-single:v1 id
# uid=0(root) gid=0(root)  ← root！
```

### 4. 测试缓存友好性

```bash
# 修改 main.go 中的返回消息，然后重新构建
docker build -t go-multi:v2 -f Dockerfile.multi .
# 注意：go mod download 层应显示 CACHED
```

---

## 🚀 进阶挑战

### 尝试 scratch 基础镜像（极致瘦身）

```bash
# scratch 是空镜像（0MB），CGO_ENABLED=0 静态编译的 Go 二进制不需要任何基础库
docker build -t go-scratch:v1 -f Dockerfile.scratch .

docker images | grep go-scratch
# go-scratch  v1  ~8MB    ← 只有二进制本身！

docker run -d -p 8081:8080 --name scratch-test go-scratch:v1
curl http://localhost:8081
# Hello from multi-stage build!

# 清理
docker rm -f scratch-test
docker rmi go-scratch:v1
```

### 其他挑战

1. 安装 `dive` 工具，可视化分析镜像分层：`dive go-multi:v1`
2. 尝试 BuildKit 缓存挂载：`RUN --mount=type=cache,target=/go/pkg/mod go build`
3. 用 `trivy image go-multi:v1` 扫描镜像漏洞

---

## ✅ 通关标准

- [ ] 单阶段镜像 ~850MB，多阶段 ~15MB（差异 > 95%）
- [ ] 多阶段镜像能正常运行并返回响应
- [ ] 能用 `docker image history` 对比两者分层
- [ ] 能解释 `COPY --from=builder` 的作用
- [ ] 能解释 `-ldflags="-s -w"` 去掉调试信息
- [ ] 能说明为什么多阶段镜像更安全（非 root + 无编译器）
- [ ] 能为自己的项目写一个多阶段 Dockerfile

---

## 🔗 下一步

→ [[09-生产安全加固]] | → Go+Docker 综合项目

---

*最后更新：2026-07-28*
