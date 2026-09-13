---
title: 03-类型声明文件与npm包
created: 2026-07-17
stage: 3
order: 3
difficulty: ⭐⭐⭐⭐
estimated_hours: 4
tags:
  - TypeScript
  - 声明文件
  - .d.ts
  - npm包
  - DefinitelyTyped
lark_doc_url: https://my.feishu.cn/docx/Pm5ZdLyrPoxZf4xiNDIc9q1cneg
---

## ⬅️ 前置知识
- 需要：[[02-模块系统与命名空间]] — 模块导入/导出
- 需要：[[01-tsconfig与编译流程]] — declaration 配置
- 需要：[[../阶段二-进阶类型与类型体操/02-泛型与类型推断]] — 泛型接口

## ➡️ 后续笔记
- [[04-测试与CI]] — 测试中的类型声明
- [[99-阶段三复习检查点]] — 阶段三综合复习

## 🔗 关联笔记
- [[../00-TypeScript学习路径总索引]] — 返回总索引
- [[../毕业项目-TS任务管理器/v7-可发布npm包版]] — 毕业项目 v7 发布实践

---

## 1. 声明文件（.d.ts）基础

### 1.1 什么是声明文件

声明文件只包含类型信息，不包含实现代码，编译器只检查类型，不生成代码：

```typescript
// math.d.ts —— 只有类型签名，没有实现
export function add(a: number, b: number): number;
export function multiply(a: number, b: number): number;
export const PI: number;
```

### 1.2 自动生成声明文件

```jsonc
// tsconfig 中开启 declaration
{
  "compilerOptions": {
    "declaration": true,
    "declarationDir": "./dist/types"
  }
}
// 自动为每个 .ts 文件生成 .d.ts 声明文件
```

---

## 2. 为 JS 代码添加类型

### 2.1 手动编写声明文件

当使用一个没有类型的 JS 库时：

```typescript
// types/my-lib.d.ts
declare module "my-lib" {
  export function init(config: {
    apiKey: string;
    debug?: boolean;
  }): void;

  export function query(sql: string): Promise<unknown[]>;

  export default class MyLib {
    constructor(options: { host: string; port: number });
    connect(): Promise<void>;
    disconnect(): void;
  }
}
```

### 2.2 非代码文件声明

```typescript
// types/assets.d.ts
declare module "*.css" {
  const content: Record<string, string>;
  export default content;
}

declare module "*.svg" {
  const content: string;
  export default content;
}

declare module "*.png" {
  const content: string;
  export default content;
}
```

### 2.3 全局声明

```typescript
// global.d.ts
declare global {
  interface Window {
    __APP_CONFIG__: {
      apiUrl: string;
      version: string;
    };
  }

  interface String {
    capitalize(): string;
  }
}

export {}; // 必须导出，确保这是模块文件
```

### 2.4 环境变量声明

```typescript
// env.d.ts
interface ImportMetaEnv {
  readonly VITE_API_URL: string;
  readonly VITE_APP_TITLE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
```

### 2.5 扩展第三方模块

```typescript
// types/express.d.ts
declare module "express" {
  interface Request {
    user?: {
      id: number;
      name: string;
    };
  }
}
```

---

## 3. JSDoc 注释与渐进式迁移

### 3.1 JSDoc 注释类型

```javascript
// utils.js
/**
 * @param {string} name
 * @param {number} age
 * @returns {{ name: string; age: number }}
 */
function createUser(name, age) {
  return { name, age };
}
```

### 3.2 allowJs + checkJs

```jsonc
{
  "compilerOptions": {
    "allowJs": true,    // 允许导入 .js 文件
    "checkJs": true     // 对 .js 文件做类型检查
  }
}
```

### 3.3 渐进式迁移策略

```
步骤：allowJs → checkJs → 逐文件加 // @ts-check → strict → 逐文件 .js → .ts
```

---

## 4. DefinitelyTyped 与 @types

### 4.1 三种类型来源

| 来源 | 安装方式 | 适用场景 |
|------|---------|---------|
| 自带类型 | 包内包含 `.d.ts` | 现代 TS 包 |
| DefinitelyTyped | `npm i -D @types/xxx` | 社区维护 |
| 手动声明 | 项目内 `declare module` | 无类型支持的包 |

### 4.2 查找优先级

```
1. 包自身的 types 字段（package.json）
2. 包根目录下的 index.d.ts
3. node_modules/@types/xxx/index.d.ts
4. 项目内的 declare module
```

### 4.3 类型缺失时的处理

```typescript
// 方案一：跳过检查（不推荐长期使用）
// @ts-ignore
import something from "untyped-lib";

// 方案二：写一个最简声明
declare module "untyped-lib" {
  const value: any;
  export default value;
}

// 方案三：贡献类型到 DefinitelyTyped
// https://github.com/DefinitelyTyped/DefinitelyTyped
```

---

## 5. 发布带类型的 npm 包

### 5.1 package.json 配置

```jsonc
{
  "name": "my-ts-lib",
  "version": "1.0.0",
  "main": "./dist/index.js",
  "module": "./dist/index.mjs",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.js",
      "types": "./dist/index.d.ts"
    }
  },
  "files": ["dist", "README.md"]
}
```

### 5.2 tsconfig 输出声明文件

```jsonc
// tsconfig.build.json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "./dist",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "exclude": ["src/**/*.test.ts"]
}
```

### 5.3 推荐工具：tsup

```bash
npm install -D tsup
```

```typescript
// tsup.config.ts
import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["cjs", "esm"],
  dts: true,           // 自动生成声明文件
  clean: true,
  splitting: false,
});
```

---

## 6. 练习

### 练习 1：为第三方库编写声明文件

选择一个你常用的无类型 npm 包，为其编写完整的 `.d.ts` 声明文件。要求：声明所有主要 API 函数/类、配置选项类型、事件回调类型。

### 练习 2：发布一个 TypeScript 包

1. 创建一个 TS 工具函数库（至少 3 个函数 + 2 个接口）
2. 配置 tsconfig 生成 `.d.ts`
3. 配置 package.json 的 `types` 和 `exports` 字段
4. 用 `npm pack` 本地验证

### 练习 3：全局类型扩展

1. 扩展 `process.env` 类型，添加自定义环境变量
2. 扩展 `Express.Request` 类型，添加 `user` 属性
3. 验证 VS Code 自动补全

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能解释 `.d.ts` 声明文件的作用和生成方式 |
| 🟢 基础 | 能为 `*.css`、`*.svg` 等非代码文件编写声明模块 |
| 🟡 进阶 | 能为无类型的 JS 库编写完整的声明文件 |
| 🟡 进阶 | 能配置 npm 包的 `types` 和 `exports`，发布附带类型声明的包 |
| 🔴 挑战 | 能扩展第三方库的类型（如 Express Request），并编写 declare global |

---

*最后更新：2026-07-17*