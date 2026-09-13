---
title: 17｜海纳百川：MCP 协议与外部工具连接
source: https://time.geekbang.org/column/article/955015
author: 黄佳
published: 2026-03-24
created: 2026-06-25
description: MCP协议连接数字世界，解决AI助手碎片化问题。实战中，连接主流MCP服务如Context7、GitHub、Notion和数据库，展示了其实用性。MCP协议支持多种传输方式，适用于不同场景，是全栈开发者的完整工具箱。此外，文章还介绍了创建自定义MCP服务器的方法以及MCP的安全原则和调试与故障排除方法。
tags:
  - clippings
  - claude-code
  - mcp
lark_doc_url: https://my.feishu.cn/docx/A6kwdmq9MorNaOxgX1lcuZ5Pnag
---

> **释题：海纳百川。** 一个开放协议，让 Claude Code 从只能操作本地文件的工具，进化为能连接整个数字世界的智能枢纽——数据库、API、云服务，百川归海，一协议通。

你好，我是黄佳。

前面两讲我们学习了 Hooks——事件驱动自动化的安全闸门。Hooks 让我们能在 Claude 执行工具前后插入自定义检查，解决了"能不能做"的问题。但即使有了 Memory 的记忆、SubAgents 的分工、Skills 的领域能力、Commands 的标准流程、Hooks 的安全防护，Claude Code 的所有能力，始终被锁在一个边界内——本地文件系统。今天我们就来打破这个边界。

Claude 能读文件、写代码、执行命令，这些你已经非常熟悉。但面对下面这些需求时，它就无能为力了：

- 帮我查一下数据库里上个月的销售数据
- 把这个 Issue 同步到 GitHub
- 从 Notion 里读取产品需求文档
- 检查一下 Sentry 上最近的错误日志

这不是 Claude 不够聪明，而是它缺少与外部世界连接的通道。读代码、写代码、跑命令——本质都是本地文件系统交互。而企业开发的真实场景中，数据库、版本控制系统、项目管理平台、监控系统才是日常主战场。如果 Claude 无法触及这些系统，它就只能做一个聪明但孤立的本地助手。

2024 年 11 月，Anthropic 推出了一项开源协议，彻底改变了这个局面——Model Context Protocol (MCP)，AI 时代的 USB-C 接口（参见佳哥之前推出的专栏 《MCP & A2A 前沿实战》）。

## MCP——AI 的 USB-C 接口

在 MCP 出现之前，如果你想让 AI 助手连接外部服务，通常有两种选择：

- **自定义开发**：为每个服务写专门的集成代码
- **平台绑定**：依赖特定平台提供的插件（如 ChatGPT Plugins）

这带来了严重的碎片化问题。假设市场上有 M 个 AI 助手和 N 个外部服务，那么理论上需要 M × N 个专用适配器。每一对组合都需要单独开发、单独维护、单独调试：

