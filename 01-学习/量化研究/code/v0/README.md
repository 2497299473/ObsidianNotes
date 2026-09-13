# v0 · 数据基线：行情获取 → PostgreSQL 落库 → 质量检查

量化实验版本线的第一步（见[[01-学习/跨技术栈学习路径总览|总览]]的 v0→v10）：不求模型，只求**一条可信的日线数据底座**。

## 管道

```text
fetch_data.py ──→ data/raw_{symbol}_qfq_{date}.csv（数据集版本快照）
                        │
schema.sql + load_to_pg.py ──→ PostgreSQL: daily_prices（幂等，可重跑）
                        │
quality_check.py ──→ 质量报告（缺失/重复/OHLC/停牌/缺口/厚尾）
```

## 快速开始（本机实录路径，2026-09-06 验证通过）

```bash
# ── 数据库：WSL(Ubuntu 24.04) 内 apt 安装 PG16（阿里源，2.7MB/s；Docker Hub 不可达时的实录方案）
sudo sed -i 's|http://archive.ubuntu.com/ubuntu|https://mirrors.aliyun.com/ubuntu|g' /etc/apt/sources.list.d/ubuntu.sources
sudo apt update && sudo apt install -y postgresql-16 python3-pandas python3-psycopg2
# 改 /etc/postgresql/16/main/postgresql.conf: port=5433, listen_addresses='*'
# pg_hba.conf 追加: host all all all scram-sha-256
sudo pg_ctlcluster 16 main restart
sudo -u postgres psql -c "CREATE ROLE quant LOGIN PASSWORD 'quant' SUPERUSER"
sudo -u postgres createdb -O quant quant

# ── 管道（Windows 侧取数；落库/质检在 WSL 内跑，绕开端口转发问题）
python fetch_data.py --symbol 510300 --years 5
wsl -e sh -c "cd /mnt/d/Obsidian/My-First-Obsidian/01-学习/量化研究/code/v0 && QUANT_PG_HOST=localhost python3 load_to_pg.py"
wsl -e sh -c "cd /mnt/d/Obsidian/My-First-Obsidian/01-学习/量化研究/code/v0 && QUANT_PG_HOST=localhost python3 quality_check.py"
```

通用替代方案（网络通畅时）：`docker run -d --name quant-pg -e POSTGRES_USER=quant -e POSTGRES_PASSWORD=quant -e POSTGRES_DB=quant -p 5433:5432 postgres:18-alpine`，其余步骤相同（`pip install psycopg2-binary`）。

## 口径备忘

- **前复权（qfq）价格只是抓取日快照**：除权后全历史会被数据源重写 → 数据集版本 = 文件名里的抓取日。复现实验必须先冻结数据集版本。
- ⚠️ **但"冻结数据集版本" ≠ "Point-in-Time"**（Snapshot versioning ≠ PIT availability）。前复权会因未来公司行动重写历史，抓取日只能回答"本次实验用了哪份快照"，回答不了"模型在当时能知道什么"。严格 PIT 回测须改用后复权（hfq）或"原始价 + 复权因子表按生效日还原"，详见[[01-学习/量化研究/00-量化研究与回测学习路径总索引|量化研究]]阶段三。
- `volume` 为数据源（东财）口径的**手**；`amount` 为**元**；`pct_chg` 为**百分数**。
- 表主键 `(symbol, trade_date)` + `ON CONFLICT DO NOTHING` + staging TRUNCATE：任何一天重跑管道都不会产生重复行。
- 装载用 `COPY ... FROM STDIN`（psycopg2 `copy_expert`）：千行级数据秒级完成，百万行级依然适用——衔接 PG 笔记的批量装载主题。
