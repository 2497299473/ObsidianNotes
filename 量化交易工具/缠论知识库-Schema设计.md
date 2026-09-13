---
lark_doc_url: https://my.feishu.cn/docx/F44RdI3zeo2GrBx3nwKc25N3nLb
lark_doc_token: F44RdI3zeo2GrBx3nwKc25N3nLb
---
# 缠论知识库 Schema 设计

> **目标**：将缠论 PDF 方法论提取为结构化知识库，供量化交易系统每日决策中 RAG 检索调用
> **创建时间**：2026-08-01
> **状态**：设计阶段
> **关联笔记**：[[基金日频买卖工具-项目分析]]

---

## 一、设计目标

| 维度 | 要求 |
|---|---|
| 检索精度 | 关键词命中率 > 90%，语义匹配支持模糊表述 |
| 覆盖范围 | 形态学（笔/段/中枢）+ 动力学（背驰/力度）+ 买卖点规则 |
| 实时性 | 静态知识库（不频繁更新），查询延迟 < 100ms |
| 可扩展 | 后续可接入更多技术分析体系（波浪/道氏/谐波） |

---

## 二、整体架构（4 层 + 向量存储）

```
┌─────────────────────────────────────────────────────┐
│  Layer 4: 应用层                                     │
│  每日操作建议生成 / 回测策略触发 / 教学问答            │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│  Layer 3: 检索层                                     │
│  混合检索（语义 + 关键词 BM25）+ RRF 融合 + 重排序    │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│  Layer 2: 存储层                                     │
│  ChromaDB（向量）+ Neo4j（概念图谱）+ JSON（元数据）   │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│  Layer 1: 数据层                                     │
│  MinerU 提取 → 人工校验 → 结构化切分 → 入库           │
└─────────────────────────────────────────────────────┘
```

---

## 三、知识类型标签（concept_type）

| 类型编码 | 类型名称 | 说明 |
|---|---|---|
| `definition` | 基础定义 | 笔、段、中枢、走势类型等基础概念 |
| `principle` | 定理/原理 | 如中枢定理、走势终完美等 |
| `rule` | 操作规则 | 买卖点的判定规则 |
| `signal` | 信号类型 | 一类买点、二类卖点等 |
| `technique` | 技巧/方法 | 背驰判断、区间套、多级别联立等 |
| `risk` | 风险控制 | 止损、仓位、失败情况 |
| `case` | 实战案例 | 原文中的案例分析 |
| `faq` | 常见问题 | 易混淆点、答疑 |

---

## 四、核心概念清单

| ID | 概念名 | 类别 | 说明 |
|---|---|---|---|
| `concept_kline_merge` | K线包含处理 | morphology | K线合并规则（方向性处理） |
| `concept_fx` | 分型 | morphology | 顶分型 / 底分型定义 |
| `concept_bi` | 笔 | morphology | 严格笔 / 宽松笔 |
| `concept_duan` | 段 | morphology | 段的划分与破坏 |
| `concept_zhongshu` | 中枢 | structure | 中枢定义 / 级别 / 扩展 |
| `concept_zoushi` | 走势 | structure | 趋势 / 盘整分类 |
| `concept_beichi` | 背驰 | dynamics | MACD背驰 / 区间套 |
| `concept_buy1` | 第一类买点 | signal | 趋势背驰后的转折 |
| `concept_buy2` | 第二类买点 | signal | 一买后次级别回抽不破低 |
| `concept_buy3` | 第三类买点 | signal | 离开中枢后回抽不进入中枢 |
| `concept_sell1` | 第一类卖点 | signal | 趋势背驰后的转折（卖） |
| `concept_sell2` | 第二类卖点 | signal | 一卖后次级别反弹不破高 |
| `concept_sell3` | 第三类卖点 | signal | 离开中枢后回抽不进入中枢（卖） |
| `concept_level` | 多级别联立 | structure | 大级别定方向，小级别找入场 |
| `concept_qujiantao` | 区间套 | dynamics | 多级别精确定位转折点 |

---

