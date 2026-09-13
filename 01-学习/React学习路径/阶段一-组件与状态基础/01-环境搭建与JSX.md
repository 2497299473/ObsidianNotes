---
title: 01-环境搭建与JSX
created: 2026-07-19
stage: 1
order: 1
difficulty: ⭐
estimated_hours: 2
tags:
  - React
  - 环境搭建
lark_doc_url: https://my.feishu.cn/docx/KCXZdKBiQomPucxZ9TIcTCzqn6U
---

## ⬅️ 前置知识
- 无需前置知识，本路径起点

## ➡️ 后续笔记
- [[02-组件与Props]] — 下一课

## 🔗 关联笔记
- [[../00-React学习路径总索引]] — 返回总索引
- [[../阶段一-组件与状态基础/99-阶段1复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 理解 环境搭建与JSX 的核心概念与设计原理
2. 在本地环境编写并运行相关代码
3. 完成下方 🟢/🟡/🔴 三级自检

---

## 1. 核心概念

1. Vite 创建 React 项目
2. JSX 语法规则与 Babel 转译
3. 函数组件与 return JSX
4. ReactDOM.createRoot 渲染
5. StrictMode 严格模式

---

## 2. 代码示例

```jsx
import React from 'react';

function App() {
  const name = 'React';
  const items = ['组件', 'JSX', '状态'];

  return (
    <div>
      <h1>Hello, {name}!</h1>
      <ul>
        {items.map((item, i) => (
          <li key={i}>{item}</li>
        ))}
      </ul>
      <p>当前时间: {new Date().toLocaleString()}</p>
    </div>
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
用自己的话解释「环境搭建与JSX」是什么，以及它解决了什么问题。

### 练习 2：代码实现
创建一个个人信息卡片组件，保存到 `code/` 目录。

### 练习 3：扩展思考
如果去掉本课知识点，项目代码会出现什么问题？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能复述 JSX 语法与组件渲染 的定义与用法 | [ ] |
| 🟡 进阶 | 能独立写出可运行的代码示例 | [ ] |
| 🔴 挑战 | 能在 React 项目中正确运用本课知识 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-07-19*
