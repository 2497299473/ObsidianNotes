---
title: 附录-Docker 到 kubectl 速查卡片
created: 2026-07-25
tags:
  - K8S
  - Docker
  - 速查
  - 命令对照
  - 附录
description: Docker 命令到 kubectl 命令的完整速查卡片，按操作场景分类，供学习和工作中快速查阅。
lark_doc_url: https://my.feishu.cn/docx/PhOUdxqnaoJWG0x5tCmcG4YYnGf
---

> 📌 这张卡片供你在学习各篇笔记时随时查阅。Docker 命令在前，kubectl 等价在后。

---

## 1. 容器/Pod 生命周期

| 你想做的事 | Docker 命令 | kubectl 命令 |
|-----------|------------|-------------|
| 启动应用 | `docker run -d nginx` | `kubectl create deployment nginx --image=nginx` |
| 启动应用（声明式） | `docker compose up -d` | `kubectl apply -f deploy.yaml` |
| 查看运行中 | `docker ps` | `kubectl get pods` |
| 查看所有命名空间 | `docker ps -a` | `kubectl get pods -A` |
| 查看详情 | `docker inspect <c>` | `kubectl describe pod <pod>` |
| 查看日志 | `docker logs -f <c>` | `kubectl logs -f <pod>` |
| 查看多个 Pod 日志 | `docker logs -f`（单个） | `kubectl logs -f -l app=web`（按标签） |
| 进入容器 | `docker exec -it <c> sh` | `kubectl exec -it <pod> -- sh` |
| 进入 Deployment（自动选 Pod） | 无对标 | `kubectl exec -it deployment/web -- sh` |
| 停止容器 | `docker stop <c>` | `kubectl delete pod <pod>`（控制器重建！） |
| 删除容器 | `docker rm <c>` | `kubectl delete deployment <name>` |
| 强制删除 | `docker rm -f <c>` | `kubectl delete pod <pod> --force --grace-period=0` |

---

## 2. 镜像管理

| 操作 | Docker | kubectl / containerd |
|------|--------|----------------------|
| 构建镜像 | `docker build -t myapp .` | `docker build`（不变！K8S 不负责构建） |
| 拉取镜像 | `docker pull nginx` | 自动拉取（K8S 节点 containerd 自动 pull） |
| 推送镜像 | `docker push myrepo/myapp` | `docker push`（不变！） |
| 查看本地镜像 | `docker images` | `kubectl get pods`（不暴露镜像列表） |
| 删除镜像 | `docker rmi <image>` | 节点上 `crictl rmi <image>` |
| 多架构构建 | `docker buildx build --platform` | `docker buildx`（不变！） |

---

## 3. 扩缩容与更新

| 操作 | Docker | kubectl |
|------|--------|---------|
| 扩缩容 | `docker service scale web=5` | `kubectl scale deployment web --replicas=5` |
| 自动扩缩 | 无 | `kubectl autoscale deployment web --min=2 --max=10 --cpu-percent=70` |
| 更新镜像 | `docker service update --image nginx:1.27 web` | `kubectl set image deployment/web nginx=nginx:1.27` |
| 查看更新状态 | `docker service ps web` | `kubectl rollout status deployment/web` |
| 查看历史版本 | 无对标 | `kubectl rollout history deployment/web` |
| 回滚 | `docker service rollback web` | `kubectl rollout undo deployment/web` |
| 回滚到指定版本 | 无对标 | `kubectl rollout undo deployment/web --to-revision=2` |
| 强制重启 | `docker service update --force web` | `kubectl rollout restart deployment/web` |

---

## 4. 网络与端口

| 操作 | Docker | kubectl |
|------|--------|---------|
| 端口映射 | `docker run -p 8080:80 nginx` | `kubectl expose deployment nginx --port=80 --type=NodePort` |
| 查看服务 | `docker service ls` | `kubectl get svc` |
| 查看端点 | 无对标 | `kubectl get endpoints` |
| 端口转发 | 无对标 | `kubectl port-forward svc/web 8080:80` |
| 查看网络 | `docker network ls` | 无对标（CNI 管理网络） |
| 创建网络 | `docker network create mynet` | CNI 自动管理（无需手动） |
| DNS 解析 | 容器名自动解析 | Service 名自动解析（CoreDNS） |
| 网络策略 | 无对标 | `kubectl get networkpolicy` |

---

## 5. 存储与配置

