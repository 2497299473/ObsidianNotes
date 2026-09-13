import React, { createContext, useContext, useReducer } from 'react';

const TaskContext = createContext();

function reducer(state, action) {
  switch (action.type) {
    case 'ADD': return [...state, { id: Date.now(), title: action.title, done: false }];
    case 'TOGGLE': return state.map(t => t.id === action.id ? { ...t, done: !t.done } : t);
    case 'REMOVE': return state.filter(t => t.id !== action.id);
    default: return state;
  }
}

function TaskProvider({ children }) {
  const [tasks, dispatch] = useReducer(reducer, []);
  return <TaskContext.Provider value={{ tasks, dispatch }}>{children}</TaskContext.Provider>;
}

function TaskList() {
  const { tasks, dispatch } = useContext(TaskContext);
  return (
    <div>
      {tasks.map(t => (
        <div key={t.id}>
          <span style={{ textDecoration: t.done ? 'line-through' : 'none' }}>{t.title}</span>
          <button onClick={() => dispatch({ type: 'TOGGLE', id: t.id })}>切换</button>
          <button onClick={() => dispatch({ type: 'REMOVE', id: t.id })}>删除</button>
        </div>
      ))}
    </div>
  );
}

function AddTask() {
  const { dispatch } = useContext(TaskContext);
  const handleSubmit = (e) => {
    e.preventDefault();
    const title = e.target.elements.task.value;
    if (title) dispatch({ type: 'ADD', title });
    e.target.reset();
  };
  return (
    <form onSubmit={handleSubmit}>
      <input name="task" placeholder="新任务..." />
      <button type="submit">添加</button>
    </form>
  );
}

export default function App() {
  return (
    <TaskProvider>
      <h1>任务看板 v5 (Context + Reducer)</h1>
      <AddTask />
      <TaskList />
    </TaskProvider>
  );
}