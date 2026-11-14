---
lark_doc_url: https://my.feishu.cn/docx/EnETdLLMjoH2RAxe6sPcvnq9nYd
---
# Redis 学习路径 · 示例代码目录

本目录存放 Redis 学习路径中可运行的 Python + redis-py 示例与配置。

## 目录结构

```
code/
├── 01-string/                     # 阶段一：字符串命令
├── 02-data-structures/            # 阶段一：五大基础数据结构
├── 03-pubsub/                     # 阶段一：发布订阅 + Stream
├── 04-persistence/                # 阶段二：持久化（redis.conf + run.sh）
├── 05-replication/                # 阶段二：主从 + 哨兵（replica.conf / sentinel.conf / run.sh）
├── 06-cluster/                    # 阶段二：Cluster（redis-7000.conf + run.sh）
├── 07-lua/                        # 阶段三：Lua 脚本（demo.py + stock.lua）
└── 08-cache-penetration/          # 阶段三：缓存三大问题
```

> 毕业项目的代码（`app.py`）与验收脚本在同级 `../毕业项目-分布式缓存网关/`（见其 `Makefile`）。

## 快速开始

```bash
pip install -r requirements.txt
python 01-string/string_demo.py
```

## 运行命令与预期输出

| 目录 | 运行命令（在本 `code/` 目录下） | 预期输出（首行/关键行） |
|------|------------------------------|------------------------|
| `01-string/` | `python 01-string/string_demo.py` | `GET: alice`、`VIEWS: 1`、`LOCK: True` |
| `02-data-structures/` | `python 02-data-structures/demo.py` | `Hash:` / `List:` / `Top 3:` 等结果行，末行 `Done!` |
| `03-pubsub/` | `python 03-pubsub/demo.py` | `Subscriber received: Breaking News!` + 两条 `Stream [...]` + `Done!` |
| `07-lua/` | `python 07-lua/demo.py` | `扣减3件: 成功, 剩余: 7` / `扣减100件: 失败, 剩余: 7` |
| `08-cache-penetration/` | `python 08-cache-penetration/demo.py` | `穿透测试（不存在的key）: None` → `击穿测试: data-NN` → `Done!` |
| `04/05/06-*` | `bash 04-persistence/run.sh`（需本机 `redis-server`） | 见各 `run.sh` 的 `echo` 分段与阶段二笔记 |

> ⚠️ 全部脚本要求本机 6379（或脚本内指定端口）有 Redis 在跑；无 `redis-server` 时先 `docker run -d -p 6379:6379 redis:7.2`。

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `pip install -r requirements.txt` 后 `python 01-string/string_demo.py` 打印 `GET: alice` / `VIEWS: 1` / `LOCK: True` 三行且不报 `ConnectionError`（证明 6379 通） |
| 🟡 进阶 | 五个 Python demo 全部跑通，逐一对照上表“预期输出”列：`07-lua` 的输出必须是“成功…失败”而非“失败…失败”（库存初始化顺序正确） |
| 🔴 挑战 | 从仓库根目录（而非 `code/`）运行 `python code/07-lua/demo.py` 仍能成功（验证脚本用 `__file__` 定位 `stock.lua`）；再故意停掉 Redis，确认报错是 `ConnectionError` 而非 `FileNotFoundError`，说明两类错误的定位区别 |
