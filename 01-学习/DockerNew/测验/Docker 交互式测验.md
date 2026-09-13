---
title: Docker 交互式测验
created: 2026-07-28
tags:
  - Docker
  - 测验
  - flashcards
  - 自检
description: Docker 学习路径全 11 章交互式测验，适配 Obsidian Spaced Repetition 插件。每章 5-8 题，以单行问答卡为主。
lark_doc_url: https://my.feishu.cn/docx/TtHAdsXeBoXdK6xsua7cCW2xnCb
---

# Docker 交互式测验

> [!tip] 使用方法
> 1. 安装 Obsidian **Spaced Repetition** 插件（社区插件搜索 `spaced repetition`）
> 2. 插件会自动识别带有 `#flashcards` 标签的笔记
> 3. 卡片格式：单行问答卡 `问题::答案`；单行双向卡 `A:::B`（`:::` 是双向分隔符，正反两个方向都会出题）；多行卡 = 问题若干行 + 单独一行 `?` + 答案若干行
> 4. 点击左侧栏的复习按钮即可开始交互式测验
> 5. 也可用 Obsidian **Quiz** 插件，格式兼容

## 第 1 章：初识 Docker

#flashcards

Docker 的三个核心概念是什么？::镜像（Image）、容器（Container）、仓库（Registry）

容器和虚拟机的本质区别是什么？::容器是隔离的进程（共享宿主内核），虚拟机是完整的操作系统（独立内核）

`docker run -d -p 8080:80 --name my-nginx nginx:alpine` 中 `-p 8080:80` 的含义是什么？::宿主机 8080 端口映射到容器 80 端口（方向：从外向内）

`docker run` 和 `docker start` 的区别？::run = 创建新容器 + 启动；start = 重新启动已存在的容器（保留数据）

`--rm` 参数的作用？::容器退出后自动删除，不留垃圾

为什么不应该用 `:latest` 标签？::latest 是可变标签，今天拉的和明天可能不同，导致环境不一致

## 第 2 章：容器管理

#flashcards

`docker stop` 和 `docker kill` 的区别？::stop 发 SIGTERM 优雅终止（10 秒超时后 SIGKILL）；kill 直接发 SIGKILL，不给清理机会

容器为什么启动后立即退出？::容器生命周期绑定在 PID 1 主进程上，主进程退出则容器退出

`docker exec -it` 中 `-it` 的作用？::`-i` 保持 STDIN 打开（能输入），`-t` 分配伪终端（有提示符），合起来才能交互式操作

生产环境推荐的重启策略？::`--restart=unless-stopped`（除非手动停止，否则总重启；Docker 重启时不会自动拉起手动停止的容器）

`docker logs -f` 的作用？::实时跟踪容器日志（类似 `tail -f`）

## 第 3 章：构建镜像

#flashcards

Dockerfile 的 5 个核心指令是什么？::FROM（基础镜像）、COPY（复制文件）、RUN（构建时执行）、WORKDIR（工作目录）、CMD（启动命令）

为什么 `CMD` 推荐用 exec 格式 `CMD ["python", "app.py"]` 而非 shell 格式？::exec 格式让 PID 1 是应用本身，能接收 SIGTERM 优雅停止；shell 格式 PID 1 是 /bin/sh，不转发信号

缓存友好的 Dockerfile 顺序是什么？::先 COPY requirements.txt → RUN pip install → 最后 COPY . .（最常变的放最后）

`docker build -t myapp .` 中最后的 `.` 代表什么？::构建上下文（当前目录），Docker 会把整个目录发送给引擎

`COPY` 和 `ADD` 的区别？推荐用哪个？::COPY 只复制文件（行为明确）；ADD 额外支持 URL 下载和自动解压 tar（容易出意外）。推荐用 COPY

## 第 4 章：镜像分层原理

#flashcards

Docker 镜像为什么是分层的？::每条 Dockerfile 指令生成一个只读层，叠加成最终镜像。分层带来三个好处：共享基础层、增量构建、写时复制

什么是写时复制（Copy-on-Write）？::容器修改文件时，从只读镜像层复制到可读写容器层，不影响原始镜像。删除容器 = 清除所有修改

构建缓存的黄金法则是什么？::某层变化后，该层及后续所有层缓存全部失效。所以把最常变的放最后，最不常变的放最前

