---
title: 01-环境搭建与Cargo
created: 2026-08-07
stage: 1
order: 1
difficulty: ⭐
estimated_hours: 3
tags:
  - Rust
  - Cargo
  - 环境搭建
description: 安装 rustup 与工具链，用 Cargo 创建并运行第一个 Hello World 项目
lark_doc_url: https://my.feishu.cn/docx/CjQHdXMT3oKRBrxPfpac5zKbnDe
---

## ⬅️ 前置知识
- 无需前置知识，本路径起点

## ➡️ 后续笔记
- [[02-变量、基本类型与控制流]] — 下一课

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引
- [[01-学习/Rust学习路径/阶段一-语言基础与所有权/99-阶段1复习检查点|99-阶段1复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 说出 Rust 的三个关键特性：内存安全、零成本抽象、无 GC
2. 用 rustup 安装 Rust，并解释 stable/beta/nightly 工具链的区别与 `rustup update` 的作用
3. 用 `cargo new` 创建项目，说明 `Cargo.toml` 与 `src/main.rs` 各自的职责
4. 熟练使用 `cargo build / run / check / test`，并从 crates.io 添加依赖

---

## 1. Rust 是什么

Rust 是一门**内存安全的系统级编程语言**。它通过所有权（ownership）系统在**编译期**消除空指针、悬垂引用、数据竞争等问题，而不是依赖垃圾回收器。

| 特性 | 说明 |
|------|------|
| 内存安全 | 编译期检查所有权与借用，杜绝野指针、use-after-free |
| 零成本抽象 | 高级抽象（迭代器、泛型、trait）编译后与手写底层代码同等高效 |
| 无 GC | 没有垃圾回收停顿，内存由所有权规则在编译期确定何时释放 |
| 无畏并发 | 数据竞争在编译期被阻止，可以放心写多线程代码 |

> 💡 提示：Rust 的口号是「零成本抽象」——你用不到的东西不付出代价，你用到的东西也没有比手写更好的实现方式。

## 2. 安装 rustup 与工具链管理

**rustup** 是 Rust 官方的工具链管理器，负责安装/切换编译器（rustc）、标准库与 Cargo。

**Windows**（二选一）：
- 下载并运行 `rustup-init.exe`：https://rustup.rs
- 或使用 winget：`winget install Rustlang.Rustup`

> 💡 提示：Windows 上默认的 `x86_64-pc-windows-msvc` 目标依赖 **MSVC 构建工具**。若未安装 Visual Studio Build Tools（勾选「使用 C++ 的桌面开发」工作负载），rustup-init 会给出提示，链接阶段会失败。

**macOS / Linux**：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后验证：

```bash
rustc --version
cargo --version
```

### 工具链概念

rustup 以「工具链」为单位管理编译器版本：

| 工具链 | 说明 |
|--------|------|
| stable | 稳定版，每 6 周发布一次，日常开发默认选择 |
| beta | 下一个稳定版的预览 |
| nightly | 每日构建，包含未稳定特性（部分 crate 的编译需要它） |

```bash
rustup update          # 更新 stable 等已安装工具链
rustup toolchain list  # 查看已安装的工具链
rustup default stable  # 设置默认工具链
```

## 3. 第一个 Cargo 项目

**Cargo** 是 Rust 的构建系统与包管理器，负责：构建、运行、测试、依赖管理、发布。

```bash
cargo new hello_world
cd hello_world
```

生成的项目结构：

```
hello_world/
├── Cargo.toml      # 项目元信息与依赖声明
└── src/
    └── main.rs     # 二进制入口文件
```

`Cargo.toml` 初始内容：

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

`src/main.rs`：

```rust
fn main() {
    println!("Hello, world!");
}
```

运行：

```bash
cargo run
# Compiling hello_world v0.1.0
# Hello, world!
```

> 💡 提示：`edition = "2021"` 表示语言版本。本路径所有代码均基于 edition 2021。2026-09 补充：Rust 1.85+ 的 `cargo new` 已默认生成 **edition 2024**——本路径示例在两种 edition 下均可编译，跟随学习时 Cargo.toml 保持 2021 或改成 2024 皆可（差异详见 [[00-Rust学习路径总索引|总索引]] 的版本口径说明）。

## 4. Cargo 常用命令

| 命令 | 作用 |
|------|------|
| `cargo new` | 创建新项目 |
| `cargo build` | 编译项目（产物在 `target/debug/`） |
| `cargo run` | 编译并运行 |
| `cargo check` | 只做类型/借用检查，跳过代码生成与链接，**速度最快** |
| `cargo test` | 运行测试 |
| `cargo build --release` | 优化编译（产物在 `target/release/`），速度更快但编译更慢 |

> 💡 提示：写代码时频繁敲的应该是 `cargo check`，它比 `cargo build` 快得多；只有在需要运行或发布时才 `cargo run` / `cargo build --release`。

## 5. 依赖与开发环境

### crates.io

crates.io 是 Rust 官方的包仓库。添加依赖只需编辑 `Cargo.toml`：

```toml
[dependencies]
rand = "0.8"
```

`"0.8"` 采用语义化版本约定：允许自动升级到兼容的 `0.8.x` 补丁版本，而首次构建后由 `Cargo.lock` 锁定精确版本，保证可复现。

```bash
cargo add rand     # 也可以用命令添加依赖
```

### VS Code + rust-analyzer

1. 安装 VS Code 扩展 **rust-analyzer**（官方推荐，替代旧的 RLS）
2. 打开项目根目录，rust-analyzer 自动索引 Cargo 项目
3. 获得：实时类型推导、错误提示、跳转定义、自动补全、内联类型标注

> 💡 提示：rust-analyzer 的报错与 rustc 基本一致，编辑器里飘红就说明编译不过，先修编辑器里的错再 `cargo check` 确认。

---

## 常见陷阱

| 陷阱 | 表现 | 对策 |
|------|------|------|
| Windows 忘记安装 VS Build Tools | rustup-init 报警告，或链接阶段报 `link.exe not found` | 安装 Visual Studio Build Tools 并勾选「使用 C++ 的桌面开发」 |
| 把 `cargo run` 当编译检查用 | 每次检查都完整生成二进制，反馈循环慢 | 日常用 `cargo check`，只在要运行时用 `cargo run` |
| 依赖版本写死，不用语义化版本 | 错过补丁修复；或版本过宽导致不兼容 | 写 `"0.8"` 这类语义化版本，交给 `Cargo.lock` 锁定精确版本 |
| 长期不执行 `rustup update` | 编译器版本过旧，新文档/新特性对不上 | 定期 `rustup update`，保持 stable 最新 |

---

## ✏️ 综合练习

### 练习 1：概念复述

不看笔记回答：Rust 为什么能在没有垃圾回收器的情况下保证内存安全？（提示：想想「所有权」在什么时候被检查。）

### 练习 2：代码实现

在本课目录的 `code/` 子目录下完成：
1. 用 `cargo new hello_cargo` 创建项目
2. 在 `Cargo.toml` 中添加 `rand = "0.8"` 依赖
3. 修改 `src/main.rs`，生成 1..=100 的随机数并打印：

```rust
use rand::Rng;

fn main() {
    let n = rand::thread_rng().gen_range(1..=100);
    println!("随机数：{}", n);
}
```

4. 分别执行 `cargo check`、`cargo run`、`cargo build --release`，观察 `target/` 目录变化

### 练习 3：扩展思考

对比 Cargo 与你熟悉的包管理/构建工具（npm、pip、maven 等任选其一）：Cargo 把「构建 + 依赖 + 测试 + 发布」整合在一个工具里，带来了什么好处？`Cargo.lock` 与 npm 的 `package-lock.json` 有什么相似之处？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | `rustc --version` 与 `cargo --version` 均有输出，`cargo run` 能打印 Hello, world! | [ ] |
| 🟡 进阶 | 能说清 stable/beta/nightly 的区别，能解释 `cargo check` 为何比 `cargo build` 快 | [ ] |
| 🔴 挑战 | 成功添加 rand 依赖并运行练习 2 的程序，能解释 `Cargo.lock` 的作用 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-08-07*
