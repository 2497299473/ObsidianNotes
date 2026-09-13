---
title: 03-异步编程与Tokio
created: 2026-08-07
stage: 3
order: 3
difficulty: ⭐⭐⭐⭐⭐
estimated_hours: 6
tags:
  - Rust
  - 异步编程
  - Tokio
  - axum
description: 理解 async/await 状态机与 Future 模型，掌握 tokio 运行时，并用 axum 写出最小 HTTP 服务
lark_doc_url: https://my.feishu.cn/docx/GOCHd5zX2oCL6JxYDM6ceN1pnWc
---

## ⬅️ 前置知识
- [[02-共享状态并发]] — 上一课：Mutex、Arc 与共享状态

## ➡️ 后续笔记
- [[04-测试、文档与工程实践]] — 下一课：测试与工程化

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引
- [[01-学习/Rust学习路径/阶段三-并发、异步与工程化/99-阶段3复习检查点|99-阶段3复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 解释 async/await 在编译期被转换成状态机、Future 通过 poll 模型驱动的原理
2. 说明为什么异步代码需要运行时，并使用 tokio 的 `spawn`、`select!`、`sleep` 组织并发任务
3. 理解 `.await` 的传染性，知道跨 await 持锁时为何要用 `tokio::sync::Mutex`
4. 用 axum 写一个返回 JSON 的最小 HTTP 服务，并知道何时使用 `spawn_blocking`

---

## 1. async/await 的本质：状态机与 Future

Rust 的异步是**零成本**的：`async fn` 并不会立即执行，而是被编译器转换成一个实现了 `Future` trait 的匿名状态机。每个 `.await` 点是状态机的一个暂停位置。`Future` 的核心签名（简化自标准库）：

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // 执行器反复调用 poll：
    // Poll::Pending 表示还没准备好，Poll::Ready(v) 表示完成
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

工作机制：
- 第一次 `poll` 时状态机开始执行，遇到 `.await` 的子 Future 未完成就返回 `Pending`
- 当底层 IO 就绪（epoll/IOCP），执行器再次 `poll`，状态机从上次的位置继续
- 全程没有额外的 OS 线程——成千上万个 Future 可以轮流跑在少量工作线程上

```rust
use tokio::time::{sleep, Duration};

async fn fetch_user() -> &'static str {
    sleep(Duration::from_millis(100)).await; // 模拟网络请求，期间让出线程
    "user: alice"
}

async fn fetch_orders() -> &'static str {
    sleep(Duration::from_millis(150)).await;
    "orders: 3"
}

#[tokio::main]
async fn main() {
    // join! 让两个 Future 并发推进：总耗时取决于较慢者（约 150ms），
    // 而不是顺序 await 的 250ms
    let (user, orders) = tokio::join!(fetch_user(), fetch_orders());
    println!("{user}, {orders}");
}
```

> 💡 提示：`async fn` 只是「构造」了一个 Future，不 `.await`（或不交给执行器）就永远不会执行——这是 Rust 与 JavaScript 的一个重要差异：Rust 的 Future 是惰性的。

## 2. 运行时：tokio 的 main、spawn、select!、sleep

Rust 语言本身**不内建运行时**（对比 Go 自带调度器）：标准库没有事件循环，Future 需要一个执行器来 poll 它。tokio 是目前最主流的运行时，`#[tokio::main]` 宏会把 `async fn main` 展开为「构建多线程运行时并 block_on 执行」的普通 main。

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // spawn 把一个 Future 交给运行时调度，返回 JoinHandle
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        42
    });

    // select!：同时等待多个分支，谁先就绪执行谁，其余分支被取消（drop）
    tokio::select! {
        v = handle => println!("任务完成，结果: {:?}", v),
        _ = sleep(Duration::from_secs(5)) => println!("超时！"),
    }
}
```

常用工具速查：
- `tokio::spawn`：提交一个独立任务（类比「轻量级线程」，成本远低于 std::thread::spawn）
- `tokio::time::sleep`：异步睡眠，让出当前工作线程（**不要**用 `std::thread::sleep`，它会阻塞整个工作线程）
- `tokio::select!`：多路等待 + 超时控制
- `tokio::join!`：并发等待多个 Future 全部完成

## 3. .await 的传染性与 tokio::sync

异步有「传染性」：调用链上只要有一环是 async，外层就必须是 async 并加 `.await`，一路传到入口。跨任务通信要用异步版的同步原语：

```rust
#[tokio::main]
async fn main() {
    // tokio::sync::mpsc：带缓冲的异步通道
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    tokio::spawn(async move {
        tx.send("来自异步任务的消息".to_string()).await.unwrap();
        // tx 随任务结束被 drop
    });

    // 所有发送端 drop 后，recv 返回 None，循环结束
    while let Some(msg) = rx.recv().await {
        println!("收到: {msg}");
    }
}
```

**跨 await 持锁必须用 `tokio::sync::Mutex`**，原因有二：
1. `std::sync::MutexGuard` 不是 `Send`：tokio 多线程运行时可能把任务在工作线程间挪动，持锁跨 await 的 Future 不再是 Send，`tokio::spawn` 直接编译失败
2. 即使能编译，任务在持锁时被挂起（让出线程），锁会一直被占着，阻塞其他工作线程

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut guard = counter.lock().await; // 异步加锁
            *guard += 1;
            // 锁跨越了下面的 await 点——这正是必须用 tokio::sync::Mutex 的场景
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
    println!("counter = {}", *counter.lock().await); // 5
}
```

