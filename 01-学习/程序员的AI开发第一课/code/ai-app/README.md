# AI 应用毕业项目 v1→v7

> 《程序员的 AI 开发第一课》毕业项目线：**同一个命令行聊天机器人的 7 次单点增量**。
> 代码提取自各篇正文（含 2026-11-13 审查修复批的 import/参数订正），每版可独立运行。
> 命名依据见 [[../../总览索引]] 的 v1→v7 梯表（审查 J2 裁决）。

## 版本梯表

| 版本 | 增量 | 来源篇 | 验收问题 |
|------|------|--------|----------|
| v1 | 最简命令行聊天（无记忆） | 09 §1 | 问「我是谁」应答不出 |
| v2 | + 会话记忆（session_id） | 09 §2 | 报过名字后能答「我是谁」 |
| v3 | + 角色扮演 + trim 控规模 | 09 §3 | 风格符合设定，长对话不爆窗口 |
| v4 | + 业务知识（RAG） | 11 | 库内答对、库外敢说不知道 |
| v5 | + 自主行动（ReAct Agent，两条实现路径） | 13 / 14 | 水果价格题收敛且 = 42 |
| v6 | + 长期记忆（mem0 跨会话） | 15 | 重启后仍记得偏好 |
| v7 | + 语义缓存 + 集中接入（可上线形态） | 16 / 17 | 同义提示词命中缓存；换模型不改业务代码 |

## 环境

- 教学基线：Python 3.10+，LangChain **0.3.x** 全家桶（版本口径见总览索引；1.x 迁移注记见 14/16 篇）
- v1–v5：`pip install langchain langchain-openai langchain-community langchain-chroma langchain-text-splitters tiktoken openai`
- v6 另需：`pip install mem0ai`
- v7 另需：Redis Stack（`docker run -p 6379:6379 redis/redis-stack-server:latest`）+ One-API（见 17 篇）
- 环境变量：`OPENAI_API_KEY` 必配；LangChain 场景用 `OPENAI_API_BASE`（原生 SDK 用 `OPENAI_BASE_URL`，两套不通用）

## ⚠️ 验证状态（坦诚声明）

- 全部代码经 `ast.parse` 语法校验通过；**运行时行为未在本机验证**（无相关依赖环境）。
- v5 的 `eval` 与 v7 之前的 ShellTool 演示有安全风险，正文已加警告块；生产须隔离/白名单/人工确认。
- 每版跑通后按梯表「验收问题」自测，再进下一版。

## 运行

```bash
cd v1 && python chat.py   # 逐版同理；v4 先 python index.py 再 python chat.py
```