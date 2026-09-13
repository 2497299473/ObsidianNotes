---
title: 04-Next.js与SSR
created: 2026-07-19
stage: 3
order: 4
difficulty: ⭐⭐⭐⭐
estimated_hours: 5
tags:
  - React
  - Next.js
lark_doc_url: https://my.feishu.cn/docx/NSkud9buxoAjNLxOC8HcXRuInGf
---

## ⬅️ 前置知识
- [[03-构建工具与优化]] — 上一课内容

## ➡️ 后续笔记
- [[01-学习/React学习路径/阶段三-工程化与全栈集成/99-阶段3复习检查点|99-阶段3复习检查点]] — 阶段复习

## 🔗 关联笔记
- [[../00-React学习路径总索引]] — 返回总索引
- [[../阶段三-工程化与全栈集成/99-阶段3复习检查点]] — 阶段复习

---

## 🎯 学习目标

学完本课后，你能够：
1. 理解 Next.js与SSR 的核心概念与设计原理
2. 在本地环境编写并运行相关代码
3. 完成下方 🟢/🟡/🔴 三级自检

---

## 1. 核心概念

1. Next.js App Router 基础
2. SSR vs SSG vs ISR
3. Server/Client 组件边界
4. API Routes 后端接口
5. Vercel 部署

---

## 2. 代码示例

```jsx
// pages/index.tsx
import { GetServerSideProps } from 'next';

interface Props {
  posts: { id: number; title: string }[];
  timestamp: string;
}

export const getServerSideProps: GetServerSideProps<Props> = async () => {
  const res = await fetch('https://jsonplaceholder.typicode.com/posts?_limit=5');
  const posts = await res.json();
  return { props: { posts, timestamp: new Date().toISOString() } };
};

export default function Home({ posts, timestamp }: Props) {
  return (
    <div>
      <h1>博客文章 (SSR)</h1>
      <p>渲染时间: {timestamp}</p>
      <ul>
        {posts.map(p => <li key={p.id}>{p.title}</li>)}
      </ul>
    </div>
  );
}
// 运行: npx next dev
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
用自己的话解释「Next.js与SSR」是什么，以及它解决了什么问题。

### 练习 2：代码实现
用 Next.js 实现一个博客首页，保存到 `code/` 目录。

### 练习 3：扩展思考
如果去掉本课知识点，项目代码会出现什么问题？

---

## 🎯 本课自检

| 层次 | 检查项 | 是否通过 |
|------|--------|----------|
| 🟢 基础 | 能复述 SSR/SSG 渲染模式 的定义与用法 | [ ] |
| 🟡 进阶 | 能独立写出可运行的代码示例 | [ ] |
| 🔴 挑战 | 能在 React 项目中正确运用本课知识 | [ ] |

---

## 📊 通过标准

- 🟢 全部通过 → 可进入下一课
- 🟡 有未通过项 → 重做对应练习后重试
- 🔴 未通过 → 回看本课重点并补做笔记

---

*最后更新：2026-07-19*
