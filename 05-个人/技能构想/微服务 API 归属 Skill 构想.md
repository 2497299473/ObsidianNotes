---
title: 微服务 API 归属 Skill 构想
date: 2026-06-21
tags:
  - skills
  - claude-code
  - microservices
  - api-design
  - 架构
  - 构想
aliases:
  - api-placement
  - service-boundary
status: 🌱 构想中
lark_doc_url: https://my.feishu.cn/docx/Ejl2d516ropfljx50BHcnwYanWg
lark_doc_token: Ejl2d516ropfljx50BHcnwYanWg
---

## 做什么

当设计新 API 时，帮助 Claude 判断"这个 API 应该放在哪个微服务里"。核心不靠 AI 推理——靠项目已有的**服务地图**、**归属规则**和**历史决策记录**。

## 核心判断

> API 在哪个微服务，这不是 Skill 应该干的事。人类是怎么判断的，AI 就可以怎么判断。至少得补充上术语——这个服务是做什么的、里边会包含哪些 API、这个 API 计划是做什么的。

对。Skill 在这里的角色不是"替你做架构决策"，而是：

1. **在你做决策时，把该有的上下文准备好**（服务地图、边界规则、历史先例）
2. **确保决策过程可追溯**（记录为什么这个 API 归属服务 A 而不是 B）
3. **在灰色地带给出 checklist，而不是拍板**

**核心规则**：`API follows data, not caller`——谁拥有数据，谁提供方法。

## Skill 结构

```
.claude/skills/api-placement/
├── SKILL.md                    ← 决策流程：分析数据归属 → 查服务地图 → 输出归属建议
├── reference/
│   ├── service-map.md          ← 服务地图（每个服务拥有什么数据/实体）
│   ├── boundary-rules.md       ← 边界规则 + 灰色地带决策清单
│   └── historical-decisions.md ← 历史决策记录（踩过什么坑、为什么迁移）
└── templates/
    └── placement-decision.md   ← 归属决策记录模板
```

## SKILL.md（流程骨架）

```yaml
---
name: api-placement
description: Determine which microservice should own a new API endpoint. Uses the project's service map, data ownership rules, and historical decisions to guide placement. Use when designing a new API and deciding which service should expose it, or when questioning whether an existing API is in the right service.
allowed-tools:
  - Read
  - Grep
  - Glob
---

# API Placement Decision Guide

## 决策流程

1. **明确 API 的职责**
   - 这个 API 要做什么？（读/写/计算/编排？）
   - 它操作哪个**主数据实体**？

2. **查服务地图**
   - 打开 [[reference/service-map]]
   - 找到拥有该数据实体的服务
   - 规则：**API follows data, not caller**

3. **处理灰色地带**
   - 如果 API 涉及多个实体 → 对照 [[reference/boundary-rules]]
   - 如果仍无法确定 → 给出两个方案，标注 trade-off

4. **记录决策**
   - 按 [[templates/placement-decision]] 模板输出
   - 任何争议决策必须记录到 [[reference/historical-decisions]]
```

## reference/service-map.md（示例）

```markdown
# 服务地图

## 核心服务与数据所有权

| 服务 | 拥有的实体 | 提供的 API 类型 |
|------|----------|---------------|
| user-service | User, Profile, Role, Permission | 用户 CRUD、认证、权限校验 |
| order-service | Order, OrderItem, Payment | 订单创建/查询/状态流转、支付回调 |
| product-service | Product, Category, Inventory, Price | 商品 CRUD、库存扣减、价格查询 |
| notification-service | Template, SendLog | 消息模板管理、发送记录查询 |
| analytics-service | Report, Metric, Dashboard | 报表生成、指标查询 |

## 服务职责边界

### user-service
- **拥有**：用户身份、登录态、角色权限
- **不拥有**：用户的订单（属于 order-service）、用户行为日志（属于 analytics-service）
- **对外契约**：`GET /users/{id}` 返回基本用户信息，不包含订单列表

### order-service
- **拥有**：订单全生命周期
- **依赖**：user-service（用户信息）、product-service（商品价格）
- **对外契约**：`GET /orders/{id}` 返回完整订单，通过 user_id 关联用户、product_id 关联商品

### 跨服务关联规则
- 服务间**不持有对方实体的主数据**——只存 ID 引用
- 需要关联展示时，由**调用方（BFF/网关）负责聚合**，而非被调用方
```

## reference/boundary-rules.md（示例）

