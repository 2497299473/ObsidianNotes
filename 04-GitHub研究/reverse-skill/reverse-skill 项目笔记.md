---
lark_doc_token: XJ83dDcbSoHQIYxK15hcLhPRnPf
lark_doc_url: https://my.feishu.cn/docx/XJ83dDcbSoHQIYxK15hcLhPRnPf
---
# reverse-skill 项目笔记

> 创建日期：2026-08-08
> 仓库：[zhaoxuya520/reverse-skill](https://github.com/zhaoxuya520/reverse-skill)
> 状态：开源 MIT，已关注 🔍

---

## 项目概述

**reverse-skill** 是一个 **逆向工程 / 授权渗透测试 / 安全研究的 AI 技能路由包**（Cybersecurity Skills Router Pack），面向 Claude Code、Kiro、Cursor、Cline、Codex、OpenCode 等代码 AI 客户端。

核心定位：**让 AI Agent 面对安全类任务时"先路由到正确方法论，再执行"，而不是盲目猜命令。**

解决的问题：

- AI Agent 面对 APK、ELF、JS、PCAP 时不知道该用 jadx、apktool、Frida、IDA 还是 BurpSuite
- 工具路径、MCP 服务、脚本入口分散在不同机器，迁移困难
- 同样的逆向/渗透问题每次重新踩坑，经验无法复用

三大支柱（官方定义）：

1. **AI 自动路由**（AI-powered routing）— 41 条路由规则（R0–R40），163 条回归用例
2. **按需自举工具链**（On-demand toolchain bootstrapping）— 检测环境、按需装配工具
3. **自动进化经验库**（Self-evolving knowledge base）— field-journal 脱敏经验沉淀

---

## 核心指标

| 指标 | 数值 |
|------|------|
| Star | 20.8k |
| Fork | 2.9k |
| 创建时间 | 2026-05-13 |
| 主语言 | PowerShell |
| 许可证 | MIT（子模块：CTF-Sandbox-Orchestrator 为 GPLv3，Pentest Swarm AI 为 AGPL-3.0） |
| 路由规则 | 41 条（R0–R40） |
| 回归基准 | 163 条用例 |
| 核心 Skill 模块 | 42 个已跟踪模块 |
| CI 平台 | Windows + Ubuntu（GitHub Actions） |
| 最近更新 | 2026-08-08（活跃维护中） |

---

## 核心机制：工作流

```
用户任务
  → RULES.md（全局路由 + scope 门）
  → MASTER-ROUTING / master-route.ps1（PRIMARY 快路径）
  → case-init / scope.md（授权 + network_profile；未就绪禁止对目标 ACT）
  → 目标 Skill → 工具 / MCP / 脚本
  → timeline + Evidence→Finding→Path → 报告 + field-journal
```

关键设计：**授权门控（scope 门）** — 未经授权禁止对目标执行动作，内置了合法渗透测试的合规约束。

架构特点：路由核心由单一结构化配置驱动，跨平台 CI 验证，与客户端适配层分离（client-neutral）。

---

## 覆盖场景矩阵

| 场景 | 入口 |
|------|------|
| APK / Android 逆向 | `skills/apk-reverse/` |
| iOS / 移动端 | `skills/mobile-reverse/` |
| 二进制逆向 (exe/dll/so/elf) | `skills/ida-reverse/` / `skills/radare2/` |
| .NET / C# | `skills/dotnet-reverse/` |
| 前端 JS 签名 / 加密参数 | `skills/js-reverse/` |
| DSL VM / 风控自定义 VM | `skills/reverse-engineering/dsl-vm-reverse/` |
| HTTP 抓包 / 请求重放 | anything-analyzer、Reqable MCP + `js-reverse/` |
| 恶意软件 / YARA | `skills/malware-analysis/` |
| 渗透测试 / 漏洞扫描 | `skills/pentest-tools/` |
| 攻击链 / 红队编排 | `skills/attack-chain/` |
| CTF 竞赛 | `CTF-Sandbox-Orchestrator/`（42 个子技能） |
| 固件 / IoT | `skills/firmware-pentest/` |
| 补丁差分 / N-day | `skills/patch-diff-exploit/` |
| Pwn / 漏洞利用 | `skills/pwn-chain/` |
| EDR 绕过 | `skills/edr-bypass-re/` |
| API / GraphQL | `skills/api-security/` |
| 供应链 / SBOM | `skills/supply-chain-security/` |
| LLM / AI 安全 | `skills/llm-security/` |
| OLLVM 脱密 | `skills/reverse-engineering/references/ollvm-deobfuscation.md` |
| 图表 / 报告 | `skills/diagram-generator/` / `skills/docs-generator/` |

---

## 热度 / 趋势

- **2026-07-31 首次登顶 GitHub Trending #1**
- Trendshift 每日榜 #1（2026-08-03）、周榜上榜（2026 年第 31 周）
- 20.8k star / 2.9k fork，仅用不到 3 个月达成（2026-05-13 创建）
- 由 Atlas Cloud（大模型服务商）赞助

---

## 安装 / 使用

```bash
git clone https://github.com/zhaoxuya520/reverse-skill.git
```

- Windows：`powershell -File skills/scripts/refresh-tool-index.ps1`
- Linux / macOS：`bash skills/scripts/refresh-tool-index.sh`
- Kali：`bash kali/scripts/refresh-tool-index.sh`
- 首次使用：让 AI 阅读 `README_AI.md` 完成环境路由与工具检查
- 依赖：Java/JDK（jadx、apktool）、Node.js 22.12+（JS 工具链 + MCP）、Python 3.x（Frida）

---

## 与我的关联度分析

**总体判断：中等偏低，但架构参考价值高。** ⭐⭐☆

| 我的领域 | 关联度 | 说明 |
|---------|--------|------|
| AI 研究 | ⭐⭐⭐ | **最有价值的部分**：它是"AI Agent Skill 体系"的完整工程化案例——路由配置驱动、回归测试、跨平台 CI、授权门控、经验库自进化。与已研究的 headroom、herdr 同属 Agent 生态，可作为技能包架构设计的最佳实践样本 |
| 全栈开发 | ⭐⭐ | 前端 JS 逆向（js-reverse）、API/GraphQL 安全、供应链 SBOM 与 Web 开发有交集，可作安全知识参考 |
| 量化交易 | ⭐ | 直接关联弱；仅当未来需要抓取加密参数的数据源（反爬/签名）时，js-reverse 部分才有用 |
| DevOps | ⭐⭐ | 工具链自举、跨平台 CI、MCP 集成思路对 DevOps 有启发，但非核心 |

**结论**：这是一个安全逆向方向的垂直技能包，不是 Summer 的核心赛道，但它是 **AI Agent 技能体系如何"工程化、可测试、可演进"的标杆案例**，与 headroom（上下文压缩）、herdr（Agent 运行时）共同构成 Agent 生态图谱的重要一块。适合作为架构参考收藏，暂不深入部署。

---

## 行动项

- [ ] 已记录仓库信息与热度数据
- [ ] 可作为「Agent 技能包架构」专题的参考案例（与 herdr、headroom 对照）
- [ ] 如未来涉及数据采集/反爬，再回头研究 `js-reverse` 模块
- [ ] 留意其 Skill 生态组织方式，对比 OpenSquilla 的 skill 机制
