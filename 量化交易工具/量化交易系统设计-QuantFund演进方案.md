---
lark_doc_url: https://my.feishu.cn/docx/DhGad1S0Xovfr9xSq7mcLGwDnDf
lark_doc_token: DhGad1S0Xovfr9xSq7mcLGwDnDf
---
# 量化交易系统设计 · QuantFund 演进方案

> 设计日期：2026-08-09
> 基线：[[基金日频参谋-QuantFund项目笔记]] MVP（`D:\QuantFund\`，10/10 数据跑通）
> 理念参考：[[勇麦YMOS-投资操作系统-项目分析与结合方案]] · [[缠论知识库-Schema设计]]
> 部署：本机 WSL + Docker Compose · 推送：飞书群自定义机器人 Webhook
> 状态：✅ Phase 1 已部署（2026-08-09），三时点 dry-run/幂等/日历校验全部通过，待配置飞书 Webhook 实测

---

## 一、目标与边界

> [!info] 一句话目标
> 每个交易日的 **08:30（盘前）/ 12:00（盘中）/ 15:30（盘后）** 三个时点，自动向飞书推送一张基金操作参谋卡片，对池中每只基金给出 **买入 / 卖出 / 不动** 建议，建议必须基于当时的真实行情数据并附带评分与理由。

**做什么**

| 能力 | 说明 |
|:---|:---|
| 三时点定时推送 | 交易日日历校验 + 错过补跑 + 幂等防重 |
| 六维信号评分 | 沿用 MVP 的均线/动量/RSI/量能/回撤/分位评分引擎 |
| 买/卖/不动建议 | 总分映射为操作建议，附分数与关键理由 |
| 数据三级降级 | 腾讯 → akshare → yfinance，失败发告警不静默 |
| 全程留痕 | K线/信号/推送记录全部落 Postgres，可审计可复盘 |

**不做什么**

- ❌ 不自动交易、不接券商接口——延续 YMOS「AI 是副驾驶，不是自动驾驶」的边界，最终下单永远由人执行
- ❌ 不荐股——消息尾部固定免责声明，仅供个人研究参考
- ❌ v1 不接缠论信号——信号引擎做成可插拔维度，缠论作为第 7 维预留（见 Phase 3）

---

## 二、总体架构

```
┌─────────────────────────────────────────────────────────────┐
│ 调度层  APScheduler（app 容器内）                             │
│   cron 08:30 / 12:00 / 15:30（Asia/Shanghai，工作日触发）     │
│   → 交易日历校验 → Redis 互斥锁 → 幂等检查                    │
└──────────────────────────┬──────────────────────────────────┘
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ 数据层（Python，沿用 MVP 三级降级）                            │
│   腾讯行情(主) → akshare东财(备) → yfinance(兜底)              │
│   + 隔夜美股（纳指/标普指数）+ 交易日历（akshare 年表）         │
└──────┬─────────────────────────────────────┬────────────────┘
       ▼                                     ▼
┌─────────────────────┐          ┌─────────────────────────┐
│ Postgres             │          │ Redis                    │
│ K线/信号/推送日志/     │          │ K线缓存 · job互斥锁        │
│ 交易日历/持仓(Phase2) │          │ 推送限流 · 最新卡片缓存     │
└──────┬──────────────┘          └─────────────────────────┘
       ▼
┌─────────────────────────────────────────────────────────────┐
│ 策略层（Python）                                              │
│   六维评分引擎（可插拔 Dimension 接口，预留缠论第7维）           │
│   → 总分映射 buy/sell/hold → 盘中修正规则 → 生成卡片            │
└──────────────────────────┬──────────────────────────────────┘
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ 推送层（Python → Phase2 迁 Go）                               │
│   飞书群自定义机器人 Webhook + HMAC-SHA256 签名                │
│   聚合为单张卡片推送 · 失败重试3次 · 全失败发告警消息            │
└─────────────────────────────────────────────────────────────┘
```

```mermaid
flowchart LR
    A[定时触发] --> B{交易日历<br>今日开市?}
    B -- 否 --> Z[跳过并记录]
    B -- 是 --> C{Redis锁+<br>幂等检查}
    C -- 已推过 --> Z
    C -- 未推 --> D[拉取行情<br>三级降级]
    D -- 全部失败 --> E[飞书告警]
    D -- 成功 --> F[六维评分<br>生成建议]
    F --> G[渲染飞书卡片]
    G --> H[Webhook推送]
    H --> I[写push_logs<br>存档reports/]