![](https://static001.geekbang.org/resource/image/1a/b2/1a71310130f2fa3359ef325cfaed2cb2.jpg?wh=3000x1443)

MCP 的出现改变了这一切。正如 Anthropic 官方博客所描述的：

> 把 MCP 想象成 AI 应用的 USB-C 接口。就像 USB-C 提供了连接设备与各种外设的标准化方式，MCP 提供了连接 AI 模型与各种数据源和工具的标准化方式。

有了 MCP，M × N 的问题变成了 M + N：

![](https://static001.geekbang.org/resource/image/4f/1a/4feef448c583abc385f42ea2afdac01a.jpg?wh=2932x1210)

一个协议，通用连接。每个 AI 助手只需要实现一次 MCP Client，每个服务只需要实现一次 MCP Server，然后任意组合即可工作。

![](https://static001.geekbang.org/resource/image/66/67/66e77ba71666a2cf1d5021e3b9a85167.jpg?wh=10473x5963)

MCP 的诞生源于一个简单的痛点。据 Wikipedia 记载，MCP 协议由 Anthropic 的两位工程师 David Soria Parra 和 Justin Spahr-Summers 构思并开发：

> 2024 年 7 月，我在做内部开发工具…作为一个开发工具背景的人，我很快就欣赏到 Claude Desktop 的强大——比如 Artifacts 功能——但也对它功能集有限、无法扩展感到沮丧。

这个痛点在当时并非个例。每一个试图将 AI 助手集成到真实工作流中的工程师都面临同样的困境。David 和 Justin 的洞察在于：这个问题不应该由每个开发者各自解决，而应该由一个开放协议统一解决——就像 HTTP 统一了 Web、LSP 统一了 IDE 语言支持那样。

MCP 发布后，迅速获得了行业认可：

- **2025 年 3 月**：OpenAI 正式采纳 MCP，将其集成到 ChatGPT 桌面应用和 Agents SDK 中
- **2025 年 9 月**：Google 宣布为 Gemini 提供官方 MCP 支持
- **2025 年 12 月**：Anthropic 将 MCP 捐赠给 Linux 基金会下的 Agentic AI Foundation (AAIF)
- **AAIF 创始成员**：OpenAI、Google、Microsoft、Amazon Web Services、Cloudflare、Bloomberg

截至 2025 年底，MCP 生态已经达到惊人的规模：

- **9700 万**月度 SDK 下载量
- **10000+** 公开 MCP 服务器（PulseMCP 目录收录 10,400+，MCP.so 收录 18,500+）

![](https://static001.geekbang.org/resource/image/5c/18/5c04e8ccf1b94ba97df7c231c9f69b18.jpg?wh=2899x979)

## MCP 架构与核心概念

MCP 采用经典的客户端 - 服务器架构。Claude Code 充当 MCP Client，负责发现和调用工具；MCP Server 则暴露工具和资源，作为外部服务的代理。两者之间通过 JSON-RPC 2.0 协议通信。

![](https://static001.geekbang.org/resource/image/0b/88/0bd8713656afef7c538ebf568ff03b88.jpg?wh=3094x1414)

这个架构的关键组件如下表所示。

![](https://static001.geekbang.org/resource/image/19/db/1990fb9c666f40139b02d17d9bcbb5db.jpg?wh=3042x1161)

MCP 复用了 Language Server Protocol (LSP) 的消息流思想。如果你用过 VS Code，你已经间接体验过这种架构——编辑器的智能提示、跳转定义等功能，都是通过 LSP 与语言服务器通信实现的。MCP 做了同样的事情，只不过它服务的不是代码编辑器，而是 AI Agent。

MCP Server 并不只是简单地"暴露一个函数"。它可以向 Client 提供三种不同类型的能力。

![](https://static001.geekbang.org/resource/image/76/07/76ef47995dbb4a63481dea6bc04dee07.jpg?wh=3010x1090)

- **Tools** 是最常用的能力类型——它让 Claude 能够"做事情"。
- **Resources** 提供只读数据，让 Claude 能够"看到东西"而不仅仅依赖你粘贴的文本。
- **Prompts** 则是一种便捷机制，让服务器预定义好特定场景的交互模板。

Claude Code 会在启动时自动发现所有配置的 MCP Server 及其提供的能力。当你说"帮我查一下数据库里的用户数量"时，Claude 会自动找到数据库 MCP Server，调用对应的查询工具，解析结果并返回给你。整个过程对用户完全透明。

## MCP 的三种传输方式

MCP 支持三种传输方式，适用于不同场景。

**Stdio 传输（本地进程）**是最简单的方式。MCP Server 作为本地子进程启动，通过标准输入（stdin）接收请求，通过标准输出（stdout）返回响应。零网络开销、零配置复杂度，适合本地工具和开发测试：

```json
{
  "mcpServers": {
    "filesystem": {
      "type": "stdio",
      "command": "npx",
      "args": ["@modelcontextprotocol/server-filesystem", "/home/user/projects"]
    }
  }
}
```

第二种方式是 **HTTP 传输（推荐用于远程）**。当 MCP Server 运行在远程服务器上时的推荐方式。通过标准 HTTP 请求 / 响应通信，支持 TLS 加密和 Bearer Token 认证。GitHub、Notion、Sentry 等云服务通常直接提供 HTTP 类型的 MCP 端点。

第三种我们也很熟悉了，**SSE 传输（Server-Sent Events）**——基于 HTTP 的单向推送技术，建立持久连接，服务器可以主动向客户端推送数据。适合实时监控和流式数据场景。在实践中使用较少，大多数场景用 stdio 或 HTTP 就够了。

这几种方式怎么选呢？原则是：**本地用 stdio，远程用 HTTP，实时用 SSE**。如果你拿不定主意，先试 stdio（本地服务器）或 HTTP（远程服务），这两个覆盖了 95% 的场景。

![](https://static001.geekbang.org/resource/image/26/cc/26a0b1a0bb07daab29efd2a54d27bfcc.jpg?wh=2695x1145) ![](https://static001.geekbang.org/resource/image/cb/23/cb985ac7dece83d76c1e6d3b26ba7523.jpg?wh=2997x964)

## MCP 的配置与管理

MCP 配置有三个 scope，对应三个不同的物理落点。注意，MCP 配置走的是和 settings.json 完全独立的体系——`mcpServers` 不是 settings.json / settings.local.json 的合法顶级键，写进去会被静默忽略。

![](https://static001.geekbang.org/resource/image/34/a2/3469756e21251f842b24f9c5f27fc2a2.jpg?wh=3814x1309)

**团队共享的服务配置**放到 `<项目根>/.mcp.json`——提交到 git，团队成员共享。

**个人常用服务**，如果是跨项目的，放到 `~/.claude.json` 顶级的 `mcpServers`（user scope）。

**敏感凭证 / 项目私有 MCP** 落在 `~/.claude.json` 的 `projects.<path>.mcpServers`（local scope），不入 git，但不要手写这一段，使用 `claude mcp add` 命令默认写到这里。

![Claude Code 里面的 MCP 配置示例](https://static001.geekbang.org/resource/image/a6/1c/a6a50223ca68aa3030f18a8d85c0981c.png?wh=813x1110)

在配置文件中硬编码敏感信息是危险的。MCP 配置支持通过 `${}` 语法引用环境变量：`${VAR_NAME}` 直接引用，变量不存在会报错；`${VAR_NAME:-default}` 在变量不存在时使用默认值。

Claude Code 提供了命令行工具来管理 MCP 服务器，这比手动编辑 JSON 更方便。

## 实战：连接主流 MCP 服务

理论讲了不少，接下来咱们动手练练。MCP 的生态已经非常成熟，从官方基础服务到第三方热门服务，覆盖了开发者日常所需的各个场景。

Anthropic 维护了一套官方 MCP 服务器集合，覆盖最常见的开发需求。这些服务器经过官方测试和维护，是入门 MCP 的最佳起点。

![](https://static001.geekbang.org/resource/image/48/e7/4896aa2e92390c9ee458cf9b597e15e7.jpg?wh=2983x1520)

社区也贡献了大量高质量的第三方 MCP 服务器。以下是开发者最常用、真正能跑的几个。

![](https://static001.geekbang.org/resource/image/6c/e5/6c99719525ebaea88d98d44cbe92bde5.jpg?wh=2919x1385)

我们来介绍几个最典型的配置，更多的实际实用的 MCP 工具，咱们可以继续在留言区分享。

### 实战 1：Context7——实时技术文档

配置极其简单，一行命令搞定。

```bash
claude mcp add context7 -- npx -y @upstash/context7-mcp @latest
```

或者在 `.mcp.json` 中手动配置：

```json
{
  "mcpServers": {
    "context7": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp@latest"]
    }
  }
}
```

配置完成后，你可以在提示中加上 `use context7` 关键词，Claude 就会自动去拉取最新的官方文档：

> Claude 会输出：
>
> 先查一下 Next.js 15 的最新文档...
>
> [调用 context7 MCP server → resolve_library_id → get_library_docs]
>
> 根据最新文档，Next.js 15 的 App Router API 路由写法如下...

不需要 API Key，不需要付费，开箱即用。这就是为什么它成了开发者的"标配"MCP。

### 实战 2：GitHub MCP——仓库操作

GitHub 官方推出的 MCP 服务器，支持完整的仓库管理操作：创建 Issue、管理 PR、搜索代码、查看 CI/CD 状态。

- **方式一：HTTP 远程连接**（推荐，GitHub 官方托管）
- **方式二：stdio 本地运行**（更灵活，可定制参数）

```bash
claude mcp add github -- docker run -i --rm \
  -e GITHUB_PERSONAL_ACCESS_TOKEN=${GITHUB_TOKEN} \
  ghcr.io/github/github-mcp-server
```

配置好之后，你可以直接在终端里操作 GitHub：

> 发现一个登录页面的 Bug，当用户输入超长密码时会崩溃，帮我创建一个 Issue

> Claude 输出如下：
>
> ……

> **注意**：`GITHUB_TOKEN` 需要事先创建。到 GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens，勾选你需要的仓库权限即可。

### 实战 3：Notion MCP——文档集成

Notion 官方开源了 MCP 服务器，让 Claude 可以直接读写你的 Notion 工作区。

> **注意**：`NOTION_API_KEY` 在 Notion Developers 创建 Internal Integration 后获取。记得在 Notion 页面的 Connections 里添加你创建的 Integration，否则 Claude 看不到页面内容。

> 从 Notion 里读取 "Q1 产品路线图"，帮我提取其中的技术任务

> Claude 输出如下：
>
> 读取 Notion 文档...
>
> [调用 Notion MCP server → search → get_page]
>
> 从 "Q1 产品路线图" 提取的技术任务：
>
>  ① 
>    • 支持 OAuth 2.0
>    • 添加双因素认证
>  ② 
>    • 实现全文搜索
>    • 添加搜索建议

### 实战 4：数据库——查询与分析

连接数据库是 MCP 最实用的场景之一。`@bytebase/dbhub` 支持 PostgreSQL、MySQL、SQLite 等多种数据库：

```json
{
  "mcpServers": {
    "database": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@bytebase/dbhub", "--dsn", "${DATABASE_URL}"]
    }
  }
}
```

> **注意**：`DATABASE_URL` 格式为 `postgresql://user:password@localhost:5432/mydb`。建议使用只读账户，防止 Claude 误操作修改数据。

> 帮我查一下数据库里上个月的订单数量和总金额

> Claude 输出如下：
>
> 查询数据库...
>
> [调用 database MCP server → run_query]
>
> 上个月（2025 年 12 月）的订单统计：
>
> • 订单数量：1,234 笔
> • 总金额：¥456,789.00
> • 平均客单价：¥370.21

把上面几个服务组合到一起，你就拥有了一个连接多个系统的完整工具箱。

以下是一个面向全栈开发者的 `.mcp.json` 配置：

```json
{
  "mcpServers": {
    "context7": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp@latest"]
    },
    "github": {
      "type": "http",
      "url": "https://api.github.com/mcp",
      "headers": {
        "Authorization": "Bearer ${GITHUB_TOKEN}"
      }
    },
    "notion": {
      "type": "http",
      "url": "https://mcp.notion.com/mcp",
      "headers": {
        "Authorization": "Bearer ${NOTION_API_KEY}"
      }
    },
    "database": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@bytebase/dbhub", "--dsn", "${DATABASE_URL}"]
    }
  }
}
```

对应的 `.env` 文件（绝对不要提交到版本控制）：

```bash
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
DATABASE_URL=postgresql://readonly:password@localhost:5432/mydb
NOTION_API_KEY=secret_xxxxxxxxxxxxxxxxxxxx
```

有了这个配置，你在终端里的对话可以跨越多个系统而不中断上下文。你可以一边讨论代码中的 Bug，一边查数据库确认问题，一边创建 GitHub Issue，一边翻 Notion 需求文档——Claude 全程保持上下文，不需要你在五个工具间来回切换。

![](https://static001.geekbang.org/resource/image/03/0c/037bbb7069f48c373aef35cb976c680c.jpg?wh=1536x1024) ![](https://static001.geekbang.org/resource/image/3f/54/3f87d8a92d2c698609f93f7c28836454.jpg?wh=2712x1321)

## 创建自定义 MCP 服务器

当现有的 MCP 服务器无法满足需求时，你可以创建自己的。MCP 官方提供了 TypeScript 和 Python 两套 SDK，开发一个基本的 MCP Server 只需要几十行代码。

### TypeScript SDK

TypeScript SDK 是使用最广泛的 MCP 开发工具。安装依赖：

```bash
npm install @modelcontextprotocol/sdk zod
```

下面是一个完整的 Todo 管理 MCP Server（`src/index.ts`）。它定义了三个工具（添加、列出、完成待办）和一个资源（统计信息）。注意每个工具都有名称、描述、参数 schema 和处理函数——Claude 通过描述来决定何时调用这个工具：

```typescript
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";

// 内存存储
const todos: { id: string; text: string; done: boolean }[] = [];

// 创建 MCP 服务器
const server = new McpServer({
  name: "my-todo-server",
  version: "1.0.0",
});

// 定义工具：添加待办
server.tool(
  "todo_add",
  "Add a new todo item",
  {
    text: z.string().describe("The todo text"),
  },
  async ({ text }) => {
    const todo = {
      id: Math.random().toString(36).substring(2, 9),
      text,
      done: false,
    };
    todos.push(todo);
    return {
      content: [
        {
          type: "text",
          text: `Added todo: ${todo.id} - ${todo.text}`,
        },
      ],
    };
  }
);

// 定义工具：列出待办
server.tool(
  "todo_list",
  "List all todo items",
  {},
  async () => {
    const text = todos.length === 0
      ? "No todos found."
      : todos
          .map((t) => `[${t.done ? "x" : " "}] ${t.id}: ${t.text}`)
          .join("\n");
    return {
      content: [{ type: "text", text: `Todos:\n${text}` }],
    };
  }
);

// 定义工具：完成待办
server.tool(
  "todo_complete",
  "Mark a todo as completed",
  {
    id: z.string().describe("The todo ID"),
  },
  async ({ id }) => {
    const todo = todos.find((t) => t.id === id);
    if (!todo) {
      return {
        content: [{ type: "text", text: `Todo not found: ${id}` }],
        isError: true,
      };
    }
    todo.done = true;
    return {
      content: [{ type: "text", text: `Completed: ${todo.text}` }],
    };
  }
);

// 定义资源：统计信息
server.resource(
  "stats",
  "stats://current",
  async (uri) => {
    return {
      contents: [
        {
          uri: uri.href,
          mimeType: "application/json",
          text: JSON.stringify({
            total: todos.length,
            completed: todos.filter((t) => t.done).length,
            pending: todos.filter((t) => !t.done).length,
          }, null, 2),
        },
      ],
    };
  }
);

// 启动服务器
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("MCP Server started");
}

main().catch(console.error);
```

### Python SDK

Python 版本使用装饰器风格，对 Python 开发者来说更加自然。

```bash
pip install mcp
```

```python
from mcp.server.fastmcp import FastMCP

server = FastMCP("my-todo-server")

todos = []

@server.tool("todo_add")
async def add_todo(text: str) -> str:
    """Add a new todo item"""
    import random
    import string

    todo_id = ''.join(random.choices(string.ascii_lowercase, k=7))
    todos.append({"id": todo_id, "text": text, "done": False})
    return f"Added todo: {todo_id} - {text}"

@server.tool("todo_list")
async def list_todos() -> str:
    """List all todo items"""
    if not todos:
        return "No todos found."
    return "\n".join(
        f"[{'x' if t['done'] else ' '}] {t['id']}: {t['text']}"
        for t in todos
    )

@server.tool("todo_complete")
async def complete_todo(id: str) -> str:
    """Mark a todo as completed"""
    for todo in todos:
        if todo["id"] == id:
            todo["done"] = True
            return f"Completed: {todo['text']}"
    return f"Todo not found: {id}"

if __name__ == "__main__":
    server.run()
```

### 配置自定义服务器

写完代码后在 `.mcp.json` 中注册。TypeScript 版本先编译再运行，Python 版本直接运行：

**TypeScript 版本：**

```json
{
  "mcpServers": {
    "my-todo": {
      "type": "stdio",
      "command": "node",
      "args": ["./mcp-server/build/index.js"]
    }
  }
}
```

**Python 版本：**

```json
{
  "mcpServers": {
    "my-todo": {
      "type": "stdio",
      "command": "python",
      "args": ["./mcp-server/server.py"]
    }
  }
}
```

## 5 条 MCP 安全原则

MCP 的强大能力也带来了安全风险。它本质上是在给 AI Agent 开放访问外部系统的权限。正如 Anthropic 官方警告里说的：

> 使用第三方 MCP 服务器需自担风险。Anthropic 未验证所有服务器的正确性和安全性。

这里给出五条 MCP 使用的安全原则：

1. **验证服务器来源**——只使用官方或知名来源的 MCP 服务器。一个恶意的 MCP Server 可以在你不知情的情况下读取敏感文件、窃取环境变量中的密钥。

2. **限制权限范围**——遵循最小权限原则，只给必要的目录和资源访问：

```json
{
  "mcpServers": {
    "filesystem": {
      "type": "stdio",
      "command": "npx",
      "args": [
        "@modelcontextprotocol/server-filesystem",
        "/safe/directory/only"
      ]
    }
  }
}
```

3. **使用只读凭证**——对于数据库等关键系统，永远不要给 MCP Server 写权限——除非你明确需要 Claude 修改数据。

4. **保护敏感凭证**——绝对不要在配置文件中硬编码 Token。使用环境变量引用，将敏感值存在不提交到 git 的文件中。

5. **审计 MCP 服务器代码**——对于开源服务器，在使用前检查其代码：它请求哪些权限？它如何处理用户数据？花十分钟审计代码，可能帮你避免一次严重的安全事故。

![](https://static001.geekbang.org/resource/image/b1/7e/b19c952d151228c2ff36e40dc548c37e.png?wh=1536x1024) ![](https://static001.geekbang.org/resource/image/10/80/10d293b9f6474fca5741596c277a0f80.jpg?wh=2838x1442)

## 调试与故障排除

MCP 配置好之后，可能不会一次就跑通。Claude Code 内置了调试工具：

```bash
# 列出所有配置的服务器
claude mcp list

# 查看服务器详细信息
claude mcp get my-server

# 启用调试模式查看 MCP 连接详情
claude --debug
```

常见问题速查表如下。

![](https://static001.geekbang.org/resource/image/00/47/00feb48bd28c519af480fa0ab7d96647.jpg?wh=2972x1176)

MCP 工具可能产生大量输出。因此在 Token 的控制方面，Claude Code 对 MCP 输出提供了两级保护。

![](https://static001.geekbang.org/resource/image/7f/20/7fbb1997774a2d9258deb08ae4ed6620.jpg?wh=2907x827)

如果需要处理大量数据，可以通过环境变量调整上限。但更好的做法是在 MCP Server 端做分页或摘要：

```bash
export MAX_MCP_OUTPUT_TOKENS=50000
```

### MCP Tool Search：渐进式工具加载

上面的两级保护针对的是 MCP **输出**端。Claude Code 在**输入**端同样有优化——当 MCP Server 暴露大量工具时，把所有工具的完整 Schema（名称、描述、参数定义）全部塞进上下文会大量消耗 token。Claude Code 的解决方案是 **MCP Tool Search**，一套自动的渐进式加载机制（[官方文档](https://code.claude.com/docs/en/mcp)）：

**工作流程：**

1. 启动时连接所有 MCP Server，通过 `tools/list` 获取全部工具定义
2. 计算所有工具 Schema 的总 token 量
3. **若总 token < 上下文的 ~10%（约 10K tokens）**：全量注入，和之前一样——小规模场景不受影响
4. **若超出阈值**：
   - 工具标记为 `defer_loading: true`
   - 上下文中**只保留工具名称列表**（不含参数 Schema）
   - Claude 获得一个特殊的 `ToolSearch` 工具
   - 需要某工具时，用关键词搜索 → 返回 **3-5 个最匹配工具的完整定义**
   - 搜索结果在**会话内缓存**，避免重复搜索

**和 Skills 的类比**：Skills 靠 Read 工具按需加载文件内容，MCP Tool Search 靠搜索工具按需加载工具 Schema。核心思路一致——先用轻量描述让 LLM 确认意图，再按需拉取完整细节。代价是损失了 LLM 的 key-value 缓存命中率（因为工具定义从「始终在上下文」变成了「动态注入」），但换来了成百上千个工具可以接入而不会撑爆上下文窗口。

这一机制是 Claude Code **官方内置的 harness 层优化**，默认开启，无需额外配置。

## 反思：MCP 的边界与 LLM 时代的协议退化

### Skill + CLI：平推大部分场景

一个无法回避的趋势是：**越来越多的场景正在从 MCP 转向 Skill + Bash 脚本**。

这不是因为 MCP 设计得不好。恰恰相反——MCP 在架构上是"更好"的方案：标准化的接口声明、结构化的错误格式、JSON Schema 的参数校验。问题在于：**工程选型从来不是选最好的，而是选"刚好够用且成本最低"的**。

Skill + Bash 的组合能覆盖 80% 以上的"连接外部世界"场景，而工程成本远低于写一个 MCP Server。当 LLM 已经足够聪明到：

- 能读懂 stderr 自行判断错误原因
- 能解析任意输出格式（JSON、CSV、纯文本）
- 能根据自然语言描述自己编排 Bash 命令流程

那么 JSON Schema 的参数校验、标准化的错误结构、显式的工具声明——这些"协议层保障"的边际价值就大幅下降了。

### 更深的趋势：LLM 理解能力正在消解结构化协议的必要性

这是比"MCP vs Skill"更大的一个趋势。

MCP、LSP、REST、gRPC……这些协议的共同假设是：**通信双方是"哑"的**，所以需要提前约定格式、显式声明接口、标准化错误路径。

LLM 正在让通信的一端变得"聪明"——它能容忍模糊、能自行修复、能从失败中学习。协议的价值与通信对象的愚蠢程度成正比。当对象变聪明了，协议就可以变薄。

这不是"MCP 设计得不好"，而是 **MCP 设计时所假设的通信模型，正在被 LLM 的能力模型超越**。

我们这代工程师的知识体系建立在一系列稳定假设上：

- **接口要显式声明** → 因为调用方不会自行推断
- **错误要标准化** → 因为错误恢复需要可编程的结构
- **类型要严格** → 因为类型错误不会被自动纠正

LLM 让这三个假设同时松动。**不是渐进改良，而是地基在动。** 站在 2024 年的最佳实践上做的完美设计，2026 年的 LLM 可能直接越过它——不是设计者做错了，而是做得太对了以至于过时了。这就是工程底座漂移带来的"尴尬的正确"。

### 实践检验：MCP 六大优势的真实权重

把 MCP 官方宣称的优势逐条放到 Skill + CLI 的视角下检验，会发现一件尴尬的事：

| MCP 优势 | 理论说法 | 实践中的真实权重 |
|---|---|---|
| **工具声明** | Claude 自动发现工具 | Skill 的 description 同样让 Claude 知道"有这个能力"。触发精度甚至更高——MCP 用 JSON Schema，Skill 用自然语言 |
| **参数校验** | JSON Schema 约束 | Claude 生成的 curl/脚本参数错误率极低。LLM 的语言理解能力让"结构化约束"变得没那么必要 |
| **错误处理** | 标准错误格式 | Claude 读 stderr 和 exit code 完全没问题。Skill 还能写"如果脚本返回非零，执行以下排错步骤" |
| **可发现性** | 启动时自动发现 | Skill 也是自动发现。且 Skill 的渐进式披露比 MCP Tool Search 更早出现、更原生 |
| **团队共享** | settings.json 统一注册 | `.claude/skills/` 目录提交 Git，同样共享。脚本也在里面 |
| **跨工具组合** | 工具列表统一 | Skill 可以在流程中自由组合多个 Bash 脚本调用 |

**六项优势中，没有一项在实践中构成"非 MCP 不可"的硬性约束。** 每一项都有 Skill + Bash 的"够用"替代方案。就连之前认为的"不可替代阵地"——有状态连接、复杂认证、大规模工具集——在实践中也能被 Skill 覆盖：数据库？`psql` 一条命令。OAuth？shell 脚本写 token 刷新。大规模工具？一个 `.md` 文件里组织几十个 Bash 函数。

### MCP 真正的剩余价值：生态惯性

那为什么 MCP 仍然存在？

只剩一条：**生态——社区已经写好了高质量 Server，"直接用"比自己写脚本省事。**

但这个优势也在被侵蚀。优秀的 MCP Server 本质上就是一个包装良好的 CLI 工具加上 JSON Schema 声明。社区同样可以发布优秀的 Skill 文件。最终比的不是协议谁更优雅，是**谁的生态先积累到临界质量**。

MCP 在这场竞赛中有一个结构性劣势：它要求 Server 作者懂协议规范、懂 SDK、懂 JSON-RPC。Skill 要求什么？**会写 Markdown 和 Bash 就行。** 准入门槛差了一个数量级。生态竞争拼的不是"谁更标准"，是"谁更容易贡献"。

### 结论：MCP 在为过度工程化支付认知税

MCP 没有被 Skill "取代"——**MCP 是在为自己的过度工程化支付认知税。**

当一个协议需要用户先理解 Client-Server 架构、三种传输方式、JSON Schema 工具定义，才能写出第一个能跑的 Demo 时，它就在系统性拒绝潜在贡献者。而 Skill 只需要打开一个 `.md` 文件，写下 `## 工具描述` 和 ` ```bash ``` `。

这和最初学 MCP 时的感受完全一致——先用 Kiro、Codex 等工具摸索 SSE、HTTP、stdio 三种传输方式，搞得有点复杂；后面发现用 HTTP 其实就够了，简单方便。再后来发现 Skill + CLI 能完全替代，链路更清晰、维护成本更低。

> **核心洞察**：结构化协议的必要性正在被 LLM 的理解能力消融。这不是 MCP 的问题，而是整个软件工程范式正在经历的底层震荡。MCP 是旧范式下的优秀设计，Skill + CLI 是新范式下的暴力美学。我们设计的底座在变，昨天的最佳实践可能明天就不再成立——我们唯一能做的，是看清趋势，在合适的场景选合适的工具，而不是固守任何一种"正确"。

## 总结一下

MCP 解决了 AI 助手与外部世界连接的碎片化问题——从 M × N 个适配器简化为 M + N 个。它采用 Client-Server 架构，支持三种传输方式（stdio、HTTP、SSE），配置文件可版本控制并团队共享。

生态系统是 MCP 最大的优势。官方提供文件系统、HTTP 请求、Git、PostgreSQL 等基础服务器；第三方则覆盖了 GitHub、Notion 等主流服务。我们动手配置了几个最典型的 MCP 服务。当然，强大的能力也带来安全风险，我们给出了五条安全原则。更多的 MCP 相关细节，可以阅读我在极客时间的 MCP 专栏。

写到这里，我们看到 MCP 让 Claude Code 从一个"只能操作本地文件"的工具，进化为"能连接整个数字世界"的智能助手。但这里有个有趣的问题——我感觉前面的基础篇课程中缺失了重要一环，就是 Claude Code 内置的那些工具本身：Read、Write、Bash、Grep……这些"原生工具"是怎么组织的，以至于产生了那么强大的效果？这也值得我们好好思考思考。

## 思考题

——

## 下一讲预告

MCP 让我们能连接外部世界，但 Claude Code 自身的工具系统——Read、Write、Bash、Grep、Glob、Agent……这十几个内置工具是怎么组织的？它们的权限如何控制？和 MCP 扩展的工具又是什么关系？

第 18 讲，我们将深入剖析 Claude Code 的工具系统（Tools）全貌——从 Agentic Loop 的三个阶段、15+ 内置工具的分类与风险等级、五种软件工程原子操作，到工具的权限控制体系。这一节是新加进来的，我认为相当重要（本应放在基础篇，不过，因为目前大家知识体系已经全面，放在这里我们理解起来更深入），因为理解了工具系统，你才能真正理解 Claude Code 这台"AI 引擎"是如何运转的。

欢迎你在留言区和我交流讨论。如果这一讲对你有启发，别忘了分享给身边更多朋友。
