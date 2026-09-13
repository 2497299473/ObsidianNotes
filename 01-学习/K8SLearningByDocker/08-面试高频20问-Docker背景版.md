---
title: 08-面试高频20问-Docker背景版
created: 2026-07-23
stage: 3
order: 8
difficulty: ⭐⭐⭐⭐
estimated_hours: 3
tags:
  - K8S
  - Docker
  - 面试
  - 迁移学习
  - 高频题
description: K8S 面试高频 20 问（Docker 背景版）：每题带 Docker 对比视角，采用黄金三段式回答框架——先结论 → 再展开原理 → 再补充生产经验。专为有 Docker 背景的面试者设计。
lark_doc_url: https://my.feishu.cn/docx/YBrudoP7yo0fpqxhDS8cKvzvnOh
---

> [!important] 面试黄金三段式
> ① **先说结论**（一句话定性）
> ② **再展开原理**（结合 Docker 对比，展示思维跃迁）
> ③ **最后补充生产经验**（踩坑、优化、选型）
>
> 有 Docker 背景是加分项——面试官喜欢能对比着说的候选人。

---

## 面试题速查

| # | 问题 | 难度 | Docker 视角 |
|---|------|------|------------|
| 1 | Docker 和 K8S 的关系与区别？ | ⭐ | 核心定位 |
| 2 | 什么是声明式 API？对比命令式 | ⭐⭐ | 命令式 vs 声明式 |
| 3 | Pod 和 Docker 容器的区别？ | ⭐⭐ | 容器 vs Pod |
| 4 | Deployment 和 `docker run -d` 的区别？ | ⭐⭐ | restart vs Controller |
| 5 | Service 的类型？对比 `-p` 端口映射 | ⭐⭐ | 端口映射 |
| 6 | K8S 网络模型和 Docker 网络有何不同？ | ⭐⭐⭐ | CNI vs bridge |
| 7 | PV/PVC 和 Docker Volume 的关系？ | ⭐⭐ | 存储抽象 |
| 8 | ConfigMap 和 `-e` 环境变量区别？ | ⭐⭐ | 配置管理 |
| 9 | K8S Secret vs Docker Secret 安全差异？ | ⭐⭐⭐ | 密钥管理 |
| 10 | HPA 工作原理？Docker 有对标吗？ | ⭐⭐⭐ | 自动扩缩 |
| 11 | K8S 1.24 移除 dockershim 后镜像还能用？ | ⭐⭐ | 容器运行时 |
| 12 | StatefulSet 和 Deployment 的区别？ | ⭐⭐⭐ | 有状态 vs 无状态 |
| 13 | Ingress 和 Docker Traefik 的区别？ | ⭐⭐ | HTTP 路由 |
| 14 | etcd 的作用？Docker 需要吗？ | ⭐⭐⭐ | 状态存储 |
| 15 | K8S 自愈能力？对比 Docker restart | ⭐⭐ | 自愈 |
| 16 | Helm 和 Docker Compose 的区别？ | ⭐⭐ | 包管理 |
| 17 | CRD 和 Operator 是什么？ | ⭐⭐⭐ | 扩展能力 |
| 18 | 如何实现零停机部署？ | ⭐⭐⭐ | 滚动更新 |
| 19 | Taint/Toleration？Docker 有对标吗？ | ⭐⭐⭐ | 节点排斥 |
| 20 | Docker Swarm 和 K8S 怎么选？ | ⭐⭐ | 选型 |

---

## Q1：Docker 和 K8S 的关系与区别？

> ① **结论**：Docker 是容器运行时，K8S 是容器编排平台。两者是互补关系，不是替代关系。
>
> ② **展开**：
> - Docker 解决"单机怎么运行容器"（构建、运行、单机编排）
> - K8S 解决"多机怎么管理容器"（调度、扩缩、自愈、服务发现）
> - K8S 1.24+ 用 containerd 代替 Docker 作为运行时，但镜像格式是 OCI 标准，Docker 构建的镜像仍可在 K8S 上运行
>
> ③ **生产经验**：CI/CD 中 Docker 负责构建镜像（`docker build`），K8S 负责部署运行（`kubectl apply`）。不要在 K8S 节点上安装 Docker CLI。

---

## Q2：什么是声明式 API？对比 Docker 命令式

