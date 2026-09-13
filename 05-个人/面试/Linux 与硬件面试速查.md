---
title: Linux 与硬件面试速查
date: 2026-06-18
tags:
  - 面试
  - Linux
  - 硬件
  - 网络
aliases:
  - Linux命令
  - 防火墙
  - 硬件基础
status: ✅ 已完成
lark_doc_url: https://my.feishu.cn/docx/UNaCdhL65oH2lWxb1tjcvkD7nLg
---

> [!ABSTRACT] 快速概览
> 面向工业上位机开发面试的 Linux 操作、硬件基础、防火墙知识速查。覆盖常用命令、防火墙规则、ModbusTCP 协议基础、GPU 选型等高频考点。

---

## 一、Linux 常用命令速查

> 面试 Q30-Q33 的扩展版——把面试官可能追问的命令都列出来。

### 系统信息

```bash
uname -a              # 完整系统信息（内核版本+系统名+硬件架构）
uname -r              # 仅内核版本
hostnamectl           # systemd 系统详细信息
cat /etc/os-release   # 发行版名称和版本
lscpu                 # CPU 详细信息（核心数、架构、频率）
lsmem                 # 内存信息
free -h               # 内存使用情况（人类可读）
```

### 磁盘管理

```bash
df -h                 # 各分区的磁盘使用（最常用）
du -sh *              # 当前目录下各文件/文件夹大小
du -sh /path/to/dir   # 指定目录大小
lsblk                 # 列出所有块设备（硬盘、分区）
fdisk -l              # 磁盘分区表（需要 root）
iostat -x 1           # 磁盘 IO 实时监控
```

### 进程管理

```bash
ps aux                # 所有进程详情
ps aux | grep python  # 查找 Python 进程
top                   # 实时进程监控（CPU/内存）
htop                  # top 的增强版（如有安装）
kill -9 <PID>         # 强制杀死进程
kill -15 <PID>        # 优雅终止（推荐，给进程清理机会）
pstree -p             # 进程树（看父子关系）
```

### 网络

```bash
ss -tlnp              # 查看所有监听的 TCP 端口（替代 netstat）
lsof -i :8080         # 查看占用 8080 端口的进程
curl -v http://localhost  # 测试 HTTP 连接
ping -c 4 192.168.1.1    # 连通性测试
ip addr               # 查看 IP 地址（替代 ifconfig）
ip route              # 查看路由表
```

### 文件与权限

```bash
chmod 755 file        # rwx r-x r-x
chmod 644 file        # rw- r-- r--
chown user:group file # 更改所有者
chown -R user:group dir  # 递归更改

# 权限数字速记
# 4 = r (读), 2 = w (写), 1 = x (执行)
# 777 = 所有人全部权限（危险！）
# 755 = 所有者全权限 + 组和其他人只读执行（目录标准）
# 644 = 所有者读写 + 其他人只读（文件标准）
# 600 = 只有所有者读写（私钥文件）
```

### 文件查找

```bash
find /path -name "*.py"           # 按名称查找
find /path -type d -name "logs"   # 查找目录
find /path -mtime -1              # 最近 1 天修改的文件
grep -r "keyword" /path           # 递归搜索文件内容
which python                      # 查找命令的位置
```

---

## 二、防火墙基础

> 面试 Q38 的扩展版。

### 核心概念

| 概念 | 说明 |
|------|------|
| **防火墙** | 网络流量的过滤系统，按规则放行或拒绝数据包 |
| **入站规则** | 控制外部 → 本机的流量 |
| **出站规则** | 控制本机 → 外部的流量 |
| **白名单** | 默认拒绝所有，只放行信任的（安全推荐） |
| **黑名单** | 默认放行所有，只拒绝特定的 |

### ufw（Ubuntu 常用，简单）

```bash
sudo ufw enable                # 开启防火墙
sudo ufw disable               # 关闭
sudo ufw status verbose        # 查看状态和规则

sudo ufw allow 22/tcp          # 允许 SSH
sudo ufw allow 80,443/tcp      # 允许 HTTP + HTTPS
sudo ufw allow 8080            # 允许 8080（TCP+UDP）

sudo ufw allow from 192.168.1.0/24  # 允许整个内网段

sudo ufw deny 3306             # 禁止外部访问 MySQL

sudo ufw default deny incoming # 默认拒绝入站（安全基线）
sudo ufw default allow outgoing # 默认允许出站
```

### iptables（底层，更强但复杂）

```bash
# 查看规则
sudo iptables -L -n -v

# 允许 22 端口
sudo iptables -A INPUT -p tcp --dport 22 -j ACCEPT

# 拒绝特定 IP
sudo iptables -A INPUT -s 192.168.1.100 -j DROP

# 保存规则（持久化）
sudo iptables-save > /etc/iptables/rules.v4
```

### 工业现场场景

上位机部署在产线工控机上的安全策略：

```bash
# 只允许：运维管理 + 推理服务 + 数据库
sudo ufw default deny incoming
sudo ufw allow from 192.168.1.0/24 to any port 22    # 内网 SSH
sudo ufw allow from 127.0.0.1 to any port 5000       # 本地推理服务
sudo ufw allow from 192.168.1.50 to any port 5432   # 指定 IP 访问数据库
sudo ufw enable
```

---

## 三、硬件基础

> 面试 Q27-Q29 的扩展。

### CPU 关键指标

