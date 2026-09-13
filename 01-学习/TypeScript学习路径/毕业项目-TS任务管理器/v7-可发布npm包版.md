---
title: v7-可发布npm包版
created: 2026-07-17
project: TS 任务管理器
version: 7
difficulty: ⭐⭐⭐⭐
estimated_hours: 3
tags:
  - TypeScript
  - 毕业项目
  - npm包
  - 发布
  - 工程化闭环
lark_doc_url: https://my.feishu.cn/docx/B1dPdT7ksovfr1x5q55c1rMlnYc
---

## ⬅️ 前置知识
- 需要：[[01-学习/TypeScript学习路径/毕业项目-TS任务管理器/v6-测试与CI版|v6-测试与CI版]] — 测试与 CI 已就绪
- 需要：[[../阶段三-工程化与实战集成/03-类型声明文件与npm包]] — npm 包发布

## ➡️ 后续版本
- 无 — 这是毕业项目的最终版本

## 🔗 关联笔记
- [[../00-TypeScript学习路径总索引]] — 返回总索引
- [[../阶段三-工程化与实战集成/01-tsconfig与编译流程]] — 构建配置

---

## 🎯 v7 目标

将 v6 的项目打包为**可发布的 npm 包**，含双模块格式（CJS + ESM）、类型声明文件、完整文档。这是"能不能向社区交付"的判定——也是整个学习路径的终点。

---

## 📋 改造任务

### 1. tsup 构建配置

```bash
npm install -D tsup
```

```typescript
// tsup.config.ts
import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["cjs", "esm"],
  dts: true,              // 自动生成 .d.ts
  clean: true,            // 清空 outDir
  splitting: false,       // 不拆分 chunk
  sourcemap: true,        // 生成 sourcemap
  minify: false,          // 库不压缩，保留可读性
  treeshake: true,        // 摇树优化
});
```

### 2. package.json 发布配置

```jsonc
{
  "name": "ts-task-manager",
  "version": "1.0.0",
  "description": "A type-safe task manager built with TypeScript",
  "license": "MIT",
  "author": "Your Name",
  "keywords": ["typescript", "task-manager", "type-safe", "functional", "generic"],
  "repository": {
    "type": "git",
    "url": "https://github.com/yourname/ts-task-manager"
  },
  "main": "./dist/index.js",
  "module": "./dist/index.mjs",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "import": {
        "types": "./dist/index.d.ts",
        "default": "./dist/index.mjs"
      },
      "require": {
        "types": "./dist/index.d.ts",
        "default": "./dist/index.cjs"
      }
    },
    "./types": {
      "import": {
        "types": "./dist/types/index.d.ts",
        "default": "./dist/types/index.mjs"
      },
      "require": {
        "types": "./dist/types/index.d.ts",
        "default": "./dist/types/index.cjs"
      }
    }
  },
  "files": ["dist", "README.md", "LICENSE"],
  "scripts": {
    "dev": "tsx src/main.ts",
    "build": "tsup",
    "typecheck": "tsc --noEmit",
    "lint": "eslint src/ --ext .ts",
    "test": "vitest run",
    "test:coverage": "vitest run --coverage",
    "check": "npm run typecheck && npm run lint && npm run test && npm run build",
    "prepublishOnly": "npm run check",
    "prepare": "husky"
  },
  "publishConfig": {
    "access": "public"
  },
  "devDependencies": {
    "typescript": "^5.5.0",
    "tsup": "^8.0.0",
    "vitest": "^2.0.0",
    "eslint": "^9.0.0"
  }
}
```

### 3. tsconfig.build.json

```jsonc
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "./dist",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "exclude": ["src/**/*.test.ts", "__tests__/**/*"]
}
```

### 4. .npmignore

```
src/
__tests__/
.github/
.eslintrc*
.prettierrc*
vitest.config.ts
tsconfig*.json
*.test.ts
```

### 5. README.md

```markdown
# ts-task-manager

A type-safe, functional task manager built with TypeScript.

## Features

- 🔒 Fully type-safe with TypeScript generics
- 🔄 Immutable state management
- 🎯 Event-driven architecture (discriminated unions)
- 📦 ESM + CJS dual module output
- 📘 Type declarations included

## Installation

\`\`\`bash
npm install ts-task-manager
\`\`\`

## Usage

\`\`\`typescript
import { createTaskState, addTask, completeTask, getStats } from "ts-task-manager";
import type { Task } from "ts-task-manager/types";

// 创建状态
let state = createTaskState();

// 添加任务
state = addTask(state, "学习 TypeScript", "high");
state = addTask(state, "写毕业项目", "high");

// 完成任务
state = completeTask(state, 1);

// 查看统计
console.log(getStats(state));
// { total: 2, completed: 1, pending: 1 }
\`\`\`

## API

### `createTaskState()`
Creates an empty task state.

### `addTask(state, title, priority?)`
Adds a new task. Returns new state (immutable).

### `completeTask(state, id)`
Marks a task as completed.

### `removeTask(state, id)`
Removes a task by id.

### `listTasks(state, filter?)`
Lists tasks with optional filter: `"all"` | `"completed"` | `"pending"`.

### `getStats(state)`
Returns `{ total, completed, pending }` stats.

## Type Safety

\`\`\`typescript
// ❌ TypeScript catches these at compile time
addTask(state, 123);                    // Error: title must be string
addTask(state, "test", "invalid");      // Error: priority must be "low"|"normal"|"high"
listTasks(state, "wrong");              // Error: filter must be "all"|"completed"|"pending"
\`\`\`

## License

MIT
```

