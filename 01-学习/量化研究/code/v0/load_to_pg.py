"""v0 数据基线 · 第 3 步：CSV 快照 → PostgreSQL（staging COPY + 幂等 Merge）。

环境变量（均有默认值）: QUANT_PG_HOST / QUANT_PG_PORT / QUANT_PG_DB / QUANT_PG_USER / QUANT_PG_PASSWORD
依赖: pip install psycopg2-binary
"""
import io
import os
import pathlib
import sys

import pandas as pd
import psycopg2

HERE = pathlib.Path(__file__).parent
COPY_COLS = "symbol,trade_date,open,high,low,close,volume,amount,pct_chg"

DSN = dict(
    host=os.getenv("QUANT_PG_HOST", "localhost"),
    port=int(os.getenv("QUANT_PG_PORT", "5433")),
    dbname=os.getenv("QUANT_PG_DB", "quant"),
    user=os.getenv("QUANT_PG_USER", "quant"),
    password=os.getenv("QUANT_PG_PASSWORD", "quant"),
)


def load(csv_path: pathlib.Path) -> None:
    df = pd.read_csv(csv_path)
    buf = io.StringIO()
    df.to_csv(buf, index=False, columns=COPY_COLS.split(","))
    schema_sql = (HERE / "schema.sql").read_text(encoding="utf-8")

    with psycopg2.connect(**DSN) as conn:
        with conn.cursor() as cur:
            cur.execute(schema_sql)
            cur.execute("TRUNCATE daily_prices_stage")
            buf.seek(0)
            cur.copy_expert(
                f"COPY daily_prices_stage ({COPY_COLS}) FROM STDIN WITH (FORMAT csv, HEADER true)",
                buf,
            )
            cur.execute(
                "INSERT INTO daily_prices SELECT * FROM daily_prices_stage "
                "ON CONFLICT (symbol, trade_date) DO NOTHING"
            )
            inserted = cur.rowcount
            cur.execute("SELECT count(*), min(trade_date), max(trade_date) FROM daily_prices")
            total, dmin, dmax = cur.fetchone()
    print(f"csv_rows={len(df)} inserted={inserted} table_total={total} range={dmin}~{dmax}")


if __name__ == "__main__":
    data_dir = HERE / "data"
    path = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else sorted(data_dir.glob("raw_*.csv"))[-1]
    print(f"loading {path}")
    load(path)
