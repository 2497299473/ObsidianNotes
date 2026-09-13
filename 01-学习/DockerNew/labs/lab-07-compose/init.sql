-- 对应 DockerNew 第 6 章：Volume 持久化 + 数据库容器化
-- 容器启动时自动执行 docker-entrypoint-initdb.d/ 下的 .sql 文件
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    done BOOLEAN DEFAULT FALSE
);

-- 插入种子数据
INSERT INTO todos (title, done) VALUES
    ('学习 Docker Compose', false),
    ('完成 Lab 07', false)
ON CONFLICT DO NOTHING;