## 五、Schema 详细定义

### 5.1 概念层（Concept）

每个缠论基础构件独立一条记录。

```json
{
  "doc_id": "concept_bi",
  "type": "concept",
  "concept_type": "definition",
  "name": "笔",
  "category": "morphology",
  "definition": "相邻顶分型和底分型之间构成一笔。一个笔必须由至少5根K线组成。",
  "formula": "笔 = 顶分型 + 中间K线 + 底分型，总K线数 ≥ 5",
  "formal_rules": [
    "顶分型后必须有底分型",
    "顶底之间至少一根独立K线",
    "向上笔：底分型→顶分型",
    "向下笔：顶分型→底分型"
  ],
  "variants": {
    "strict": "严格笔：要求顶底分型间至少5根K线",
    "loose": "宽松笔：允许包含处理后的4根K线"
  },
  "example": "某股票日线图中，顶分型在第10根K线，底分型在第16根K线，中间间隔5根K线，构成一笔。",
  "counter_example": "顶分型与底分型相邻（无间隔K线），不构成笔。",
  "level": ["日线", "30分钟", "5分钟"],
  "conditions": [
    "存在明确的顶分型",
    "存在明确的底分型",
    "顶底之间至少间隔1根K线",
    "总K线数 ≥ 5"
  ],
  "signals": ["笔的生成", "笔的破坏", "笔的延伸"],
  "operations": [
    "新笔生成时，可判断当前走势方向",
    "笔被破坏时，需重新评估走势结构"
  ],
  "risk": "笔的判定存在严格型与宽松型差异，不同标准可能产生不同结果。",
  "related_concepts": ["分型", "段", "K线包含处理"],
  "source": {
    "pdf": "缠论.pdf",
    "chapter": "第三章 笔",
    "page_range": [45, 62]
  },
  "tags": ["笔", "分型", "基础定义", "形态学"],
  "embedding_text": "笔是缠论最基础的结构单元..."
}
```

### 5.2 规则层（Rule）

从概念中派生的可执行判断规则。

```json
{
  "doc_id": "rule_buy3_confirm",
  "type": "rule",
  "concept_type": "rule",
  "name": "第三类买点确认条件",
  "concept_ref": "concept_buy3",
  "conditions": [
    {
      "field": "zhongshu_existed",
      "operator": "==",
      "value": true,
      "desc": "必须存在至少一个完整中枢"
    },
    {
      "field": "leave_direction",
      "operator": "==",
      "value": "up",
      "desc": "次级别走势向上离开中枢"
    },
    {
      "field": "pullback_low",
      "operator": ">",
      "value": "{zhongshu.zg}",
      "desc": "回抽最低点不进入中枢上沿(ZG)"
    },
    {
      "field": "volume_confirm",
      "operator": "optional",
      "desc": "离开时放量，回抽时缩量（辅助确认）"
    }
  ],
  "confidence_weights": {
    "structure_match": 0.4,
    "volume_confirm": 0.2,
    "macd_support": 0.2,
    "multi_level_align": 0.2
  },
  "false_signal_filters": [
    "大盘系统性下跌时降低买点置信度",
    "中枢震荡中不确认三类买卖点",
    "量能持续萎缩时谨慎对待"
  ],
  "stop_loss": "三类买点形成的最低点下方 3%",
  "price_target": "中枢上沿 + 离开段的一倍幅度",
  "source": {
    "pdf": "缠论.pdf",
    "chapter": "第七章 三类买卖点",
    "page_range": [120, 145]
  },
  "tags": ["三类买点", "确认条件", "规则"],
  "embedding_text": "第三类买点的核心确认条件..."
}
```

**规则清单**：

