import React, { useState } from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';

function TaskItem({ title, done, onToggle }) {
  return (
    <div onClick={onToggle} style={{ textDecoration: done ? 'line-through' : 'none', cursor: 'pointer' }}>
      {title}
    </div>
  );
}

function App() {
  const [tasks, setTasks] = useState([
    { id: 1, title: 'CI 测试', done: false },
  ]);
  const toggle = (id) => {
    setTasks(tasks.map(t => t.id === id ? { ...t, done: !t.done } : t));
  };
  return (
    <div>
      <h1>任务看板 v6</h1>
      {tasks.map(t => <TaskItem key={t.id} {...t} onToggle={() => toggle(t.id)} />)}
    </div>
  );
}

// 测试
describe('TaskItem', () => {
  test('显示标题', () => {
    render(<TaskItem title="测试" done={false} onToggle={() => {}} />);
    expect(screen.getByText('测试')).toBeInTheDocument();
  });

  test('点击触发 onToggle', () => {
    const fn = jest.fn();
    render(<TaskItem title="点击" done={false} onToggle={fn} />);
    fireEvent.click(screen.getByText('点击'));
    expect(fn).toHaveBeenCalled();
  });
});
// 运行: npx jest --verbose