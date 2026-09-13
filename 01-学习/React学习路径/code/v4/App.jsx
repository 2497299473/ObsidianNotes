import React from 'react';
import { BrowserRouter, Routes, Route, Link, useParams } from 'react-router-dom';

function TaskList() {
  const tasks = [
    { id: 1, title: '学习路由', done: false },
    { id: 2, title: '实现多页面', done: true },
  ];
  return (
    <div>
      <h2>任务列表</h2>
      {tasks.map(t => (
        <div key={t.id}>
          <Link to={`/task/${t.id}`}>{t.title}</Link>
        </div>
      ))}
    </div>
  );
}

function TaskDetail() {
  const { id } = useParams();
  return <h2>任务 #{id} 详情</h2>;
}

function Nav() {
  return (
    <nav style={{ marginBottom: 16 }}>
      <Link to="/">首页</Link> | <Link to="/tasks">任务</Link> | <Link to="/about">关于</Link>
    </nav>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <Nav />
      <Routes>
        <Route path="/" element={<h2>任务看板 v4</h2>} />
        <Route path="/tasks" element={<TaskList />} />
        <Route path="/task/:id" element={<TaskDetail />} />
        <Route path="/about" element={<h2>关于页面</h2>} />
      </Routes>
    </BrowserRouter>
  );
}