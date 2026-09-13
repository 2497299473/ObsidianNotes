//! v6-测试与CI版（纯 std：rustc main.rs 仍可运行，测试部分需配合 cargo）
//!
//! 本版本的重点不是加功能，而是给项目系上安全带：
//!   1. 单元测试：#[cfg(test)] mod tests（见本文件末尾），覆盖 store 的 add/toggle/remove 与解析逻辑
//!   2. 集成测试：放在项目 tests/ 目录（示例见下方注释）
//!   3. 基准测试：criterion（示例见下方注释），对比串行统计与 v5 的并行统计
//!   4. CI：GitHub Actions 跑 cargo fmt --check / clippy / test（workflow 见下方注释）
//!
//! 快速体验：
//!   rustc main.rs && ./main add 买牛奶 add 写测试 done 1 list stats
//! 真实测试流程：
//!   cargo init 后把本文件粘为 src/main.rs，然后 cargo test / cargo bench
//!
//! ─────────────────────────────────────────────────────────────────────
//! 集成测试：tests/cli.rs
//! 真实项目中请把 TaskStore 等逻辑挪到 src/lib.rs（main.rs 只做 CLI 粘合层），
//! 集成测试即可直接引用：
//!
//! ```rust
//! // tests/cli.rs
//! use todo_app::TaskStore; // crate 名来自 Cargo.toml 的 [package] name（连字符写为下划线）
//!
//! #[test]
//! fn add_then_find_by_iteration() {
//!     let mut store = TaskStore::new();
//!     store.add("写集成测试");
//!     assert!(store.iter().any(|t| t.title == "写集成测试"));
//! }
//!
//! // 也可以用 assert_cmd 对 CLI 二进制做端到端测试：
//! //   cargo add --dev assert_cmd predicates
//! //   Command::cargo_bin("todo-app").unwrap().args(["add", "你好"]).assert().success();
//! ```
//! ─────────────────────────────────────────────────────────────────────
//! 基准测试：benches/stats_bench.rs
//! Cargo.toml 需追加：
//!   [dev-dependencies]
//!   criterion = "0.5"
//!
//!   [[bench]]
//!   name = "stats_bench"
//!   harness = false
//!
//! ```rust
//! use criterion::{criterion_group, criterion_main, Criterion};
//! use todo_app::{generate_tasks, parallel_stats, serial_stats};
//!
//! fn bench_stats(c: &mut Criterion) {
//!     let tasks = generate_tasks(100_000);
//!     // 对比 v5 的并行统计与串行统计，观察大数据量下的多线程收益
//!     c.bench_function("serial_stats", |b| b.iter(|| serial_stats(&tasks)));
//!     c.bench_function("parallel_stats_x4", |b| b.iter(|| parallel_stats(&tasks, 4)));
//! }
//!
//! criterion_group!(benches, bench_stats);
//! criterion_main!(benches);
//! ```
//! 运行：cargo bench
//! ─────────────────────────────────────────────────────────────────────
//! GitHub Actions：.github/workflows/ci.yml
//!
//! name: CI
//! on: [push, pull_request]
//! jobs:
//!   quality:
//!     runs-on: ubuntu-latest
//!     steps:
//!       - uses: actions/checkout@v4
//!       - uses: dtolnay/rust-toolchain@stable
//!         with:
//!           components: rustfmt, clippy
//!       - name: Format check
//!         run: cargo fmt --check
//!       - name: Clippy
//!         run: cargo clippy -- -D warnings
//!       - name: Tests
//!         run: cargo test
//! ─────────────────────────────────────────────────────────────────────

use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

struct TaskStore {
    tasks: Vec<Task>,
    next_id: u64,
}

impl TaskStore {
    fn new() -> Self {
        TaskStore {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// impl Into<String>：&str 与 String 都能传——
    /// 调用方决定是借出内容还是移交所有权，调用点不被迫 clone
    fn add(&mut self, title: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            title: title.into(),
            done: false,
        });
        id
    }