| 操作 | Docker | kubectl |
|------|--------|---------|
| 创建 Volume | `docker volume create mydata` | PV 自动创建（StorageClass 动态供给） |
| 挂载 Volume | `docker run -v mydata:/data` | Pod YAML 中 `persistentVolumeClaim` |
| 挂载本地目录 | `docker run -v /host:/container` | Pod YAML 中 `hostPath` |
| 查看存储 | `docker volume ls` | `kubectl get pv,pvc` |
| 删除存储 | `docker volume rm mydata` | `kubectl delete pvc mydata` |
| 环境变量 | `docker run -e VAR=value` | ConfigMap + `envFrom` |
| 配置文件 | `docker run -v ./conf:/etc/app` | ConfigMap Volume 挂载 |
| 密钥管理 | `docker secret create` | `kubectl create secret` |
| 查看配置 | `docker config ls` | `kubectl get configmap` |
| 查看密钥 | 无对标（加密存储） | `kubectl get secret`（base64 编码） |

---

## 6. 集群管理

| 操作 | Docker Swarm | kubectl |
|------|-------------|---------|
| 初始化集群 | `docker swarm init` | `minikube start` / `kubeadm init` |
| 加入节点 | `docker swarm join --token` | `kubeadm join` |
| 查看节点 | `docker node ls` | `kubectl get nodes` |
| 查看节点详情 | `docker node inspect <n>` | `kubectl describe node <n>` |
| 节点标签 | `docker node update --label-add` | `kubectl label nodes <n> key=value` |
| 节点排水 | `docker node update --availability drain` | `kubectl drain <n> --ignore-daemonsets` |
| 查看集群信息 | `docker info` | `kubectl cluster-info` |
| 查看集群资源 | 无对标 | `kubectl top nodes` |

---

## 7. 调试与排查

| 操作 | Docker | kubectl |
|------|--------|---------|
| 查看容器/Pod 状态 | `docker ps -a` | `kubectl get pods -o wide` |
| 查看事件 | 无对标 | `kubectl get events --sort-by='.lastTimestamp'` |
| 查看 Pod 详情 | `docker inspect` | `kubectl describe pod <pod>` |
| 查看资源使用 | `docker stats` | `kubectl top pods` |
| 查看节点资源 | 无对标 | `kubectl top nodes` |
| 查看 API 资源 | 无对标 | `kubectl explain deployment.spec` |
| 查看 YAML | `docker inspect` | `kubectl get pod <pod> -o yaml` |
| 临时 Pod 调试 | `docker run --rm -it alpine sh` | `kubectl run debug --rm -it --image=alpine -- sh` |
| 端口转发调试 | 无对标 | `kubectl port-forward pod/<pod> 8080:80` |
| 临时容器调试 | 无对标 | `kubectl debug -it <pod> --image=busybox` |

---

## 8. 清理

| 操作 | Docker | kubectl |
|------|--------|---------|
| 清理停止的容器 | `docker container prune` | 无直接对标 |
| 清理无用镜像 | `docker image prune -a` | 节点上 `crictl rmi --prune` |
| 清理无用 Volume | `docker volume prune` | 无直接对标（需手动删 PVC） |
| 一键清理所有 | `docker system prune -a` | 无直接对标（需手动删资源） |
| 删除所有 Pod | 无对标 | `kubectl delete pods --all` |
| 删除所有 Deployment | 无对标 | `kubectl delete deployment --all` |
| 删除命名空间所有资源 | 无对标 | `kubectl delete all --all -n <ns>` |

> [!warning] K8S 没有 `docker system prune` 等价命令
> K8S 不会自动清理无用资源。需要手动删除或用 CronJob 定期清理。
> `kubectl delete all --all` 只删工作负载，不删 ConfigMap/Secret/PVC。

---

## 9. Compose / Helm 对照

| 操作 | Docker Compose | Helm |
|------|---------------|------|
| 创建项目 | 无对标（Docker Desktop 的 `docker init` 是近似体验） | `helm create myapp` |
| 部署 | `docker compose up -d` | `helm install myapp ./myapp` |
| 更新 | `docker compose up -d` | `helm upgrade myapp ./myapp` |
| 部署+更新（幂等） | `docker compose up -d` | `helm upgrade --install myapp ./myapp` |
| 卸载 | `docker compose down` | `helm uninstall myapp` |
| 查看状态 | `docker compose ps` | `helm list` |
| 多环境 | `-f docker-compose.yml -f docker-compose.prod.yml` | `-f values.yaml -f values-prod.yaml` |
| 查看历史 | 无对标 | `helm history myapp` |
| 回滚 | 无对标 | `helm rollback myapp 1` |
| 打包 | 无对标 | `helm package ./myapp` |
| 搜索仓库 | `docker search` | `helm search repo` |

---

*最后更新：2026-07-25*