"""v0 数据基线 · 第 1 步：获取日线行情，落地 CSV 快照。

用法:
    python fetch_data.py --symbol 510300 --years 5
    python fetch_data.py --symbol 510300 --years 5 --synthetic   # 离线演示

输出:
    data/raw_{symbol}_qfq_{yyyymmdd}.csv —— 文件名即"数据集版本号"（含复权方式与抓取日）

设计要点（对应量化研究阶段一/三）:
    * 前复权(qfq)因子随未来除权变动 → 复权价只是"当次抓取的快照"，
      这就是文件名带抓取日的原因。
    * ⚠️ 文件名带抓取日 = **数据集版本冻结 / 可复现性**的最小实现，
      **不等于 Point-in-Time**（Snapshot versioning ≠ PIT availability）。
      前复权会因未来的公司行动重写历史价格：2026 年抓出来的 2024-01-01 价格，
      与 2024-01-02 当时看到的价格可能不同。抓取日只能回答"本次实验用的是
      哪份快照"，回答不了"模型在当时到底能知道什么"。
      严格 PIT 回测须改用后复权(hfq)或"原始价 + 复权因子表按生效日还原"，
      详见量化研究阶段三。
    * pct_chg 保留数据源口径（百分数，1.23 表示 +1.23%）。
"""
import argparse
import datetime as dt
import pathlib
import sys

import numpy as np
import pandas as pd

HERE = pathlib.Path(__file__).parent
COLS = ["trade_date", "open", "high", "low", "close", "volume", "amount", "pct_chg"]


def fetch_akshare(symbol: str, years: int, adjust: str = "qfq") -> pd.DataFrame:
    import akshare as ak

    end = dt.date.today()
    start = end.replace(year=end.year - years)
    df = ak.fund_etf_hist_em(
        symbol=symbol, period="daily",
        start_date=start.strftime("%Y%m%d"),
        end_date=end.strftime("%Y%m%d"), adjust=adjust,
    )
    df = df.rename(columns={
        "日期": "trade_date", "开盘": "open", "收盘": "close",
        "最高": "high", "最低": "low", "成交量": "volume",
        "成交额": "amount", "涨跌幅": "pct_chg",
    })
    df = df[COLS].copy()
    df["trade_date"] = pd.to_datetime(df["trade_date"]).dt.date
    return df


def fetch_synthetic(symbol: str, years: int) -> pd.DataFrame:
    """离线兜底：几何随机游走，仅用于跑通管道，不用于任何研究结论。"""
    rng = np.random.default_rng(42)
    end = dt.date.today()
    dates = pd.bdate_range(start=end.replace(year=end.year - years), end=end)
    n = len(dates)
    close = 4.0 * np.exp(np.cumsum(rng.normal(0.0003, 0.012, size=n)))
    open_ = close * np.exp(rng.normal(0, 0.003, size=n))
    high = np.maximum(open_, close) * (1 + np.abs(rng.normal(0, 0.004, size=n)))
    low = np.minimum(open_, close) * (1 - np.abs(rng.normal(0, 0.004, size=n)))
    pct = np.concatenate([[np.nan], (close[1:] / close[:-1] - 1) * 100])
    return pd.DataFrame({
        "trade_date": dates.date, "open": open_, "high": high, "low": low,
        "close": close, "volume": rng.integers(5e7, 2e8, size=n),
        "amount": close * rng.integers(5e7, 2e8, size=n),
        "pct_chg": pct,
    })


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--symbol", default="510300", help="ETF 代码，如 510300（沪深300ETF）")
    ap.add_argument("--years", type=int, default=5)
    ap.add_argument("--synthetic", action="store_true", help="强制使用离线模拟数据")
    args = ap.parse_args()

    source = "akshare"
    if args.synthetic:
        df = fetch_synthetic(args.symbol, args.years)
        source = "synthetic"
    else:
        try:
            df = fetch_akshare(args.symbol, args.years)
        except Exception as exc:  # 网络失败自动降级，并在输出里大声说明
            print(f"[warn] akshare 获取失败（{type(exc).__name__}: {exc}），降级为 synthetic 模拟数据",
                  file=sys.stderr)
            df = fetch_synthetic(args.symbol, args.years)
            source = "synthetic(fallback)"

    df.insert(0, "symbol", args.symbol)
    out = HERE / "data" / f"raw_{args.symbol}_qfq_{dt.date.today():%Y%m%d}.csv"
    out.parent.mkdir(exist_ok=True)
    df.to_csv(out, index=False)
    print(f"source={source} rows={len(df)} range={df['trade_date'].min()}~{df['trade_date'].max()}")
    print(f"saved -> {out}")


if __name__ == "__main__":
    main()
