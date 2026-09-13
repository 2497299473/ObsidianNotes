//! v5-多线程并发版（纯 std，rustc main.rs 即可编译）
//!
//! Part 1 —— thread::scope 并行统计：
//!   生成一大批任务并分片，每个线程统计自己分片的 done/pending 数量，
//!   通过 mpsc 通道把结果发回主线程汇总。
//! Part 2 —— Arc<Mutex<TaskStore>> + mpsc 命令循环：
//!   单工作线程串行执行命令（保证唯一写者，避免数据竞争），
//!   主线程负责接收用户输入、把命令经通道发给工作线程。
//!   体会理念："不要通过共享内存来通信，而要通过通信来共享内存"。
//!
//! 运行：./main          # 先打印并行统计结果，再进入命令循环（stdin EOF 自动退出）
//! 输入：add 买牛奶 / done 1 / list / quit

use std::io::BufRead;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
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

    fn add(&mut self, title: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            title,
            done: false,
        });
        id
    }

    fn finish(&mut self, id: u64) -> bool {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                true
            }
            None => false,
        }
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("[worker] （暂无任务）");
            return;
        }
        for task in &self.tasks {
            let mark = if task.done { "x" } else { " " };
            println!("[worker] [{}] #{} {}", mark, task.id, task.title);
        }
    }
}

/// 生成统计演示用的示例数据：每 3 条任务有 1 条已完成
fn sample_tasks(count: u64) -> Vec<Task> {
    (1..=count)
        .map(|i| Task {
            id: i,
            title: format!("示例任务 #{}", i),
            done: i % 3 == 0,
        })
        .collect()
}

/// 并行统计：分片 + 作用域线程 + mpsc 汇总，返回 (done, pending)
fn parallel_stats(tasks: &[Task], workers: usize) -> (usize, usize) {
    let workers = workers.max(1);
    let chunk_size = (tasks.len() + workers - 1) / workers; // 向上取整
    let (tx, rx) = mpsc::channel::<(usize, usize)>();

    if chunk_size > 0 {
        thread::scope(|s| {
            for piece in tasks.chunks(chunk_size) {
                let tx = tx.clone(); // 每个线程持有自己的 Sender 克隆
                // 作用域线程可以直接借用 tasks：scope 保证函数返回前
                // 所有线程都已 join，借用不会比数据活得久，无需 Arc / 'static
                s.spawn(move || {
                    let done = piece.iter().filter(|t| t.done).count();
                    let pending = piece.len() - done;
                    let _ = tx.send((done, pending));
                });
            }
        });
    }
    drop(tx); // 丢弃最后一个 Sender 后，rx 的迭代在收完所有结果时结束

    let (mut done, mut pending) = (0, 0);
    for (d, p) in rx {
        done += d;
        pending += p;
    }
    (done, pending)
}

/// 主线程发给工作线程的命令。
/// 注意：mpsc 的消息必须满足 'static + Send，
/// 所以标题用 String（移交所有权），不能用 &str
enum WorkerCommand {
    Add(String),
    Done(u64),
    List,
    Shutdown,
}

/// 工作线程：从通道逐条取命令、串行执行，是存储的唯一写者
fn worker_loop(store: Arc<Mutex<TaskStore>>, rx: mpsc::Receiver<WorkerCommand>) {
    for cmd in rx {
        match cmd {
            WorkerCommand::Shutdown => break,
            WorkerCommand::Add(title) => {
                let mut guard = store.lock().unwrap();
                let id = guard.add(title);
                println!("[worker] 已添加任务 #{}", id);
            }
            WorkerCommand::Done(id) => {
                let mut guard = store.lock().unwrap();
                if guard.finish(id) {
                    println!("[worker] 任务 #{} 已完成", id);
                } else {
                    println!("[worker] 未找到任务 #{}", id);
                }
            }
            WorkerCommand::List => {
                let guard = store.lock().unwrap();
                guard.list();
            }
        }
    }
}

fn main() {
    // ── Part 1：并行统计演示 ──
    let tasks = sample_tasks(100_000);
    let (done, pending) = parallel_stats(&tasks, 4);
    println!(
        "[stats] 共 {} 条，已完成 {}，未完成 {}（4 线程并行统计）",
        tasks.len(),
        done,
        pending
    );

    // ── Part 2：主线程接收输入 + 工作线程执行命令 ──
    let store = Arc::new(Mutex::new(TaskStore::new()));
    let (tx, rx) = mpsc::channel::<WorkerCommand>();

    // 若主线程之后也需要读取状态（如定期汇总），可继续使用手里的 store；
    // 这里把工作副本的 Arc 移交给工作线程
    let worker_store = Arc::clone(&store);
    let worker = thread::spawn(move || worker_loop(worker_store, rx));

    println!("命令循环已就绪（add <标题> / done <id> / list / quit）");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(text) => text,
            Err(_) => break, // stdin 出错则退出
        };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, ' ');
        let head = parts.next().unwrap_or("");
        let tail = parts.next();
        match head {
            "quit" | "exit" => break,
            "add" => {
                if let Some(title) = tail {
                    // String 的所有权经通道移交给工作线程
                    let _ = tx.send(WorkerCommand::Add(title.to_string()));
                } else {
                    println!("用法: add <标题>");
                }
            }
            "done" => match tail.and_then(|s| s.parse::<u64>().ok()) {
                Some(id) => {
                    let _ = tx.send(WorkerCommand::Done(id));
                }
                None => println!("用法: done <id>"),
            },
            "list" => {
                let _ = tx.send(WorkerCommand::List);
            }
            other => println!("未知命令: {}", other),
        }
    }

    // 优雅退出：发送 Shutdown，等工作线程处理完再 join
    let _ = tx.send(WorkerCommand::Shutdown);
    drop(tx);
    let _ = worker.join();
    println!("工作线程已退出，程序结束");
}