> ① **结论**：声明式 API 是你告诉系统"要什么状态"，系统自动调和。Docker 命令式是你告诉系统"做什么"。
>
> ② **展开**：
> - Docker：`docker run` → `docker stop` → `docker rm` → `docker run`（换配置重来）
> - K8S：写 YAML（`replicas: 3`）→ `kubectl apply` → Controller 通过 Reconcile Loop 自动让集群达到 3 个副本
>
> ③ **生产经验**：YAML 纳入 git 管理，可审计、可回滚、可重复。`kubectl apply` 多次执行无副作用（幂等性）。CI/CD 中始终用 `apply`，不用 `create`。

---

## Q3：Pod 和 Docker 容器的区别？为什么要有 Pod？

> ① **结论**：Pod 是 K8S 最小调度单位，可包含多个共享网络和存储的容器。
>
> ② **展开**：
> - Docker 最小单位是容器，Pod 多了一层抽象
> - Pod 引入是为了支持多容器协作（Sidecar 模式）
> - 同一 Pod 内容器通过 `localhost` 互访，共享 Volume
> - Docker 中需 `--volumes-from` 和 `--network container:xxx` 模拟，体验差
>
> ③ **生产经验**：紧密耦合的容器（Web + 日志采集器）放一个 Pod；松耦合的服务（Web + Redis）各自独立 Pod + Service。

---

## Q4：Deployment 和 `docker run -d --restart=always` 的区别？

> ① **结论**：Deployment 是声明式 Pod 管理器，提供自愈、扩缩、滚动更新，远超 Docker restart 策略。
>
> ② **展开**：
>
> | 能力 | `docker run --restart=always` | Deployment |
> |------|------------------------------|-----------|
> | 自愈 | 进程退出重启 | Pod 挂了自动重建 |
> | 扩缩容 | 无 | `kubectl scale` / HPA |
> | 滚动更新 | 无 | 零停机更新 |
> | 回滚 | 无 | `kubectl rollout undo` |
>
> ③ **生产经验**：设置 `maxSurge: 1` + `maxUnavailable: 0` 实现零停机更新。用 `readinessProbe` 确保新版本就绪后才删旧版本。

---

## Q5：Service 的类型？对比 Docker `-p` 端口映射

> ① **结论**：Service 有四种类型：ClusterIP、NodePort、LoadBalancer、ExternalName。
>
> ② **展开**：
> - Docker `-p 8080:80`：直接端口转发，绑定单机
> - K8S Service：通过标签选择器自动关联 Pod，提供稳定的虚拟 IP 和 DNS 名，支持负载均衡
> - Pod IP 是临时的，Service IP 是稳定的
>
> ③ **生产经验**：集群内用 ClusterIP，对外用 LoadBalancer（云环境）或 Ingress（本地/裸机）。不要用 NodePort 对外暴露生产服务。

---

## Q6：K8S 网络模型和 Docker 网络有何不同？

> ① **结论**：K8S 网络更扁平化，所有 Pod 默认可跨节点直接通信。
>
> ② **展开**：
> - Docker bridge 需端口映射，overlay 需 Swarm
> - K8S 通过 CNI 插件实现：所有 Pod 有独立 IP，Pod 间可跨节点直接通信
> - CNI 插件化（Flannel/Calico/Cilium），NetworkPolicy 提供细粒度隔离
>
> ③ **生产经验**：入门用 Flannel，生产用 Calico（支持 NetworkPolicy + BGP 路由）。

---

## Q7：PV/PVC 和 Docker Volume 的关系？

> ① **结论**：PV/PVC 是 Docker Volume 的集群级抽象升级。
>
> ② **展开**：
> - Docker Volume 绑定在单台宿主机
> - K8S PV（集群级资源）+ PVC（用户请求）+ StorageClass（动态供给，Docker 没有！）
> - Pod 调度到任何节点都能挂载同一份数据
>
> ③ **生产经验**：生产环境用 StorageClass 动态供给，`reclaimPolicy: Retain` 保护数据。

---

## Q8：ConfigMap 和 Docker `-e` 环境变量区别？

> ① **结论**：ConfigMap 将配置从容器中解耦为独立资源，比 `-e` 更灵活。
>
> ② **展开**：
> - Docker `-e VAR=value`：写在命令行或 Compose 文件
> - K8S ConfigMap：独立 YAML 资源，支持环境变量（`envFrom`）和文件挂载（Volume）
> - 文件挂载方式支持热更新
>
> ③ **生产经验**：需要热更新的配置用 Volume 挂载，不需要的用环境变量。不要把敏感信息放 ConfigMap，用 Secret。

