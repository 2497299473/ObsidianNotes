---
title: 附录-K8S 术语表（Docker 视角对照）
created: 2026-07-25
tags:
  - K8S
  - Docker
  - 术语表
  - 对照
  - 附录
description: K8S 核心术语表，每个术语附 Docker 视角对照和一句话解释，供学习过程中随时查阅。
lark_doc_url: https://my.feishu.cn/docx/AzsmdoHAQoOvDCxojmkcRHKUnrf
---

> 📌 学习 K8S 时遇到陌生术语，在这里查 Docker 对应和一句话解释。

---

## 核心对象

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **Pod** | 容器 | K8S 最小调度单位，可包含多个共享网络和存储的容器 |
| **Deployment** | `docker run -d --restart=always` | 管理 Pod 副本数、滚动更新、回滚 |
| **StatefulSet** | 带持久化 Volume 的容器 + 固定名称 | 有状态应用（数据库），Pod 有序编号 + 稳定 DNS + 独立 PVC |
| **DaemonSet** | `docker service --mode global` | 每个节点自动运行一个副本（日志收集、监控 Agent） |
| **Job** | `docker run --rm` | 一次性任务，完成后退出 |
| **CronJob** | `cron + docker run` | 定时任务（定时备份、清理） |
| **ReplicaSet** | 无直接对标 | Deployment 的底层，维护 Pod 副本数（通常不直接用） |

---

## 网络

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **Service** | `-p` 端口映射 + 负载均衡 | 给 Pod 提供稳定的网络入口，Pod 重建后 IP 变但 Service 不变 |
| **ClusterIP** | Compose 内部网络 | 集群内部访问的虚拟 IP |
| **NodePort** | `-p 30080:80` | 在节点上开端口，集群外可访问 |
| **LoadBalancer** | 手动配 Nginx 反代 | 云厂商 LB，自动分配外部 IP |
| **Ingress** | Traefik / Nginx 反代 | L7 HTTP 路由，基于域名和路径 |
| **Ingress Controller** | Traefik 容器 | 实现 Ingress 规则的控制器（Nginx/Traefik/HAProxy） |
| **Headless Service** | 无对标 | `clusterIP: None`，每个 Pod 有独立 DNS（StatefulSet 用） |
| **Endpoints** | 无对标 | Service 背后的 Pod IP 列表 |
| **CoreDNS** | Docker 内置 DNS | K8S 的 DNS 服务，解析 Service 名到 ClusterIP |
| **CNI** | bridge / overlay 网络驱动 | 容器网络接口标准（Flannel/Calico/Cilium） |
| **NetworkPolicy** | 无对标 | Pod 间流量控制，类似云安全组 |
| **kube-proxy** | Docker iptables 规则 | 维护 Service → Pod 的路由规则 |

---

## 存储与配置

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **PersistentVolume (PV)** | `docker volume create` 创建的卷 | 集群级存储资源（管理员提供） |
| **PersistentVolumeClaim (PVC)** | `docker run -v vol:/data` | 开发者的存储申请 |
| **StorageClass** | 无对标 | 动态供给策略，PVC 创建时自动创建 PV |
| **emptyDir** | 无对标 | Pod 生命周期内的临时共享存储 |
| **hostPath** | `-v /host:/container` | 直接挂载宿主机目录 |
| **ConfigMap** | `-e VAR=value` + 配置文件 | 将配置从容器中解耦为独立资源 |
| **Secret** | `docker secret` | 密钥管理（默认 base64 编码，非加密！） |
| **Volume Mount** | `-v` / `--mount` | 将存储挂载到容器内路径 |

---

## 调度与扩缩

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **Scheduler** | Swarm Manager | 决定 Pod 跑在哪个节点（Filter → Score → Bind） |
| **nodeSelector** | `--constraint` | 简单节点约束（标签匹配） |
| **nodeAffinity** | `--constraint`（增强版） | 高级节点约束（硬约束+软约束） |
| **podAffinity** | 无对标 | 让 Pod 靠近（web + cache 同节点） |
| **podAntiAffinity** | 无对标 | 让 Pod 分散（副本不同节点，高可用） |
| **Taint** | 无对标 | 给节点打污点，排斥不容忍的 Pod |
| **Toleration** | 无对标 | Pod 声明可以容忍某些污点 |
| **HPA** | 无对标 | 水平自动扩缩（根据 CPU/内存调副本数） |
| **VPA** | 无对标 | 垂直自动扩缩（调资源 requests/limits） |
| **Cluster Autoscaler** | 无对标 | 集群节点自动扩缩 |

