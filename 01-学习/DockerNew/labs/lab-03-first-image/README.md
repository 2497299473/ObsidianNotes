---
lark_doc_url: https://my.feishu.cn/docx/GQZQdvrKYojLi8xujK4cxNR7n3e
---
# Lab 03：构建你的第一个镜像

> 📖 对应章节：[[03-构建你的第一个镜像]]
> ⏱️ 预计用时：45 分钟
> 🎯 目标：从零写 Dockerfile，构建镜像，运行自己的容器

---

## 前置条件

- 完成 Lab 01
- 本目录已包含 `app.py`、`Dockerfile`、`requirements.txt`、`.dockerignore`

---

## 🚀 步骤

### 1. 构建镜像

```bash
cd lab-03-first-image
docker build -t my-flask-app:v1 .
```

### 2. 运行容器

```bash
docker run -d -p 5000:5000 --name my-flask my-flask-app:v1
```

访问 http://localhost:5000，应看到 "Hello from my Docker container!"

### 3. 修改代码后重新构建（观察缓存）

```bash
# 修改 app.py 中的返回内容，然后：
docker build -t my-flask-app:v2 .
# 注意观察哪些 Step 显示 CACHED
```

### 4. 查看镜像分层

```bash
docker image history my-flask-app:v1
```

### 5. 清理

```bash
docker stop my-flask && docker rm my-flask
docker rmi my-flask-app:v1 my-flask-app:v2
```

---

## ✅ 通关标准

- [ ] 镜像构建成功，浏览器能访问 Flask 应用
- [ ] 修改代码后重新构建，能观察到 CACHED 步骤
- [ ] 能用 `docker image history` 查看分层
- [ ] 能解释 Dockerfile 中 FROM/COPY/RUN/CMD 的作用

---

## 🔗 下一步

→ [[04-镜像分层原理：为什么构建这么快]] | → Lab 07

---

*最后更新：2026-07-28*