| 指标 | 含义 | 对开发的影响 |
|------|------|-------------|
| **核心数** | 并行处理能力 | 决定 `multiprocessing` 能开多少进程 |
| **线程数** | 超线程技术，虚拟核心 | 对 IO 密集型多线程有帮助 |
| **主频** | 单核速度 | 单线程 Python 代码的执行速度 |
| **缓存（L1/L2/L3）** | CPU 与内存之间的高速缓冲 | 大数据计算时命中率影响性能 |

### GPU 关键指标

| 指标 | 含义 | 重要性 |
|------|------|:---:|
| **显存（VRAM）** | GPU 专用内存 | ⭐⭐⭐——决定能加载多大模型、多大 batch size |
| **CUDA 核心数** | 并行计算单元数量 | ⭐⭐——影响推理速度 |
| **Tensor Core** | 矩阵运算加速单元 | ⭐⭐——FP16 推理翻倍加速 |
| **显存带宽** | 数据读写速度 | ⭐——大模型推理的瓶颈常在这 |
| **功耗（TDP）** | 散热和供电要求 | ⭐——工控机通常有限制 |

### 工业相机基础（JD 相关）

> 面试 Q34-35 的硬件背景补充。

| 概念 | 说明 |
|------|------|
| **ONVIF** | 安防相机的国际标准协议，提供设备发现（WS-Discovery）、视频流（RTSP）、云台控制（PTZ）的统一接口 |
| **GigE Vision** | 工业相机的高速以太网传输标准，适合高分辨率、高帧率场景 |
| **USB3 Vision** | 基于 USB 3.0 的工业相机标准 |
| **Camera Link** | 老牌高速工业相机接口，逐渐被 GigE Vision 替代 |
| **GenICam** | 通用相机编程接口，统一不同厂商相机的 API |

### ModbusTCP 协议基础（JD 相关）

> 面试中 JD 要求但你没用过的最核心协议。

```
ModbusTCP 帧结构：
┌──────────────────────────────────────────────────┐
│  MBAP 头 (7 bytes)  │  功能码 (1 byte) │ 数据区   │
├──────┬──────┬──────┬──────┬──────┬──────┬──────┤
│事务ID │协议ID│ 长度  │单元ID│                      │
│ 2B   │ 2B   │ 2B   │ 1B   │   1B   │  N B   │
└──────┴──────┴──────┴──────┴──────┴──────┴──────┘
      ↑                                  ↑
   自增序号                        03=读寄存器
   请求响应匹配                     06=写单个寄存器
                                   16=写多个寄存器
```

**常用功能码**：

| 功能码 | 名称 | 用途 |
|:---:|------|------|
| 01 | 读线圈 | 读取 PLC 的数字输出状态 |
| 02 | 读离散输入 | 读取传感器/按钮状态 |
| 03 | 读保持寄存器 | **最常用**——读取模拟量（温度、压力、速度） |
| 04 | 读输入寄存器 | 读取外部模拟量输入 |
| 06 | 写单个寄存器 | 写一个设定值 |
| 16 | 写多个寄存器 | 批量写设定值 |

```python
# Python 库：pymodbus
from pymodbus.client import ModbusTcpClient

client = ModbusTcpClient("192.168.1.100", port=502)
client.connect()

# 读保持寄存器（从地址 0 开始读 10 个寄存器）
result = client.read_holding_registers(address=0, count=10)
if not result.isError():
    values = result.registers  # [温度, 压力, 速度, ...]

# 写单个寄存器
client.write_register(address=5, value=3000)  # 设置转速 3000 rpm

client.close()
```

> 面试话术：ModbusTCP 本质是 TCP 上跑的请求-响应协议，帧结构很简单——7 字节 MBAP 头 + 功能码 + 数据。Python 用 `pymodbus` 库几行代码就能读写 PLC 寄存器。我之前虽然没直接用过，但对协议对接的通用模式很熟，上手 1-2 天就能跑通。

---

## 四、面试实时问答应变

| 可能追问 | 一句话回答 |
|---------|-----------|
| **「Linux 怎么查某个端口被谁占用了？」** | `ss -tlnp \| grep :8080` 或 `lsof -i :8080` |
| **「怎么看实时日志？」** | `tail -f /var/log/app.log`，加 `-n 100` 先看最后 100 行 |
| **「怎么看 CPU 使用率？」** | `top` 或 `htop`（实时），`mpstat`（统计） |
| **「防火墙规则写错了把自己锁外面了怎么办？」** | 云服务器用控制台恢复；物理机用带外管理（IPMI/iLO）或接显示器键盘 |
| **「为什么不建议用 777 权限？」** | 安全——任何人都能读写执行；生产环境用最小权限原则 |
| **「GPU 选型考虑什么？」** | 显存第一（模型大小 + batch）→ CUDA 核心数 → 功耗/散热 → 预算 |
| **「你拆过电脑吗？」** | 诚实回答。如果有：可以提组装时注意 CPU 散热硅脂、静电防护。如果没有：了解组件构成和兼容性即可 |

---

## 🔗 关联笔记

- [[面试/2026-06-17 AI视觉检测面试复盘]] | 四、五部分（硬件+Linux 原题）
- [[面试/AI视觉检测-JD映射与面试预测]] | JD 中的硬件/协议要求
- [[面试/PySide 快速对比]] | PySide vs PyQt
- [[面试/设计模式面试速查]] | 设计模式