```

---

## 三、三时点消息设计（核心）

> [!warning] 信号权威性分级
> 日频信号依赖日线收盘数据，**只有盘后 15:30 的建议是「日线定型版」**；盘前基于 T-1 日 K 线，盘中基于当日早盘快照，均为参考值。卡片上会明确标注数据基准时点，避免误读。

| 时点 | 市场状态 | 数据基准 | 消息定位 | 核心内容 |
|:---|:---|:---|:---|:---|
| **08:30 盘前** | 未开盘 | T-1 日 K 线已定型 + 隔夜美股 | 今日操作计划 | 六维信号（基于 T-1）→ 每只基金今日买/卖/不动；隔夜纳指/标普涨跌；跨境 ETF 溢价提示 |
| **12:00 盘中** | 午休（早盘 11:30 已收） | 当日早盘实况 | 盘中修正参考 | 早盘涨跌幅/量比快照；对盘前建议给出修正备注（见下方规则）；不动则维持盘前建议 |
| **15:30 盘后** | 已收盘 | 当日日 K 定型 | 权威信号 + 次日计划 | 全天数据重算六维评分 → 官方建议；与盘前方向不一致时标注「盘中反转」及原因；次日预案 |

### 盘中修正规则（config 驱动，示例）

| 盘前建议 | 早盘表现 | 盘中修正备注 |
|:---|:---|:---|
| 买入 | 跌 ≥ 2% | 「早盘回调，建议分批建仓勿一次打满」 |
| 买入 | 涨 ≥ 3% | 「短线过热，建议减半追入或等回踩」 |
| 卖出 | 涨 ≥ 3% | 「可趁早盘冲高减仓」 |
| 卖出 | 跌 ≥ 3% | 「避免恐慌杀跌，可等反抽再减」 |
| 不动 | 任意 | 维持盘前判断 |

### 卡片样例（盘前，富文本 post 格式）

```
📊 基金日频参谋 | 盘前参考 · 2026-08-10 周一
数据基准：2026-08-07 收盘 · 数据源：腾讯行情 ✅

🟢 买入（2）
 纳指ETF 513100　+40分 | 均线多头+动量强；RSI 76 略超买，勿追高
 标普500ETF 513500　+35分 | 均线全多头，隔夜标普 +0.8%

🟡 不动（8）
 上证50ETF 510050 +20分 | 沪深300ETF 510300 +16分 | 酒ETF 512690 +16分
 创业板ETF 159915 +7分 | 中证500ETF 510500 +5分 | 科创50ETF 588000 +2分
 证券ETF 512880 -1分 | 红利低波ETF 512890 -3分

🔴 卖出（0）—— 无

隔夜美股：纳指 +1.2% · 标普 +0.8%
提示：跨境 ETF 存在溢价，操作前请核对盘中溢价率

⚠️ 量化信号自动生成，仅供个人研究参考，不构成投资建议
```

---

## 四、信号引擎：六维 + 可插拔接口

评分逻辑沿用 MVP（不动），重构点是把每个维度抽象为插件，为缠论第 7 维预留位置：

```python
class Dimension(Protocol):
    name: str
    def score(self, df: pd.DataFrame, cfg: dict) -> tuple[int, str]:
        """返回 (分数, 理由摘要)"""

