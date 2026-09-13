-- v0 行情表：主键 (symbol, trade_date) 天然防重复导入
-- BRIN 索引：时序数据按日期近似有序，BRIN 体积远小于 B-tree（衔接 PG 笔记 04 的时序优化）
CREATE TABLE IF NOT EXISTS daily_prices (
    symbol     TEXT           NOT NULL,
    trade_date DATE           NOT NULL,
    open       NUMERIC(12, 4),
    high       NUMERIC(12, 4),
    low        NUMERIC(12, 4),
    close      NUMERIC(12, 4),
    volume     BIGINT,               -- 数据源（东财）口径为"手"
    amount     NUMERIC(18, 2),       -- 成交额（元）
    pct_chg    NUMERIC(10, 6),       -- 涨跌幅（%，数据源口径）
    PRIMARY KEY (symbol, trade_date)
);

CREATE INDEX IF NOT EXISTS idx_daily_prices_trade_date
    ON daily_prices USING brin (trade_date);

-- 装载中转表：无约束，COPY 高速写入后幂等 Merge 进主表
CREATE UNLOGGED TABLE IF NOT EXISTS daily_prices_stage (
    LIKE daily_prices INCLUDING DEFAULTS
);
