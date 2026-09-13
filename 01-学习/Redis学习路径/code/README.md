---
lark_doc_url: https://my.feishu.cn/docx/EnETdLLMjoH2RAxe6sPcvnq9nYd
---
# Redis 学习路径 · 示例代码目录

本目录存放 Redis 学习路径中可运行的 Python + redis-py 示例与配置。

## 目录结构建议

```
code/
├── 01-string/                     # 阶段一：字符串命令
├── 02-data-structures/            # 阶段一：五大基础数据结构
├── 03-pubsub/                     # 阶段一：发布订阅
├── 04-persistence/                # 阶段二：持久化配置
├── 05-replication/                # 阶段二：主从配置
├── 06-cluster/                    # 阶段二：Cluster 配置
├── 07-lua/                        # 阶段三：Lua 脚本
├── 08-cache-penetration/          # 阶段三：缓存三大问题
└── project/                       # 毕业项目：缓存网关
```

## 快速开始

```bash
pip install -r requirements.txt
python code/01-string/string_demo.py
```