# v1 注册的维度（现有 signals.py 拆分而来）
DIMENSIONS = [MA(), Momentum(), RSI(), Volume(), Drawdown(), Quantile()]
# Phase 3 追加：ChanTheory()  ← 笔/段/中枢/买卖点，chan.py 计算
```

| 维度 | 逻辑 | 防追高 |
|:---|:---|:---|
| 均线 | 站上 MA(5/10/20/60) 数量 + 多空排列 | — |
| 动量 | 5/20/60 日收益率分段打分 | — |
| RSI(14) | 超卖+10 / 中性+5 / 超买 **-20** | ✅ |
| 量能 | 涨放量加分 / 跌放量扣分 | ✅ |
| 回撤 | 距 60 日高点回撤越深越加分 | ✅ |
| 分位 | 近 60 日价格分位 >80% 扣分 | ✅ |

**总分 → 建议映射**（阈值在 `config.json` 可调）：

| 总分 | 建议 |
|:---|:---|
| ≥ +30 | 🟢 买入 |
| ≤ -10 | 🔴 卖出 |
| 其余 | 🟡 不动 |

---

## 五、飞书推送设计

| 项 | 方案 |
|:---|:---|
| 通道 | 群自定义机器人 Webhook（用户侧 5 分钟配置，无需审批） |
| 安全 | 开启「签名校验」：`timestamp + "\n" + secret` 作 HMAC-SHA256 密钥签名，随请求带 timestamp+sign |
| 消息形式 | v1 用富文本 post（表格+emoji），Phase 2 可升级交互卡片（「已执行」打卡按钮，需自建应用） |
| 聚合策略 | 每个时点发 **一张** 聚合卡片（10 只基金合并），远低于机器人 100条/分钟 限流 |
| 重试 | HTTP 失败/非 code=0 重试 3 次（指数退避 1s/3s/9s） |
| 幂等 | `push_logs` 表 `UNIQUE(trade_date, slot)`，重复触发不会重复推送 |
| 告警 | 任务失败/数据全挂 → 向同群推送一条 ⚠️ 告警消息，**绝不静默** |

`.env` 配置（不入库）：

```
FEISHU_WEBHOOK=https://open.feishu.cn/open-apis/bot/v2/hooks/xxxx
FEISHU_SECRET=xxxx
POSTGRES_DSN=postgresql://quantfund:****@postgres:5432/quantfund
REDIS_URL=redis://redis:6379/0
```

---

## 六、存储设计

### Postgres 表结构（migrations/001_init.sql）

```sql
CREATE TABLE trade_calendar (
  trade_date DATE PRIMARY KEY,
  is_open    BOOLEAN NOT NULL
);

CREATE TABLE klines (
  code       VARCHAR(10) NOT NULL,
  trade_date DATE NOT NULL,
  open NUMERIC(12,4), high NUMERIC(12,4),
  low  NUMERIC(12,4), close NUMERIC(12,4),
  volume BIGINT, amount NUMERIC(18,2),
  source VARCHAR(16),          -- tencent / akshare / yfinance（记录降级情况）
  fetched_at TIMESTAMPTZ DEFAULT now(),
  PRIMARY KEY (code, trade_date)
);

CREATE TABLE signals (
  id BIGSERIAL PRIMARY KEY,
  trade_date DATE NOT NULL,
  slot VARCHAR(16) NOT NULL,   -- pre_open / midday / post_close
  code VARCHAR(10) NOT NULL,
  dims JSONB NOT NULL,         -- 各维度得分与理由
  total_score INT NOT NULL,
  action VARCHAR(8) NOT NULL,  -- buy / sell / hold
  note TEXT,
  UNIQUE (trade_date, slot, code)
);

CREATE TABLE push_logs (
  id BIGSERIAL PRIMARY KEY,
  trade_date DATE NOT NULL,
  slot VARCHAR(16) NOT NULL,
  status VARCHAR(16) NOT NULL, -- success / failed / skipped
  card_json JSONB,
  response TEXT,
  pushed_at TIMESTAMPTZ,
  UNIQUE (trade_date, slot)    -- 幂等：同日同时点只推一次
);

