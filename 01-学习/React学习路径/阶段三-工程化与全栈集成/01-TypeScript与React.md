---
title: 01-TypeScript与React
created: 2026-07-19
stage: 3
order: 1
difficulty: ⭐⭐⭐
estimated_hours: 4
tags:
  - React
  - TypeScript
lark_doc_url: https://my.feishu.cn/docx/QOErdNgY0oTCuCx75LucuL4hnKb
---

## ⬅️ 前置知识
- [[../阶段二-Hooks与状态管理/99-阶段2复习检查点]] — 完成上一阶段复习

## ➡️ 后续笔记
- [[02-测试React应用]] — 下一课

## 🔗 关联笔记
- [[../00-React学习路径总索引]] — 返回总索引
- [[../阶段三-工程化与全栈集成/99-阶段3复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 理解 TypeScript与React 的核心概念与设计原理
2. 在本地环境编写并运行相关代码
3. 完成下方 🟢/🟡/🔴 三级自检

---

## 1. 核心概念

1. React.FC 与 Props 类型定义
2. useState/useRef 泛型
3. 事件处理函数类型
4. 组件库类型导出
5. tsconfig 与 strict 模式

---

## 2. 代码示例

```tsx
import React, { useState } from 'react';

interface Task {
  id: number;
  title: string;
  completed: boolean;
}

interface Props {
  initialTasks?: Task[];
  onComplete?: (id: number) => void;
}

const TaskList: React.FC<Props> = ({ initialTasks = [], onComplete }) => {
  const [tasks, setTasks] = useState<Task[]>(initialTasks);

  const toggle = (id: number) => {
    setTasks(prev => prev.map(t =>
      t.id === id ? { ...t, completed: !t.completed } : t
    ));
    onComplete?.(id);
  };

  return (
    <ul>
      {tasks.map(task => (
        <li key={task.id}>
          <input type="checkbox" checked={task.completed} onChange={() => toggle(task.id)} />
          {task.title}
        </li>
      ))}
    </ul>
  );
};

export default TaskList;
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
用自己的话解释「TypeScript与React」是什么，以及它解决了什么问题。

### 练习 2：代码实现
将 JSX 组件迁移为 TypeScript，保存到 `code/` 目录。

### 练习 3：扩展思考
如果去掉本课知识点，项目代码会出现什么问题？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能复述 React + TypeScript 集成 的定义与用法 | [ ] |
| 🟡 进阶 | 能独立写出可运行的代码示例 | [ ] |
| 🔴 挑战 | 能在 React 项目中正确运用本课知识 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-07-19*
