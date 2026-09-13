---
title: 01-tsconfig与编译流程
created: 2026-07-17
stage: 3
order: 1
difficulty: ⭐⭐⭐
estimated_hours: 3
tags:
  - TypeScript
  - tsconfig
  - 编译流程
  - 工程化
lark_doc_url: https://my.feishu.cn/docx/GIxmdqSzFoab4XxRNh3cY7uTnWb
---

## ⬅️ 前置知识
- 需要：[[../阶段二-进阶类型与类型体操/99-阶段二复习检查点]] — 类型系统已扎实
- 需要：[[../阶段一-类型系统基础/01-环境搭建与基本类型]] — 基础 tsconfig

## ➡️ 后续笔记
- [[02-模块系统与命名空间]] — module/moduleResolution 配置
- [[03-类型声明文件与npm包]] — declaration 与 outDir

## 🔗 关联笔记
- [[../00-TypeScript学习路径总索引]] — 返回总索引
- [[04-测试与CI]] — 测试环境 tsconfig

---

## 1. TypeScript 编译流程

```mermaid
flowchart LR
    SRC[".ts 源码"] --> SCAN["Scanner 扫描器<br/>词法分析"]
    SCAN --> PARSE["Parser 解析器<br/>生成 AST"]
    PARSE --> BIND["Binder 绑定器<br/>构建符号表"]
    BIND --> CHECK["Checker 类型检查器<br/>类型检查"]
    CHECK --> EMIT["Emitter 生成器<br/>输出代码"]
    EMIT --> OUT["输出 .js + .d.ts + .map"]
```

### 1.1 关键命令

```bash
# 编译单个文件
tsc src/app.ts

# 编译整个项目（按 tsconfig.json）
tsc

# 监听模式（文件变化自动编译）
tsc --watch

# 只做类型检查不生成文件
tsc --noEmit

# 生成声明文件（仅 .d.ts）
tsc --declaration --emitDeclarationOnly

# 项目引用增量编译
tsc --build

# 生成编译轨迹（性能分析）
tsc --generateTrace ./trace
```

> **关键认知**：即使有类型错误，TypeScript 也会生成 JS 输出（除非开启 `noEmitOnError`）——类型检查不影响编译产物，只影响你的信心。

---

## 2. tsconfig.json 完整配置

### 2.1 推荐配置（项目起点）

```jsonc
{
  "compilerOptions": {
    // === 目标与模块 ===
    "target": "ES2020",            // 编译目标 JS 版本
    "module": "ESNext",            // 模块系统
    "moduleResolution": "bundler", // 模块解析策略（TS 5.0+）
    "lib": ["ES2020", "DOM"],      // 可用的内置 API 类型库

    // === 严格模式 ===
    "strict": true,                // 总开关（下面 8 项全开）

    // === 互操作 ===
    "esModuleInterop": true,       // 兼容 CommonJS 默认导入
    "allowSyntheticDefaultImports": true,

    // === 输出 ===
    "outDir": "./dist",            // JS 输出目录
    "rootDir": "./src",           // 源码根目录
    "declaration": true,           // 生成 .d.ts
    "declarationMap": true,       // 生成 .d.ts.map
    "sourceMap": true,            // 生成 .js.map
    "removeComments": true,        // 移除注释

    // === 性能与体验 ===
    "skipLibCheck": true,          // 跳过 .d.ts 检查加速
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,    // 允许 import .json

    // === 高级 ===
    "isolatedModules": true,      // 每个文件独立编译（配合 Vite/esbuild）
    "verbatimModuleSyntax": true   // import type 严格模式（TS 5.0+）
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### 2.2 strict 模式的 8 项子开关

| 开关 | 作用 | 为什么重要 |
|------|------|-----------|
| `noImplicitAny` | 禁止隐式 any | 防止类型信息丢失 |
| `strictNullChecks` | null/undefined 不可赋值给其他类型 | 消除最常见的运行时错误 |
| `strictFunctionTypes` | 函数参数检查改为逆变 | 更安全的回调类型 |
| `strictBindCallApply` | 严格 bind/call/apply | 防止 this 绑定错误 |
| `strictPropertyInitialization` | 类属性必须初始化 | 防止使用未初始化属性 |
| `noImplicitThis` | 禁止不明确的 this | 防止 this 指向错误 |
| `alwaysStrict` | 输出文件带 "use strict" | 以严格模式运行 |
| `useUnknownInCatchVariables` | catch 变量为 unknown | 防止 error 类型假设错误 |

### 2.3 target 与 lib 的关系

```jsonc
{
  // target 决定输出 JS 的语法版本（class、箭头函数等是否保留）
  "target": "ES2020",
  // lib 决定可用的内置 API 类型（Promise、Array.flat 等）
  "lib": ["ES2020", "DOM"]
  // 如果只用 Node.js，不需要 "DOM"
}
```

---

## 3. 多 tsconfig 策略

### 3.1 继承模式（extends）

```jsonc
// tsconfig.base.json — 团队共享基础配置
{
  "compilerOptions": {
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "target": "ES2020"
  }
}

