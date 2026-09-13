import React, { useState, useEffect, useRef } from 'react';

function App() {
  const [tasks, setTasks] = useState(() => {
    const saved = localStorage.getItem('tasks');
    return saved ? JSON.parse(saved) : [];
  });
  const [filter, setFilter] = useState('all');
  const inputRef = useRef(null);

  useEffect(() => {
    localStorage.setItem('tasks', JSON.stringify(tasks));
  }, [tasks]);

  const addTask = () => {
    const title = inputRef.current?.value;
    if (title?.trim()) {
      setTasks([...tasks, { id: Date.now(), title, done: false }]);
      inputRef.current.value = '';
    }
  };

  const toggleTask = (id) => {
    setTasks(tasks.map(t => t.id === id ? { ...t, done: !t.done } : t));
  };

  const filtered = tasks.filter(t => {
    if (filter === 'done') return t.done;
    if (filter === 'active') return !t.done;
    return true;
  });

  return (
    <div style={{ padding: 20 }}>
      <h1>任务看板 v3 (Hooks)</h1>
      <div>
        <input ref={inputRef} placeholder="新任务..." />
        <button onClick={addTask}>添加</button>
      </div>
      <div>
        {['all', 'active', 'done'].map(f => (
          <button key={f} onClick={() => setFilter(f)}
                  style={{ fontWeight: filter === f ? 'bold' : 'normal' }}>
            {f}
          </button>
        ))}
      </div>
      {filtered.map(t => (
        <div key={t.id} onClick={() => toggleTask(t)}
             style={{ textDecoration: t.done ? 'line-through' : 'none', padding: 4, cursor: 'pointer' }}>
          {t.title}
        </div>
      ))}
    </div>
  );
}

export default App;