-- Phase 2 追加：人工持仓录入，消息可结合持仓给针对性建议
CREATE TABLE holdings (
  code VARCHAR(10) PRIMARY KEY,
  shares NUMERIC(18,2), cost NUMERIC(12,4),
  updated_at TIMESTAMPTZ
);
```

### Redis 键规划

| Key | 用途 | TTL |
|:---|:---|:---|
| `qf:lock:{date}:{slot}` | job 互斥锁，防重复执行 | 30min |
| `qf:kline:{code}:{date}` | K线缓存（盘中反复读） | 24h |
| `qf:latest:{slot}` | 最新卡片 JSON（Phase 2 TS 看板读取） | 24h |
| `qf:feishu:rl` | 推送限流令牌桶（≤5条/秒） | — |

---

## 七、调度与交易日历

**触发配置**（Asia/Shanghai）：`08:30 pre_open` / `12:00 midday` / `15:30 post_close`，仅工作日进入日历校验。

**交易日历**：启动时用 akshare `tool_trade_date_hist_sina` 拉取当年交易日表写入 `trade_calendar`（每年首次启动同步一次）。节假日、周末直接 skip 并写 push_logs(status=skipped)。

**补跑策略**（机器错过时点时，下次启动/下个任务时检查）：

| 错过的时点 | 当前时间 | 动作 |
|:---|:---|:---|
| 盘前 08:30 | < 09:30 | 补跑（标注「迟发」） |
| 盘前 08:30 | ≥ 09:30 | 放弃（开盘后盘前建议已失效） |
| 盘中 12:00 | 12:00–13:00 | 补跑 |
| 盘中 12:00 | ≥ 13:00 | 放弃 |
| 盘后 15:30 | < 20:00 | 补跑（复盘数据不受影响） |
| 盘后 15:30 | ≥ 20:00 | 放弃 |

---

## 八、技术栈分工（每种语言的落点）

| 语言/组件 | 职责 | 阶段 |
|:---|:---|:---|
| **Python** | 数据抓取、六维信号引擎、调度、飞书推送——核心全链路（akshare/chan.py 生态决定） | Phase 1 |
| **Postgres** | K线/信号/推送日志/交易日历持久化，审计与复盘的地基 | Phase 1 |
| **Redis** | K线缓存、job 互斥锁、推送限流、最新结果缓存 | Phase 1 |
| **Go** | API 网关 + 推送服务 + 健康探针：常驻、并发、资源占用低，适合 7×24 网关角色 | Phase 2 |
| **TypeScript** | Web 看板（React/Vite）：信号历史、推送记录、手动触发、持仓录入 | Phase 2 |
| **Rust** | 指标计算核心 + 回测引擎（PyO3 供 Python 调用）；tdxrs 本地行情源也是 Rust | Phase 3 |

> [!note] 诚实说明
> 个人日频规模下纯 Python 完全够用。Go/Rust/TS 是演进目标而非起点，每种语言都有明确职责落点，不为用而用。

---

## 九、分阶段路线图

### Phase 1 · 推送上线（1–2 天）✅ 优先

1. 改造 QuantFund：数据落 Postgres、读 Redis 缓存
2. APScheduler 三时点（08:30/12:00/15:30）+ 交易日历 + 幂等锁 + 补跑
3. 飞书 Webhook 签名 + 卡片渲染 + 重试 + 告警
4. docker-compose 一键起（postgres + redis + app）

**验收**：三时点模拟推送手机正常收到；节假日不推；重复触发不重复推；断网/源挂有告警。

### Phase 2 · 服务化与可视化（1–2 周）

1. Go 网关：`/api/latest` `/api/signals?date=` `/api/trigger` `/healthz`，推送逻辑迁 Go，Python 引擎只产 JSON
2. TS 看板：信号历史表格、推送日志、手动触发按钮、持仓录入页
3. `holdings` 表启用，卡片增加「你的持仓视角」区块
4. 飞书升级自建应用（可选）：交互卡片「已执行/忽略」打卡回传留痕

### Phase 3 · 计算深化（跑稳后）

1. Rust 指标/回测核心（PyO3），历史回测验证六维信号胜率与盈亏比，用数据调阈值
2. 缠论第 7 维接入：chan.py 计算笔/段/中枢/买卖点，按 [[缠论知识库-Schema设计]] 的 strategy 层融合评分
3. （可选）LLM 对信号生成中文解读，随卡片下发

---

## 十、落地目录结构

> [!note] 实施调整（2026-08-09）
> 为不破坏已跑通的 MVP，服务端独立成 `D:\QuantFundServer\`，数据层与六维引擎代码从 MVP 移植复用；`D:\QuantFund\` 原样保留可继续手动使用。

```
D:\QuantFundServer\
├── docker-compose.yml             # postgres + redis + app
├── Dockerfile                     # python:3.11-slim + 依赖
├── .env / .env.example            # webhook/secret/PG/Redis 配置（不入库）
├── config.json                    # 基金池 + 阈值 + 时点(08:30/12:00/15:30) + 盘中修正规则
├── app\
│   ├── main.py                    # 入口：daemon / test / init-calendar / status
│   ├── calendar_util.py           # 交易日历（akshare 年表 + PG 缓存 + 工作日兜底）
│   ├── fetch_data.py              # 三级降级 + 实时快照 + 隔夜美股 + 落 PG
│   ├── signals.py                 # 六维评分（阈值参数注入，预留缠论维度）
│   ├── advisor.py                 # 总分→买/卖/不动 + 盘中修正 + 盘后反转标注
│   ├── feishu.py                  # webhook + HMAC 签名 + 重试
│   ├── cards.py                   # 飞书卡片 + Markdown 存档渲染
│   ├── db.py / cache.py           # PG / Redis 客户端
│   └── jobs.py                    # 三时点编排：锁/幂等/补跑/告警
├── migrations\001_init.sql
├── reports\                       # Markdown 存档（宿主机可见）
└── data\klines\                   # K线文件缓存（2h TTL）
```

---

## 十一、需要你做的事（用户侧清单）

- [ ] **飞书机器人**：建一个群（可只有自己）→ 群设置添加「自定义机器人」→ 复制 Webhook 地址 → 安全设置勾选「签名校验」并复制密钥 → 填入 `.env`
- [ ] **开机自启**：确认 Windows 开机后 WSL + Docker 自动启动（Docker Desktop 设开机自启即可）；交易日 **8:30 前电脑需开机联网**
- [ ] （可选）注册 Tushare 配 token，作为 A 股更稳的备用源
- [ ] （可选，Phase 2）录入当前持仓，让建议贴合你的实际仓位
- [ ] 首次运行用 `python main.py --test-slot pre_open` 模拟推送，在手机上确认卡片排版

---

## 十二、风险与补充事项

> [!warning] 上线前必须想清楚的几件事
1. **信号未回测**：六维阈值目前是拍脑袋值，建议 Phase 3 回测（或至少手工抽验几段历史行情）后再重仓跟随；v1 阶段请把它当「第二视角」而非指令
2. **机器依赖**：推送依赖本机开机联网，错过时点走补跑策略；如需 7×24 可靠，后续再议云服务器
3. **数据源风险**：免费源随时可能限流/改接口，已有三级降级 + 降级标注 + 告警兜底，但**无法保证 100% 有数据**
4. **跨境 ETF 溢价**：纳指/标普 ETF 二级价格与净值常有溢价差，卡片会提示，下单前务必自行核对
5. **密钥安全**：Webhook/签名/DB 密码只进 `.env`（加入 .gitignore），不写代码不传聊天工具
6. **合规边界**：所有消息带免责声明；系统永不实现下单执行功能
7. **场外基金 T+1 差异**：本系统标的均为场内 ETF（实时价交易）；若将来扩展场外基金，信号需在 14:30 前生成才有当日操作意义（见 [[缠论知识库-Schema设计]] 注意事项）

---

## 十三、验收标准（Phase 1）

- [x] 三个时点模拟推送成功，卡片渲染正常（2026-08-09 dry-run 验证；手机实测待 Webhook）
- [x] 非交易日（周末/法定节假日）不推送，push_logs 记 skipped（2026-08-09 日历校验通过）
- [x] 同一时点重复触发不重复推送（2026-08-09 幂等测试：already_pushed）
- [x] 数据源降级不静默（2026-08-09 实测：yfinance 限流被优雅跳过，主源腾讯正常）
- [ ] 模拟腾讯源故障 → 自动降级 akshare 且卡片标注「数据源降级」
- [ ] 模拟全部数据源失败 → 收到飞书 ⚠️ 告警而非静默
- [x] 盘后建议与 MVP `latest_close.md` 手工核对一致（2026-08-09 十只基金评分逐一对齐）
- [ ] 断掉网络 10 分钟后恢复 → 任务按补跑策略处理，无重复推送

---

## 相关笔记

- [[QuantFundServer-实施进度]] —— 已完成/待完成清单与上线阻塞项（进度追踪）
- [[基金日频参谋-QuantFund项目笔记]] —— 本设计的 MVP 基线（六维引擎 + 三级降级）
- [[勇麦YMOS-投资操作系统-项目分析与结合方案]] —— 三层架构与 Human-in-the-loop 理念来源
- [[缠论知识库-Schema设计]] —— Phase 3 缠论第 7 维与 RAG 的落地蓝图
- [[YMOS实测记录-2026-08-03]] —— 数据源实测结论（Yahoo 对 ETF 可用等）

---

> **一句话总结**：QuantFund 已有信号引擎不动，给它装上「交易日历调度 + Postgres/Redis 留痕 + 飞书 Webhook 推送」三件套，1–2 天内三时点买/卖/不动建议直达手机；Go 网关、TS 看板、Rust 回测与缠论维度按阶段渐进叠加。
>
> 状态：Phase 1 已部署 WSL Docker · 待配置飞书 Webhook 实测推送 · 2026-08-09