### 6. CHANGELOG.md

```markdown
# Changelog

## 1.0.0 (2026-07-17)

- Initial release
- Generic entity management with CRUD operations
- Type-safe property lookup with `keyof T`
- Immutable state management
- ESM + CJS dual module format
- Type declarations included
```

---

## 📋 本地验证流程

### 1. 构建并检查产物

```bash
npm run build

# 检查 dist/ 目录
ls dist/
# 应有：index.cjs, index.mjs, index.d.ts
```

### 2. npm pack 验证包内容

```bash
npm pack
# 生成 ts-task-manager-1.0.0.tgz

tar -tzf ts-task-manager-1.0.0.tgz
# 应只包含 dist/、README.md、LICENSE、package.json
# 不应包含 src/、__tests__/、tsconfig.json 等
```

### 3. npm link 本地测试

```bash
# 在当前项目
npm link

# 在另一个测试项目中
cd ../test-project
npm link ts-task-manager

# ESM 导入测试
node --input-type=module -e "
import { createTaskState, addTask, getStats } from 'ts-task-manager';
let s = createTaskState();
s = addTask(s, '测试任务', 'high');
console.log(getStats(s));
"

# CJS 导入测试
node -e "
const { createTaskState, addTask } = require('ts-task-manager');
let s = createTaskState();
s = addTask(s, '测试任务', 'high');
console.log(s.items[0]);
"
```

### 4. 版本管理

```bash
npm version patch   # 1.0.0 → 1.0.1
npm version minor   # 1.0.0 → 1.1.0
npm version major   # 1.0.0 → 2.0.0
```

### 5. CI 自动发布工作流

```yaml
# .github/workflows/publish.yml
name: Publish

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      id-token: write
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          registry-url: https://registry.npmjs.org
      - run: npm ci
      - run: npm run check
      - run: npm publish --provenance
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

---

## ✏️ 你的任务

1. 用 `tsup` 构建项目，输出 CJS + ESM + `.d.ts`
2. 完善 `package.json` 的 `exports` 字段和 `files` 字段
3. 编写 README.md（含安装、使用、API 文档）和 CHANGELOG.md
4. 用 `npm pack` 验证包内容
5. 用 `npm link` 在另一个项目中测试 ESM 和 CJS 导入

---

## ✅ 验收标准

| 检查项 | 方式 |
|--------|------|
| `npm run build` 成功生成 CJS + ESM + .d.ts | `ls dist/` |
| `package.json` 的 `exports` 正确配置条件导出 | ESM 和 CJS 导入都可用 |
| `npm pack` 包只含必要文件 | `tar -tzf *.tgz` |
| `npm link` 后外部项目可导入 | ESM + CJS 双向测试 |
| README 包含安装和使用说明 | 文档完整 |

```bash
# 最终验证
npm run check           # typecheck + lint + test + build
npm pack                # 打包验证
tar -tzf *.tgz          # 检查包内容
```

---

## 💡 参考实现

<details>
<summary>点击展开参考答案</summary>

```typescript
// 完整实现见 code/v7/ 目录（⚠️ 参考实现暂未落地，以本篇代码为准）
// 关键文件：
// - tsup.config.ts
// - package.json（exports 字段）
// - README.md
// - CHANGELOG.md
// - .npmignore
// - .github/workflows/publish.yml
```

</details>

---

## 🎯 本版自检（毕业项目终版）

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | `npm run build` 成功生成 CJS + ESM + .d.ts |
| 🟢 基础 | 编写完整的 README.md 和 package.json |
| 🟡 进阶 | `npm pack` 生成的 .tgz 可以在外部项目正常使用 |
| 🟡 进阶 | `exports` 字段配置了条件导出，ESM 和 CJS 都能正确导入 |
| 🔴 挑战 | 能从 v1 到 v7 独立走通整个流程，并解释每个版本的设计决策 |

---

## 🎉 毕业项目完成！

回顾 v1 到 v7 的递进：

| 版本 | 核心技能 | 一句话总结 |
|------|---------|-----------|
| v1 | 类型标注 | 从 JS 到 TS 的第一步 |
| v2 | 函数式编程 | 不可变、纯函数、高阶函数、可辨识联合 |
| v3 | 泛型抽象 | 从 Task 到通用的 Entity 设计 |
| v4 | 类型体操 | 条件类型、映射类型、模板字面量 |
| v5 | 模块化 | 拆分模块、路径别名、声明文件 |
| v6 | 测试与 CI | 测试驱动、自动化流水线 |
| v7 | npm 包发布 | 双模块格式、文档、发布 |

完成 v7 后你已经具备了用 TypeScript 从零搭建生产级项目的能力。🦐

**下一步建议**：
- 刷 [type-challenges](https://github.com/type-challenges/type-challenges) 提升类型体操能力
- 为开源项目贡献类型声明到 DefinitelyTyped
- 阅读 TypeScript 编译器源码理解类型系统的实现

---

*最后更新：2026-07-17*