// tsconfig.build.json — 构建配置
{
  "extends": "./tsconfig.base.json",
  "compilerOptions": {
    "outDir": "./dist",
    "declaration": true
  },
  "exclude": ["src/**/*.test.ts", "src/**/*.spec.ts"]
}

// tsconfig.dev.json — 开发配置
{
  "extends": "./tsconfig.base.json",
  "compilerOptions": {
    "sourceMap": true,
    "noUnusedLocals": false
  }
}
```

### 3.2 Project References（多项目）

```jsonc
// 根 tsconfig.json
{
  "files": [],
  "references": [
    { "path": "./packages/core" },
    { "path": "./packages/shared" },
    { "path": "./packages/app" }
  ]
}

// packages/core/tsconfig.json
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "outDir": "./dist",
    "composite": true,       // 必须开启
    "declaration": true
  },
  "references": [
    { "path": "../shared" } // 依赖 shared
  ]
}
```

```bash
# 用 --build 模式增量编译所有引用项目
tsc --build
```

---

## 4. 构建工具链对比

| 工具 | 特点 | 适用场景 |
|------|------|---------|
| `tsc` | 官方编译器，完整类型检查 | 库项目、需要 .d.ts |
| `ts-node` | 直接运行 TS | 脚本、测试 |
| `tsx` | ts-node 替代，更快 | 开发时运行 |
| `esbuild` | 极快，不做类型检查 | 生产构建 |
| `SWC` | Rust 编写，极快 | 生产构建 |
| `Vite` | 基于 esbuild + Rollup | 前端项目 |

> **最佳实践**：类型检查和构建分离。`tsc --noEmit` 做类型检查，用 esbuild/SWC 做快速构建。

```jsonc
// package.json scripts
{
  "scripts": {
    "typecheck": "tsc --noEmit",
    "build": "tsc -p tsconfig.build.json",
    "dev": "tsx watch src/index.ts",
    "build:fast": "esbuild src/index.ts --bundle --outfile=dist/index.js --platform=node"
  }
}
```

---

## 5. 路径别名配置

```jsonc
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"],
      "@utils/*": ["./src/utils/*"],
      "@types/*": ["./src/types/*"]
    }
  }
}
```

```typescript
import { Button } from "@components/Button"; // TS 编译时解析
// 但运行时 Node.js 不认识 @/，需要额外配置：
// 方案一：tsconfig-paths（ts-node 场景）
// 方案二：Vite/Webpack 的 alias 配置
// 方案三：tsc-alias（编译后自动替换路径）
```

---

## 6. 练习

### 练习 1：配置一个企业级 tsconfig

为 Node.js 后端服务创建 tsconfig，要求：
- `strict: true`
- 输出到 `dist/` 并生成 `.d.ts`
- sourcemap 仅在开发模式
- 排除 `__tests__` 目录
- 配置 `@/` 路径别名

### 练习 2：strict 模式实战

将以下非严格代码改写为通过 strict 检查：

```typescript
class Counter {
  count;  // 未初始化 → 报错
  increment() {
    this.count++;  // 隐式 any → 报错
  }
}

function parse(data) {  // 隐式 any → 报错
  return JSON.parse(data);
}
```

### 练习 3：Project References

将一个 monorepo 拆分为 `shared` + `app` 两个子项目，配置 references。

---

## 🎯 本文学完自检

| 层次 | 检查项 |
|------|--------|
| 🟢 基础 | 能解释 `target`、`module`、`outDir`、`rootDir`、`strict` 的作用 |
| 🟢 基础 | 能使用 `tsc`、`tsc --watch`、`tsc --noEmit` |
| 🟡 进阶 | 能列出 strict 模式的 7 项子开关及其作用 |
| 🟡 进阶 | 能配置 Project References 拆分大型项目 |
| 🔴 挑战 | 能用 `tsc --generateTrace` 分析编译性能瓶颈 |

---

*最后更新：2026-07-17*