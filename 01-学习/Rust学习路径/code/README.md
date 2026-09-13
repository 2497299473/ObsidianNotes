---
lark_doc_url: https://my.feishu.cn/docx/U0oTdhlahofB0zxJu7ycd4gxn0d
---
# 毕业项目参考实现：Rust 任务管理器

本目录存放毕业项目各版本的参考实现，每个版本一个目录、一个单文件 `main.rs`。

> ⚠️ 请先对照各版本笔记自己实现，卡住后再来看答案——先挣扎再对照，收获最大。
> 参考实现不是唯一正确答案；如果你的写法不同但能通过验收标准，以你的版本为准。

## 版本一览

| 目录 | 版本 | 第三方依赖 | 运行方式 |
|------|------|-----------|----------|
| v1/ | CLI 骨架版 | 无（纯 std） | `rustc main.rs` |
| v2/ | 所有权建模版 | 无（纯 std） | `rustc main.rs` |
| v3/ | 持久化与错误处理版 | serde / serde_json | cargo（见下） |
| v4/ | Trait 抽象版 | serde / serde_json | cargo（见下） |
| v5/ | 多线程并发版 | 无（纯 std） | `rustc main.rs` |
| v6/ | 测试与 CI 版 | 无（纯 std） | `rustc main.rs`（真正跑测试请用 cargo） |
| v7/ | 异步 Web 服务版 | tokio / axum / serde | cargo（见下） |

## 如何运行

### 纯 std 版本（v1 / v2 / v5 / v6）

```bash
cd v1
rustc main.rs            # edition 2021
./main add 买牛奶 done 1 list   # Windows 下为 main.exe
```

### 需要依赖的版本（v3 / v4 / v7）

```bash
cargo init todo-vN          # 任意项目名
# 把对应 main.rs 粘贴为 src/main.rs
# 按下文补全 Cargo.toml 依赖，然后：
cargo run -- add 买牛奶
```

v3 / v4 的 Cargo.toml 依赖：

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

v7 的 Cargo.toml 依赖：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## 备注

- v1/v2/v5/v6 不依赖任何第三方 crate，`rustc main.rs` 即可编译运行（edition 2021）。
- v6 的集成测试、criterion 基准测试、GitHub Actions 工作流均以注释形式写在 `v6/main.rs` 中，
  实际练习时请把它们分别落到 `tests/`、`benches/`、`.github/workflows/` 目录。
- v7 的 sqlx + PostgreSQL 扩展方向同样以注释形式给出。