```markdown
# 边界规则与决策清单

## 核心原则

**API follows data, not caller。** API 归属于持有该实体主数据的服务。

## 决策清单

当一个 API 归属不明确时，按以下顺序判断：

### 1. 数据归属（权重最高）
- 这个 API 主要操作哪个实体的数据？
- → 归该实体所属的服务

### 2. 事务边界
- 这个 API 是否需要在同一个数据库事务中操作多个实体？
- → 如果跨服务，拆成 saga/outbox 模式，API 仍归属主实体服务

### 3. 读写分离
- 纯读 API，且数据来自多个服务？
- → 考虑放在 BFF 层或独立查询服务，**不属于任何领域服务**

### 4. 编排逻辑
- API 的主要职责是编排多个服务调用（而非操作某个实体）？
- → **BFF / API Gateway**，不属于领域服务

## 灰色地带案例

### 案例 1：订单导出
> "导出用户最近 30 天的订单为 CSV"

- 数据归属：订单数据 → order-service
- 但：导出是报表行为，非订单领域逻辑
- **决策**：归属 analytics-service（报表服务），由它调用 order-service 获取数据
- **理由**：导出格式、频率限制、文件管理是报表域的职责，订单域不应感知

### 案例 2：搜索
> "全局搜索：输入关键词，返回匹配的用户、商品、订单"

- 数据归属：跨多个服务
- **决策**：独立 search-service，通过数据同步（CDC/ES）维护搜索索引
- **理由**：搜索是独立的查询模型，不应耦合到任何领域服务

### 案例 3：用户画像
> "根据用户行为生成兴趣标签"

- 数据归属：用户行为数据（多来源）+ 标签存储
- **决策**：归属 analytics-service，由它读行为数据、写标签到 user-service
- **理由**：计算逻辑属于分析域，但标签写入通过 user-service API
```

## reference/historical-decisions.md（示例）

```markdown
# 历史决策记录

## 2025-03: 支付回调从 order-service 移到 payment-service

**背景**：支付回调逻辑（验签、幂等、状态更新）起初在 order-service
**问题**：支付宝/微信各有不同 SDK 和验签方式，order-service 变得臃肿
**决策**：新建 payment-service，只做支付渠道对接和回调处理
**trade-off**：多了一个服务 + 一次 RPC 调用（payment-service → order-service 更新订单状态）
**结论**：值得。order-service 回归订单核心逻辑，payment-service 可以独立管理渠道适配

## 2025-07: 库存扣减从 product-service 移到 inventory-service

**背景**：库存逻辑（预占、释放、过期回收）与商品信息（名称、描述、图片）差异很大
**问题**：product-service 承担了两个不相关的职责
**决策**：拆出 inventory-service，product-service 保留商品基础信息
**教训**：**服务按"数据生命周期"切分，而非按"页面"切分**——商品信息和库存的生命周期完全不同
```

## templates/placement-decision.md（示例）

```markdown
# API 归属决策记录

## API 概述
- **API 名称**：
- **职责描述**：
- **涉及的实体**：

## 归属判断

### 数据归属分析
- 主实体：`_______` → 所属服务：`_______`

### 边界规则适用
- 适用规则：`_______`
- 判断过程：

### 方案对比

| 维度 | 方案 A: 归属 `___` | 方案 B: 归属 `___` |
|------|-------------------|-------------------|
| 数据就近 | | |
| 事务边界 | | |
| 职责内聚 | | |
| 额外调用 | | |
| 未来风险 | | |

## 最终决定
- **归属服务**：
- **理由**：
- **决策日期**：
- **决策人**：

## 关联决策
- 是否有类似先例？（见 [[reference/historical-decisions]]）
```

## 这个 Skill 的意义是什么

回到你的问题——Skill 的意义不在于"AI 帮你决定 API 放哪"。意义在于：

1. **避免每次重新争论**。团队对"这个 API 放哪个服务"的争论，本质上是各自脑中的服务地图不一致。把地图写下来（service-map.md），争论就变成了查地图。

2. **决策可追溯**。半年后有人问"为什么这个 API 在 order-service 而不是 payment-service"——查 historical-decisions.md，有记录、有 trade-off 分析。

3. **新人不需要 oral tradition**。新同事不需要逮着老员工问"咱们有哪些服务、各自管什么数据"——service-map.md 就是答案。

4. **Claude 的上下文补齐**。Claude 不知道你的系统长什么样。没有 service-map.md，它只能给泛泛的建议（"建议遵循微服务原则……"）；有了 service-map.md，它可以针对你的具体服务给出具体判断。

**本质**：这不是一个"判断型" Skill（AI 替你做决策），这是一个**"知识检索型" Skill**——在正确的时刻，把正确的那一页服务地图翻给 AI 和人类看。

## 当前状态

- [ ] 需要画实际项目的服务地图（目前示例是电商，需要替换）
- [ ] boundary-rules 中的灰色地带案例需要团队补充真实案例
- [ ] historical-decisions 目前为空，需要填入团队真实的历史决策
- [ ] 如果有 API 文档仓库（OpenAPI/Swagger），可以考虑让 Skill 自动对比 API 实际位置和 service-map 是否一致
- [ ] 这个 Skill 和 CLAUDE.md 的关系——建议在 CLAUDE.md 中加一行 `新 API 归属判断见 api-placement Skill`

## 两种 Skill 的对比

| 维度 | SQL 索引优化 | 微服务 API 归属 |
|------|------------|---------------|
| 类型 | 技术优化型 | 架构决策型 |
| AI 自主性 | 高（Claude 懂索引） | 低（需要项目上下文） |
| Skill 价值来源 | 项目规范 + 反模式 | 服务地图 + 历史决策 |
| reference/ 重要性 | 重要（规范、坑） | **至关重要**（没有服务地图 = 没有判断依据） |
| 输出 | 优化方案（可执行 DDL） | 决策记录（可追溯） |
