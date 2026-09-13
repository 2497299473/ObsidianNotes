---
title: 02-测试React应用
created: 2026-07-19
stage: 3
order: 2
difficulty: ⭐⭐⭐
estimated_hours: 4
tags:
  - React
  - 测试React应用
lark_doc_url: https://my.feishu.cn/docx/Xi3Idrb1lom7iGxbHaBcOZP4nuh
---

## ⬅️ 前置知识
- [[01-TypeScript与React]] — 上一课内容

## ➡️ 后续笔记
- [[03-构建工具与优化]] — 下一课

## 🔗 关联笔记
- [[../00-React学习路径总索引]] — 返回总索引
- [[../阶段三-工程化与全栈集成/99-阶段3复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 理解 测试React应用 的核心概念与设计原理
2. 在本地环境编写并运行相关代码
3. 完成下方 🟢/🟡/🔴 三级自检

---

## 1. 核心概念

1. Jest 配置与测试运行
2. React Testing Library
3. render/screen/fireEvent
4. MSW 模拟网络请求
5. 测试覆盖率与 CI

---

## 2. 代码示例

```jsx
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';

function Counter() {
  const [count, setCount] = React.useState(0);
  return (
    <div>
      <span data-testid="count">{count}</span>
      <button onClick={() => setCount(count + 1)}>增加</button>
      <button onClick={() => setCount(0)}>重置</button>
    </div>
  );
}

describe('Counter', () => {
  test('初始值为 0', () => {
    render(<Counter />);
    expect(screen.getByTestId('count')).toHaveTextContent('0');
  });

  test('点击增加后数值变化', () => {
    render(<Counter />);
    fireEvent.click(screen.getByText('增加'));
    expect(screen.getByTestId('count')).toHaveTextContent('1');
  });

  test('点击重置归零', () => {
    render(<Counter />);
    fireEvent.click(screen.getByText('增加'));
    fireEvent.click(screen.getByText('增加'));
    fireEvent.click(screen.getByText('重置'));
    expect(screen.getByTestId('count')).toHaveTextContent('0');
  });
});
// 运行: npx jest --verbose
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
用自己的话解释「测试React应用」是什么，以及它解决了什么问题。

### 练习 2：代码实现
为组件库编写单元测试，保存到 `code/` 目录。

### 练习 3：扩展思考
如果去掉本课知识点，项目代码会出现什么问题？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能复述 组件测试与 RTL 的定义与用法 | [ ] |
| 🟡 进阶 | 能独立写出可运行的代码示例 | [ ] |
| 🔴 挑战 | 能在 React 项目中正确运用本课知识 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-07-19*