`.dockerignore` 的三大作用？::① 减小构建上下文（不传 node_modules 等）② 防止缓存失效（不传 .git）③ 防止密钥泄露（不传 .env）

如何查看镜像的分层结构？::`docker image history IMAGE` 或安装 `dive` 工具可视化分析

## 第 5 章：容器隔离原理

#flashcards

Namespace 和 Cgroups 各自解决什么问题？::Namespace 隔离「看到什么」（进程 ID、网络、文件系统等）；Cgroups 限制「能用多少」（CPU、内存、IO 等）

容器内 PID 1 和宿主机上看到的 PID 有什么关系？::完全不同。容器内 PID 1 是它自己的启动进程，宿主机上看到的是一个大数字 PID。这就是 PID Namespace 的隔离效果

`Exited (137)` 表示什么？::137 = 128 + 9（SIGKILL），通常是被 OOM Killer 杀死的（容器内存超过 --memory 限制）

默认情况下容器内 root 用户等于宿主机 root 吗？::是的，默认 UID 0 就是宿主机 UID 0。这是容器安全的最大隐患之一（User Namespace 默认未启用）

`--memory=256m` 是预留还是上限？::是上限（最多用 256MB），不是预留。多个容器可以超卖（总和大于宿主内存），但触发 OOM 时内核会杀进程

## 第 6 章：网络与存储

#flashcards

默认 bridge 和自定义 bridge 的关键区别？::自定义 bridge 支持容器名 DNS 互访（`ping web`），默认 bridge 不支持

Volume 和 Bind Mount 的区别？适用场景？::Volume 由 Docker 管理（独立于容器生命周期），适合生产数据（数据库）；Bind Mount 直接挂载宿主目录，适合开发环境代码热更新

`docker compose down` 会删除 Volume 吗？::默认不会！需要加 `-v` 才删除 Volume（⚠️ 数据丢失）

如何备份 Volume 数据？::`docker run --rm -v db-data:/data:ro -v $(pwd):/backup alpine tar cvf /backup/bak.tar -C /data .`

开发用 ___，生产用 ___。::开发用 Bind Mount，生产用 Volume

## 第 7 章：Docker Compose

#flashcards

`depends_on` 能保证服务就绪吗？::默认不能！只等容器启动。必须配 `condition: service_healthy` + healthcheck 才能真正等待服务可用

`.env` 和 `env_file` 的区别？::`.env` 文件做变量插值（构建时替换 `${VAR}`）；`env_file` 把文件内容作为容器环境变量注入（运行时）

如何实现多环境配置？::基础 compose.yaml + 覆盖文件（compose.dev.yaml / compose.prod.yaml），用 `-f compose.yaml -f compose.dev.yaml up -d`，后面的覆盖前面的

Compose Watch 的三种 action？::sync（只同步文件）、rebuild（重建镜像并重启）、sync+restart（同步文件并重启服务）

Compose 适合生产环境吗？::适合开发、测试、CI。生产需要 Swarm 或 K8S 提供自愈、扩缩容能力

## 第 8 章：镜像优化

#flashcards

多阶段构建的核心思想？::构建阶段可以很大（含编译器、源码），最终镜像只保留运行必需的二进制产物。通过 `COPY --from=builder` 只复制最终产物

如何把 Go 镜像从 1GB 减到 15MB？::多阶段构建 + alpine 基础镜像 + `-ldflags="-s -w"` 去掉调试信息

基础镜像大小排序？::scratch(0MB) < alpine(7MB) < distroless(20MB) < slim(150MB) < full(1GB)

alpine 的潜在坑是什么？::alpine 使用 musl libc 而不是 glibc，某些 Python C 扩展（如 numpy、Pillow）可能编译失败。遇到问题换 slim

BuildKit `--mount=type=cache` 的作用？::跨构建复用缓存（如 pip/npm 下载的包），第一次构建下载的包第二次构建直接用缓存

## 第 9 章：生产安全加固

#flashcards

容器安全的三大原则？::最小权限、最小攻击面、纵深防御

为什么永远不要用 `--privileged`？::等于给容器几乎所有内核权限，容器逃逸 = 宿主机 root。用 `--cap-add` 按需加 capabilities