    /// 翻转完成状态；返回是否找到该任务
    fn toggle(&mut self, id: u64) -> bool {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = !task.done;
                true
            }
            None => false,
        }
    }

    /// 移除并返回任务（Vec::remove 直接移出所有权，无 clone）
    fn remove(&mut self, id: u64) -> Option<Task> {
        let pos = self.tasks.iter().position(|t| t.id == id)?;
        Some(self.tasks.remove(pos))
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn is_done(&self, id: u64) -> bool {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.done)
            .unwrap_or(false)
    }

    fn iter(&self) -> std::slice::Iter<'_, Task> {
        self.tasks.iter()
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

/// 生成示例任务：id 为 1..=count，每 3 条有 1 条已完成
fn generate_tasks(count: u64) -> Vec<Task> {
    (1..=count)
        .map(|i| Task {
            id: i,
            title: format!("任务 #{}", i),
            done: i % 3 == 0,
        })
        .collect()
}

/// 串行统计（bench 对照基线）：返回 (done, pending)
fn serial_stats(tasks: &[Task]) -> (usize, usize) {
    let done = tasks.iter().filter(|t| t.done).count();
    (done, tasks.len() - done)
}

/// 并行统计（沿用 v5 实现）：criterion bench 的对比对象
fn parallel_stats(tasks: &[Task], workers: usize) -> (usize, usize) {
    let workers = workers.max(1);
    let chunk_size = (tasks.len() + workers - 1) / workers;
    let (tx, rx) = mpsc::channel::<(usize, usize)>();
    if chunk_size > 0 {
        thread::scope(|s| {
            for piece in tasks.chunks(chunk_size) {
                let tx = tx.clone();
                s.spawn(move || {
                    let done = piece.iter().filter(|t| t.done).count();
                    let _ = tx.send((done, piece.len() - done));
                });
            }
        });
    }
    drop(tx);
    let mut total = (0, 0);
    for (d, p) in rx {
        total.0 += d;
        total.1 += p;
    }
    total
}

#[derive(Debug)]
enum Command {
    Add(String),
    List,
    Done(u64),
    Remove(u64),
    Stats,
    Help,
}

const KEYWORDS: [&str; 6] = ["add", "list", "done", "remove", "stats", "help"];

fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word)
}

/// 支持一次串联多个命令（本版本为纯内存存储）
fn parse_args(args: &[String]) -> Result<Vec<Command>, String> {
    let mut commands: Vec<Command> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "add" => {
                i += 1;
                let mut parts: Vec<&str> = Vec::new();
                while i < args.len() && !is_keyword(&args[i]) {
                    parts.push(&args[i]);
                    i += 1;
                }
                if parts.is_empty() {
                    return Err("add 需要任务标题".to_string());
                }
                commands.push(Command::Add(parts.join(" ")));
            }
            "list" => {
                commands.push(Command::List);
                i += 1;
            }
            "done" => {
                if i + 1 >= args.len() {
                    return Err("done 需要任务 id".to_string());
                }
                let id: u64 = args[i + 1]
                    .parse()
                    .map_err(|_| format!("无法解析任务 id: {}", args[i + 1]))?;
                commands.push(Command::Done(id));
                i += 2;
            }
            "remove" => {
                if i + 1 >= args.len() {
                    return Err("remove 需要任务 id".to_string());
                }
                let id: u64 = args[i + 1]
                    .parse()
                    .map_err(|_| format!("无法解析任务 id: {}", args[i + 1]))?;
                commands.push(Command::Remove(id));
                i += 2;
            }
            "stats" => {
                commands.push(Command::Stats);
                i += 1;
            }
            "help" => {
                commands.push(Command::Help);
                i += 1;
            }
            other => return Err(format!("未知子命令: {}", other)),
        }
    }
    if commands.is_empty() {
        commands.push(Command::Help);
    }
    Ok(commands)
}

fn print_help() {
    println!("Rust 任务管理器 v6（测试与CI版）");
    println!("用法: main <命令> [参数] ...");
    println!("  add <标题>   添加任务");
    println!("  list         列出所有任务");
    println!("  done <id>    翻转任务完成状态");
    println!("  remove <id>  删除任务");
    println!("  stats        统计 done/pending 数量");
    println!("  help         显示帮助");
}

