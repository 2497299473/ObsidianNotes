---
title: ziwei-doushu 项目笔记
created: 2026-11-09
tags:
  - GitHub研究
  - 紫微斗数
  - Skills
  - 已启用
description: Renhuai123/ziwei-doushu 排盘引擎研究 + ziwei-paipan Skill 落地实录：2500 盘对官方数据集零误差，两个真坑（server-only / 宫名不统一）全记录
---

# ziwei-doushu 项目笔记

> 一句话：倪海厦《天纪》体系的开源排盘引擎（商业站 ouranic.cn 的开放快照），已落地为本机 `ziwei-paipan` Skill，排盘正确性经 2500 盘对官方数据集零误差验证。

- 仓库：https://github.com/Renhuai123/ziwei-doushu
- 基本盘（2026-11-09 API 实测）：**17,001★ / 3,289 fork / TypeScript / MIT（代码）**，2026-05-01 创建，持续更新
- 官网：ouranic.cn（北京王多鱼科技，京ICP备2026069066号-1）
- 状态：**已启用**（Skill 形态，2026-11-09）

## 一、开源边界（关键，四段授权）

| 内容 | 协议 | 商用 |
|---|---|---|
| 代码 `lib/`（排盘引擎/古籍/SEO图谱/Demo界面） | MIT | ✅ |
| 数据集 v3.0/v4.0（Releases） | CC BY 4.0 | ✅ 需署名 |
| 古籍原文（骨髓赋/全书/全集） | 公有领域 | ✅ |
| **倪师原话**（`docs/tianji-ziwei/` 1848 条 + 数据集内引文） | **CC BY-NC-SA 4.0** | ❌ 非商用 |

**不开源**：断语库（`db-analysis.ts` 是占位）、AI prompt、后端 API、用户系统、商业版前端。Demo 画得出盘，给不出线上解读。

## 二、实测勘误（README 易误读处）

- `patterns.ts`「1100+ 行规则库」是**行数**不是条数：实际 57,937 字节 / **42 个 detect 检测器**（含定贵/定富/定贫三模块）
- `heming-knowledge.ts`：零 import 纯数据，14 主星 × 6 字段 + 9 条倪师原话 + 2,866 字符合盘方法论
- v4.0 数据集：674,976 盘 × 13 维 = 877 万段，解压 56.6GB / 压缩 161MB；`year-1990.jsonl.zst` 仅 1.96MB，SHA256 与官方逐字核验一致
- **数据集语料价值**：1990 全年 8,760 盘去重后仅 **83 条**带集数时间码的倪师原话（模板复用导致引文高度重复）。补 nihaixia 语料应去姊妹仓 `nihai-tianji-corpus`（1848 条全量），数据集适合当断语风格样本/回归测试集

## 三、ziwei-paipan Skill 落地（2026-11-09）

**位置**：`C:\Users\Turn-\.opensquilla\workspace\skills\ziwei-paipan\`（31 文件 378KB + node_modules 18MB）

| 部分 | 内容 |
|---|---|
| `assets/engine/` | 19 个 .ts / 284,985 字节上游完整快照 + SNAPSHOT.sha256 |
| `references/` | heming-knowledge.json / patterns-rules.json（42 检测器）/ heming-methodology.md |
| `scripts/` | paipan.ts（单盘）· heming.ts（合盘）· validate_vs_dataset.ts · selfcheck.py · setup-shim.mjs |

**验证证据**：
- 对官方 v4.0 数据集 `ms` 字段逐盘比对：**500/500 + 2000/2000 全部吻合，零误差**
- 边界四项全过：晚子时 23:30 归次日 / 早子时 00:30 不换日 / 2月30日抛错 / hour=13 抛错
- 删 node_modules 重装 → postinstall shim 自动恢复 → 自检 ALL PASS
- Skill loader 已识别（7 个 workspace skills 之一）

**两个真坑（已修，SKILL.md 有警示）**：
1. **server-only**：patterns.ts 首行 `import 'server-only'`；npm 官方包在非 React 环境**导入即抛异常**。解法：`setup-shim.mjs` postinstall 铺空操作 shim，不能把 server-only 写进 dependencies
2. **宫名不统一**：命宫带「宫」字，夫妻/福德/父母不带；合盘取宫需 `replace(/宫$/,'')` 后比较，否则取到空数组

**用法**：
```bash
cd C:\Users\Turn-\.opensquilla\workspace\skills\ziwei-paipan
npx tsx scripts/paipan.ts --year 1999 --month 5 --day 12 --hour <0-11> --gender female
npx tsx scripts/heming.ts --a YYYY-MM-DD-h-male --b YYYY-MM-DD-h-female
python scripts/selfcheck.py   # 四项自检，零网络
```

## 四、命理 Agent 原型架构（两块底座已就位）

```
OpenSquilla
├── mcp__scrapling（17 工具）→ 取外部资料（语料/对照盘/字幕）
└── Skill: ziwei-paipan → 领域知识 + 确定性排盘
     第1层 排盘 algorithm.ts（真太阳时/夏令时/闰月四口径/晚子时）
     第2层 格局 patterns.ts（42 检测器）
     第3层 断语 heming + v4.0 数据集（补不开源的断语层）
```

注意：排盘的「数」是算出来的不是抓出来的，Scrapling 只负责外部资料，不承担排盘取数。

## 五、关联

- [[04-GitHub研究/Scrapling/Scrapling 项目笔记]] — 取数能力那一半
- nihaixia skill（managed）— 倪师医理分支；本仓库是天纪命理分支，语料互补但授权不同（原话非商用）
- 待办：实排 Summer 盘（1999-05-12，需确认时辰与出生城市）；语料扩充走 nihai-tianji-corpus