> 💡 提示：如果临界区内**没有** await，用 `std::sync::Mutex` 反而更快。规则是「锁是否跨越 await 点」，而不是「异步代码一律用 tokio Mutex」。

## 4. 实战：用 axum 写最小 HTTP 服务

axum 是 tokio 生态的 Web 框架。下面是一个返回 JSON 任务列表的最小服务：

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

// handler 是 async fn，返回 Json<T> 会自动序列化为 application/json
async fn list_tasks() -> Json<Vec<Task>> {
    let tasks = vec![
        Task { id: 1, title: "学习所有权".into(), done: true },
        Task { id: 2, title: "写一个异步服务".into(), done: false },
    ];
    Json(tasks)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/tasks", get(list_tasks));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("监听 http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

运行后访问 `http://127.0.0.1:3000/tasks`，得到 JSON 数组。注意 serde 的 `Serialize` derive：它让结构体与 JSON 之间的转换由编译器生成，这正是毕业项目中任务数据序列化的基础。

## 5. spawn_blocking 与选型

异步只解决「等待」，不解决「计算」。在 async 任务里做 CPU 密集计算或同步 IO，会把整个工作线程卡住，拖垮同一运行时上的所有任务。对策是 `spawn_blocking`：

```rust
#[tokio::main]
async fn main() {
    // 把阻塞工作丢到专门的阻塞线程池，不占用异步工作线程
    let result = tokio::task::spawn_blocking(|| {
        // 模拟 CPU 密集计算或同步文件 IO
        (0..1_000_000).filter(|n| n % 7 == 0).count()
    })
    .await
    .unwrap();

    println!("结果: {result}");
}
```

异步 vs 多线程选型：

| 场景 | 推荐方案 |
|------|----------|
| 海量并发 IO（HTTP、数据库、文件读写等待） | async/await + tokio |
| CPU 密集计算 | rayon 数据并行，或 spawn_blocking |
| 少量简单并行任务 | std::thread / thread::scope |
| IO 为主、夹杂少量重计算 | 异步主循环 + spawn_blocking 处理计算 |

---

## 常见陷阱

| 陷阱 | 表现 | 对策 |
|------|------|------|
| 忘记 .await | Future 不执行，编译器警告「unused implementer of Future」 | 看到警告立即补上 .await 或显式 drop |
| 在异步里做同步阻塞（std::thread::sleep、同步 HTTP 客户端） | 整个工作线程卡住，其他任务全部停滞 | 用 tokio::time::sleep、异步客户端，或 spawn_blocking |
| 跨 .await 持 std::sync::Mutex | Future 不再是 Send，spawn 编译失败；或锁长期不释放 | 改用 tokio::sync::Mutex，或把临界区控制在 await 之前 |
| 误以为 select! 的落选分支会完成 | 落选分支的 Future 被直接 drop（取消） | 理解取消语义；需要保底完成的任务单独 spawn |
| 在同步 fn 里调用 async fn | 编译错误「expected opaque type Future」 | 把调用链改成 async，或在入口处 block_on |

---

## ✏️ 综合练习

### 练习 1：概念复述
1. `Future` trait 的核心方法是什么？`Poll::Pending` 和 `Poll::Ready` 分别代表什么？
2. 为什么 Rust 的异步代码需要一个「运行时」？`#[tokio::main]` 大致展开成了什么？
3. 什么情况下必须用 `tokio::sync::Mutex` 而不是 `std::sync::Mutex`？

### 练习 2：代码实现
在 `code/` 目录新建一个 cargo 项目（如 `code/async-server/`），实现练习 4 中的最小 axum 服务：`GET /tasks` 返回至少包含 `id`、`title`、`done` 三个字段的 JSON 数组（至少 3 条数据）。用 `curl` 或浏览器验证返回结果，并把验证命令写进项目 README。

### 练习 3：扩展思考
Rust 语言刻意不内建运行时（对比 Go 自带调度器），异步生态因此出现了 tokio、async-std、smol 等多种运行时。这种「库化」的设计给库作者和使用方分别带来了什么好处与代价？（提示：想想「颜色问题」与依赖选择。）

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能解释 Future/poll 模型，写出 tokio::main + spawn 的最小示例 | [ ] |
| 🟡 进阶 | 能跑通 axum 的 /tasks 服务并说明 serde 的作用 | [ ] |
| 🔴 挑战 | 能说明跨 await 持 std Mutex 的两个具体问题，并用 tokio Mutex 写出正确版本 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-08-07*
