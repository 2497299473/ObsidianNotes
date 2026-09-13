import React from 'react';

function App() {
  const tasks = [
    { id: 1, title: '学习 React', done: false },
    { id: 2, title: '搭建项目', done: true },
    { id: 3, title: '写组件', done: false },
  ];

  return (
    <div style={{ padding: 20 }}>
      <h1>任务看板 v1</h1>
      {tasks.map(task => (
        <div key={task.id} style={{
          border: '1px solid #ddd', padding: 10, margin: 5,
          borderRadius: 4, backgroundColor: task.done ? '#e8f5e9' : '#fff',
        }}>
          <span style={{ textDecoration: task.done ? 'line-through' : 'none' }}>
            {task.title}
          </span>
        </div>
      ))}
    </div>
  );
}

export default App;