---
title: v6-测试与CI版
created: 2026-08-07
project: 毕业项目-Rust任务管理器
version: 6
difficulty: ⭐⭐⭐⭐
estimated_hours: 4
tags:
  - Rust
  - 毕业项目
lark_doc_url: https://my.feishu.cn/docx/P1FzdveYIo5BlRxRfpCc2q4Onng
---

## ⬅️ 前置知识
- [[v5-多线程并发版]] — 上一版本

## ➡️ 后续版本
- [[v7-异步Web服务版]] — 下一版本

## 🔗 关联笔记
- [[../00-Rust学习路径总索引]] — 返回总索引

---

## 🎯 v6 目标

给项目系上安全带：为 store（add / toggle / remove）与命令行解析逻辑补 `#[cfg(test)]` 单元测试；把可复用逻辑挪进 lib 后在 `tests/` 写集成测试；用 criterion 基准测试对比 v5 的并行统计与串行统计；配置 GitHub Actions 自动跑 `cargo fmt --check` / `clippy` / `test`。核心关注点是测试与质量保障，对应阶段三（并发、异步与工程化）中工程化的知识。

---

## 📋 改造任务

### 1. 任务说明
基于上一版本代码，完成以下改造：
1. 为 `TaskStore`（add / toggle / remove）与解析逻辑编写 `#[cfg(test)] mod tests` 单元测试
2. 将 store 逻辑放入 `src/lib.rs`（main.rs 只做 CLI 粘合层），在 `tests/cli.rs` 写集成测试（可了解 assert_cmd 测二进制的方式）
3. 在 `benches/` 添加 criterion 基准测试，对比串行统计与 v5 的并行统计
4. 添加 GitHub Actions 工作流：`cargo fmt --check` / `cargo clippy` / `cargo test`
5. 确保 `cargo test` 全绿、已有 CLI 功能不被破坏
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

单元测试（与实现同文件）：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assigns_incrementing_ids() {
        let mut store = TaskStore::new();
        assert_eq!(store.add("first"), 1);
        assert_eq!(store.add("second"), 2);
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
    fn parse_done_rejects_non_numeric_id() {
        let args = vec!["done".to_string(), "abc".to_string()];
        assert!(parse_args(&args).is_err());
    }
}
```

集成测试放 `tests/cli.rs`（需要 lib target，`use 你的crate名::TaskStore;`）；criterion 基准放 `benches/`，Cargo.toml 加：

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "stats_bench"
harness = false
```

完整参考：`code/v6/main.rs`（集成测试、bench、CI workflow 均以注释形式内联在文件头部与尾部）

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