---

## Q9：K8S Secret vs Docker Secret 安全差异？

> ① **结论**：概念相似，但 K8S Secret 默认只 base64 编码（不加密），Docker Swarm Secret 是加密存储。
>
> ② **展开**：
> - Docker Swarm：`echo "pwd" | docker secret create` → 加密存储，容器内 `/run/secrets/<name>`
> - K8S：`kubectl create secret` → base64 编码，以环境变量或文件挂载
> - K8S Secret **不是加密**！
>
> ③ **生产经验**：启用 etcd 静态加密（Encryption at Rest），或用 Vault / Sealed Secrets。不要把 Secret 写在 YAML 中提交到 git。

---

## Q10：HPA 工作原理？Docker 有对标吗？

> ① **结论**：HPA 根据 CPU/内存/自定义指标自动调整 Pod 副本数，Docker 没有原生对标。
>
> ② **展开**：
> - Docker 的 `docker service scale web=5` 是手动扩缩
> - K8S HPA：Metrics Server 采集指标 → 计算期望副本数 → 自动扩缩
> - 扩容：CPU > 70% → 增加副本；缩容：有 5 分钟冷却期
>
> ③ **生产经验**：HPA 要求 Deployment 设置 `resources.requests.cpu`。缩容冷却期默认 5 分钟，避免频繁抖动。自定义指标（如 QPS）需 Prometheus Adapter。

---

## Q11：K8S 1.24 移除 dockershim 后，Docker 镜像还能用吗？

> ① **结论**：**能！** 移除的是 Docker 运行时适配层，不是镜像格式。
>
> ② **展开**：
> - Docker 镜像遵循 OCI 标准，containerd 和 CRI-O 都支持
> - dockershim 只是一个适配层，移除后 K8S 直接通过 CRI 接口与 containerd 通信
> - `docker build` → push → `kubectl apply` 流程不变
>
> ③ **生产经验**：CI/CD 流程不变。K8S 节点用 containerd，不需要安装 Docker。

---

## Q12：StatefulSet 和 Deployment 的区别？

> ① **结论**：StatefulSet 用于有状态应用（稳定标识 + 独立存储），Deployment 用于无状态应用。
>
> ② **展开**：
> - Deployment Pod 名随机（`nginx-5d59d-abcde`）
> - StatefulSet Pod 有序编号（`mysql-0`、`mysql-1`）、稳定 DNS、独立 PVC
> - `volumeClaimTemplates` 自动为每个 Pod 创建独立 PVC
>
> ③ **生产经验**：数据库/消息队列用 StatefulSet，Web/API 用 Deployment。StatefulSet 删除后 PVC 不会自动删除（保护数据）。生产数据库用 Operator。

---

## Q13：Ingress 和 Docker Traefik 的区别？

> ① **结论**：Ingress 是 K8S 的 HTTP 路由抽象，概念等同于 Docker 中 Traefik 反向代理。
>
> ② **展开**：
> - Docker：Traefik 通过容器标签配置路由
> - K8S：Ingress 是声明式 YAML 资源，由 Ingress Controller（Nginx/Traefik）实现
> - 支持 TLS 终止、基于域名/路径的路由
>
> ③ **生产经验**：用 Nginx Ingress Controller + cert-manager 自动管理 TLS 证书。Ingress 是 L7，Service 是 L4。

---

## Q14：etcd 的作用？Docker 需要吗？

> ① **结论**：etcd 是 K8S 的分布式状态存储（单一真相来源），Docker 单机不需要。
>
> ② **展开**：
> - Docker 状态存在 dockerd 内存和本地文件中
> - Docker Swarm 用 Raft 内置于引擎
> - K8S 用独立 etcd 集群（3/5 节点），存储所有集群状态
>
> ③ **生产经验**：etcd 宕了集群无法变更（但已有 Pod 继续运行）。至少 3 节点 etcd，定期 `etcdctl snapshot save` 备份。

---

## Q15：K8S 如何处理故障自愈？对比 Docker restart

> ① **结论**：K8S 自愈是 Controller 自动调和，Docker `--restart=always` 是进程级重启。
>
> ② **展开**：
> - Docker 只能在容器退出时重启
> - K8S：Pod 挂了 → Controller 重建；节点挂了 → 调度到其他节点；livenessProbe 失败 → 重启容器；readinessProbe 失败 → 从 Service 移除
>
> ③ **生产经验**：配置 liveness/readiness Probe，合理设置 `initialDelaySeconds`。用 startupProbe 保护慢启动应用。

