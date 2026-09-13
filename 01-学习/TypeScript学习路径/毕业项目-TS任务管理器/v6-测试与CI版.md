---
title: v6-测试与CI版
created: 2026-07-17
project: TS 任务管理器
version: 6
difficulty: ⭐⭐⭐⭐
estimated_hours: 2
tags:
  - TypeScript
  - 毕业项目
  - 测试
  - CI
  - Vitest
  - ESLint
lark_doc_url: https://my.feishu.cn/docx/FG5Udes56oeEV1xQTlfcFPJTnSf
---

## ⬅️ 前置知识
- 需要：[[v5-模块化拆分版]] — 模块化结构
- 需要：[[../阶段三-工程化与实战集成/04-测试与CI]] — Vitest、ESLint、CI

## ➡️ 后续版本
- [[v7-可发布npm包版]] — 发布为 npm 包

## 🔗 关联笔记
- [[../00-TypeScript学习路径总索引]] — 返回总索引

---

## 🎯 v6 目标

为 v5 的模块化项目添加**完整的测试套件和 CI 流水线**。这是"能不能交付生产级代码"的判定。

---

## 📋 测试覆盖目标

| 模块 | 覆盖率目标 |
|------|-----------|
| `entity-manager.ts` | 函数覆盖 100% |
| `task-service.ts` | 分支覆盖 ≥ 90% |
| 类型工具 | 编译时通过（expectTypeOf） |
| 事件派发 | 分支覆盖 ≥ 80% |

---

## 1. 安装依赖

```bash
npm install -D vitest @vitest/coverage-v8
npm install -D eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin
npm install -D prettier eslint-config-prettier
npm install -D husky lint-staged
```

---

## 2. Vitest 配置

```typescript
// vitest.config.ts
import { defineConfig } from "vitest/config";
import path from "path";

export default defineConfig({
  test: {
    include: ["src/**/*.test.ts", "__tests__/**/*.test.ts"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      include: ["src/**/*.ts"],
      exclude: ["src/**/*.test.ts", "src/types/**", "src/index.ts"],
      thresholds: {
        lines: 80,
        functions: 80,
        branches: 70,
      },
    },
  },
  resolve: {
    alias: {
      "@types": path.resolve(__dirname, "src/types"),
      "@utils": path.resolve(__dirname, "src/utils"),
      "@core": path.resolve(__dirname, "src/core"),
    },
  },
});
```

---

## 3. 实体管理器测试

```typescript
// __tests__/entity-manager.test.ts
import { describe, it, expect, beforeEach } from "vitest";
import {
  createInitialState,
  createEntity,
  updateEntity,
  removeEntity,
  findById,
} from "../src/core/entity-manager";
import type { Task } from "../src/types/task";

const createTaskInput = (title: string, priority: Task["priority"] = "normal") => ({
  title,
  priority,
  completed: false,
  createdAt: new Date(),
});

describe("EntityManager", () => {
  let state = createInitialState<Task>();

  beforeEach(() => {
    state = createInitialState<Task>();
  });

  describe("createEntity", () => {
    it("creates an entity with auto-incremented id", () => {
      const newState = createEntity(state, createTaskInput("Test"));
      expect(newState.items).toHaveLength(1);
      expect(newState.items[0].id).toBe(1);
      expect(newState.nextId).toBe(2);
    });

    it("does not mutate original state", () => {
      createEntity(state, createTaskInput("Test"));
      expect(state.items).toHaveLength(0);
    });

    it("assigns incrementing IDs", () => {
      let s = createEntity(state, createTaskInput("A"));
      s = createEntity(s, createTaskInput("B"));
      expect(s.items[0].id).toBe(1);
      expect(s.items[1].id).toBe(2);
    });
  });

  describe("updateEntity", () => {
    it("updates specified fields", () => {
      const created = createEntity(state, createTaskInput("Test"));
      const updated = updateEntity(created, 1, { completed: true });
      expect(updated.items[0].completed).toBe(true);
      expect(updated.items[0].title).toBe("Test"); // 未改字段保持
    });

    it("does not mutate original state on update", () => {
      const created = createEntity(state, createTaskInput("Test"));
      updateEntity(created, 1, { completed: true });
      expect(created.items[0].completed).toBe(false);
    });

    it("returns same state for non-existent id", () => {
      const updated = updateEntity(state, 999, { completed: true });
      expect(updated).toEqual(state);
    });
  });

  describe("removeEntity", () => {
    it("removes entity by id", () => {
      const created = createEntity(state, createTaskInput("Test"));
      const removed = removeEntity(created, 1);
      expect(removed.items).toHaveLength(0);
    });
  });

  describe("findById", () => {
    it("finds an entity by id", () => {
      const created = createEntity(state, createTaskInput("Test"));
      const found = findById(created, 1);
      expect(found?.title).toBe("Test");
    });

    it("returns undefined for non-existent id", () => {
      const found = findById(state, 999);
      expect(found).toBeUndefined();
    });
  });
});
```

