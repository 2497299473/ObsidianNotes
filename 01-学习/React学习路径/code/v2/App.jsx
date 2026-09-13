import React, { useState } from 'react';

function App() {
  const [tasks, setTasks] = useState([
    { id: 1, title: '学习 React 状态', done: false },
    { id: 2, title: '实现添加功能', done: false },
  ]);
  const [newTitle, setNewTitle] = useState('');

  const addTask = () => {
    if (newTitle.trim()) {
      setTasks([...tasks, { id: Date.now(), title: newTitle, done: false }]);
      setNewTitle('');
    }
  };

  const toggleTask = (id) => {
    setTasks(tasks.map(t => t.id === id ? { ...t, done: !t.done } : t));
  };

  return (
    <div style={{ padding: 20 }}>
      <h1>任务看板 v2</h1>
      <div>
        <input value={newTitle} onChange={e => setNewTitle(e.target.value)}
               placeholder="输入任务..." onKeyDown={e => e.key === 'Enter' && addTask()} />
        <button onClick={addTask}>添加</button>
      </div>
      {tasks.map(task => (
        <div key={task.id} onClick={() => toggleTask(task)}
             style={{ padding: 8, cursor: 'pointer',
                      textDecoration: task.done ? 'line-through' : 'none' }}>
          {task.title}
        </div>
      ))}
    </div>
  );
}

export default App;