---
title: huashu-chrome 项目笔记
created: 2026-09-08
tags:
  - GitHub研究
  - AI工具
  - 浏览器自动化
  - MCP
description: 让任何 AI agent 操控你真实 Chrome（带全部登录态）的 MCP server + 扩展，含 2026-09-08 / 09-09 本机实测、QuantV1 场景评估与 ego-lite 对比
aliases:
  - huashu-chrome
---
# huashu-chrome 项目笔记

> [!info] 项目信息
> - **仓库地址**：https://github.com/alchaincyf/huashu-chrome
> - **作者**：alchaincyf（花叔）
> - **协议**：MIT
> - **分发**：npm 包 `huashu-chrome`（2026-09-08 时最新 v1.1.1）
> - **形态**：MCP server + Chrome 扩展 + 本地桥（三件套）
> - **要求**：Node **≥20**（✅ 2026-09-09 复查：本机 `D:\Node\node.exe` 已为 **v24.20.0**，环境坑已消除）
> - **npm 最新版**：v1.2.0（2026-09-09 复查；前一天记录为 1.1.1，作者迭代很快）
> - **记录日期**：2026-09-08 首记（深度实测 + QuantV1 评估）；2026-09-09 复测（桥/握手链路跑通）+ ego-lite 对比
> - **状态**：**可立即启用**，只差在 Chrome 里点一下装扩展（见「启用最后一步」）

---

## 一句话定位

**让任何支持 MCP 的 AI agent（Claude Code / Codex / Cursor / Gemini CLI 通用）直接操控你正在用的那个真实 Chrome——带着全部登录态，不用 API key、不用重新登录、不碰验证码。** 口号：「工具返回『已点击』不算数，页面真的动了才算。」

---

##  架构（五个器官）

```
npm 包 → CLI → MCP server → 本地桥(127.0.0.1:8899) → Chrome 扩展 → 你真实浏览器的真实页面
```

- MCP 侧是标准工具集（22 个浏览器工具），agent 像调普通工具一样调它
- 真正执行落在**你已登录的 Chrome 扩展**里，所以天然携带全部 cookie/登录态
- 有 CLI 形态：`node src/cli.js call <tool> '<json>'`——**可以不过 LLM 纯确定性调用**，适合脚本化取数

## 🧰 核心工具与能力

| 工具 | 用途 |
|---|---|
| `network` | 看页面**实际在调哪些接口**、返回什么（字段名以站点自己写的为准，不用猜） |
| `fetch` | **带你的 cookie 直接调接口**；`pages` 参数自动翻页、可落盘 JSONL |
| `read_text` / `query` | 解析渲染后的页面文本 / 结构化查询 |
| `act` | 一次跑多步（省 agent 回合数） |
| 失败检测 | 内置「页面动没动」校验，防工具返回"已点击"但实际静默失败 |
| 验证码/扫码 | 自动识别并**交还给你**处理 |

- 站点经验库：`~/.huashu-chrome/learnings/`，首次跑某站点用 `network` 现摸，摸清后自动沉淀
- README 收录 21 个站点经验（东财不在其中）
- 已知边界：浏览器保护页（如 Chrome 应用商店后台）无法注入，项目明确标注

---

## ⚖️ 适用 vs 不适用（2026-09-08 实测结论，重点！）

评估背景：当时想用它救 QuantV1 板块 K 线被东财掐断的问题。跑完全套五通道诊断后结论：

### ✅ 适合（卖点的真实落点）
1. **需要登录态的页面操作**：12306 查票、往已登录 CRM 录数据、小红书发布、B 站拉视频字幕、飞书多维表格加记录
2. **页面能正常显示但 API 要 cookie** 的取数（README 12306 例：「不用登录，但必须在浏览器里发」）
3. 用 `network` 摸未知站点的真实接口（侦察利器）