| ID | 规则名 | 关联概念 | 用途 |
|---|---|---|---|
| `rule_bi_strict` | 严格笔判定 | concept_bi | 笔的合法性校验 |
| `rule_duan_break` | 段破坏条件 | concept_duan | 段结束的判断 |
| `rule_zhongshu_form` | 中枢形成 | concept_zhongshu | 三笔重叠构成中枢 |
| `rule_zhongshu_expand` | 中枢扩展 | concept_zhongshu | 中枢级别升级 |
| `rule_beichi_macd` | MACD背驰 | concept_beichi | 面积对比判断背驰 |
| `rule_buy1_confirm` | 一买确认 | concept_buy1 | 趋势背驰 + 区间套 |
| `rule_buy2_confirm` | 二买确认 | concept_buy2 | 回抽不破前低 |
| `rule_buy3_confirm` | 三买确认 | concept_buy3 | 回抽不进中枢 |
| `rule_sell1_confirm` | 一卖确认 | concept_sell1 | 趋势背驰（向上） |
| `rule_sell2_confirm` | 二卖确认 | concept_sell2 | 反弹不破前高 |
| `rule_sell3_confirm` | 三卖确认 | concept_sell3 | 反弹不进中枢 |
| `rule_multi_level` | 多级别共振 | concept_level | 大级别+小级别同向 |

### 5.3 策略层（Strategy）

将规则组合为可执行的交易策略。

```json
{
  "doc_id": "strategy_chan_fund_daily",
  "type": "strategy",
  "name": "缠论基金日频交易策略",
  "target_assets": ["ETF", "LOF", "场外指数基金"],
  "timeframe": ["daily", "weekly"],

  "signal_pipeline": [
    {
      "step": 1,
      "name": "大级别定方向",
      "action": "判断周线级别走势类型（趋势/盘整）",
      "rules": ["rule_zhongshu_form", "rule_multi_level"],
      "output": "weekly_trend: up | down | consolidation"
    },
    {
      "step": 2,
      "name": "本级别找信号",
      "action": "在日线级别识别买卖点",
      "rules": ["rule_buy1_confirm", "rule_buy2_confirm", "rule_buy3_confirm",
                "rule_sell1_confirm", "rule_sell2_confirm", "rule_sell3_confirm"],
      "output": "daily_signal: {type, confidence, price_level}"
    },
    {
      "step": 3,
      "name": "小级别精确定位",
      "action": "30分钟级别区间套确认入场时机",
      "rules": ["rule_beichi_macd"],
      "output": "entry_timing: immediate | wait | cancel"
    },
    {
      "step": 4,
      "name": "基本面过滤",
      "action": "用基本面数据验证信号可靠性",
      "external_data": ["pe_percentile", "roe", "fund_flow", "north_bound"],
      "filters": [
        "PE分位 < 50% 时买点信号加权",
        "PE分位 > 80% 时买点信号降权",
        "北向持续流出时降低买点置信度"
      ],
      "output": "adjusted_confidence: float"
    },
    {
      "step": 5,
      "name": "生成操作建议",
      "output_schema": {
        "action": "buy | sell | hold | watch",
        "position_change": "+20% | -15% | 0",
        "stop_loss": "float",
        "take_profit": "float",
        "confidence": "float",
        "reasoning": "string"
      }
    }
  ],

  "position_rules": {
    "max_single_position": 0.3,
    "incremental_buy": [0.1, 0.1, 0.1],
    "stop_loss_default": 0.05,
    "take_profit_targets": [0.08, 0.15, 0.25]
  }
}
```

### 5.4 概念关系图谱（Neo4j）

```cypher
// 节点创建
CREATE (:Concept {name: "K线包含处理", category: "morphology", level: "basic"})
CREATE (:Concept {name: "分型", category: "morphology", level: "basic"})
CREATE (:Concept {name: "笔", category: "morphology", level: "basic"})
CREATE (:Concept {name: "段", category: "morphology", level: "intermediate"})
CREATE (:Concept {name: "中枢", category: "structure", level: "intermediate"})
CREATE (:Concept {name: "背驰", category: "dynamics", level: "advanced"})
CREATE (:Concept {name: "一类买点", category: "signal", level: "advanced"})
CREATE (:Concept {name: "二类买点", category: "signal", level: "advanced"})
CREATE (:Concept {name: "三类买点", category: "signal", level: "advanced"})
CREATE (:Concept {name: "MACD", category: "indicator", level: "tool"})

// 关系创建
CREATE (K线包含处理)-[:COMPOSES {order: 1}]->(分型)
CREATE (分型)-[:COMPOSES {order: 2}]->(笔)
CREATE (笔)-[:COMPOSES {order: 3}]->(段)
CREATE (段)-[:COMPOSES {order: 4}]->(中枢)
CREATE (中枢)-[:GENERATES]->(三类买点)
CREATE (背驰)-[:CONFIRMS]->(一类买点)
CREATE (一类买点)-[:EVOLVES_TO]->(二类买点)
CREATE (二类买点)-[:EVOLVES_TO]->(三类买点)
CREATE (MACD)-[:ASSISTS]->(背驰)
```