---

## 4. Task Service 集成测试

```typescript
// __tests__/task-service.test.ts
import { describe, it, expect, beforeEach } from "vitest";
import {
  createTaskState,
  addTask,
  completeTask,
  removeTask,
  listTasks,
  getStats,
} from "../src/services/task-service";
import type { EntityState } from "../src/core/entity-manager";
import type { Task } from "../src/types/task";

describe("TaskService", () => {
  let state: EntityState<Task>;

  beforeEach(() => {
    state = createTaskState();
  });

  describe("addTask", () => {
    it("creates a task with default priority", () => {
      const newState = addTask(state, "Learn TS");
      expect(newState.items[0].priority).toBe("normal");
      expect(newState.items[0].completed).toBe(false);
    });

    it("creates a task with specified priority", () => {
      const newState = addTask(state, "Learn TS", "high");
      expect(newState.items[0].priority).toBe("high");
    });
  });

  describe("completeTask", () => {
    it("marks task as completed", () => {
      let s = addTask(state, "Learn TS");
      s = completeTask(s, 1);
      expect(s.items[0].completed).toBe(true);
    });
  });

  describe("removeTask", () => {
    it("removes the task", () => {
      let s = addTask(state, "Learn TS");
      s = removeTask(s, 1);
      expect(s.items).toHaveLength(0);
    });
  });

  describe("listTasks", () => {
    beforeEach(() => {
      state = addTask(state, "Task 1");
      state = addTask(state, "Task 2");
      state = completeTask(state, 1);
    });

    it("lists all tasks by default", () => {
      expect(listTasks(state)).toHaveLength(2);
    });

    it("filters completed", () => {
      expect(listTasks(state, "completed")).toHaveLength(1);
    });

    it("filters pending", () => {
      expect(listTasks(state, "pending")).toHaveLength(1);
    });
  });

  describe("getStats", () => {
    it("returns correct stats", () => {
      let s = addTask(state, "Task 1");
      s = addTask(s, "Task 2");
      s = completeTask(s, 1);
      expect(getStats(s)).toEqual({ total: 2, completed: 1, pending: 1 });
    });

    it("returns zeros for empty state", () => {
      expect(getStats(state)).toEqual({ total: 0, completed: 0, pending: 0 });
    });
  });
});
```

---

## 5. 类型安全测试

> 💡 `expectTypeOf` 是 **Vitest ≥ 1.0** 内置的类型测试工具，无需额外安装。

```typescript
// __tests__/type-tools.test.ts
import { describe, it, expectTypeOf } from "vitest";
import type { CreateInput, UpdateInput, DeepPartial, CrudEvent } from "../src/utils/type-tools";
import type { Task } from "../src/types/task";

describe("type-tools", () => {
  it("CreateInput excludes id and createdAt", () => {
    type Input = CreateInput<Task>;
    expectTypeOf<Input>().not.toHaveProperty("id");
    expectTypeOf<Input>().not.toHaveProperty("createdAt");
    expectTypeOf<Input>().toHaveProperty("title");
  });

  it("UpdateInput is Partial of CreateInput", () => {
    type Input = UpdateInput<Task>;
    expectTypeOf<Input["title"]>().toEqualTypeOf<string | undefined>();
  });

  it("DeepPartial makes all properties optional", () => {
    type PartialTask = DeepPartial<Task>;
    const partial: PartialTask = {};
    expectTypeOf<PartialTask>().toMatchTypeOf<Partial<Task>>();
  });

  it("CrudEvent is a discriminated union", () => {
    type E = CrudEvent<Task>;
    expectTypeOf<E>().toMatchTypeOf<{ type: string }>();
  });
});
```

