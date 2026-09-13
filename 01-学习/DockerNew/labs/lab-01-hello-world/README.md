---
lark_doc_url: https://my.feishu.cn/docx/Bzr5dXsL3oNNapxYbwVcybkFnNe
---
# Lab 01：初识 Docker — 从安装到运行你的第一个容器

> 📖 对应章节：[[01-初识 Docker：从安装到运行你的第一个容器]]
> ⏱️ 预计用时：30 分钟
> 🎯 目标：亲手跑起来 Docker，感受容器的秒级启动

---

## 前置条件

- 已安装 Docker Desktop（Windows/Mac）或 Docker Engine（Linux）
- 终端可运行 `docker --version`

---

## 🚀 步骤

### 1. 验证安装

```bash
docker --version
docker compose version
docker run --rm hello-world
```

✅ 看到 "Hello from Docker!" 即安装成功。

### 2. 运行你的第一个实用容器 — Nginx

```bash
docker run -d -p 8080:80 --name my-nginx nginx:alpine
```

浏览器打开 http://localhost:8080，应看到 Nginx 欢迎页。

### 3. 感受秒级启动

```bash
# Windows PowerShell
Measure-Command { docker run --rm alpine echo "hello" }

# Linux / macOS
time docker run --rm alpine echo "hello"
```

对比：启动虚拟机通常需要 30 秒到 2 分钟。

### 4. 进入交互式容器

```bash
docker run -it --rm alpine sh
# 在容器内执行：
#   ls /               # 查看文件系统
#   cat /etc/os-release # 查看操作系统
#   hostname           # 容器内的主机名（随机 ID）
#   exit               # 退出
```

### 5. 同时运行两个容器（验证隔离）

```bash
# 先清理步骤 2 中占用 8080 端口的 my-nginx，否则下面 nginx1 映射同端口会冲突
docker rm -f my-nginx

docker run -d -p 8080:80 --name nginx1 nginx:alpine
docker run -d -p 8081:80 --name nginx2 nginx:alpine

# 修改 nginx2 的首页
docker exec nginx2 sh -c "echo '<h1>Nginx 2</h1>' > /usr/share/nginx/html/index.html"

# 分别访问 http://localhost:8080 和 http://localhost:8081
# 验证两个容器完全隔离！
```

### 6. 清理

```bash
docker stop nginx1 nginx2 && docker rm nginx1 nginx2
```

---

## ✅ 通关标准

- [ ] `docker run hello-world` 输出欢迎信息
- [ ] 浏览器能访问 Nginx 欢迎页
- [ ] 容器启动时间 < 1 秒
- [ ] 两个 Nginx 容器互不影响（隔离验证）
- [ ] 能解释镜像、容器、仓库三者的关系

---

## 🔗 下一步

→ [[02-容器管理：查看、停止、调试与端口映射]] | → Lab 03

---

*最后更新：2026-07-28*