### 5.5 元数据层（Meta）

```json
{
  "doc_id": "meta_source_chanlun_pdf",
  "type": "meta",
  "source_type": "pdf",
  "file_path": "./data/缠论.pdf",
  "extraction": {
    "tool": "MinerU",
    "date": "2026-08-01",
    "output_format": "markdown",
    "chunks_count": 0,
    "quality_score": 0.0
  },
  "chapters": [
    {"id": "ch1", "title": "K线包含处理", "page_range": [10, 25]},
    {"id": "ch2", "title": "分型", "page_range": [26, 44]},
    {"id": "ch3", "title": "笔", "page_range": [45, 62]},
    {"id": "ch4", "title": "段", "page_range": [63, 85]},
    {"id": "ch5", "title": "中枢", "page_range": [86, 119]},
    {"id": "ch6", "title": "走势类型", "page_range": [120, 140]},
    {"id": "ch7", "title": "背驰", "page_range": [141, 165]},
    {"id": "ch8", "title": "买卖点", "page_range": [166, 200]},
    {"id": "ch9", "title": "多级别联立", "page_range": [201, 230]},
    {"id": "ch10", "title": "区间套", "page_range": [231, 260]}
  ]
}
```

---

## 六、统一元数据规格

所有文档共享以下字段：

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `doc_id` | string | ✅ | 唯一标识符，格式 `{type}_{name}` |
| `type` | enum | ✅ | `concept` / `rule` / `strategy` / `case` / `meta` |
| `concept_type` | enum | ✅ | 知识类型标签（见第三节） |
| `name` | string | ✅ | 知识点标题 |
| `related_concepts` | list | ✅ | 关联概念列表 |
| `tags` | list | ✅ | 检索标签 |
| `source` | object | ✅ | PDF来源（章节+页码） |
| `embedding_text` | string | ✅ | 用于向量化的文本 |
| `verified` | bool | ✅ | 是否经过人工校验 |
| `created_at` | datetime | ✅ | 入库时间 |

---

## 七、向量存储设计

### 7.1 ChromaDB Collection

```python
# Collection 1: 缠论概念与规则
collection_concepts = {
    "name": "chanlun_concepts",
    "metadata_schema": {
        "type": "concept | rule | strategy",
        "category": "morphology | dynamics | structure | signal",
        "chapter": "str",
        "page_start": "int",
        "difficulty": "basic | intermediate | advanced",
    },
    "embedding_model": "BAAI/bge-large-zh-v1.5",  # 1024维，中文优化
    "distance_metric": "cosine"
}

# Collection 2: 实战案例
collection_cases = {
    "name": "chanlun_cases",
    "metadata_schema": {
        "signal_type": "buy1 | buy2 | buy3 | sell1 | sell2 | sell3",
        "asset": "str",
        "date": "str",
        "outcome": "success | failure | partial",
        "market_condition": "bull | bear | consolidation",
    }
}

# Collection 3: 决策模板
collection_templates = {
    "name": "chanlun_decision_templates",
    "metadata_schema": {
        "scenario": "trend_reversal | consolidation_break | pullback_entry",
        "market_env": "bull | bear | neutral",
        "asset_type": "etf | index_fund | lof"
    }
}
```

### 7.2 分块策略