---

## 6. ESLint + Prettier 配置

```javascript
// eslint.config.mjs
import tsParser from "@typescript-eslint/parser";
import tsPlugin from "@typescript-eslint/eslint-plugin";
import prettier from "eslint-config-prettier";

export default [
  {
    files: ["src/**/*.ts"],
    languageOptions: { parser: tsParser },
    plugins: { "@typescript-eslint": tsPlugin },
    rules: {
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-unused-vars": "error",
      "@typescript-eslint/consistent-type-imports": "error",
    },
  },
  {
    files: ["__tests__/**/*.ts"],
    languageOptions: { parser: tsParser },
    plugins: { "@typescript-eslint": tsPlugin },
    rules: {
      "@typescript-eslint/no-explicit-any": "off",
    },
  },
  { ignores: ["dist/", "node_modules/", "coverage/"] },
  prettier,
];
```

---

## 7. Git Hooks

```bash
npx husky init
```

```bash
# .husky/pre-commit
npx lint-staged
npx tsc --noEmit
```

```json
// package.json
{
  "lint-staged": {
    "*.ts": ["eslint --fix", "prettier --write"]
  }
}
```

---

## 8. GitHub Actions CI

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20, 22]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: "npm"
      - run: npm ci
      - run: npx tsc --noEmit
      - run: npx eslint src/
      - run: npx vitest run --coverage
      - run: npm run build
```

---

## 9. package.json scripts

```jsonc
{
  "scripts": {
    "dev": "tsx src/main.ts",
    "build": "tsc -p tsconfig.build.json",
    "typecheck": "tsc --noEmit",
    "lint": "eslint src/ --ext .ts",
    "lint:fix": "eslint src/ --ext .ts --fix",
    "format": "prettier --write \"src/**/*.ts\" \"__tests__/**/*.ts\"",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage",
    "check": "npm run typecheck && npm run lint && npm run test && npm run build",
    "prepare": "husky"
  }
}
```

---

## ✏️ 你的任务

1. 为 v5 的每个模块编写单元测试（entity-manager + task-service，≥ 15 个用例）
2. 用 `expectTypeOf` 编写类型安全测试
3. 配置 ESLint + Prettier + husky
4. 配置 GitHub Actions CI（多 Node 版本矩阵）
5. 确保覆盖率 ≥ 80%

---

## ✅ 验收标准

| 检查项 | 方式 |
|--------|------|
| 单元测试覆盖核心逻辑 | `vitest run` 全绿 |
| 覆盖率 ≥ 80% | `vitest run --coverage` |
| `no-explicit-any` 规则生效 | ESLint 报错 |
| pre-commit hook 运行 | `git commit` 触发 |
| CI 多版本矩阵通过 | GitHub Actions 绿 |

```bash
# 一键验证全部
npm run check   # typecheck + lint + test + build
```

---

## 💡 参考实现

<details>
<summary>点击展开参考答案</summary>

```typescript
// 完整实现见 code/v6/ 目录（⚠️ 参考实现暂未落地，以本篇代码为准）
// 关键点：
// 1. beforeEach 重置状态保证测试隔离
// 2. expectTypeOf 做类型层面测试
// 3. 不可变性测试——确认原状态未被修改
// 4. CI 检查顺序：typecheck → lint → test → build
```

</details>

---

## 🎯 本版自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 为核心模块编写了单元测试，`vitest run` 全绿 |
| 🟢 基础 | 配置了 ESLint + Prettier，`npm run lint` 无错误 |
| 🟡 进阶 | 用 `expectTypeOf` 编写了类型安全测试 |
| 🟡 进阶 | 配置了 husky + lint-staged + GitHub Actions CI |
| 🔴 挑战 | 测试覆盖率 ≥ 80%，CI 多 Node 版本矩阵通过 |

---

*最后更新：2026-07-19*