fn main() {
    // 先跑一遍两个"bench 对象"，验证并行与串行结果一致（真实对比请用 cargo bench）
    let sample = generate_tasks(100_000);
    let (sd, sp) = serial_stats(&sample);
    let (pd, pp) = parallel_stats(&sample, 4);
    assert_eq!((sd, sp), (pd, pp));
    println!(
        "[bench 预览] 共 {} 条：串行(done={}, pending={}) == 并行(done={}, pending={})",
        sample.len(),
        sd,
        sp,
        pd,
        pp
    );

    let args: Vec<String> = std::env::args().skip(1).collect();
    let commands = match parse_args(&args) {
        Ok(cmds) => cmds,
        Err(msg) => {
            eprintln!("解析错误: {}", msg);
            print_help();
            std::process::exit(1);
        }
    };

    let mut store = TaskStore::new();
    for command in commands {
        match command {
            Command::Add(title) => {
                let id = store.add(title);
                println!("已添加任务 #{}", id);
            }
            Command::List => store.list(),
            Command::Done(id) => {
                if store.toggle(id) {
                    let status = if store.is_done(id) { "已完成" } else { "未完成" };
                    println!("任务 #{} 状态已切换，现在是：{}", id, status);
                } else {
                    println!("未找到任务 #{}", id);
                }
            }
            Command::Remove(id) => match store.remove(id) {
                Some(task) => println!("已删除任务 #{}：{}", task.id, task.title),
                None => println!("未找到任务 #{}", id),
            },
            Command::Stats => {
                let total = store.len();
                let done = store.iter().filter(|t| t.done).count();
                println!("共 {} 条，已完成 {}，未完成 {}", total, done, total - done);
            }
            Command::Help => print_help(),
        }
    }
}

// ── 单元测试：与实现同文件，#[cfg(test)] 保证只在 cargo test 时编译 ──
#[cfg(test)]
mod tests {
    use super::*;

    // ── store 单元测试：add / toggle / remove ──

    #[test]
    fn add_assigns_incrementing_ids() {
        let mut store = TaskStore::new();
        assert_eq!(store.add("first"), 1);
        assert_eq!(store.add("second"), 2);
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn add_accepts_str_and_string() {
        let mut store = TaskStore::new();
        store.add("借用的 &str");
        store.add(String::from("拥有的 String"));
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn toggle_flips_done_status() {
        let mut store = TaskStore::new();
        let id = store.add("task");
        assert!(store.toggle(id));
        assert!(store.is_done(id));
        assert!(store.toggle(id));
        assert!(!store.is_done(id));
    }

    #[test]
    fn toggle_missing_id_returns_false() {
        let mut store = TaskStore::new();
        assert!(!store.toggle(42));
    }

    #[test]
    fn remove_returns_taken_task() {
        let mut store = TaskStore::new();
        let id = store.add("待删除");
        let removed = store.remove(id).expect("应当能删除");
        assert_eq!(removed.title, "待删除");
        assert_eq!(store.len(), 0);
        assert!(store.remove(id).is_none());
    }

    // ── 解析逻辑单元测试 ──

    #[test]
    fn parse_add_joins_title_words() {
        let args = vec!["add".to_string(), "buy".to_string(), "milk".to_string()];
        match &parse_args(&args).expect("应能解析")[0] {
            Command::Add(title) => assert_eq!(title, "buy milk"),
            other => panic!("期望 Add，实际是 {:?}", other),
        }
    }

    #[test]
    fn parse_done_rejects_non_numeric_id() {
        let args = vec!["done".to_string(), "abc".to_string()];
        assert!(parse_args(&args).is_err());
    }

    #[test]
    fn parse_unknown_command_is_error() {
        let args = vec!["fly".to_string()];
        assert!(parse_args(&args).is_err());
    }

    // ── 统计逻辑单元测试：用串行结果交叉验证并行结果 ──

    #[test]
    fn parallel_stats_matches_serial() {
        let tasks = generate_tasks(1000);
        assert_eq!(serial_stats(&tasks), parallel_stats(&tasks, 4));
    }
}