```python
from langchain.text_splitter import RecursiveCharacterTextSplitter

splitter = RecursiveCharacterTextSplitter(
    chunk_size=500,           # 中文约250字
    chunk_overlap=50,         # 10% 重叠保证跨块语义连贯
    separators=[
        "\n# ",               # 一级标题
        "\n## ",              # 二级标题
        "\n### ",             # 三级标题
        "\n**定义**",          # 定义标识
        "\n**操作步骤**",      # 步骤标识
        "\n**案例**",          # 案例标识
        "\n\n",               # 段落分隔
        "。",                 # 句号
    ]
)
```

**分块原则**：
- 每个概念定义独立成块
- 操作流程按步骤拆分为独立块
- 案例保持完整不拆分
- 规则逐条独立成块

### 7.3 检索策略

```python
retrieval_config = {
    "concept_query": {
        "collection": "chanlun_concepts",
        "k": 3,
        "filter": {"type": "concept"},
        "rerank": True,              # bge-reranker-v2-m3 二次排序
        "rerank_top_n": 2
    },
    "rule_query": {
        "collection": "chanlun_concepts",
        "k": 5,
        "filter": {"type": "rule"},
        "mmr_diversity": 0.3         # MMR去重
    },
    "case_query": {
        "collection": "chanlun_cases",
        "k": 3,
        "filter_by_signal": True     # 按当前信号类型过滤
    },
    "hybrid_search": {
        "dense_weight": 0.7,         # 语义检索权重
        "sparse_weight": 0.3,        # 关键词检索权重（BM25）
        "fusion": "RRF"              # Reciprocal Rank Fusion
    }
}
```

### 7.4 检索意图分类

| 用户输入模式 | 检索意图 | 优先 Collection | 示例 |
|---|---|---|---|
| "什么是X" / "X的定义" | 概念查询 | concepts (concept) | "什么是背驰" |
| "如何判断X" / "X的步骤" | 流程查询 | concepts (rule) | "如何确认三类买点" |
| "案例" / "实战" | 案例查询 | cases | "茅台三类买点案例" |
| "能不能做X" | 规则查询 | concepts (rule) + templates | "现在能不能买" |
| "X和Y的区别" | 关系查询 | concepts + 图谱 | "笔和段的区别" |

---

## 八、LLM 集成

### 8.1 每日决策 Prompt 模板

```
你是基于缠论的A股基金投资顾问。

## 缠论知识库参考
{rag_concepts}          ← 检索到的概念定义
{rag_rules}             ← 检索到的判断规则
{rag_cases}             ← 检索到的历史案例

## 当前缠论信号（chan.py 计算输出）
- 周线走势类型: {weekly_trend}
- 日线信号: {daily_signal_type}
- 信号置信度(技术面): {technical_confidence}%
- 关键价位: 支撑 {support} / 压力 {resistance}
- 中枢位置: [{zs_zd}, {zs_zg}]

## 基本面数据
- PE分位: {pe_percentile}%
- ROE: {roe}%
- 北向资金5日: {north_5d}
- 资金流向: {fund_flow}

## 请输出
1. 【缠论结构判断】当前走势处于什么阶段
2. 【信号确认分析】对照知识库规则，信号是否成立
3. 【操作建议】具体买卖方向和仓位比例
4. 【止损止盈】明确的价位
5. 【风险提示】主要风险因素
```

### 8.2 调用示例

```python
from langchain_community.chat_models import ChatDeepSeek
from langchain_community.vectorstores import Chroma

llm = ChatDeepSeek(model="deepseek-chat", temperature=0.3)

# 检索 + 生成
query = "日线级别出现底背驰，是否形成一类买点？"
results = hybrid_search(query, top_k=5)
context = build_context(query, results)

response = llm.invoke(DECISION_PROMPT.format(
    rag_concepts=context["concepts"],
    rag_rules=context["rules"],
    rag_cases=context["cases"],
    weekly_trend="下跌趋势中",
    daily_signal_type="一类买点候选",
    technical_confidence=72,
    support=3.78, resistance=4.15,
    zs_zd=3.85, zs_zg=4.05,
    pe_percentile=35, roe=12.5,
    north_5d="+15亿", fund_flow="净流入2.3亿"
))
```

