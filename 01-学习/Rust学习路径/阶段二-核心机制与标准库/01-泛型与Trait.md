---
title: 01-泛型与Trait
created: 2026-08-07
stage: 2
order: 1
difficulty: ⭐⭐⭐
estimated_hours: 5
tags:
  - Rust
  - 泛型
  - Trait
  - 零成本抽象
description: 掌握泛型函数/结构体、单态化、Trait 定义与约束、derive 派生以及静态分发与动态分发的取舍。
lark_doc_url: https://my.feishu.cn/docx/JUgidNLVOo0XbjxWZqzcsx3Ondh
---

## ⬅️ 前置知识
- [[../阶段一-语言基础与所有权/99-阶段1复习检查点]] — 阶段一毕业，已掌握所有权、借用与结构体/枚举基础

## ➡️ 后续笔记
- [[02-错误处理]] — 下一课

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引
- [[01-学习/Rust学习路径/阶段二-核心机制与标准库/99-阶段2复习检查点|99-阶段2复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 编写泛型函数与泛型结构体（如 `Point<T>`），并解释单态化为何是零成本抽象
2. 为「任务管理器」中的类型定义并实现 `Printable`、`Summary` 等 trait，会使用默认实现
3. 使用 `T: Trait`、`+` 组合约束与 `where` 子句编写带约束的泛型函数，会用 `derive` 派生 `Debug`/`Clone`/`PartialEq`
4. 区分静态分发与 `Box<dyn Trait>` 动态分发，说出对象安全的限制及各自适用场景

---

## 1. 泛型函数与泛型结构体

泛型用于在「类型」层面抽象：同一份逻辑适用于多种类型，避免为 `i32`、`f64`、`String` 各复制一份代码。

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 两个坐标可以不同类型
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("largest = {}", largest(&nums));

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p = {:?}", p.x(), p);

    let mp = MixedPoint { x: 1, y: 2.5 };
    println!("mp = {:?}", mp);
}
```

要点：
- `impl<T> Point<T>` 表示「对所有 `T` 实现方法」；也可以写 `impl Point<f32>` 只为特定类型实现方法。
- 结构体可以声明多个类型参数，每个参数相互独立。

## 2. 单态化与零成本抽象

Rust 的泛型在编译期做**单态化（monomorphization）**：编译器为每个实际用到的类型生成一份专用代码。

| 对比项 | Rust 泛型（单态化） | 动态语言的「万能类型」 |
|--------|---------------------|------------------------|
| 运行时类型检查 | 无，编译期确定 | 需要运行时判断 |
| 调用开销 | 与普通具体类型相同 | 通常有分发/装箱开销 |
| 代价 | 编译时间与二进制体积 | 运行时性能 |

```rust
// 编译器实际上会生成类似这样的两份代码：
// largest_i32(list: &[i32]) -> &i32
// largest_f64(list: &[f64]) -> &f64
```

> 💡 「零成本抽象」的含义：你用泛型写出的抽象代码，运行时性能不劣于手写针对每个具体类型的重复代码——抽象的成本由编译期承担。

## 3. Trait：定义共享行为

Trait 定义一组方法签名，类似接口。下面为任务管理器设计两个 trait：`Printable`（渲染成一行文本）和 `Summary`（生成摘要），其中 `Summary` 的两个方法都提供**默认实现**，实现者可以按需覆盖。

```rust
pub trait Printable {
    fn render(&self) -> String;
}

pub trait Summary {
    fn headline(&self) -> String {
        String::from("(无标题)")
    }

    // 默认实现可以调用本 trait 的其他方法
    fn summary(&self) -> String {
        format!("《{}》的摘要", self.headline())
    }
}

struct Task {
    title: String,
    done: bool,
}

struct Report {
    topic: String,
    pages: u32,
}

impl Printable for Task {
    fn render(&self) -> String {
        let mark = if self.done { "x" } else { " " };
        format!("[{mark}] {}", self.title)
    }
}

impl Summary for Task {
    fn headline(&self) -> String {
        self.title.clone()
    }

    fn summary(&self) -> String {
        let state = if self.done { "已完成" } else { "待办" };
        format!("任务《{}》（{state}）", self.title)
    }
}

impl Printable for Report {
    fn render(&self) -> String {
        format!("报告《{}》共 {} 页", self.topic, self.pages)
    }
}

// Report 直接复用 Summary 的两个默认实现
impl Summary for Report {}

fn main() {
    let task = Task { title: String::from("发布 v1.0"), done: false };
    let report = Report { topic: String::from("周报"), pages: 3 };
    println!("{}", task.render());     // [ ] 发布 v1.0
    println!("{}", task.summary());    // 任务《发布 v1.0》（待办）
    println!("{}", report.summary());  // 《周报》的摘要
}
```

> 💡 **孤儿规则**：只有当 trait 或要实现它的类型至少有一个属于当前 crate 时才能实现；不能为外部类型实现外部 trait，防止两个 crate 互相冲突。

## 4. trait bound、where 子句与 derive

```rust
// impl Trait 简写：参数是某个实现了 Printable + Summary 的类型
fn render_card(item: &(impl Printable + Summary)) -> String {
    format!("{} | {}", item.render(), item.summary())
}

// 等价的泛型写法：T: Printable + Summary
fn render_pair<T: Printable>(a: &T, b: &T) -> String {
    format!("{}\n{}", a.render(), b.render())
}

// 约束多了就用 where 子句，签名更清晰
fn render_all<T, F>(items: &[T], header: &F) -> Vec<String>
where
    T: Printable + Summary,
    F: AsRef<str>,
{
    let mut out: Vec<String> = vec![header.as_ref().to_string()];
    for item in items {
        out.push(format!("{} => {}", item.render(), item.summary()));
    }
    out
}
```

**derive 常用派生**：让编译器自动实现标准 trait。

```rust
#[derive(Debug, Clone, PartialEq)]
struct TaskConfig {
    name: String,
    retries: u32,
}

fn main() {
    let a = TaskConfig { name: String::from("backup"), retries: 3 };
    let b = a.clone();          // Clone：显式深拷贝一份
    assert_eq!(a, b);           // PartialEq：支持 == 比较
    println!("{a:?}");          // Debug：{:?} 格式化打印
}
```

| derive | 作用 | 常见用途 |
|--------|------|----------|
| `Debug` | 支持 `{:?}` 打印 | 调试输出、`unwrap` 报错信息 |
| `Clone` | `.clone()` 深拷贝 | 需要复制数据的集合、共享配置 |
| `PartialEq` / `Eq` | `==` 比较 | 断言、集合去重、`contains` |
| `Hash` | 可计算哈希 | 作为 `HashMap` 的键（需与 `Eq` 一起派生） |

## 5. 静态分发 vs 动态分发

**静态分发**（泛型 + 单态化）在编译期确定调用目标，可内联优化；**动态分发**通过 `dyn Trait` + vtable 在运行期查找方法，支持「同一个容器装不同类型」。

```rust
fn render_static(item: &impl Printable) {
    println!("{}", item.render());
}

fn render_many(items: &[Box<dyn Printable>]) {
    for item in items {
        println!("{}", item.render());
    }
}

fn main() {
    let task = Task { title: String::from("合并 PR"), done: true };
    render_static(&task);

    // 异构容器：Task 和 Report 放一起，只能用动态分发
    let mixed: Vec<Box<dyn Printable>> = vec![
        Box::new(Task { title: String::from("评审"), done: false }),
        Box::new(Report { topic: String::from("月报"), pages: 12 }),
    ];
    render_many(&mixed);
}
```

**对象安全（object safety）**：只有对象安全的 trait 才能用 `dyn`。两条主要限制：
1. 方法不能返回 `Self`（除非带 `where Self: Sized` 排除）；
2. 方法不能带泛型类型参数。

**标准库常用 trait 速览**：

| Trait | 方法 | 用途 |
|-------|------|------|
| `Display` | `fmt` | `{}` 打印给用户看的文本 |
| `From` / `Into` | `from` / `into` | 类型转换，如 `String::from("hi")` |
| `Iterator` | `next` | 迭代器协议，第 3 课详解 |

```rust
use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "x" } else { " " };
        write!(f, "[{mark}] {}", self.title)
    }
}

