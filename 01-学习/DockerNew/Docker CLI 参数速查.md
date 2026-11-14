---
title: Docker CLI 参数速查
created: 2026-10-27
tags:
  - Docker
  - CLI
  - 命令速查
  - 容器化
description: "Docker CLI 参数速查：单短横线与双短横线规则、-d/--rm 等高频参数、docker run/ps/exec/logs/build 常用选项与短选项对照。"
---

> [!info] 这篇笔记怎么用
> 这是一篇**速查手册**，不是课程章节，不必从头读到尾。遇到不认识的参数回来查即可；命令的系统讲解见 [[02-容器管理：查看、停止、调试与端口映射]]。

## 1. 参数写法：`-` 与 `--` 的区别

### 1.1 一张表看懂

| 形式 | 名称 | 规则 | 示例 |
|------|------|------|------|
| `-x` | 短选项（short option） | 单个字母；可以合并；值可紧贴或空格 | `-d`、`-p 8080:80`、`-m256m` |
| `--word` | 长选项（long option） | 完整单词；不能合并；赋值用 `=` 或空格 | `--detach`、`--rm`、`--memory=256m` |

**来历**：短选项源自早期 Unix 的 `getopt`（只支持单字母），长选项由 GNU 引入（`getopt_long`），目的是让脚本可读。Docker 用 Go 的 `pflag` 库，遵循 GNU 风格——所以 `-d` 和 `--detach` 是同一个参数的两个名字，短的是高频别名，长的是自解释全名。

### 1.2 四条书写规则

1. **短选项可以合并**：`-it` = `-i -t`，`-dit` = `-d -i -t`。但只能合并短选项，不能把长选项拼起来。
2. **赋值有两种写法**：`-m 256m` / `-m256m`；`--memory 256m` / `--memory=256m`。推荐空格写法，最不容易看错。
3. **大小写敏感**：`-p` 是 `--publish`（映射端口），`-P` 是 `--publish-all`（发布所有端口），完全不同。
4. **选项必须在镜像名前**：`docker run [OPTIONS] IMAGE [COMMAND] [ARG...]`。镜像名之后的内容全部传给容器内命令，不再归 Docker 解析。

### 1.3 单独一个 `--`

`--` 单独出现表示「选项结束」，后面的内容即使以 `-` 开头也当普通参数处理。Docker 日常几乎用不到，因为镜像名天然就是分界线。

## 2. 两个高频参数：`-d` 与 `--rm`

| 参数 | 全名 | 作用 |
|------|------|------|
| `-d` | `--detach` | **后台运行**。容器启动后立刻把终端还给你，日志不再输出到当前终端 |
| `--rm` | 无短选项 | **容器退出后自动删除**容器本身（连同匿名数据卷），`docker ps -a` 不留 `Exited` 记录 |

```bash
# Detached: terminal returns immediately
docker run -d --name web -p 8080:80 nginx:alpine

# One-off: container is removed on exit
docker run --rm alpine ps aux

# Common combo for a long-running interactive container
docker run -dit --name shell-box alpine sh
```

几个细节：

- 不加 `-d` 就是**前台运行**，终端被占住，Ctrl+C 会把容器停掉。
- `--rm` 只删**匿名卷**；`-v mydata:/data` 这类命名卷会保留。
- `--rm` 和 `--restart` **不能同时用**（一个要退出即删，一个要退出即拉起），Docker 会报 `Conflicting options`。
- 带 `--rm` 的容器，`docker stop` 之后就已经被删除了，**不需要也不能再 `docker rm`**。
- `-dit` 常用于需要保持 STDIN 的常驻容器；`-d` 和 `-it` 并不互斥。

## 3. `docker run` 常用参数

