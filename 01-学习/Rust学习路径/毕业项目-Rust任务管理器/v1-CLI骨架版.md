---
title: v1-CLI骨架版
created: 2026-08-07
project: 毕业项目-Rust任务管理器
version: 1
difficulty: ⭐⭐
estimated_hours: 3
tags:
  - Rust
  - 毕业项目
lark_doc_url: https://my.feishu.cn/docx/PSYldvOTZoBI2MxPZDHc4H9vnEh
---

## ⬅️ 前置知识
- [[../阶段三-并发、异步与工程化/99-阶段3复习检查点]] — 完成全部阶段复习

## ➡️ 后续版本
- [[v2-所有权建模版]] — 下一版本

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引

---

## 🎯 v1 目标

从零搭出任务管理器的骨架：不依赖 clap，手工解析 `std::env::args()` 实现 `add` / `list` / `done` 三个子命令，用 `enum Command` + `match` 建模并分发子命令，用 `Vec<Task>` 做内存存储。核心关注点是结构体 / 枚举 / match 的扎实运用与程序骨架组织，对应阶段一（语言基础与所有权）的知识。

---

## 📋 改造任务

### 1. 任务说明
本版本是毕业项目的起点，从零开始搭建，完成以下改造：
1. 定义 `struct Task { id: u64, title: String, done: bool }` 与 `enum Command { Add, List, Done, Help }`
2. 手工解析 `std::env::args()`：未知子命令要报错并展示帮助，`done` 的 id 必须校验为数字
3. 实现内存存储 `TaskStore`（Vec + 自增 id），完成 add / list / done 的增删查改逻辑
4. 每个子命令可独立使用；进程退出即丢失数据是本版本的已知局限，v3 会解决
5. 确保已有功能不被破坏（本版本从零开始，无历史包袱，但要保证帮助里列出的每个子命令都可用）
6. 提供最小可运行步骤（`rustc main.rs` 编译后即可运行）

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

```rust
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

/// 子命令：用枚举建模"用户想做什么"
enum Command {
    Add(String),
    List,
    Done(u64),
    Help,
}

/// 手工解析命令行参数（调用方已跳过程序名）
fn parse_args(args: &[String]) -> Result<Vec<Command>, String> {
    // 要点：顺序遍历参数，match 子命令关键字；
    // add 收集到下一个关键字之前的所有词作为标题；
    // done 用 parse::<u64>() 校验 id，失败返回 Err
    todo!("遍历 + match")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect(); // 第一个元素是程序名
    let commands = match parse_args(&args) {
        Ok(cmds) => cmds,
        Err(msg) => {
            eprintln!("解析错误: {}", msg);
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
            Command::Done(id) => { /* 标记完成，未找到要提示 */ }
            Command::Help => print_help(),
        }
    }
}
```

完整参考：`code/v1/main.rs`

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
