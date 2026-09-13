# OpenMAIC 项目笔记

> 记录时间：2026-09-01 · 来源：GitHub README（raw）/ open.maic.chat / JCST 2026 论文 / 中文深度解读
> 项目地址：https://github.com/THU-MAIC/OpenMAIC · Live Demo：https://open.maic.chat

## 一句话定位

**OpenMAIC（Open Multi-Agent Interactive Classroom）**：清华 THU-MAIC 团队开源的 AI 互动课堂平台——输入一个主题或上传文档，几分钟内由多 Agent 协作生成一整堂可交互的课（幻灯片+语音+测验+交互模拟+白板推演+圆桌辩论+PBL），AI 教师和 AI 同学实时讲课、讨论、点名。

**范式主张**：MOOC（被动看录播）→ MAIC（多智能体主动互动课堂），"模拟一整间教室"而非"生成一份课件"。

## 基础信息

| 项 | 值 |
|---|---|
| Stars / Forks | ~21k / 4.1k（2026-09-01） |
| 协议 | **MIT**（v0.3.0 于 2026-06-28 从 AGPL-3.0 改 MIT，商业友好） |
| 技术栈 | Next.js 16 + React 19 + TypeScript 5 + Tailwind 4 + **LangGraph 1.1**（多 Agent 状态机） |
| 学术背书 | JCST 2026 论文 *"From MOOC to MAIC"*（DOI: 10.1007/s11390-025-6000-0，24 作者），清华 700+ 学生、2+ 年真实课堂验证 |
| 版本 | v1.0.0（2026-08-27，最新） |
| 环境要求 | Node.js ≥ 20，pnpm ≥ 10 |

## 核心架构（三大引擎）

### 1. 两阶段生成流水线（`lib/generation/`）
- **Stage 1 大纲生成**：分析主题/上传文档 → 结构化课程大纲（JSON DSL）；v0.2.2 起生成前**大纲可人工编辑**
- **Stage 2 场景生成**：每个大纲项 → 自包含 JSON DSL 场景（幻灯片内容/测验题/交互代码/白板指令/语音台词）
- **per-stage model routing**（v0.3.0）：不同场景可用不同 LLM（幻灯片用快的，PBL 用强的）

### 2. 多智能体编排（`lib/orchestration/`，LangGraph 状态机）
| 角色 | 职责 |
|---|---|
| AI 教师 | 主导授课、控节奏、提问引导 |
| AI 助教（TA） | 补充说明、答疑、个别辅导 |
| AI 同学 | 主动提问、辩论、提不同观点 |

三种交互模式：课堂讨论（可被点名）/ 圆桌辩论（多人设 + 白板图示论点）/ Q&A（学生自由提问）

### 3. 回放引擎（`lib/playback/`）
28+ 动作类型：Speech（TTS）、Whiteboard Draw、Spotlight、Laser Pointer、Slide Navigation、Quiz Interaction、Simulation Launch 等；v0.3.1 起支持 **action-level 时间轴跳转/回看**。

## 六大场景类型

1. 智能幻灯片讲座（LaTeX/图表/图片，逐页讲解+语音+激光笔）
2. 互动测验 + AI 批改（即时反馈、识别盲区、建议个性化路径，状态持久化）
3. 交互式 HTML 模拟（物理模拟/算法可视化/流程图；Deep Interactive Mode：3D、游戏、思维导图、在线编程）
4. PBL 项目制学习 v2（选角色 + 与 Agent 协作 + 里程碑/交付物；含职业学习任务引擎）
5. 协作白板（实时绘图/解方程/流程图；几何冲突检测器，白板质量分 5.4→6.1）
6. 语音双向交互（多 TTS 提供商；VoxCPM2 声音克隆 + Auto Voice 按人设自动配音；ASR 语音参与）

## 模型与生态

- **18+ LLM 提供商**：OpenAI/Azure/Anthropic/Bedrock/Gemini/DeepSeek/Qwen/Kimi/MiniMax/Grok/GLM/小米 MiMo/腾讯混元/OpenRouter/Doubao + **Ollama、Lemonade（全本地，免 key）** + 任意 OpenAI 兼容 API
- 官方推荐：Gemini 3 Flash（默认）/ Gemini 3.1 Pro（最高质量）
- **`@openmaic/*` SDK 家族**（npm，v0.3.0 起）：`dsl`（场景 DSL）/ `renderer`（渲染引擎，可嵌入任意 Web 应用）/ `importer`
- 文档解析：PDF/DOCX/PPTX/图片/音频/视频；可选 MinerU（复杂表格/公式/OCR）、AliDocMind；本地 ffmpeg 音视频抽取
- 本地语音栈：FunASR（本地 ASR）、VoxCPM2（本地 TTS+克隆，三后端：vLLM-Omni/Python API/Nano-vLLM）

## 部署方式

