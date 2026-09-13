---
title: 02-组件与Props
created: 2026-07-19
stage: 1
order: 2
difficulty: ⭐⭐
estimated_hours: 3
tags:
  - React
  - 组件
lark_doc_url: https://my.feishu.cn/docx/Kkmkd3moLo3ct0x3wyRcctO0nkd
---

## ⬅️ 前置知识
- [[01-环境搭建与JSX]] — 上一课内容

## ➡️ 后续笔记
- [[03-状态与事件]] — 下一课

## 🔗 关联笔记
- [[../00-React学习路径总索引]] — 返回总索引
- [[../阶段一-组件与状态基础/99-阶段1复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 理解 组件与Props 的核心概念与设计原理
2. 在本地环境编写并运行相关代码
3. 完成下方 🟢/🟡/🔴 三级自检

---

## 1. 核心概念

1. Props 父传子与 children
2. 组件组合与拆分
3. Props 默认值
4. 组件纯函数原则
5. 组件命名规范与文件组织

---

## 2. 代码示例

```jsx
import React from 'react';

function Button({ label, onClick, variant = 'primary' }) {
  const styles = {
    primary: { backgroundColor: '#007bff', color: '#fff' },
    secondary: { backgroundColor: '#6c757d', color: '#fff' },
  };

  return (
    <button style={styles[variant]} onClick={onClick}>
      {label}
    </button>
  );
}

function Card({ title, children }) {
  return (
    <div style={{ border: '1px solid #ddd', padding: 16, borderRadius: 8 }}>
      <h3>{title}</h3>
      {children}
    </div>
  );
}

function App() {
  return (
    <Card title="操作面板">
      <Button label="保存" onClick={() => alert('保存')} />
      <Button label="取消" variant="secondary" onClick={() => alert('取消')} />
    </Card>
  );
}

export default App;
`````

> 💡 请在本地环境中运行并修改以上示例，参考 React 官方文档获取最新 API 用法。

---

## 3. 常见陷阱

1. 只看不练，导致概念模糊——务必动手敲代码
2. 跳过前置基础，直接挑战高难度——按顺序学习
3. 不记录错误与解决方案——建立自己的错误笔记本

---

## ✏️ 综合练习

### 练习 1：概念复述
用自己的话解释「组件与Props」是什么，以及它解决了什么问题。

### 练习 2：代码实现
实现一个可复用的 Button 组件，保存到 `code/` 目录。

### 练习 3：扩展思考
如果去掉本课知识点，项目代码会出现什么问题？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能复述 Props 传递与组件组合 的定义与用法 | [ ] |
| 🟡 进阶 | 能独立写出可运行的代码示例 | [ ] |
| 🔴 挑战 | 能在 React 项目中正确运用本课知识 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-07-19*
