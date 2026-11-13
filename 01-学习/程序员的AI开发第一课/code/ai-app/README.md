# AI 应用毕业项目 v1→v7

> 《程序员的 AI 开发第一课》毕业项目线：**v1–v4 累积演进（同一个聊天机器人逐步变强），v5–v7 为并列的单点能力演示**。
> v5（Agent）/ v6（长期记忆）/ v7（缓存+网关）各自独立可跑、不继承前版能力——这是原作的教学取舍，
> 不是缺陷；把 v5–v7 合成一个全功能应用是结课后的进阶练习（需实跑验证）。
> 代码提取自各篇正文（含 2026-11-13 审查修复批的 import/参数订正）。
> 命名依据见 [[../../总览索引]] 的 v1→v7 梯表（审查 J2 裁决；叙事口径依复审 J8 裁决修正）。

## 版本梯表

| 版本 | 增量 | 来源篇 | 代码 | 验收问题 |
|------|------|--------|------|----------|
| v1 | 最简命令行聊天（无记忆） | 09 §1 | `v1/chat.py` | 问「我是谁」应答不出 |
| v2 | + 会话记忆（session_id） | 09 §2 | `v2/chat.py` | 报过名字后能答「我是谁」 |
| v3 | + 角色扮演 + trim 控规模 | 09 §3 | `v3/chat.py` | 风格符合设定，长对话不爆窗口 |
| v4 | + 业务知识（RAG） | 11 | `v4/index.py` + `v4/chat.py` | 库内答对、库外敢说不知道 |
| v5 | 自主行动（ReAct Agent）：**v5a** 手写 / **v5b** LangChain 两条实现路径 | 13 / 14 | `v5/agent_handwritten.py` / `v5/agent_langchain.py` | 水果价格题在 max_turns 内收敛且结果=42（若得 22 是模型改题意，见 13 篇勘误） |
| v6 | 长期记忆（mem0 跨会话；以 09 篇角色机器人为底座，**不含 v5 的 Agent**） | 15 | `v6/chat.py` | 重启后仍记得偏好 |
| v7 | **v7a** 语义缓存 + **v7b** 集中接入（单文件演示两件事，**非完整聊天应用**） | 16 / 17 | `v7/chat.py` | 同义提示词命中缓存；换模型不改业务代码 |

> 18–21 篇（HF / Ollama / 微调 / Router）为模型侧专题，代码见各篇正文，**不计入 v1→v7 主线**。

## 环境

- 教学基线：Python 3.10+，LangChain **0.3.x** 全家桶（版本口径见总览索引；1.x 迁移注记见 14/16 篇）。
  **必须钉版本安装**——不带约束的 `pip install langchain` 会装到 1.x，`v5b`/`v7` 的 import 会直接失效：

  ```bash
  pip install -r requirements.txt          # v1–v5 公共依赖（已钉 0.3.x 区间）
  # 等价手写：pip install "langchain>=0.3,<0.4" "langchain-openai>=0.3,<0.4" "langchain-community>=0.3,<0.4" "langchain-chroma>=0.2,<0.3" "langchain-text-splitters>=0.3,<0.4" "tiktoken>=0.7,<1.0" "openai>=1.104.2,<3.0"
  ```

  版本区间经 PyPI 元数据核实（2026-11-13）：0.3.x 末版相互兼容、`langchain-core` 被约束在 <1.0；**pip 实跑验证待环境就绪**。
  若环境里已装 1.x：请新建虚拟环境装本文件；或改装 `langchain-classic` 并按 14/16 篇版本注记改 import（依 J6 本库代码不迁就 1.x）。
  ⚠️ 安全注记（2026-11-13 PyPI 披露）：0.3.x 教学基线存在已披露 CVE（langchain 路径穿越 CVE-2026-55443；langchain-openai <1.1.14 图片 token 计数 SSRF CVE-2026-41488，本库代码不走该路径）。**仅限本机教学演示**；生产环境请升级到修复版本并重新评估 import 迁移。
- v6 另需：`pip install -r requirements-v6.txt`（mem0ai 0.1.x）
- v7 另需：`pip install -r requirements-v7.txt`（**redis-py 客户端**，Redis Stack 容器不提供 Python 包）
  + Redis Stack：`docker run -p 6379:6379 redis/redis-stack-server:latest`
  + One-API（见 17 篇；DSN 用 `host.docker.internal`，勿用容器内 localhost）
- 环境变量：
  - v1–v6：`OPENAI_API_KEY` 必配；LangChain 场景网关变量用 `OPENAI_API_BASE`（原生 SDK 用 `OPENAI_BASE_URL`，两套不通用，见 05/08 篇注记）
  - v7：另配 `ONEAPI_KEY` = One-API 令牌（**不是** OpenAI 官方 Key；未设置会在启动时报 KeyError）

## ⚠️ 验证状态（坦诚声明）

- 全部代码经 `ast.parse` 语法校验通过；**运行时行为未在本机验证**（无相关依赖环境）。
- v5a 的 `eval` 与 **14 篇正文的 ShellTool 演示**（未收入本库）有安全风险，已加警告；生产须隔离/白名单/人工确认。
- 每版跑通后按梯表「验收问题」自测，再进下一版。

## 运行

```bash
# v1–v3、v6：进入对应目录直接跑
cd v1 && python chat.py

# v4：两步。默认语料 sample_data/introduction.txt 已随库落盘（摘编自开篇词，
#     含「作者是谁/还写过哪些专栏」两问的事实）；换自己的资料替换该文件或改 index.py 的 loader 路径
cd v4 && python index.py && python chat.py

# v5：两条路径任选（v5a 手写 / v5b LangChain）
cd v5 && python agent_handwritten.py   # 或 python agent_langchain.py

# v7：先起服务再跑脚本（顺序不能反）
# ① docker run -p 6379:6379 redis/redis-stack-server:latest   （Redis Stack）
# ② 启动 One-API 并配好渠道与令牌（17 篇），export ONEAPI_KEY=<你的One-API令牌>
# ③ cd v7 && python chat.py
```

## 受限环境怎么跑（2026-11-13 复审 P0-3）

| 你的情况 | 替代路径 | 依据 | 验证状态 |
|----------|---------|------|----------|
| 无 OpenAI Key | 本地 Ollama 起 qwen2.5 小档位，`OPENAI_API_BASE=http://localhost:11434/v1` + `OPENAI_API_KEY=ollama`（占位即可），`ChatOpenAI(model="qwen2.5:0.5b")` | 19 篇 Ollama 兼容 OpenAI 接口 | ⏸️ 待实跑 |
| 有 Key 但想统一管理/换供应商 | 起 One-API 网关，业务只改 base_url/key | 17 篇 | ⏸️ 待实跑 |
| 无 GPU | 跳过 20 篇微调实操（标 ⏸️），用社区现成微调 GGUF 与基座同题对比，完成选型论证 | 20 篇自检 🔴 | ⏸️ 待实跑 |
| 无 Docker | 16 篇先用 `InMemoryCache` 演示精确缓存；v7a 语义缓存需 Redis Stack，标 ⏸️ 待环境 | 16 篇正文 | ⏸️ 待实跑 |

> 四条替代路径均为文档级方案（依各篇正文与官方接口约定编写），**本机未实跑**；跑通后请把 ⏸️ 改 ✅ 并注明环境。