### ❌ 不适合（今天的实证反例）
- **IP 层/接口层封禁的数据接口**：东财 push2his kline/get 在封禁期，**东财板块页自己发出的 K 线请求都 ERR_EMPTY_RESPONSE、页面 DOM 无日期（图本身就是空的）**——huashu-chrome 驱动真 Chrome 打开同一页面，读到的也是空图。它与宿主机**共用同一出口 IP**，"真实指纹 + 登录态"两大卖点在这里都不是瓶颈
- 教训公式：**换浏览器 ≠ 换 IP**。被掐的是 IP/接口，它救不了；被拦的是"没登录"，它专治

### 当时裁决
未安装（Node 版本不达标 + 结论已明），详见 `D:\Obsidian\My-First-Obsidian\量化交易工具\板块K线数据源-东财频控诊断与兜底通道-20260908.md`

---

## 🔧 环境坑（本机 2026-09-08 实测）

1. **Node 版本**：要求 ≥20。本机 `D:\Node\node.exe` v18.16.0，无 nvm。实测现象：`npx huashu-chrome@1.1.1 doctor` → 桥能起（pid 21272，端口 8899 ✅），但 CLI 在 `probe()` 处崩 `ReferenceError: WebSocket is not defined`（Node 18 无全局 WebSocket，Node 22+ 才有）。**用之前先把 `D:\Node` 升到 20+**
   > [!success] 已解决（2026-09-09 复查）
   > 本机 Node 现为 **v24.20.0**，此坑不复存在，升级动作可跳过。复测详见下文「🔧 复测记录（2026-09-09）」
2. 安装：`npx huashu-chrome install`；体检：`npx huashu-chrome doctor`
3. 配置目录 `C:\Users\Turn-\.huashu-chrome`（首次跑 `huashu-chrome mcp` 自动创建）
4. 前提：取数时 Chrome 开着且扩展在线；**真实登录态**在手，别拿它跑高频循环（触发目标站风控的锅是你的）

## 📋 后期用途清单（Summer 说用得上时从这里取）

- [ ] 小红书：内容/评论/标签批量采集（已登录态）
- [ ] B 站：视频数据、字幕拉取
- [ ] 12306 / 政务 / CRM 类登录页的数据录入与查询
- [ ] 给 QuantV1 摸需要登录态的数据接口（注意：东财封禁期仍无解，属 IP 层）
- [x] ~~前置任务：升级 `D:\Node` 到 20+~~（2026-09-09 复查：已是 v24.20.0，天然满足）
- [ ] 新增前置任务（唯一剩下的）：Chrome 里点一下装扩展，见「启用最后一步」

---

## 🆚 与 ego-lite 的对比（2026-09-09）

对比对象：[[04-GitHub研究/ego-lite/ego-lite 项目笔记|ego-lite 项目笔记]]（citrolabs，~15.1k stars）。

### 根本差异：不在同一层

- **ego-lite = 重造浏览器**。改 Chromium 内核，Agent 任务跑在**独立 Space**（多任务并行、不抢你的焦点），登录态首次启动**一次性从 Chrome 迁移**。
- **本项目 = 给现有 Chrome 外挂一层**。扩展 + 本地桥(8899) + MCP，直接操控**你正在用的那个 Chrome**，登录态实时、零拷贝。

### 对照表

| 维度 | huashu-chrome | ego-lite |
|---|---|---|
| 平台 | ✅ **Windows 可跑**（本机已验证） | ❌ 仅 macOS（Win/Linux 无日期） |
| 并行多任务 | ❌ 单实例串行 | ✅ 多 Space 并行 |
| 打扰你 browsing | ❌ 在你窗口开标签 | ✅ Space 隔离 + 任务标签静音 |
| token 效率 | 常规 read_text | ✅ 语义快照 ~200–400 tok/页 |
| 执行范式 | 22 工具调用循环（+ CLI 可脱离 LLM 确定性调用） | Code-based：一次写完整条 JS heredoc |
| **接口侦察** | ✅ **`network` 看页面真实调的 API/字段** | ❌ 无对等工具 |
| **批量取数** | ✅ **`fetch` 带 cookie + `pages` 自动翻页 + 落盘 JSONL** | ❌ 需自己写脚本 |
| 失败检测 | ✅ 内置「页面动没动」校验 | 有，但 0.4.5.5 才修「脚本失败仍报成功」 |
| 登录态长期稳定性 | ✅ 更强（就用你的活浏览器） | ⚠️ 靠拷贝 + 定期同步，重登/cookie 轮换有漏 |
| 验证码 | ✅ 自动识别交还你 | 观察/接管 Space（手动） |
| 体量 | 个人项目 | 公司团队 + 完整文档站 |

