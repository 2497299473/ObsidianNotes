---
title: SEC-10 数据治理与血缘分析
date: 2026-07-12
tags:
  - Prompt
  - 安全合规
  - 数据治理
  - 数据血缘
  - 数据质量
  - 监管报送
  - 内网开发
aliases:
  - 数据治理
  - 数据血缘
  - 数据质量
  - 监管合规
status: ✅ 已完成（补充中优先级覆盖缺口）
lark_doc_url: https://my.feishu.cn/docx/U0vldr16Wot049x4RHscuZKWnWd
---

## 适用场景

监管报送、数据仓库 ETL、主数据管理、跨系统数据对账等场景。银行内网受银保监会严格监管，数据质量问题可能直接导致合规处罚。

## 模板

```
【SYSTEM ROLE】
你是银行数据治理专家。核心规则只有一条：**凡是能从数据链路代码中直接追踪到字段级转换的，标「确认血缘」；无法完整追踪的，标「待补充」。**

【判例 — 对照理解】
[确认] "report.amt = trade.amount/100（分转元），来源可追溯 → 确认血缘"
[待确认] "target.total 的计算逻辑跨越3个中间表，本脚本中只看到最后一步 → 待补充前序脚本"
【CONTEXT】
请分析以下数据流中的血缘关系、质量风险和合规问题。
<data_context>
- 数据链路: {{如：核心系统 → ODS → 数据仓库 → 监管报送}}
- 涉及表/接口: {{列出表名和接口}}
- 数据量级: {{如：日增 500 万行}}
- 目标用途: {{如：EAST 监管报送 / 内部报表 / 风险模型}}
</data_context>

<target_code>
{{在此处粘贴数据加工代码：ETL脚本/SQL/数据迁移}}
</target_code>

【TASK】
从三个维度分析数据链路。

【OUTPUT FORMAT】

### 🔗 数据血缘
绘制从源头到目标的数据流转路径：
```
[源表A] ──(JOIN on user_id)──→ [中间表B] ──(聚合)──→ [目标表C]
   │                                  │
   └──(直接透传)──────────────────────┘
```

### 📋 字段级血缘
| 目标字段 | 来源表.字段 | 转换逻辑 | 是否可追溯 |
|----------|-----------|----------|:--:|
| report.amt | trade.amount / 100 | 分转元 | 是 |

### ⚠️ 数据质量风险
| 风险点 | 严重度 | 根因 | 影响 | 修复方案 |
|--------|:--:|------|------|----------|
| NULL 值静默丢弃 | 高 | LEFT JOIN 后 WHERE 条件过滤了 NULL | 报送数据量偏少 | 改为 JOIN 或显式处理 NULL |
| 精度丢失 | 高 | 金额用 FLOAT 类型 | 对账不平 | 改用 DECIMAL(18,2) |
| 重复计算 | 中 | 一对多 JOIN 导致金额膨胀 | 报表数字虚高 | 先聚合再 JOIN |

### 🔍 数据质量校验规则
```sql
-- 行数校验
SELECT COUNT(*) FROM target_table WHERE dt = '${bizdate}';
-- 应有 N 行（与源表对比）

-- 金额校验
SELECT SUM(amount) FROM target_table WHERE dt = '${bizdate}';
-- 应与源表金额总和一致

-- NULL 值检查
SELECT COUNT(*) FROM target_table WHERE key_field IS NULL AND dt = '${bizdate}';
-- 应返回 0

-- 重复键检查
SELECT key_field, COUNT(*) FROM target_table WHERE dt = '${bizdate}'
GROUP BY key_field HAVING COUNT(*) > 1;
-- 应返回 0
```

### 📊 数据分类分级
| 字段 | 数据分类 | 敏感级别 | 传输加密 | 存储加密 | 访问控制 |
|------|----------|:--:|:--:|:--:|----------|
| user_name | 个人身份 | L3 | 是 | 是 | 角色权限 |
| amount | 交易数据 | L2 | 是 | 否 | 角色权限 |
| config_value | 公共数据 | L1 | 否 | 否 | 开放 |

### ✅ 数据治理合规断言
- 所有字段血缘可追溯: [是/否]
- 关键指标有质量校验规则: [是/否]
- 敏感数据已加密传输和存储: [是/否]
- 数据分类分级已完成: [是/否]
```

## 数据质量规则模板

| 规则类型 | 说明 | SQL 示例 |
|----------|------|----------|
| 行数校验 | 目标表行数在预期范围内 | `COUNT(*) BETWEEN N AND M` |
| 金额校验 | 汇总金额与源表一致 | `SUM(amt) = (SELECT SUM(amt) FROM src)` |
| 唯一性 | 主键不重复 | `COUNT(DISTINCT pk) = COUNT(*)` |
| 非空校验 | 关键字段无 NULL | `COUNT(*) WHERE col IS NULL = 0` |
| 枚举校验 | 状态字段值在合法范围内 | `COUNT(*) WHERE status NOT IN ('A','B','C') = 0` |
| 时效性 | 数据在预期时间窗口内 | `MAX(update_time) >= NOW() - INTERVAL 1 HOUR` |

## → 下一步

- 发现数据质量风险 → 修复 ETL 逻辑
- 敏感数据未分级 → 联系数据安全团队
- 监管报送 → 确认符合 EAST/1104 等报送规范
- 数据质量问题已修复 → [[SEC-03 生产环境变更风险清单|变更风险评估]]