---

## 九、与量化系统的数据流

```
08:30  定时任务触发
  │
  ├─→ [数据采集]
  │    ├─ tdxrs: ETF日K线 + 基金行情
  │    ├─ a-stock-data: 资金流/北向/龙虎榜/研报
  │    └─ 基本面API: PE/PB/ROE/估值分位
  │
  ├─→ [缠论计算]  chan.py
  │    ├─ K线包含处理 → 分型识别 → 笔 → 段 → 中枢
  │    ├─ 背驰检测 (MACD面积对比)
  │    └─ 输出: 当前买卖点信号列表
  │
  ├─→ [知识检索]  RAG
  │    ├─ 检索当前信号类型的确认条件 (rule)
  │    ├─ 检索相似历史案例 (case)
  │    └─ 检索多级别联立策略 (strategy)
  │
  ├─→ [LLM综合决策]
  │    ├─ 输入: 缠论信号 + 基本面 + 检索结果
  │    ├─ 约束: 策略层定义的仓位规则
  │    └─ 输出: 结构化操作建议
  │
  └─→ [推送]  飞书/微信 webhook
       └─ 格式化卡片 → 手机接收
```

---

## 十、实施 Checklist

- [ ] **Phase 1: PDF提取**（2天）
  - [ ] 安装 MinerU (`pip install magic-pdf[full]`)
  - [ ] 提取缠论 PDF → Markdown
  - [ ] 人工校验核心定义（笔/段/中枢/买卖点）
  - [ ] 按章节切分，填充 Concept Schema

- [ ] **Phase 2: 知识库构建**（2天）
  - [ ] 部署 ChromaDB（本地或 Docker）
  - [ ] 下载 bge-large-zh-v1.5 embedding 模型
  - [ ] 灌入概念层 + 规则层数据
  - [ ] 测试检索质量（人工验证 top-3 准确率 > 90%）

- [ ] **Phase 3: 缠论计算引擎**（2天）
  - [ ] 安装 chan.py (`pip install chan.py`)
  - [ ] 接入 tdxrs / a-stock-data 作为数据源
  - [ ] 实现日频缠论信号计算
  - [ ] 输出标准化信号格式

- [ ] **Phase 4: LLM决策集成**（1天）
  - [ ] 配置 DeepSeek API
  - [ ] 实现 RAG 检索 + Prompt 组装
  - [ ] 结构化输出解析（JSON mode）
  - [ ] 回测验证决策质量

- [ ] **Phase 5: 推送上线**（1天）
  - [ ] 配置飞书/企业微信 webhook
  - [ ] 设计推送卡片模板
  - [ ] 配置定时任务（每日 8:30）
  - [ ] 手机端验证接收

---

## 十一、关键注意事项

1. **缠论主观性**：笔/段的划分存在严格/宽松两种标准，知识库中应同时收录两种定义，实际计算时选用一种并保持一致
2. **PDF 提取质量**：缠论含大量手绘图表，MinerU 可能无法完美提取图形逻辑，需人工将图表关键信息转为文字描述
3. **知识库维护**：实战中发现新的案例或规则修正时，及时更新知识库并重新生成 embedding
4. **回测先行**：缠论信号上线前，必须用历史数据回测买卖点的胜率和盈亏比
5. **场外基金 T+1**：场外基金按当日净值确认，信号应在 14:30 前生成以便当日操作

---

## 十二、相关文件

- [[基金日频买卖工具-项目分析]] — 5个项目对比与结合方案
- [MinerU](https://github.com/opendatalab/MinerU) — PDF 提取工具
- [chan.py](https://github.com/Vespa314/chan.py) — 缠论 Python 实现
- [ChromaDB](https://github.com/chroma-core/chroma) — 向量数据库
- [bge-large-zh-v1.5](https://huggingface.co/BAAI/bge-large-zh-v1.5) — 中文 embedding 模型