| 方式 | 说明 |
|---|---|
| 本地开发 | `pnpm install` → 配至少一个 LLM key → `pnpm dev` → localhost:3000 |
| Docker | `docker compose up --build`；支持清华源镜像构建参数 `ALPINE_MIRROR` / `NPM_REGISTRY`（国内网络友好） |
| Vercel | 一键部署（填 env） |
| Postgres 持久化 | `server-persistence` profile，App + PG 两容器；**注意**：PERSISTENCE_DEV_TOKEN 明文打进前端 bundle，无用户隔离，只适合 localhost/可信网络单用户，生产要重写 `lib/persistence/server-auth.ts` |
| MP4 视频导出 | 可选 `video-export` profile 起独立 render-service（Chromium+FFmpeg, Node 22），浏览器内生成 Hyperframes 工程后一键渲染 |
| ACCESS_CODE | 站点级密码保护（共享部署） |

## v1.0.0 亮点（2026-08-27，当前版本）

- **Pro workbench（Agent 工作台）**：chat-first 的课程构建 Agent——规划多课时课程、逐页构建/修改、直接从上传材料（文档/音视频/网页）建课；通过**显式验证过的工具**操作（原子 patch 单场景、生成/插入/删除/重排页面、导入 .pptx、生成图/视频/配音），而非编辑不透明 blob
- **Durable sessions**：server-backed 运行可跨重启恢复，可取消/续跑/随时追加指令，事件历史可回放
- **20 个内置 skills**（课程规划、深度研究、授课风格、PPTX 导入、风格复用等）+ 用户自写 skill
- 默认关闭，需 `NEXT_PUBLIC_PRO_WORKBENCH_ENABLED=true` + `OPENMAIC_AGENT_RUNTIME_ENABLED=true` + `DATABASE_URL`，且 `MODEL_ROUTES` 必须显式指定 `maic-agent-driver` 的模型（无兜底）

## OpenClaw 集成（与本机环境直接相关）

`clawhub install openmaic`（或直接让 Claw 装 skill）后，在飞书/Slack/Telegram 等 20+ 聊天 App 里说"教我量子物理"即可生成课堂：
- **托管模式**：open.maic.chat 拿访问码，零本地部署
- **自托管模式**：skill 引导 clone/配置/启动

## 版本演进

| 版本 | 日期 | 核心更新 |
|---|---|---|
| v0.1.0 | 2026-03-26 | 首发：讨论 TTS、沉浸模式、白板 |
| v0.1.1 | 2026-04-14 | ACCESS_CODE、课堂 ZIP 导入导出、Ollama |
| v0.2.0 | 2026-04-20 | Deep Interactive Mode（3D/模拟/游戏/思维导图/在线编程） |
| v0.2.1 | 2026-04-26 | VoxCPM2 声音克隆、per-model thinking |
| v0.2.2 | 2026-06-02 | MAIC Editor Pro、大纲可编辑、离线导出 |
| v0.3.0 | 2026-06-28 | PBL v2、`@openmaic/*` SDK、per-stage 模型路由、**AGPL→MIT** |
| v0.3.1 | 2026-07-21 | MP4 视频导出、Postgres 持久化、JSON Patch AI 编辑 |
| v0.3.2 | 2026-08-14 | 视频导出加固、增量保存、四新语言、FunASR |
| **v1.0.0** | **2026-08-27** | Pro workbench、durable sessions、20 skills、可插拔持久化 |

迭代极快：5 个月 9 个版本，每周级发版节奏。

## 相关度评估（AI 研究 / 量化 / 全栈 / DevOps）：⭐⭐⭐⭐

**对 AI 研究（架构参考价值）**：
- LangGraph 状态机做**角色分工多 Agent 协作**（教师/TA/同学 + 三种交互模式）是可复用的编排范式——与 HarnessX（harness 可训练）不同，它展示的是"固定角色分工 + 状态机"这条更工程化的路
- 两阶段生成（大纲↔场景解耦 + 每阶段独立模型路由）是"长任务生成"的通用设计模式，可迁移到其他生成型 Agent
- 28+ 动作类型回放引擎 + action-level 时间轴 = "Agent 行为可回放可调试"的参考实现（呼应 OpenViking 的检索轨迹可观测思路）
- 学术验证硬：JCST 论文 + 700 学生 2 年实测，非 demo

**对全栈开发**：
- Next.js 16 + React 19 + TS5 + LangGraph 1.1 的完整参考实现；`@openmaic/renderer` npm 包可直接嵌入自己产品
- Pro workbench 的"Agent 通过验证过的显式工具操作结构化文档（JSON Patch 原子改）"设计值得借鉴——比让模型直接改大 JSON 稳得多

**对 DevOps**：
- 多容器 profile 设计（persistence / video-export 按需挂载）、国内镜像构建参数、Vercel 一键部署

**实际用法（与现有工作流结合点）**：
1. **论文解读**：上传量化/AI 论文 PDF → 自动生成拆解课程（配合 MinerU 解析公式）
2. **学习路径课程化**：Obsidian 里已有「技术学习路径创建方法论」——用 OpenMAIC 把生成的学习路径/文档直接转成可交互课程，闭环
3. **OpenClaw skill**：本机生态直接可装（`clawhub install openmaic`），聊天里生成课堂
4. 快速体验：open.maic.chat 免费 Live Demo，无需部署

**注意点**：
- Postgres 持久化的 token 认证机制**不适合多用户生产**（README 明确说明），团队共享部署需重写 auth
- Pro workbench 默认关闭且配置项多（4 个 env + MODEL_ROUTES），新手先用经典一键生成
- 课堂质量强依赖 LLM（官方推 Gemini 3 Flash）
