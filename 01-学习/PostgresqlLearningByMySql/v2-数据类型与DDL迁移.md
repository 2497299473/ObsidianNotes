---
title: v2-数据类型与DDL迁移
stage: 毕业项目
order: 101
difficulty: ⭐⭐⭐
created: 2026-07-24
tags:
  - PostgreSQL
  - MySQL
  - 毕业项目
  - 数据类型
  - JSONB
  - 数组
description: 毕业项目 v2：在 v1 基础上迁移复杂数据类型——JSONB、数组、ENUM、TIMESTAMPTZ、CHECK 约束、触发器时间戳。
lark_doc_url: https://my.feishu.cn/docx/ZtyydTWl8ojyjLx6y7ec1bUIn8e
---

## 毕业项目 v2：数据类型与 DDL 迁移

**改造维度**：类型迁移 + 建表 + 约束
**MySQL 锚点**：INT/AUTO_INCREMENT/JSON/ENUM/DATETIME → SERIAL/JSONB/CREATE TYPE/TIMESTAMPTZ
**验收标准**：DDL 完整迁移 + 约束对齐 + JSONB 查询可用

### 目标

在 v1 的基础表结构上，引入 JSONB、数组、ENUM、CHECK 约束和触发器，完成完整的 DDL 迁移。

### 任务清单

- [ ] 创建 ENUM 类型（替代 MySQL 内联 ENUM）
- [ ] 迁移 JSON 列为 JSONB + GIN 索引
- [ ] 用数组替代 MySQL 的逗号分隔标签
- [ ] 用 CHECK 约束替代 UNSIGNED
- [ ] 创建触发器实现 ON UPDATE CURRENT_TIMESTAMP
- [ ] 完成 JSONB 包含查询和数组查询

### 完整代码

```sql
\c graduation_db

-- ============================================
-- Step 1：创建 ENUM 类型（PG 需先创建类型）
-- ============================================
-- MySQL: status ENUM('draft','published','soldout')
CREATE TYPE product_status AS ENUM ('draft', 'published', 'soldout');
CREATE TYPE order_status AS ENUM ('pending', 'paid', 'shipped', 'cancelled');

-- ============================================
-- Step 2：创建商品表（JSONB + 数组 + CHECK）
-- ============================================
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    -- MySQL JSON → PG JSONB（支持 GIN 索引）
    specs JSONB NOT NULL DEFAULT '{}',
    price NUMERIC(10, 2) NOT NULL CHECK (price > 0),  -- CHECK 替代业务校验
    stock INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0),  -- 替代 UNSIGNED
    status product_status DEFAULT 'draft',
    -- MySQL: tags VARCHAR(255) 存逗号字符串 → PG: TEXT[] 数组
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- GIN 索引：加速 JSONB 包含查询
CREATE INDEX idx_products_specs ON products USING GIN (specs);
-- GIN 索引：加速数组包含查询
CREATE INDEX idx_products_tags ON products USING GIN (tags);
-- 部分索引：只索引在售商品
CREATE INDEX idx_products_active ON products (name) WHERE is_active = true;

-- ============================================
-- Step 3：升级 orders 表（使用 ENUM 状态）
-- ============================================
DROP TABLE IF EXISTS orders;
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    status order_status DEFAULT 'pending',
    total_amount NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES orders(id),
    product_id INT NOT NULL REFERENCES products(id),
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(10, 2) NOT NULL
);

-- ============================================
-- Step 4：触发器实现 ON UPDATE CURRENT_TIMESTAMP
-- ============================================
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_products_updated
    BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trigger_orders_updated
    BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- ============================================
-- Step 5：插入测试数据
-- ============================================
INSERT INTO products (name, specs, price, stock, status, tags) VALUES
('iPhone 15',
    '{"brand":"Apple","colors":["black","white"],"specs":{"ram":"6GB","storage":"128GB"}}',
    7999, 100, 'published', ARRAY['phone','premium','apple']),
('Galaxy S24',
    '{"brand":"Samsung","colors":["blue"],"specs":{"ram":"8GB","storage":"256GB"}}',
    5999, 50, 'published', ARRAY['phone','android']),
('iPad Air',
    '{"brand":"Apple","colors":["gray"],"specs":{"ram":"4GB","storage":"64GB"}}',
    3999, 30, 'draft', ARRAY['tablet','apple']);

INSERT INTO orders (user_id, status, total_amount) VALUES
(1, 'paid', 7999), (2, 'pending', 5999);

INSERT INTO order_items (order_id, product_id, quantity, unit_price) VALUES
(1, 1, 1, 7999), (2, 2, 1, 5999);

-- ============================================
-- Step 6：JSONB 查询（走 GIN 索引）
-- ============================================
-- 包含查询：品牌为 Apple
SELECT name, price FROM products
WHERE specs @> '{"brand":"Apple"}';

-- 键存在查询
SELECT name FROM products WHERE specs ? 'colors';

-- 嵌套路径提取
SELECT name, specs#>>'{specs,ram}' AS ram FROM products;

-- 数组包含查询（走 GIN 索引）
SELECT name FROM products WHERE tags @> ARRAY['apple'];

-- 数组展开为多行
SELECT name, unnest(tags) AS tag FROM products;

-- ============================================
-- Step 7：UPSERT 实战（幂等写入）
-- ============================================
-- ON CONFLICT (name) 要求 name 上有唯一约束/索引，否则报错：
-- "there is no unique or exclusion constraint matching the ON CONFLICT specification"
CREATE UNIQUE INDEX IF NOT EXISTS idx_products_name ON products(name);

INSERT INTO products (name, specs, price, stock, tags)
VALUES ('iPhone 15', '{"brand":"Apple","new":true}', 7999, 150, ARRAY['phone'])
ON CONFLICT (name) DO UPDATE
    SET stock = products.stock + EXCLUDED.stock,
        specs = products.specs || EXCLUDED.specs
RETURNING id, name, stock, (xmax = 0) AS is_insert;
-- 首次：is_insert=true（新增）；再次：is_insert=false（更新，stock 累加）

-- ============================================
-- 验证触发器
-- ============================================
SELECT pg_sleep(1);
UPDATE products SET price = 6999 WHERE name = 'iPhone 15';
SELECT name, price, updated_at FROM products WHERE name = 'iPhone 15';
-- updated_at 应该自动更新为更晚的时间
```

### 验收标准

- [ ] ENUM 类型创建成功并在表中使用
- [ ] JSONB GIN 索引创建成功，包含查询走索引
- [ ] 数组查询和展开操作正确
- [ ] CHECK 约束生效（插入负数 stock 会报错）
- [ ] 触发器自动更新 updated_at
- [ ] UPSERT 幂等写入正确区分插入和更新

---

## 相关笔记

- ⬅️ 前置：[[v1-环境搭建与基础查询迁移]]
- ➡️ 后续：[[v3-查询与索引优化]]
- 🔗 关联：[[02-数据类型与DDL对比]] | [[03-SQL查询语法差异与进阶]]
- 🔗 实战场景：[[08-业务场景实战合集]] 场景 1（电商订单 JSONB）· 场景 2（CMS 标签数组）

---
*最后更新：2026-07-24*