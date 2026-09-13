//! v7-异步Web服务版：tokio + axum 把任务管理器升级为 REST API
//!
//! Cargo.toml 依赖：
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! axum = "0.7"
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```
//!
//! 运行与测试：
//!   cargo run
//!   curl http://127.0.0.1:3000/tasks
//!   curl -X POST http://127.0.0.1:3000/tasks -H "Content-Type: application/json" -d "{\"title\":\"买牛奶\"}"
//!   curl -X PATCH http://127.0.0.1:3000/tasks/1 -H "Content-Type: application/json" -d "{\"done\":true}"
//!
//! 要点：
//!   1. #[tokio::main] 启动异步运行时
//!   2. 状态用 Arc<tokio::sync::Mutex<Vec<Task>>> 在多个 handler 之间共享；
//!      跨 .await 的临界区必须用 tokio 的异步锁（std Mutex 的 guard 不是 Send 友好的用法）
//!   3. 请求体/响应体全部走 serde JSON
//!
//! 注意：axum 0.7 的路径参数写作 "/tasks/:id"；0.8+ 改为 "/tasks/{id}"。
//!
//! ── 扩展方向：接 PostgreSQL（sqlx），呼应跨技术栈路径 ──────────────────
//!   Cargo.toml 追加：
//!     sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "macros"] }
//!   做法：
//!     let pool = PgPool::connect("postgres://user:pass@localhost/todo").await?;
//!     把 AppState 换成 Arc<PgPool>，handler 内部改为：
//!       sqlx::query_as!(Task, "SELECT id, title, done FROM tasks ORDER BY id")
//!           .fetch_all(&*store).await
//!     路由层一行不用改——这就是分层的意义。

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// 共享状态：Arc 让多个 handler 都能持有，Mutex 保证同一时刻只有一个在写
type AppState = Arc<Mutex<Vec<Task>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

/// POST /tasks 的请求体
#[derive(Debug, Deserialize)]
struct CreateTaskInput {
    title: String,
}

/// PATCH /tasks/:id 的请求体：所有字段可选，只更新提供了的字段
#[derive(Debug, Deserialize)]
struct PatchTaskInput {
    title: Option<String>,
    done: Option<bool>,
}

/// GET /tasks：列出全部任务
async fn list_tasks(State(store): State<AppState>) -> Json<Vec<Task>> {
    let store = store.lock().await;
    Json(store.clone())
}

/// POST /tasks：新建任务，返回 201 与创建的任务
async fn create_task(
    State(store): State<AppState>,
    Json(input): Json<CreateTaskInput>,
) -> (StatusCode, Json<Task>) {
    let mut store = store.lock().await;
    let id = store.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let task = Task {
        id,
        title: input.title,
        done: false,
    };
    store.push(task.clone());
    (StatusCode::CREATED, Json(task))
}

/// PATCH /tasks/:id：局部更新（标题 / 完成状态），未找到返回 404
async fn patch_task(
    State(store): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<PatchTaskInput>,
) -> Result<Json<Task>, StatusCode> {
    let mut store = store.lock().await;
    match store.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            if let Some(title) = input.title {
                task.title = title;
            }
            if let Some(done) = input.done {
                task.done = done;
            }
            Ok(Json(task.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    // 预置一条任务，方便启动后立即体验 GET /tasks
    let store: AppState = Arc::new(Mutex::new(vec![Task {
        id: 1,
        title: String::from("学习 Rust 异步编程"),
        done: false,
    }]));

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", patch(patch_task))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("绑定端口失败");
    println!("任务管理器 API 已启动：http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("服务运行异常");
}