| 类别 | 参数 | 说明 |
|------|------|------|
| 运行模式 | `-d, --detach` | 后台运行 |
| | `-i, --interactive` | 保持 STDIN 打开 |
| | `-t, --tty` | 分配伪终端；与 `-i` 合用即 `-it` |
| | `--rm` | 退出后自动删除容器 |
| | `--name` | 指定容器名，便于引用 |
| | `--restart` | 重启策略：`no`（默认）、`on-failure`、`always`、`unless-stopped` |
| | `--init` | 用 tini 当 PID 1，回收僵尸进程 |
| 网络 | `-p, --publish 宿主机:容器` | 端口映射，如 `-p 8080:80` |
| | `-P, --publish-all` | 把所有 EXPOSE 端口随机映射到宿主机 |
| | `--network` | 网络模式：`bridge`（默认）、`host`、`none` 或自定义网络 |
| | `--hostname` | 容器主机名（短选项是 `-h`，注意不是 help） |
| | `--dns` | 自定义 DNS 服务器 |
| | `--add-host` | 追加 `/etc/hosts` 记录 |
| 环境与配置 | `-e, --env KEY=VALUE` | 注入环境变量，可重复多次 |
| | `--env-file .env` | 从文件批量注入环境变量 |
| | `-l, --label` | 打标签，便于 `ps --filter label=...` 过滤 |
| | `-w, --workdir` | 容器内工作目录 |
| | `--entrypoint` | 覆盖镜像的 ENTRYPOINT |
| 存储 | `-v, --volume 宿主机:容器` | Bind Mount 或命名卷 |
| | `--mount` | 更明确的挂载写法，生产推荐 |
| | `--tmpfs` | 挂载内存文件系统 |
| | `--read-only` | 根文件系统只读 |
| 资源限制 | `--cpus` | CPU 核数上限，如 `--cpus="1.5"` |
| | `-m, --memory` | 内存上限，如 `-m 256m` |
| | `--memory-swap` | 内存 + swap 总上限；设为与 `--memory` 相同即禁用 swap |
| | `--pids-limit` | 进程数上限，防 fork 炸弹 |
| | `--cpuset-cpus` | 绑定到指定 CPU 核，如 `0-1` |
| | `--shm-size` | `/dev/shm` 大小 |
| 权限与安全 | `-u, --user` | 指定 UID:GID |
| | `--cap-add` / `--cap-drop` | 增删 Linux capabilities |
| | `--privileged` | 特权模式，几乎不隔离，慎用 |
| | `--security-opt` | seccomp / AppArmor 等安全配置 |
| 其他 | `--platform` | 指定架构，如 `linux/amd64` |
| | `--pull` | 镜像拉取策略：`always`、`missing`、`never` |
| | `--log-driver` / `--log-opt` | 日志驱动与选项 |
| | `--device` | 直通宿主机设备 |

## 4. 其他常用子命令

| 命令 | 常用参数 | 用途 |
|------|---------|------|
| `docker ps` | `-a`（含已停止）、`-q`（只出 ID）、`-l`（最近一个）、`-n 5`、`-f/--filter`、`--format`、`--no-trunc` | 查看容器 |
| `docker exec` | `-it`、`-d`、`-u`、`-w`、`-e` | 进入容器或执行命令 |
| `docker logs` | `-f`（持续跟随）、`--tail 100`、`-t`（时间戳）、`--since`、`--until` | 查看日志 |
| `docker stop` | `-t 5`（等 5 秒再 SIGKILL，默认 10 秒） | 停止容器 |
| `docker rm` | `-f`（强制删运行中的）、`-v`（连带匿名卷） | 删除容器 |
| `docker images` | `-a`、`-q`、`-f`、`--format`、`--no-trunc` | 查看镜像 |
| `docker build` | `-t`（标签）、`-f`（Dockerfile 路径）、`--no-cache`、`--build-arg`、`--target`、`--platform`、`--progress` | 构建镜像 |
| `docker inspect` | `-f/--format`、`-s`（含大小）、`--type` | 查看完整配置 |
| `docker stats` | `--no-stream`（只取一次）、`-a`、`--format` | 查看资源占用 |
| `docker top` | 无常用选项 | 从宿主机视角看容器内进程 |
| `docker system` | `df`、`prune -a -f`、`prune --volumes` | 磁盘占用与清理 |

## 5. 同一个字母，含义随命令变化

短选项的含义是**按命令定义**的，不是全局统一。最容易踩坑的几个：

| 短选项 | 在这个命令里 | 在另一个命令里 |
|--------|------------|--------------|
| `-a` | `docker ps -a` = `--all` | `docker run -a` = `--attach` |
| `-f` | `docker ps -f` = `--filter` | `docker rm -f` = `--force`；`docker logs -f` = `--follow`；`docker build -f` = `--file` |
| `-l` | `docker ps -l` = `--latest` | `docker run -l` = `--label` |
| `-t` | `docker run -t` = `--tty` | `docker stop -t` = `--time`；`docker build -t` = `--tag` |
| `-v` | `docker run -v` = `--volume` | `docker -v` = `--version`；`docker rm -v` = `--volumes` |
| `-h` | `docker run -h` = `--hostname` | `docker -h` / `docker ps -h` = 帮助；看 run 的帮助要用 `--help` |
| `-i` | `docker run -i`、`docker exec -i` = `--interactive` | `docker build` 无此短选项 |
| `-q` | `docker ps -q`、`docker images -q` = `--quiet` | — |
| `-u` | `docker run -u`、`docker exec -u` = `--user` | — |
| `-w` | `docker run -w`、`docker exec -w` = `--workdir` | — |

