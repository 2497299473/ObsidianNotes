---
title: 如何在 Windows 启动 Claude
date: 2026-06-01
tags:
  - Claude
  - Windows
  - 教程
aliases:
  - Windows启动Claude
  - Claude Windows setup
status: 📝 待处理
lark_doc_url: https://my.feishu.cn/docx/KhFSdeuP3oV4tTxbOG4c0Klunae
---

> [!ABSTRACT] 快速概览
> 在 Windows 上通过代理 + PowerShell/WSL 启动 Claude 命令行工具的完整步骤指南。

---

## 📝 正文内容

### 1. 打开代理软件

- 连接 **新加坡、美国或日本** 节点

### 2. 设置终端代理

按 `Win + R`，输入 `cmd` 或 `powershell`，回车，然后设置系统监听代理软件的端口：

```powershell
$env:HTTP_PROXY = "http://127.0.0.1:7892"
$env:HTTPS_PROXY = "http://127.0.0.1:7892"
```

> 请根据你的代理软件实际端口号调整上面的地址。

### 3. 进入项目目录（WSL 环境）

1. 打开 Windows 的 **文件资源管理器**
2. 在地址栏中输入 `\\wsl$` 并回车
3. 找到你安装的 Linux 发行版（如 `Ubuntu`），进入项目文件夹
4. 复制完整路径（通常为 `\\wsl.localhost\Ubuntu\home\你的用户名\你的项目目录`）
5. 打开 PowerShell 或 CMD
6. 切换到项目路径：

```powershell
cd \\wsl.localhost\Ubuntu\home\你的用户名\你的项目目录
```

### 4. 启动 Claude

```powershell
claude
```

### 5. 切换模型

使用 `CC switch` 命令切换 Claude 模型。

---

## ✅ 待办事项

- [ ] 确认代理端口号并更新配置
- [ ] 测试 WSL 路径是否正确


## 🔗 关联笔记

- [[Claude的使用/如何在 Windows 启动 claude]] | 本文


## 📎 参考资料

- Claude 官方文档
- 代理软件（Clash/SSR/V2Ray 等）端口配置说明


## 📊 元数据

| 字段 | 值 |
|------|-----|
| 创建日期 | 2026-06-01 |
| 更新时间 | 2026-06-01 |
| 类型 | `教程` |
| 状态 | 📝 待处理 |

---

> [!TIP]
> 请根据你具体使用的云服务（如 Cloudflare、某云盘客户端等）补充具体命令和说明。
