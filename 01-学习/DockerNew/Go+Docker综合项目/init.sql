-- PostgreSQL 初始化 SQL
-- 对应 DockerNew 第 6 章：数据库容器化 + Volume 持久化
-- 容器启动时自动执行 docker-entrypoint-initdb.d/ 下的 .sql 文件
-- 注意：此处在插入种子数据前先建表，兼容首次启动（此时 Go API 尚未运行、
-- GORM AutoMigrate 还没执行）；API 启动后的 AutoMigrate 遇到已存在的表是幂等的

-- 建表（列与 main.go 中 GORM Todo 模型对齐）
CREATE TABLE IF NOT EXISTS todos (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 插入示例 TODO 数据
INSERT INTO todos (title, done) VALUES
    ('学习 Docker 基础', false),
    ('构建第一个镜像', false),
    ('掌握 Compose 编排', false),
    ('生产安全加固', false),
    ('完成 Go+Docker 综合项目', false)
ON CONFLICT DO NOTHING;

-- 重置序列
SELECT setval(pg_get_serial_sequence('todos', 'id'), (SELECT MAX(id) FROM todos));
