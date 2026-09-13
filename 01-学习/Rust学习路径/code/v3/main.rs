//! v3-持久化与错误处理版
//!
//! 本版本需要第三方依赖，请用 cargo 建项目后把本文件粘贴为 src/main.rs。
//! Cargo.toml 依赖：
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```
//!
//! 核心要点：
//!   1. Task 序列化/反序列化为 JSON，保存到 tasks.json，启动时加载
//!   2. 自定义 AppError 统一错误类型 + From 实现 + ? 传播
//!   3. main 返回 Result<(), AppError>，不再靠 panic / process::exit 处理错误
//!
//! 运行：
//!   cargo run -- add 买牛奶
//!   cargo run -- list
//!   cargo run -- done 1

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// 数据文件（与运行时工作目录同级）
const DB_FILE: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

/// 文件整体结构：任务列表 + next_id 一起持久化，保证重启后 id 继续递增
#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    next_id: u64,
    tasks: Vec<Task>,
}

/// 统一错误类型：聚合本程序可能出现的三类错误
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

// 有了这两个 From，? 运算符就能自动把 io::Error / serde_json::Error 转换成 AppError
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

struct TaskStore {
    data: FileData,
    path: PathBuf,
}

impl TaskStore {
    /// 从文件加载；文件不存在视为首次运行，返回空存储
    fn load(path: &Path) -> Result<Self, AppError> {
        let data = match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)?, // ? 自动走 From<serde_json::Error>
            Err(e) if e.kind() == ErrorKind::NotFound => FileData {
                next_id: 1,
                tasks: Vec::new(),
            },
            Err(e) => return Err(e.into()),
        };
        Ok(TaskStore {
            data,
            path: path.to_path_buf(),
        })
    }

    fn save(&self) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.path, json)?; // ? 自动走 From<io::Error>
        Ok(())
    }

    fn add(&mut self, title: String) -> u64 {
        let id = self.data.next_id;
        self.data.next_id += 1;
        self.data.tasks.push(Task {
            id,
            title,
            done: false,
        });
        id
    }

    fn finish(&mut self, id: u64) -> Result<(), AppError> {
        match self.data.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                Ok(())
            }
            // 业务错误：作为 NotFound 返回，让调用方决定怎么处理（这里一路 ? 到 main）
            None => Err(AppError::NotFound(id)),
        }
    }

    fn list(&self) {
        if self.data.tasks.is_empty() {
            println!("（暂无任务）");
            return;
        }
        for task in &self.data.tasks {
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

/// 数据已持久化，不再需要一次串联多个命令，解析简化为单命令
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
    println!("Rust 任务管理器 v3（持久化与错误处理版）");
    println!("用法: main <命令> [参数]");
    println!("  add <标题>   添加任务");
    println!("  list         列出所有任务");
    println!("  done <id>    标记任务完成");
    println!("  help         显示帮助");
    println!("数据保存在当前目录的 {} 中", DB_FILE);
}

/// main 返回 Result：出错时运行时打印 Err 并以非零码退出
fn main() -> Result<(), AppError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = match parse_args(&args) {
        Ok(cmd) => cmd,
        Err(msg) => {
            eprintln!("解析错误: {}", msg);
            print_help();
            std::process::exit(1);
        }
    };

    let mut store = TaskStore::load(Path::new(DB_FILE))?;

    match command {
        Command::Add(title) => {
            let id = store.add(title);
            store.save()?; // 修改数据后立即落盘
            println!("已添加任务 #{}", id);
        }
        Command::List => store.list(),
        Command::Done(id) => {
            store.finish(id)?; // 未找到时 NotFound 经 ? 传播，main 返回 Err
            store.save()?;
            println!("任务 #{} 已完成", id);
        }
        Command::Help => print_help(),
    }
    Ok(())
}
