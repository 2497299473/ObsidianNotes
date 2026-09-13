---
title: README - K8S 学习路径（Docker 迁移版）
created: 2026-07-23
tags:
  - K8S
  - Docker
  - 迁移学习
  - 索引
  - 云原生
description: K8SLearningByDocker 学习路径的快速入口：面向有 Docker 基础的开发者，基于 Docker 概念迁移学习 Kubernetes。
lark_doc_url: https://my.feishu.cn/docx/Lr1MdCtPQolUK5xxgncctpvenYb
---

# 🦐 K8SLearningByDocker

> 🐳 **Docker 已掌握？用你熟悉的容器知识，迁移学习 Kubernetes！**

本学习路径面向 **有 Docker 基础的开发者**，以 Docker 为认知锚点迁移学习 Kubernetes。不再从零讲起，而是告诉你：**哪些概念和 Docker 一样、哪些完全不同、哪些是 K8S 独有的新武器**。

---

## 你适合这条路径吗？

- ✅ 已经会用 `docker run`、`docker compose up`、`docker build`
- ✅ 理解镜像、容器、Volume、网络、Dockerfile
- ✅ 想系统学习 K8S，但不想从零开始背概念
- ❌ 完全没有接触过容器 → 建议先去学 [[01-学习/DockerNew/00-Docker 总览索引]]

---

## 快速开始

1. **先看** [[00-K8S 总览索引（Docker 迁移版）]] → 了解全局结构和学习路线
2. **三周计划**：第一周（会用 K8S）→ 第二周（深入调度与存储）→ 第三周（生产实战）
3. **每篇末尾有 🎯 自检清单**，确保 🟢 全部通过再进入下一篇
4. **完成毕业项目 v1→v6**，体会从 Docker Compose 到 K8S Helm + 监控告警的完整迁移

---

## 学习路线总览

| 阶段 | 笔记 | 核心目标 | 预计学时 |
|------|------|---------|---------|
| 第一周 | [[00-K8S 总览索引（Docker 迁移版）]] | 建立 Docker→K8S 全局认知 | 0.5h |
| 第一周 | [[01-环境搭建与核心概念映射]] | 用 Docker 经验理解 K8S 架构与声明式 API | 2h |
| 第一周 | [[02-Pod 与工作负载：从容器到 Pod]] | 理解 Pod 和 Deployment/StatefulSet | 3.5h |
| 第一周 | [[03-Service 与网络：从 Docker 网络到 K8S]] | 掌握 K8S 网络和服务暴露 | 3.5h |
| 第二周 | [[04-存储与配置：从 Volume 到 PV-PVC]] | 理解 K8S 持久化与配置管理 | 2.5h |
| 第二周 | [[05-调度与扩缩容：从 scale 到 HPA]] | 掌握调度器、亲和性、自动扩缩容 | 2.5h |
| 第二周 | [[06-Docker有K8S无与K8S有Docker无]] | 系统梳理双向特性差异 | 3h |
| 第三周 | [[01-学习/K8SLearningByDocker/07-业务场景实战合集|07-业务场景实战合集]] | 10 大场景从 Docker 迁移到 K8S | 3h |
| 第三周 | [[08-面试高频20问-Docker背景版]] | 面试高频题，每题带 Docker 对比 | 3h |

> **总学时**：约 24h（Docker 背景开发者） + 15h 毕业项目 = 约 39h

---

## 目录结构

```
K8SLearningByDocker/
├── README.md                          ← 你在这里
├── 00-K8S 总览索引（Docker 迁移版）    ← 课程地图 + 学习路线
├── 01-环境搭建与核心概念映射.md         ← 从 docker run 到 kubectl apply
├── 02-Pod 与工作负载：从容器到 Pod.md   ← 从容器到 Pod/Deployment
├── 03-Service 与网络：从 Docker 网络到 K8S.md ← 从 -p 端口映射到 Service/Ingress
├── 04-存储与配置：从 Volume 到 PV-PVC.md ← 从 Volume 到 PV/PVC/ConfigMap
├── 05-调度与扩缩容：从 scale 到 HPA.md  ← 从 docker service scale 到 HPA
├── 06-Docker有K8S无与K8S有Docker无.md  ← 双向特性全景对比
├── 07-业务场景实战合集.md              ← 10 大场景 Docker→K8S 迁移
├── 08-面试高频20问-Docker背景版.md     ← 每问带 Docker 视角对比
├── 99-第一周复习检查点.md
├── 99-第二周复习检查点.md
├── 99-第三周复习检查点.md
├── 05b-Helm 与 Chart 模板.md           ← Chart 结构、模板语法、多环境
├── 毕业项目-Web应用K8S化/              ← v1→v6 版本递进
│   ├── v1-Deployment基础版.md
│   ├── v2-Service网络版.md
│   ├── v3-存储持久化版.md
│   ├── v4-ConfigMap与Secret版.md
│   ├── v5-Helm包管理版.md
│   └── v6-监控告警版.md
├── 附录-Compose 到 K8S 逐行对照.md     ← 完整迁移实战
├── 附录-Docker 到 kubectl 速查卡片.md  ← 命令级对照
├── 附录-K8S 术语表（Docker 视角）.md    ← 术语速查
└── reviews/                            ← 三维审查报告
    ├── 01-结构审查报告.md
    ├── 02-技术校验报告.md
    └── 03-体验优化报告.md
```

---

## 推荐学习方法

1. **不要跳过 01-02**：这两篇建立了 Docker→K8S 的核心映射，后续都依赖这个基础
2. **每篇先过一遍 Docker 锚点**：如果 Docker 对应的概念不熟，先回顾 [[01-学习/DockerNew/00-Docker 总览索引]]
3. **动手为先**：每篇都配有实践练习，完成 🟢 基础项后再进入下一篇
4. **双向对比篇 06 是核心**：迁移式学习路径的差异化价值就在这篇
5. **用 minikube 或 kind 练手**：参考 [[00-K8S 总览索引（Docker 迁移版）]] 的环境准备

---

## 前置要求

- ✅ 掌握 Docker 基础（容器、镜像、Dockerfile、Compose）
- ✅ 了解 Docker Swarm 或至少用过 `docker service`
- ✅ 有 Linux 命令行基础
- ✅ 有 YAML 编写经验

---

## 环境准备

- **minikube**（推荐入门）或 **kind**（Kubernetes in Docker）
- **kubectl**（K8S 命令行工具）
- 本机已安装 Docker（minikube/kind 需要）
- **Helm**（v5/v6 毕业项目需要）

---

## 相关资源

- [[03-AI工具/AI协作方法论/技术学习路径创建方法论]] — 本路径的创建方法论
- [[03-AI工具/AI协作方法论/迁移式学习路径创建方法论]] — 迁移式学习路径设计方法论
- [[01-学习/DockerNew/00-Docker 总览索引]] — Docker 学习路径入口（迁移锚点）
- [[../跨技术栈学习路径总览]] — 跨技术栈学习全景

---

*基于 [[03-AI工具/AI协作方法论/迁移式学习路径创建方法论]] 构建 · 最后更新：2026-07-25*