如何在构建时安全使用密钥（不进镜像层）？::BuildKit `--mount=type=secret`，密钥只在构建时存在，不进镜像层，`docker history` 不可见

`--read-only` 的作用和如何处理需要写入的目录？::让容器根文件系统只读，攻击者无法篡改。需要写入的目录用 `--tmpfs /tmp:size=64m` 单独开放

如何在 CI 中用 Trivy 做安全门禁？::`trivy image --exit-code 1 --severity CRITICAL myapp:v1`，发现致命漏洞时返回非 0 退出码，CI 流水线失败

## 第 10 章：业务场景实战

#flashcards

开发环境和生产环境的 Docker 配置差异？::开发：build + Bind Mount + Watch + DEBUG=1 + 暴露所有端口；生产：image + Volume + 资源限制 + 非root + 只读 + 安全加固

Volume 和备份的关系？::Volume 不是备份！Volume 只是 Docker 管理的持久化存储。必须定期用 `docker run --rm -v vol:/data alpine tar cvf` 做离线备份

静态站点最优 Docker 方案？::多阶段构建：Node 阶段 `npm run build` → Nginx 阶段只 COPY dist，最终镜像 ~20MB

多架构构建的命令？::`docker buildx build --platform linux/amd64,linux/arm64 -t myrepo/app:v1 --push .`

容器日志收集的推荐方案？::Fluent Bit（收集）+ Loki（存储）+ Grafana（查询），轻量级替代 ELK

## 第 11 章：面试高频 20 问

#flashcards

面试回答的黄金三段式？::先结论 → 再展开 → 再补充生产经验

Docker 比虚拟机轻量的原因？::① 共享内核不需要 Guest OS ② 进程级隔离（Namespace）开销小 ③ 镜像分层（UnionFS）可复用 ④ 秒级启动 vs 分钟级

镜像为什么分层？三个好处？::① 共享基础层（10 个容器用同一 alpine 只存一份）② 增量构建（修改只重建变化层）③ 写时复制（容器修改不影响镜像）

`CMD` 和 `ENTRYPOINT` 的区别？::ENTRYPOINT 是固定入口（不容易被覆盖），CMD 是默认参数（docker run 参数可覆盖）。两者配合：CMD 作为 ENTRYPOINT 的参数

Swarm 和 K8S 怎么选？::节点 < 20 选 Swarm（简单、Docker 原生）；节点 > 50 选 K8S（生态丰富、自动扩缩）。如果最终要上 K8S，直接学 K8S 别在 Swarm 上浪费时间

设计一个生产级 Docker 部署方案需要考虑哪几方面？::① 镜像（多阶段+扫描）② 构建（CI/CD+版本化tag）③ 运行（资源限制+非root+只读+健康检查）④ 编排（Swarm/K8S）⑤ 网络（自定义bridge/overlay）⑥ 存储（Volume+定期备份）⑦ 监控（日志收集+告警）

---

## 📊 测验使用指南

### 安装 Spaced Repetition 插件

1. Obsidian 设置 → 第三方插件 → 关闭安全模式
2. 社区插件 → 搜索 `Spaced Repetition` → 安装并启用
3. 插件设置中确认 Flashcard tag 为 `flashcards`
4. 打开本笔记，左侧栏会出现复习图标

### 卡片格式说明

| 格式 | 用途 | 示例 |
|------|------|------|
| `问题::答案` | 单行问答卡 | `Namespace 和 Cgroups 的区别？::前者隔离视图，后者限制资源` |
| `A:::B` | 单行双向卡（`:::` 为双向分隔符，正反两向出题） | `镜像:::Image` |
| 问题若干行 + 单独一行 `?` + 答案若干行 | 多行卡 | 适合答案较长的情况 |

### 推荐复习节奏

| 时间 | 动作 |
|------|------|
| 学完每章后 | 完成该章的卡片，标记知道/不知道 |
| 第 2 天 | 复习标记为「不知道」的卡片 |
| 第 4 天 | 复习全部卡片 |
| 第 7 天 | 模拟面试，用黄金三段式口述每题答案 |

---

## 🔗 相关笔记

- 📖 [[00-Docker 总览索引]] — 回到课程总览
- ⬅️ 各章节自检清单 — 每章结尾的 🟢🟡🔴 三级自检

---

*最后更新：2026-07-28*