> [!tip] 判断方法
> 短选项永远只对**当前子命令**生效。拿不准时直接看 `docker <子命令> --help`，帮助里会并排显示 `-d, --detach` 这样的对应关系。

## 6. 常见易错点

> [!warning] 坑 1：`--rm` 和 `--restart` 同时用
> Docker 会报 `Conflicting options: --restart and --rm`。临时容器用 `--rm`，常驻服务用 `--restart`，二选一。

> [!warning] 坑 2：选项写在镜像名后面
> `docker run alpine -d` 里的 `-d` 是传给容器内命令的，不是 Docker 的参数。正确写法是 `docker run -d alpine`。同理，`docker run alpine echo -n hi` 的 `-n` 属于 `echo`。

> [!warning] 坑 3：以为 `docker run -h` 是帮助
> `-h` 在 `docker run` 里是 `--hostname`，会设置容器主机名。查看帮助请用 `docker run --help`。

> [!warning] 坑 4：把长选项拼成一个横线
> `-it` 可以（两个短选项合并），`-rm` 不行——它会被拆成 `-r -m` 逐个解析并报 `unknown shorthand flag`。长选项必须各写各的：`--rm --name`。

> [!warning] 坑 5：以为 `--rm` 会清理所有数据
> `--rm` 只删容器和匿名卷。命名卷（`-v mydata:/data`）以及宿主机目录（Bind Mount）都不会被删，需要手动清理。

## 7. 怎么查参数

```bash
# Authoritative help: shows short and long options side by side
docker run --help
docker ps --help

# Search a specific option
docker run --help | grep -i memory
```

```powershell
# Windows PowerShell equivalent
docker run --help | Select-String memory
```

> [!tip] 优先看 `--help`
> 网上的教程可能对应旧版本。`docker <子命令> --help` 永远和本机版本一致，是最可靠的参数来源。

## 🎯 三个小练习（建议全做，约 10 分钟）

```bash
# 1. Detached vs foreground
docker run -d --name cli-test -p 8080:80 nginx:alpine
docker ps
docker logs cli-test
docker stop cli-test && docker rm cli-test

# 2. --rm removes the container on exit
docker run --rm alpine echo "one-off"
docker ps -a --filter ancestor=alpine   # no output

# 3. Combined short options
docker run --rm -it alpine sh
# Inside the container: run `hostname`, then `exit`
```

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 不看笔记说出 `-d`=`--detach`、`-p`=`--publish`、`--rm` 没有短选项；跑 `docker run -d --name web -p 8080:80 nginx:alpine`，再用 `docker ps --format "table {{.Names}}\t{{.Ports}}"` 看到 `0.0.0.0:8080->80/tcp` |
| 🟡 进阶 | 现场验证"选项必须在镜像名前"：`docker run --rm alpine echo -n hi` 输出 `hi` 且不换行（说明 `-n` 归 echo），并能解释 `docker run alpine -d` 里的 `-d` 去了哪里；说出 `docker stop -t 5` 与 `docker run -t` 中两个 `-t` 的全名分别是 `--time` 和 `--tty` |
| 🔴 挑战 | 任取一条不熟的命令（`docker stats` / `docker system` / `docker inspect`），60 秒内用 `docker <cmd> --help` 找出 3 个常用选项并写出一条能跑的命令；再判断 `--rm` 与 `--restart=always` 能否同时用，并说出报错原因（`Conflicting options`） |

---

## 🔗 相关笔记

- [[00-Docker 总览索引]] — 回到课程总览
- [[02-容器管理：查看、停止、调试与端口映射]] — 命令的系统讲解
- [[05-容器隔离原理：Namespace 与 Cgroups]] — `--cpus`、`--memory` 背后的原理
- [[09-生产安全加固]] — `-u`、`--cap-drop`、`--read-only` 的实战用法
- [[11-面试高频 20 问]] — 命令与原理的面试问法

---

*最后更新：2026-10-27*
