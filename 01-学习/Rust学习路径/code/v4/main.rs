//! v4-Trait抽象版
//!
//! Cargo.toml 依赖（与 v3 相同）：
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```
//!
//! 核心要点：
//!   1. 定义 Storage trait（load / save），把"数据存在哪"抽象出来
//!   2. 实现 FileStorage（tasks.json）与 MemoryStorage（不持久化）
//!   3. --backend=file|memory 运行时选择实现，Box<dyn Storage> 注入（动态分发）
//!
//! 运行：
//!   cargo run -- --backend=file add 买牛奶
//!   cargo run -- --backend=memory add 临时任务 list
//!
//! 另一种写法（静态分发）：用泛型注入 `struct App<S: Storage> { storage: S, tasks: Vec<Task> }`。
//! 泛型会单态化、性能更好，但每种实现生成一份代码且无法运行时选择；
//! Box<dyn Storage> 多一层 vtable 间接寻址，但可以运行时切换实现——本版本选后者。

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    NotFound(u64),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误: {}", e),
            AppError::Json(e) => write!(f, "JSON 解析错误: {}", e),
            AppError::NotFound(id) => write!(f, "未找到任务 #{}", id),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Json(e)
    }
}

/// 存储抽象：TaskStore 不关心"数据存在哪"，只通过这个 trait 交互
trait Storage {
    fn load(&self) -> Result<Vec<Task>, AppError>;
    fn save(&self, tasks: &[Task]) -> Result<(), AppError>;
    /// 后端名称，用于提示输出
    fn name(&self) -> &'static str;
}

/// 文件后端：tasks.json
struct FileStorage {
    path: PathBuf,
}

impl Storage for FileStorage {
    fn load(&self) -> Result<Vec<Task>, AppError> {
        match fs::read_to_string(&self.path) {
            Ok(content) => Ok(serde_json::from_str(&content)?),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()), // 首次运行
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, tasks: &[Task]) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "file"
    }
}

/// 内存后端：load 恒为空，save 是空操作——进程退出即丢失，
/// 适合演示 / 测试等不想落盘的场景
struct MemoryStorage;

impl Storage for MemoryStorage {
    fn load(&self) -> Result<Vec<Task>, AppError> {
        Ok(Vec::new())
    }

    fn save(&self, _tasks: &[Task]) -> Result<(), AppError> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "memory"
    }
}

/// 纯内存工作区：不碰任何 IO，业务逻辑只与它交互
struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    /// id 从现有数据推导（最大值 + 1），不再依赖持久化的 next_id
    fn add(&mut self, title: String) -> u64 {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task {
            id,
            title,
            done: false,
        });
        id
    }

    fn finish(&mut self, id: u64) -> Result<(), AppError> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                Ok(())
            }
            None => Err(AppError::NotFound(id)),
        }
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("（暂无任务）");
            return;
        }
        for task in &self.tasks {
            let mark = if task.done { "x" } else { " " };
            println!("[{}] #{} {}", mark, task.id, task.title);
        }
    }
}

#[derive(Debug)]
enum Command {
    Add(String),
    List,
    Done(u64),
    Help,
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    let mut iter = args.iter().map(String::as_str);
    match iter.next() {
        None => Ok(Command::Help),
        Some("add") => {
            let title: Vec<&str> = iter.collect();
            if title.is_empty() {
                Err("add 需要任务标题".to_string())
            } else {
                Ok(Command::Add(title.join(" ")))
            }
        }
        Some("list") => Ok(Command::List),
        Some("done") => {
            let raw = iter
                .next()
                .ok_or_else(|| "done 需要任务 id".to_string())?;
            let id: u64 = raw
                .parse()
                .map_err(|_| format!("无法解析任务 id: {}", raw))?;
            Ok(Command::Done(id))
        }
        Some("help") | Some("--help") | Some("-h") => Ok(Command::Help),
        Some(other) => Err(format!("未知子命令: {}", other)),
    }
}

fn print_help() {
    println!("Rust 任务管理器 v4（Trait 抽象版）");
    println!("用法: main [--backend=file|memory] <命令> [参数]");
    println!("  add <标题>   添加任务");
    println!("  list         列出所有任务");
    println!("  done <id>    标记任务完成");
    println!("  help         显示帮助");
}

fn main() -> Result<(), AppError> {
    let all: Vec<String> = std::env::args().skip(1).collect();

    // 先抽出 --backend=xxx，其余参数留给子命令解析
    let mut backend = String::from("file");
    let mut rest: Vec<String> = Vec::new();
    for arg in all {
        if arg.starts_with("--backend=") {
            backend = arg["--backend=".len()..].to_string();
        } else {
            rest.push(arg);
        }
    }

    // 运行时选择实现，以 Box<dyn Storage> 注入（动态分发）
    let storage: Box<dyn Storage> = match backend.as_str() {
        "file" => Box::new(FileStorage {
            path: PathBuf::from("tasks.json"),
        }),
        "memory" => Box::new(MemoryStorage),
        other => {
            eprintln!("未知后端: {}（可选 file / memory）", other);
            std::process::exit(1);
        }
    };
    println!("[info] 使用 {} 后端", storage.name());

    let command = match parse_args(&rest) {
        Ok(cmd) => cmd,
        Err(msg) => {
            eprintln!("解析错误: {}", msg);
            print_help();
            std::process::exit(1);
        }
    };

    let mut store = TaskStore {
        tasks: storage.load()?,
    };

    match command {
        Command::Add(title) => {
            let id = store.add(title);
            storage.save(&store.tasks)?; // 修改数据后立即落盘
            println!("已添加任务 #{}", id);
        }
        Command::List => store.list(),
        Command::Done(id) => {
            store.finish(id)?;
            storage.save(&store.tasks)?;
            println!("任务 #{} 已完成", id);
        }
        Command::Help => print_help(),
    }
    Ok(())
}