---

## Q16：Helm 和 Docker Compose 的区别？

> ① **结论**：Helm 是 K8S 的包管理器（类似 apt/yum），Docker Compose 是单机编排工具。
>
> ② **展开**：
> - Docker Compose：单文件编排，无模板化，无版本管理
> - Helm Chart：模板化 YAML + values 参数化，支持 `helm install/upgrade/rollback`
>
> ③ **生产经验**：用 Helm 部署第三方应用（Redis、Prometheus），用 `helm upgrade --install` 实现幂等部署。

---

## Q17：CRD 和 Operator 是什么？

> ① **结论**：CRD 扩展 K8S API，Operator 将运维知识编码为控制器。Docker 没有对标。
>
> ② **展开**：
> - CRD：定义新的资源类型（如 `Prometheus`、`MySQLCluster`）
> - Operator = CRD + Controller，自动管理部署、升级、备份
> - 例：Prometheus Operator 自动管理 Prometheus 的完整生命周期
>
> ③ **生产经验**：复杂有状态应用（数据库、消息队列、监控）优先用 Operator。不要自己写 Operator，优先用社区的。

---

## Q18：如何实现零停机部署？

> ① **结论**：通过 Deployment 滚动更新 + readinessProbe 实现。
>
> ② **展开**：
> - 配置 `maxSurge: 1` + `maxUnavailable: 0`：先创建新 Pod，readiness 通过后才删旧 Pod
> - 对比 Docker Swarm 的 `docker service update --update-parallelism 1`
>
> ③ **生产经验**：配置 `terminationGracePeriodSeconds: 30` 和 `preStop` 钩子，让旧 Pod 优雅关闭。Canary 发布用 Argo Rollouts。

---

## Q19：Taint/Toleration？Docker 有对标吗？

> ① **结论**：Taint/Toleration 是 K8S 独有的节点排斥机制，Docker 没有对标。
>
> ② **展开**：
> - Taint：给节点打污点，排斥不容忍的 Pod
> - Toleration：Pod 声明可以容忍某些污点
> - Docker 只有 `--constraint` 正向约束，没有反向排斥
>
> ③ **生产经验**：Master 节点默认有 `NoSchedule` 污点。用 Taint 保护专用节点（如 GPU 节点只跑 GPU 任务）。

---

## Q20：Docker Swarm 和 K8S 怎么选？

> ① **结论**：Swarm 适合中小规模（< 20 节点），K8S 适合中大规模和云原生生态。
>
> ② **展开**：
>
> | 维度 | Docker Swarm | Kubernetes |
> |------|-------------|------------|
> | 学习曲线 | 低（1 天） | 高（1-2 周） |
> | 自动扩缩 | 无 | HPA/VPA |
> | 网络策略 | 无 | NetworkPolicy |
> | 生态 | 小 | 庞大（Helm/Operator/Istio） |
>
> ③ **生产经验**：如果最终要上 K8S，直接学 K8S，别在 Swarm 上浪费时间转型。节点 > 50、微服务 > 50 个 → K8S。

---

## 🧪 实践练习

### 🟢 基础练习：模拟面试

```bash
# 1. 随机选 5 道题，口述回答
# 2. 录音并对照三段式框架检查：
#    - 是否先说结论？
#    - 是否有 Docker 对比视角？
#    - 是否有生产经验补充？
# 3. 重复练习直到每题能在 2 分钟内清晰回答
```

### 🟡 进阶练习：准备加分点

```bash
# 针对每道题，准备 2 个加分点：
# - 你项目中的真实实践
# - 你遇到的坑和解决方案
# 例如 Q10 HPA 加分点：
# 1. 我们 HPA 配置了 scaleDown 稳定窗口，避免抖动
# 2. 我们用自定义指标（QPS）触发扩缩，而非仅 CPU
```

### 🔴 挑战练习：架构设计题

#### 架构设计题 1：Docker Compose 微服务迁移到 K8S

```bash
# 问题：如何把一个 Docker Compose 微服务（Web + API + DB + Redis）迁移到 K8S？
# 黄金三段式回答框架：
# 1. 概念映射：compose.yaml → 多 YAML / Helm Chart
# 2. 网络：-p → Service + Ingress
# 3. 存储：docker volume → PVC + StorageClass
# 4. 配置：-e → ConfigMap/Secret
# 5. 部署：docker build → kubectl set image / ArgoCD GitOps
```

