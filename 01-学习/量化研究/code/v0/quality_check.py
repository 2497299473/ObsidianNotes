"""v0 数据基线 · 第 4 步：数据质量检查（CSV 为准，PG 行数为交叉验证）。

检查项对应量化研究阶段一：缺失 / 重复 / OHLC 违规 / 停牌嫌疑 /
长假与缺口 / 收益分布形态（偏度峰度 → 厚尾证据，衔接统计学 01）。

⚠️ 能力边界：本脚本查的是"数据本身是否干净"，不是"数据是否可用于回测"。
全绿 PASS ≠ 回测无偏——它查不出前复权带来的复权信息前视泄漏、成分股
survivorship bias、公告/财报发布日错配。这些属于 Point-in-Time /
Universe / Corporate Action 检查，需要多标的与历史成分数据，排进 v3/v4
（单 ETF 的 v0 阶段无从验证）。
"""
import os
import pathlib
import sys

import numpy as np
import pandas as pd

COLS = ["symbol", "trade_date", "open", "high", "low", "close", "volume", "amount", "pct_chg"]


def skew_kurt(x):
    m, s = x.mean(), x.std(ddof=1)
    return ((x - m) ** 3).mean() / s ** 3, ((x - m) ** 4).mean() / s ** 4 - 3


def main(csv_path: pathlib.Path) -> int:
    df = pd.read_csv(csv_path, parse_dates=["trade_date"])
    fail = []

    print(f"== v0 数据质量检查: {csv_path.name} ==")
    print(f"rows={len(df)}  range={df.trade_date.min().date()} ~ {df.trade_date.max().date()}")

    dup = int(df.duplicated(["symbol", "trade_date"]).sum())
    print(f"[{'FAIL' if dup else 'PASS'}] 重复 (symbol,trade_date): {dup}")
    if dup:
        fail.append("duplicates")

    miss = df[COLS].isna().sum()
    miss = miss[miss > 0]
    print(f"[{'FAIL' if len(miss) else 'PASS'}] 缺失值: {dict(miss) if len(miss) else '无'}")
    if len(miss):
        fail.append("missing")

    bad_ohlc = int(((df.high < df.low)
                    | (df.high < df[["open", "close"]].max(axis=1))
                    | (df.low > df[["open", "close"]].min(axis=1))).sum())
    print(f"[{'FAIL' if bad_ohlc else 'PASS'}] OHLC 违规 (high<low 等): {bad_ohlc}")
    if bad_ohlc:
        fail.append("ohlc")

    zero_vol = int((df.volume == 0).sum())
    print(f"[{'WARN' if zero_vol else 'PASS'}] 零成交天数（停牌嫌疑）: {zero_vol}")

    gaps = df.trade_date.diff().dt.days
    long_gaps = gaps[gaps > 5]
    print(f"[INFO] >5 自然日间隔（长假/停牌）: {len(long_gaps)} 段；最长 {int(gaps.max())} 天")
    if len(long_gaps):
        for idx, g in long_gaps.sort_values(ascending=False).head(3).items():
            print(f"       {df.trade_date[idx].date()} 前，间隔 {int(g)} 天")

    lr = np.log(df.close / df.close.shift(1)).dropna()
    sk, ku = skew_kurt(lr.values)
    print(f"[INFO] 日对数收益: mean={lr.mean():.5f} std={lr.std(ddof=1):.4f} "
          f"skew={sk:.2f} kurt={ku:.1f}")
    print(f"[INFO] 5% 分位={np.percentile(lr, 5):.4f}  单日最大跌幅={lr.min():.4f}")
    print(f"[{'WARN' if ku > 3 else 'INFO'}] 超额峰度 {ku:.1f} → "
          f"{'明显厚尾：VaR/检验避开正态假设（统计学01坑2）' if ku > 3 else '厚尾不明显'}")

    try:
        import psycopg2
        with psycopg2.connect(host=os.getenv("QUANT_PG_HOST", "localhost"),
                              port=int(os.getenv("QUANT_PG_PORT", "5433")),
                              dbname=os.getenv("QUANT_PG_DB", "quant"),
                              user=os.getenv("QUANT_PG_USER", "quant"),
                              password=os.getenv("QUANT_PG_PASSWORD", "quant")) as conn:
            with conn.cursor() as cur:
                cur.execute("SELECT count(*), min(trade_date), max(trade_date) FROM daily_prices")
                pg_total, pg_min, pg_max = cur.fetchone()
        ok = pg_total >= len(df)
        print(f"[{'PASS' if ok else 'FAIL'}] PG 交叉验证: 表内 {pg_total} 行"
              f"（{pg_min}~{pg_max}） vs CSV {len(df)} 行")
        if not ok:
            fail.append("pg_mismatch")
    except Exception as exc:
        print(f"[SKIP] PG 交叉验证不可用: {type(exc).__name__}")

    print(f"== 结论: {'FAIL: ' + ','.join(fail) if fail else 'PASS（v0 数据基线达标）'} ==")
    return 1 if fail else 0


if __name__ == "__main__":
    here = pathlib.Path(__file__).parent
    path = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else sorted((here / "data").glob("raw_*.csv"))[-1]
    sys.exit(main(path))