impl From<&str> for Task {
    fn from(s: &str) -> Self {
        Task { title: s.to_string(), done: false }
    }
}

fn demo() {
    let t: Task = "整理文档".into(); // Into 由 From 自动获得
    println!("{t}");                 // 走 Display：[ ] 整理文档
}
```

> 💡 选择经验：性能敏感路径或参数类型单一 → 静态分发；需要异构集合、插件式扩展（如存储后端可替换）→ `Box<dyn Trait>`。

---

## 常见陷阱

| 陷阱 | 表现 | 对策 |
|------|------|------|
| 泛型滥用 | 编译时间暴涨、二进制膨胀（每种实例化类型生成一份代码） | 仅在「逻辑真正相同、只是类型不同」时用泛型；异构场景改用 `dyn` |
| 忘记 trait bound | 泛型函数里用 `>`、`==`、`println!` 报「没有实现 PartialOrd / PartialEq / Display」 | 按报错补上对应 bound，如 `T: PartialOrd + Display` |
| 对非对象安全的 trait 用 `dyn` | 编译错误「the trait cannot be made into an object」 | 检查是否返回 `Self` 或带泛型参数；改静态分发或拆分 trait |
| `dyn` 到处用导致性能下降 | vtable 间接调用阻碍内联，热路径变慢 | 热路径用泛型静态分发，`dyn` 留给边界（配置、插件、异构容器） |

---

## ✏️ 综合练习

### 练习 1：概念复述
1. 什么是单态化？为什么说泛型是「零成本抽象」，代价由谁承担？
2. `impl Summary for Report {}` 空实现为什么能编译通过？换成 `Printable` 可以吗，为什么？
3. 说出 trait 对象安全的两条限制，并各举一个违反的例子（口头描述即可）。

### 练习 2：代码实现
在 `code/01-generics-traits/` 目录新建一个 cargo 项目，实现：
1. 泛型结构体 `Pair<T>`（两个元素），为其实现 `new` 与 `larger`（要求 `T: PartialOrd`）；
2. trait `Describable`（方法 `describe(&self) -> String`，带默认实现）；
3. 为 `Task` 实现 `Describable`，写一个函数 `fn describe_all(items: &[Box<dyn Describable>]) -> Vec<String>`，把至少两种不同类型放进同一个 `Vec` 调用它。

### 练习 3：扩展思考
任务管理器要支持「多种通知渠道（邮件、短信、IM）」：
- 如果用泛型 `fn send<C: Channel>(channel: &C)`，调用方代码会怎样？
- 如果改成 `fn send(channel: &dyn Channel)`，运行时行为与二进制体积有何变化？
- 哪种方案支持「从配置文件动态选择渠道」？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能写出泛型函数与 `Point<T>`，并解释单态化 | [ ] |
| 🟢 基础 | 能定义 trait、提供默认实现并为自己的类型实现 | [ ] |
| 🟡 进阶 | 会用 `T: A + B`、`where` 子句和 `derive` 组合约束 | [ ] |
| 🔴 挑战 | 能说明静态/动态分发的机制差异，并判断一个 trait 是否对象安全 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-08-07*