**参考答案**：
- **先说结论**：我会采用"先平移、再优化"的策略，先用 Helm Chart 将 Compose 服务映射为 K8S 资源，再逐步引入 StatefulSet、ConfigMap/Secret、Ingress 和 HPA。
- **详细设计**：
  - Web（Nginx）→ Deployment + NodePort/LoadBalancer Service + Ingress（外部访问）
  - API（Flask）→ Deployment + ClusterIP Service + ConfigMap/Secret + liveness/readiness Probe
  - DB（PostgreSQL）→ StatefulSet + Headless Service + PVC（动态供给）+ Secret 凭据
  - Redis → Deployment + ClusterIP Service（缓存无需持久化，若需持久化改用 PVC）
  - 所有镜像通过 CI/CD 构建后推送镜像仓库，K8S 通过 `imagePullPolicy: Always` 拉取
  - 多环境通过 `values-dev.yaml` / `values-prod.yaml` 区分
- **生产优化**：引入 HPA 自动扩缩、NetworkPolicy 隔离、Prometheus 监控、PodDisruptionBudget 保证可用性。

#### 架构设计题 2：设计一个高可用的 K8S 集群架构

```bash
# 问题：设计一个承载 100 个微服务、10 万 QPS 的 K8S 高可用架构？
```

**参考答案**：
- **先说结论**：采用多可用区 + 多 Master + 多 Worker + 分层流量管理的架构。
- **详细设计**：
  - **控制平面**：3/5 个 Master 节点跨可用区部署，etcd 单独 3/5 节点集群，API Server 前置 LB
  - **工作节点**：按业务分层（Web/API/数据），跨可用区部署，预留 30% 缓冲容量
  - **网络**：CNI 用 Calico/Cilium，Ingress 用 Nginx Ingress Controller + External LB
  - **存储**：多副本存储（如 Rook/Ceph 或云厂商 CSI），按业务需求区分 SSD/HDD StorageClass
  - **监控**：Prometheus Operator + Grafana + AlertManager + Loki 日志
  - **CI/CD**：GitLab CI / GitHub Actions 构建镜像，ArgoCD 做 GitOps 部署
  - **安全**：RBAC + NetworkPolicy + Pod Security Admission/Standards（PSA/PSS，通过给 namespace 打 `pod-security.kubernetes.io/*` 标签实施 enforce/audit/warn）+ 镜像扫描 + Secret 加密
- **对比 Docker**：Docker 单机或 Swarm 无法实现这种规模和可用性，K8S 的声明式 + 调度器 + 控制器模式是核心差异。

#### 架构设计题 3：K8S 生产故障排查流程

```bash
# 问题：生产环境用户反馈服务不可用，你的排查流程是什么？
```

**参考答案**：
- **第一步：确认范围**：是单个 Pod 还是整个服务？`kubectl get pods,svc,events`
- **第二步：分层定位**
  - DNS 解析失败？`nslookup service.default.svc.cluster.local`
  - Service 后端无 Pod？`kubectl get endpoints`
  - Pod 异常？`kubectl describe pod` + `kubectl logs`
  - 节点异常？`kubectl get nodes` + `kubectl describe node`
- **第三步：常见根因**：OOMKilled、镜像拉取失败、资源不足、探针配置错误、网络策略误拦截
- **第四步：修复与预防**：扩容、调整 limit、修复镜像、增加告警、完善 runbook
- **对比 Docker**：Docker 单机排查只需 `docker ps` + `docker logs`，K8S 需要分对象、分层、分命名空间排查。

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能用三段式框架清晰回答 20 道高频题 |
| 🟢 基础 | 能在每题中体现 Docker→K8S 的迁移思维 |
| 🟡 进阶 | 能为每道题准备 2 个生产加分点 |
| 🔴 挑战 | 能在 2 分钟内清晰回答一道题，包含对比、原理、实践 |

---

## 相关笔记

- ⬅️ 前置：[[01-环境搭建与核心概念映射]] ~ [[01-学习/K8SLearningByDocker/07-业务场景实战合集|07-业务场景实战合集]] — 全部核心笔记
- 🔗 关联：[[../DockerNew/11-面试高频 20 问]] — Docker 面试题对照

---

*最后更新：2026-07-23*