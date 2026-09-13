---
title: v7-异步Web服务版
created: 2026-08-07
project: 毕业项目-Rust任务管理器
version: 7
difficulty: ⭐⭐⭐⭐⭐
estimated_hours: 6
tags:
  - Rust
  - 毕业项目
lark_doc_url: https://my.feishu.cn/docx/BujrdMV5ko2EJlx4LQ1cwY8LnyU
---

## ⬅️ 前置知识
- [[01-学习/Rust学习路径/毕业项目-Rust任务管理器/v6-测试与CI版|v6-测试与CI版]] — 上一版本

## ➡️ 后续版本
- 无，毕业项目终点，可尝试跨技术栈组合项目

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引

---

## 🎯 v7 目标

毕业项目的最终形态：用 tokio + axum 把任务管理器升级为 REST API——`GET /tasks`、`POST /tasks`、`PATCH /tasks/:id`，各 handler 通过 `Arc<tokio::sync::Mutex<Vec<Task>>>` 共享状态，请求与响应全部走 serde JSON。扩展方向：把内存存储换成 PostgreSQL（sqlx），呼应跨技术栈路径。核心关注点是异步编程与工程集成，是阶段三（并发、异步与工程化）的收官。

---

## 📋 改造任务

### 1. 任务说明
基于上一版本代码，完成以下改造：
1. 引入 tokio + axum，把 main 改造为异步运行时入口（`#[tokio::main]`）
2. 实现三个路由：`GET /tasks`、`POST /tasks`、`PATCH /tasks/:id`（serde JSON 交互）
3. 用 `Arc<tokio::sync::Mutex<Vec<Task>>>` 在各 handler 间共享状态，理解异步锁的选择
4. 用 curl 测通 API，并把常用请求示例写进 README / 注释
5. （扩展方向）用 sqlx + PostgreSQL 替换内存存储，路由层保持不变
6. 提供最小可运行步骤

### 2. 验收标准

| 检查项 | 方式 |
|--------|------|
| 功能完整 | 运行并通过手动测试 |
| 代码规范 | 符合 Rust 社区最佳实践（cargo fmt/clippy） |
| 可复现 | 提供最小运行步骤 |

---

## 💡 参考实现

<details>
<summary>点击展开参考答案</summary>

Cargo.toml 依赖：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

关键代码：

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};

type AppState = Arc<tokio::sync::Mutex<Vec<Task>>>;

async fn list_tasks(State(store): State<AppState>) -> Json<Vec<Task>> {
    Json(store.lock().await.clone())
}

async fn create_task(
    State(store): State<AppState>,
    Json(input): Json<CreateTaskInput>,
) -> (StatusCode, Json<Task>) {
    let mut store = store.lock().await;
    let id = store.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let task = Task { id, title: input.title, done: false };
    store.push(task.clone());
    (StatusCode::CREATED, Json(task))
}

#[tokio::main]
async fn main() {
    let store: AppState = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", patch(patch_task)) // 注意：axum 0.7 用 :id，0.8+ 改为 {id}
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

扩展方向（跨技术栈）：加 `sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }`，把 `AppState` 换成 `Arc<PgPool>`，handler 内改为 SQL 查询——路由层一行不用改，这就是分层的意义。

完整参考：`code/v7/main.rs`

</details>

---

## 🎯 本版自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能按步骤跑通本版本代码 | [ ] |
| 🟡 进阶 | 能解释本版本涉及的核心原理 | [ ] |
| 🔴 挑战 | 能独立扩展一个功能点 | [ ] |

---

*最后更新：2026-08-07*
