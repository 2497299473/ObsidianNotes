// task-manager.js — 毕业项目 v1 初始 JS 代码
// 将本文件复制到你的项目，然后逐函数添加类型标注

const tasks = [];
let nextId = 1;

function addTask(title, priority = "normal") {
  const task = {
    id: nextId++,
    title,
    completed: false,
    priority,
    createdAt: new Date(),
  };
  tasks.push(task);
  return task;
}

function completeTask(id) {
  const task = tasks.find((t) => t.id === id);
  if (task) task.completed = true;
}

function removeTask(id) {
  const idx = tasks.findIndex((t) => t.id === id);
  if (idx !== -1) tasks.splice(idx, 1);
}

function listTasks(filter = "all") {
  if (filter === "completed") return tasks.filter((t) => t.completed);
  if (filter === "pending") return tasks.filter((t) => !t.completed);
  return tasks;
}

function getStats() {
  return {
    total: tasks.length,
    completed: tasks.filter((t) => t.completed).length,
    pending: tasks.filter((t) => !t.completed).length,
  };
}

// 使用示例
addTask("学习 TypeScript", "high");
addTask("写毕业项目", "high");
completeTask(1);
console.log(getStats());
