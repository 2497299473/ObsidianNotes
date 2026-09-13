---
title: SQL 索引优化 Skill 构想
date: 2026-06-21
tags:
  - skills
  - claude-code
  - postgresql
  - sql
  - 索引优化
  - 构想
aliases:
  - sql-index-optimization
status: 🌱 构想中
lark_doc_url: https://my.feishu.cn/docx/B3PndXVOOogyb7xp7ZocHpfFnsF
lark_doc_token: B3PndXVOOogyb7xp7ZocHpfFnsF
---

## 做什么

帮助 Claude 对项目中的慢查询和常用 SQL 提出索引优化建议。Claude 本身懂索引原理、会读 EXPLAIN——Skill 不是教它 SQL，而是提供**这个项目的具体上下文**，让它从"通用建议"升级为"符合团队规范的专家建议"。

## 核心判断

> 重点应该是：要给哪些表加索引。至于怎么加索引，数据库早就想好了，EXPLAIN 最基础了。

对。Skill 的价值不在"索引怎么建"（那是 Claude 自己的知识），而在：

1. **哪些查询需要关注**——从项目里提取高频 SQL、慢查询日志
2. **团队约定**——这个项目对索引数量、命名、类型的约束
3. **历史踩坑**——"别在 JSON 字段上建 B-tree"这类具体教训

## Skill 结构

```
.claude/skills/sql-index-tuning/
├── SKILL.md              ← 流程骨架：怎么收集 SQL、怎么读 EXPLAIN、怎么输出建议
├── reference/
│   ├── index-conventions.md   ← 项目索引规范（命名、类型选择、数量上限）
│   ├── anti-patterns.md       ← 团队踩过的索引坑
│   ├── core-tables.md         ← 核心表结构速查（哪些表量大、哪些表高写入）
│   └── historical-decisions.md ← 历史决策记录（为什么在这个字段加了这个索引）
└── templates/
    └── tuning-report.md       ← 优化建议输出模板
```

## SKILL.md（流程骨架）

```yaml
---
name: sql-index-tuning
description: Analyze slow queries and recommend index optimizations for this project. Understands the project's table structures, index conventions, and past performance decisions. Use when the user asks about query performance, slow SQL, index design, or EXPLAIN results.
allowed-tools:
  - Read
  - Grep
  - Glob
  - Bash(psql:*)     # 如果有数据库访问权限
---

# SQL Index Tuning

## 工作流程

1. **收集目标 SQL**
   - 从项目代码中搜索高频查询路径
   - 如果用户提供了慢查询日志，解析并提取
   - 识别 TOP N 查询（按频率 × 耗时排序）

2. **获取 EXPLAIN 结果**
   - 对每条目标 SQL 执行 EXPLAIN (ANALYZE, BUFFERS)
   - 如果没有数据库连接，让用户提供 EXPLAIN 输出

3. **分析 + 建议**
   - 对照 [[reference/core-tables]] 了解表规模和高写入特征
   - 对照 [[reference/index-conventions]] 确保建议符合命名和类型规范
   - 对照 [[reference/anti-patterns]] 避免推荐已知坑
   - 优先级排序：高频大表 > 低频大表 > 高频小表

4. **输出**
   - 按 [[templates/tuning-report]] 模板输出
   - 每个建议附带：影响的查询、预估收益、风险（写入开销）
```

正文就这么短——剩下的价值全在 reference/。

## reference/index-conventions.md（示例）

```markdown
# 索引规范

## 命名约定
- 普通索引：`idx_{table}_{column}`
- 唯一索引：`udx_{table}_{column}`
- 复合索引：`idx_{table}_{col1}_{col2}`
- 部分索引：`idx_{table}_{column}_where_{condition}`

## 类型选择
| 场景 | 索引类型 |
|------|---------|
| 等值 + 范围查询 | B-tree（默认） |
| JSON 字段查询 | **GIN**（B-tree 对 JSON 几乎无效） |
| 全文搜索 | GIN + tsvector |
| 几何/地理 | GiST |
| 精确匹配 + 高并发 | Hash（仅 `=` 操作） |

## 数量约束
- 单表索引数不超过 **5 个**（OLTP 高写入场景）
- 每个索引必须有明确的业务 SQL 对应——不允许"预防性建索引"
- 复合索引列数不超过 **3 列**

## 核心表写入特征
| 表 | 日均写入 | 可接受索引数上限 |
|----|---------|----------------|
| orders | ~10 万 | 3 |
| users | ~1000 | 5 |
| logs | ~500 万 | 1（仅时间索引） |
```

## reference/anti-patterns.md（示例）

```markdown
# 索引反模式

## 已知坑（团队教训）

### JSON 字段 + B-tree
❌ 在 `extra_data` JSON 字段上建了 B-tree 索引，查询永远不走索引
✅ 用 GIN 索引 `CREATE INDEX idx_orders_extra ON orders USING GIN (extra_data)`

### 低基数列索引
❌ 在 `status`（只有 3 个值）上单独建索引，查询优化器选全表扫描
✅ 如果必须查 status，放在复合索引的第二列

### 隐式类型转换导致索引失效
❌ `WHERE user_id = '123'`（user_id 是 int 类型）
✅ `WHERE user_id = 123`

### OR 条件导致索引失效
❌ `WHERE a = 1 OR b = 2`
✅ 改写为 UNION ALL 两个独立查询
```

## templates/tuning-report.md（示例）

```markdown
# 索引优化建议报告

## 分析范围
- 分析了 X 条查询，其中 Y 条存在优化空间
- 目标表：orders (120 万行), users (50 万行)

## 建议汇总

| 优先级 | 表 | 建议索引 | 影响查询 | 预估收益 | 写入代价 |
|--------|---|---------|---------|---------|---------|
| P0 | orders | `idx_orders_user_created` | 用户订单列表 | -90% 扫描行 | 中（日均 10 万写入） |
| P1 | ... | ... | ... | ... | ... |

## 详细分析

### P0: orders 表加 `idx_orders_user_created`

**原始 SQL**：
SELECT * FROM orders WHERE user_id = ? ORDER BY created_at DESC LIMIT 20;

**EXPLAIN 结果**：
Seq Scan on orders (cost=0.00..4500.00 rows=50 width=200)
  Filter: (user_id = 12345)

**建议**：
CREATE INDEX idx_orders_user_created ON orders (user_id, created_at DESC);

**预期效果**：扫描行数从 120 万降到约 200，查询耗时 -95%

**风险**：orders 日均写入 10 万，复合索引额外开销约 5% 写入性能，可接受
```

## 和 MCP 的配合

如果项目接了数据库 MCP（Model Context Protocol），Skill 的工作流可以更自动化：

```
用户: "帮我优化 orders 表的查询"
→ Claude 通过 MCP 直接 EXPLAIN
→ 对照 reference/ 中的表特征和规范
→ 输出 tuning-report
```

不需要用户手动贴 SQL 和 EXPLAIN 结果。这个在后续课程（[[Claude的使用/09｜触类旁通：SKILL.md 结构与触发机制|第 9 讲]] 也提到了 MCP 方向）。

## 当前状态

- [ ] 需要整理项目实际的慢查询日志
- [ ] 需要从代码中提取高频 SQL（可以用 Grep 搜 ORM 调用点）
- [ ] reference/ 中的反模式清单需要团队补充真实踩坑记录
- [ ] 如果数据库是 MySQL，GIN 不适用，需要调整类型建议
- [ ] 索引数量上限（单表 5 个）这个数字需要根据实际负载确认