### 谁更胜一筹 —— 三条结论

1. **论项目实力：ego-lite 赢**，量级差异（内核改造 + Space 并行 + token 工程 + 生态文档）。它的自测基准宣称比 agent-browser 快 3.45×，但第三方核查发现官方自己的基准图只有 1.4×/1.9×/2.6×，**数字有水分**（详见其笔记）。
2. **论「Summer 今天能用」：huashu-chrome 赢**。本机是 Windows，ego-lite 直接出局。
3. **论 QuantV1 场景（摸接口 + 登录态批量取数）：huashu-chrome 的 `network`/`fetch` 无替代**。ego-lite 定位是「通用网页操作代理」，不是取数器。

**共同盲区**：两者都跑在同一台机器、共用同一出口 IP → **换浏览器 ≠ 换 IP**，东财那类 IP/接口层封禁都救不了（2026-09-08 已实证）。

**两者不互斥**：将来有 Mac 时，ego-lite 管「并行通用网页操作」，本项目管「接口侦察 + 批量取数」。

---

## 🔧 复测记录（2026-09-09，Windows）

1. ✅ `D:\Node\node.exe` = **v24.20.0**（无 nvm）。旧记录 v18.16.0 已失效，`WebSocket is not defined` 崩溃**自然消失**：`--help` / `doctor` / `bridge` / `install --dry-run` 全部正常，**无需任何 patch**
2. ✅ 桥可拉起：`npx huashu-chrome@1.2.0 bridge --foreground` → 监听 `127.0.0.1:8899`，`doctor` 显示 **pid 正常 + 握手正常**
3. ❌ 唯一卡点：**Chrome 扩展未安装**。已在 `Secure Preferences` 里确认无 `foiljmap...` 扩展 ID（本机 Default profile 共 17 个扩展）→ 浏览器不允许脚本代装扩展，这一下必须人手点
4. `install --dry-run` 探测结果：本机只检测到 **Claude Code** 一个 agent，实跑会写入 1 个 MCP 配置文件（自动备份 `.bak-<时间戳>`）
5. npm 版本从 1.1.1 → **1.2.0**（一天一次小版本，注意 API 可能漂移）

### 启用最后一步（约 1 分钟）

1. Chrome 开 `chrome://extensions` → 打开右上角「开发者模式」
2. 把这个文件夹拖进该页（或点「加载已解压的扩展程序」选中它）：
   `C:\Users\Turn-\AppData\Local\npm-cache\_npx\7ca4003ccaf576e9\node_modules\huashu-chrome\extension`
   （走商店更省事：https://chromewebstore.google.com/detail/foiljmaplphdfimfcnfdpekhdnfbgfbf ）
3. 回终端 `npx huashu-chrome doctor` → 应见「Chrome 扩展在线」
4. 要接进 Claude Code：去掉 `--dry-run` 重跑 `npx huashu-chrome install`

---

## 🔗 相关笔记

- [[04-GitHub研究/ego-lite/ego-lite 项目笔记|ego-lite 项目笔记]]（同赛道但不同层的对手；macOS only）
- [[量化交易工具/板块K线数据源-东财频控诊断与兜底通道-20260908|板块K线数据源-东财频控诊断与兜底通道-20260908]]（本项目评估的完整实证链）
