//! v2-所有权建模版：模块拆分 + 所有权设计（纯 std）
//!
//! 编译运行：
//!   rustc main.rs
//!   ./main add 买牛奶 add 写周报 done 1 list
//!
//! 本版本用 mod 模拟多文件结构（真实项目请拆成 src/models.rs、src/cli.rs、src/store.rs）。
//! 所有权设计要点：
//!   1. Task.title 用 String —— 结构体是"数据拥有者"，不需要生命周期参数
//!   2. 查询 / 只读接口用 &self / &Task 借用 —— 不移动、不复制数据
//!   3. Command::Add(String) 直接 move 进 store —— 全链路零 clone

mod models {
    /// 任务：title 用 String，拥有数据。
    /// 若用 &str 就得给结构体加生命周期参数，且数据不能活得比借用来源久，
    /// 因此"程序自己拥有的长期数据"用 String
    pub struct Task {
        pub id: u64,
        pub title: String,
        pub done: bool,
    }

    impl Task {
        /// 构造函数：直接接收 String 的所有权（不做任何复制）
        pub fn new(id: u64, title: String) -> Self {
            Task {
                id,
                title,
                done: false,
            }
        }

        pub fn finish(&mut self) {
            self.done = true;
        }

        /// 只读的小方法用 &self 借用
        pub fn status_mark(&self) -> &'static str {
            if self.done {
                "x"
            } else {
                " "
            }
        }
    }
}

mod store {
    use crate::models::Task;

    /// 任务存储：字段私有，外部统一走方法访问，
    /// "id 如何维护、数据如何存放"成为内部细节
    pub struct TaskStore {
        tasks: Vec<Task>,
        next_id: u64,
    }

    impl TaskStore {
        pub fn new() -> Self {
            TaskStore {
                tasks: Vec::new(),
                next_id: 1,
            }
        }

        /// add 按值接收 String：调用方移交所有权，函数内零额外分配。
        /// 另一种习惯写法是 add(&str) + 内部 to_string()（多一次分配）；
        /// 本程序的 Command::Add 已经持有 String，直接 move 更省
        pub fn add(&mut self, title: String) -> u64 {
            let id = self.next_id;
            self.next_id += 1;
            self.tasks.push(Task::new(id, title));
            id
        }

        /// 可变借用查找：返回 Option<&mut Task>，避免 clone 整个任务
        fn find_mut(&mut self, id: u64) -> Option<&mut Task> {
            self.tasks.iter_mut().find(|t| t.id == id)
        }

        pub fn finish(&mut self, id: u64) -> bool {
            match self.find_mut(id) {
                Some(task) => {
                    task.finish();
                    true
                }
                None => false,
            }
        }

        /// 只读列出：流向 println! 的只是 &Task 借用，全程无复制
        pub fn list(&self) {
            if self.tasks.is_empty() {
                println!("（暂无任务）");
                return;
            }
            for task in &self.tasks {
                println!("[{}] #{} {}", task.status_mark(), task.id, task.title);
            }
            let done_count = self.count_where(|t| t.done);
            println!(
                "共 {} 条，已完成 {}，未完成 {}",
                self.tasks.len(),
                done_count,
                self.tasks.len() - done_count
            );
        }

        /// 传入闭包按条件计数：数据始终不离开 store
        pub fn count_where(&self, mut pred: impl FnMut(&Task) -> bool) -> usize {
            self.tasks.iter().filter(|t| pred(t)).count()
        }
    }
}

mod cli {
    use crate::store::TaskStore;

    #[derive(Debug)]
    pub enum Command {
        Add(String),
        List,
        Done(u64),
        Help,
    }

    const KEYWORDS: [&str; 4] = ["add", "list", "done", "help"];

    fn is_keyword(word: &str) -> bool {
        KEYWORDS.contains(&word)
    }

    /// 解析逻辑与 v1 一致：支持一次串联多个命令
    pub fn parse(args: &[String]) -> Result<Vec<Command>, String> {
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

    /// store 以 &mut 借用传入：函数只是"使用"，所有权仍在调用方
    pub fn run(commands: Vec<Command>, store: &mut TaskStore) {
        for command in commands {
            match command {
                Command::Add(title) => {
                    // title 是 Command::Add 拥有的 String，直接 move 进 store，无 clone
                    let id = store.add(title);
                    println!("已添加任务 #{}", id);
                }
                Command::List => store.list(),
                Command::Done(id) => {
                    if store.finish(id) {
                        println!("任务 #{} 已完成", id);
                    } else {
                        println!("未找到任务 #{}", id);
                    }
                }
                Command::Help => print_help(),
            }
        }
    }

    pub fn print_help() {
        println!("Rust 任务管理器 v2（所有权建模版）");
        println!("用法: main <命令> [参数] ...");
        println!("  add <标题>   添加任务");
        println!("  list         列出所有任务");
        println!("  done <id>    标记任务完成");
        println!("  help         显示帮助");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let commands = match cli::parse(&args) {
        Ok(cmds) => cmds,
        Err(msg) => {
            eprintln!("解析错误: {}", msg);
            cli::print_help();
            std::process::exit(1);
        }
    };

    // store 在 main 中创建，cli::run 只拿到 &mut 借用：
    // 所有权留在 main，函数只是使用——这是最常见的分工方式
    let mut store = store::TaskStore::new();
    cli::run(commands, &mut store);
}
