//! v1-CLI骨架版：手工参数解析 + 内存存储的任务管理器
//!
//! 编译运行（纯 std，无第三方依赖，edition 2021）：
//!   rustc main.rs
//!   ./main add 买牛奶            # Windows 下是 main.exe
//!
//! 因为数据只存在于当前进程内存中，所以支持一次运行串联多个命令：
//!   ./main add 买牛奶 add 写周报 done 1 list
//!
//! 已知局限：进程退出数据即丢失，v3 引入持久化解决。
//! 本版本核心：struct / enum / match 与 CLI 程序骨架。

/// 一条任务
struct Task {
    id: u64,
    title: String,
    done: bool,
}

/// 内存任务存储：Vec<Task> + 自增 id
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

    /// 添加任务，返回分配到的自增 id
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

    /// 列出所有任务
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

    /// 标记任务完成，返回是否找到了该任务
    fn done(&mut self, id: u64) -> bool {
        for task in &mut self.tasks {
            if task.id == id {
                task.done = true;
                return true;
            }
        }
        false
    }
}

/// 子命令：用枚举建模"用户想做什么"
#[derive(Debug)]
enum Command {
    Add(String),
    List,
    Done(u64),
    Help,
}

/// 所有可识别的子命令关键字（用于判断 add 的标题在哪里结束）
const KEYWORDS: [&str; 4] = ["add", "list", "done", "help"];

fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word)
}

/// 手工解析命令行参数为命令序列（调用方需先跳过程序名）
fn parse_args(args: &[String]) -> Result<Vec<Command>, String> {
    let mut commands: Vec<Command> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "add" => {
                i += 1;
                let mut parts: Vec<&str> = Vec::new();
                // 收集到下一个关键字之前的所有词，作为任务标题
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
    println!("Rust 任务管理器 v1（CLI 骨架版）");
    println!("用法: main <命令> [参数] ...");
    println!("  add <标题>   添加任务");
    println!("  list         列出所有任务");
    println!("  done <id>    标记任务完成");
    println!("  help         显示帮助");
    println!("示例: main add 买牛奶 done 1 list");
}

fn main() {
    // std::env::args() 的第一个元素是程序名，跳过
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
                if store.done(id) {
                    println!("任务 #{} 已完成", id);
                } else {
                    println!("未找到任务 #{}", id);
                }
            }
            Command::Help => print_help(),
        }
    }
}
