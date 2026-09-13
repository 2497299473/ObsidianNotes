---
title: v4-Trait抽象版
created: 2026-08-07
project: 毕业项目-Rust任务管理器
version: 4
difficulty: ⭐⭐⭐⭐
estimated_hours: 4
tags:
  - Rust
  - 毕业项目
lark_doc_url: https://my.feishu.cn/docx/O7HwdPMkTozJLKxeypTccjqynce
---

## ⬅️ 前置知识
- [[v3-持久化与错误处理版]] — 上一版本

## ➡️ 后续版本
- [[v5-多线程并发版]] — 下一版本

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引

---

## 🎯 v4 目标

让"数据存在哪里"可以被替换而业务代码不动：定义 `Storage` trait（load / save），实现 `FileStorage` 与 `MemoryStorage` 两种后端，通过 `--backend=file|memory` 参数在运行时选择实现，并以 `Box<dyn Storage>` 注入（动态分发）；同时理解泛型注入（静态分发）的替代方案与取舍。核心关注点是 trait 抽象与依赖注入，对应阶段二（核心机制与标准库）中 trait 与动态分发的知识。

---

## 📋 改造任务

### 1. 任务说明
基于上一版本代码，完成以下改造：
1. 定义 `trait Storage { fn load(&self) -> Result<Vec<Task>, AppError>; fn save(&self, tasks: &[Task]) -> Result<(), AppError>; }`
2. 实现 `FileStorage`（tasks.json，行为同 v3）与 `MemoryStorage`（load 恒空、save 空操作）
3. CLI 增加 `--backend=file|memory` 参数（默认 file），用 `Box<dyn Storage>` 注入选择的实现
4. `TaskStore` 不再碰任何 IO，退化为纯内存工作区；加载/保存统一走 Storage
5. 确保 file 后端行为与 v3 一致、已有功能不被破坏
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

Cargo.toml 依赖（与 v3 相同）：

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

关键代码：

```rust
/// 存储抽象：TaskStore 不关心"数据存在哪"，只通过这个 trait 交互
trait Storage {
    fn load(&self) -> Result<Vec<Task>, AppError>;
    fn save(&self, tasks: &[Task]) -> Result<(), AppError>;
    fn name(&self) -> &'static str;
}

struct FileStorage { path: PathBuf }   // tasks.json
struct MemoryStorage;                  // 不持久化，进程退出即丢

fn main() -> Result<(), AppError> {
    // ...解析 --backend 参数...
    let storage: Box<dyn Storage> = match backend.as_str() {
        "file" => Box::new(FileStorage { path: PathBuf::from("tasks.json") }),
        "memory" => Box::new(MemoryStorage),
        _ => std::process::exit(1),
    };

    let mut store = TaskStore { tasks: storage.load()? };
    // ...业务逻辑只与 store 交互...
    storage.save(&store.tasks)?;
    Ok(())
}
```

另一种写法（静态分发）：`struct App<S: Storage> { storage: S, tasks: Vec<Task> }`。泛型会单态化、性能更好，但每种实现生成一份代码且无法运行时选择；`Box<dyn Storage>` 多一层 vtable 间接寻址，但可以运行时切换实现——本版本选后者。

完整参考：`code/v4/main.rs`

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