---

## 架构组件

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **Control Plane** | Swarm Manager 节点 | 集群大脑（API Server + etcd + Scheduler + CM） |
| **kube-apiserver** | dockerd REST API | K8S 统一入口，所有操作都经过它 |
| **etcd** | dockerd 本地状态文件 | 分布式 KV 存储，集群的"单一真相来源" |
| **kube-scheduler** | Swarm 调度器 | 决定 Pod 跑在哪个节点 |
| **kube-controller-manager** | Swarm Manager 控制循环 | 运行所有内置控制器（Deployment/ReplicaSet 等） |
| **kubelet** | dockerd（节点侧） | 节点代理，管理本节点 Pod 生命周期 |
| **kube-proxy** | Docker iptables 规则 | 维护 Service 网络规则 |
| **containerd** | containerd | 容器运行时（和 Docker 用的一样！） |
| **runc** | runc | OCI 容器运行时底层 |
| **CRI** | 无对标 | 容器运行时接口（K8S 与 containerd 的通信协议） |
| **CNI** | Docker 网络驱动 | 容器网络接口标准 |
| **CSI** | Docker Volume 驱动 | 容器存储接口标准 |

---

## 健康检查

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **livenessProbe** | `HEALTHCHECK` | 存活探针：失败则重启容器 |
| **readinessProbe** | 无对标 | 就绪探针：失败则从 Service 移除（不重启） |
| **startupProbe** | 无对标 | 启动探针：保护慢启动应用（通过后才开 liveness） |

---

## 安全与隔离

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **Namespace** | Docker Compose 项目名 | 逻辑隔离边界（dev/prod/test） |
| **RBAC** | Docker 节点角色（Manager/Worker） | 细粒度权限控制（Role/ClusterRole + Binding） |
| **ServiceAccount** | 无对标 | Pod 的身份标识 |
| **NetworkPolicy** | 无对标 | Pod 间网络流量控制 |

---

## 包管理与扩展

| K8S 术语 | Docker 对应 | 一句话解释 |
|---------|------------|-----------|
| **Helm** | Docker Compose（增强版） | K8S 包管理器，模板化 YAML + 版本管理 |
| **Chart** | docker-compose.yml（参数化版） | Helm 的打包格式（.tgz） |
| **values.yaml** | docker-compose.yml 默认值 | Chart 的参数文件 |
| **Kustomize** | 多 Compose 文件 overlay | K8S 原生多层叠加（无模板化） |
| **CRD** | 无对标 | 自定义资源定义（扩展 K8S API） |
| **Operator** | 无对标 | CRD + Controller，将运维知识编码为自动化控制器 |
| **Controller** | Swarm Manager 控制循环 | 持续观察并调和实际状态向期望状态靠拢 |
| **Reconcile Loop** | 无对标 | 控制器的核心逻辑：比较期望 vs 实际 → 执行操作 |

---

## 常用缩写

| 缩写 | 全称 | 含义 |
|------|------|------|
| K8S | Kubernetes | K 和 s 之间有 8 个字母 |
| Pod | Pod（非缩写） | K8S 最小调度单位 |
| PVC | PersistentVolumeClaim | 持久化存储申请 |
| PV | PersistentVolume | 持久化存储资源 |
| HPA | HorizontalPodAutoscaler | 水平 Pod 自动扩缩器 |
| VPA | VerticalPodAutoscaler | 垂直 Pod 自动扩缩器 |
| CRD | CustomResourceDefinition | 自定义资源定义 |
| CNI | Container Network Interface | 容器网络接口 |
| CRI | Container Runtime Interface | 容器运行时接口 |
| CSI | Container Storage Interface | 容器存储接口 |
| RBAC | Role-Based Access Control | 基于角色的访问控制 |
| OCI | Open Container Initiative | 开放容器标准（镜像格式） |
| FQDN | Fully Qualified Domain Name | 完全限定域名 |

---

*最后更新：2026-07-25*