---
title: 坚果云 Nutstore Sync 多端同步配置
tags:
  - Obsidian
  - 同步
  - 坚果云
  - 配置
created: 2026-06-08
lark_doc_url: https://my.feishu.cn/docx/ZvxTdfFS6oeg2GxgWsNcBNGsnhx
---

## 目标

以**这台电脑**为唯一写入源，其他设备只拉取更新，不反向覆盖。

---

## 配置方案

### 🖥️ 源头设备（这台电脑）

插件设置 `.obsidian/plugins/nutstore-sync/data.json`：

```json
{
  "syncMode": "strict",
  "conflictStrategy": "latest-timestamp",
  "confirmBeforeSync": true,
  "realtimeSync": true,
  "autoSyncIntervalSeconds": 300
}
```

行为：写了就推，实时上传到坚果云。

---

### 📱 其他设备（只读查看）

插件设置：

```json
{
  "syncMode": "strict",
  "conflictStrategy": "latest-timestamp",
  "confirmBeforeSync": true,
  "realtimeSync": false,
  "autoSyncIntervalSeconds": 0
}
```

行为：**不自动同步**、不定时同步。只在手动触发时从云端拉取。

---

## 原理

```
你的电脑                    坚果云                   其他设备
───────                    ──────                   ────────
realtimeSync: true         云端 Vault/         realtimeSync: false
autoSync: 300s             （唯一真实源）       autoSync: 0
↓ 写笔记即推                                    ↓ 手动拉取，不写回
```

---

## 关键参数说明

| 参数 | 含义 | 源头值 | 只读值 |
|------|------|--------|--------|
| `syncMode` | `strict` 严格追踪每个文件变化；`loose` 跳过同名同大小文件 | `strict` | `strict` |
| `conflictStrategy` | 冲突时 `latest-timestamp` 按时间戳（新胜旧）；`diff-match-patch` 合并文本差异；`skip` 跳过 | `latest-timestamp` | `latest-timestamp` |
| `realtimeSync` | 文件修改后是否立即同步 | `true` | **`false`** |
| `autoSyncIntervalSeconds` | 定时同步间隔（秒），`0` 禁用 | `300` | **`0`** |
| `confirmBeforeSync` | 同步前弹窗确认 | `true` | `true` |

---

## 排除规则（不上传的文件）

2026-07-20 新增：排除所有代码文件，只同步 Markdown 等非代码内容。

```json
"filterRules": {
  "exclusionRules": [
    { "expr": "**/*.py", "options": { "caseSensitive": false } },
    { "expr": "**/*.pyc", "options": { "caseSensitive": false } },
    { "expr": "**/*.ts", "options": { "caseSensitive": false } },
    { "expr": "**/*.tsx", "options": { "caseSensitive": false } },
    { "expr": "**/*.js", "options": { "caseSensitive": false } },
    { "expr": "**/*.jsx", "options": { "caseSensitive": false } },
    { "expr": "**/*.mjs", "options": { "caseSensitive": false } },
    { "expr": "**/*.go", "options": { "caseSensitive": false } },
    { "expr": "**/*.rs", "options": { "caseSensitive": false } },
    { "expr": "**/*.java", "options": { "caseSensitive": false } },
    { "expr": "**/*.cpp", "options": { "caseSensitive": false } },
    { "expr": "**/*.c", "options": { "caseSensitive": false } },
    { "expr": "**/*.h", "options": { "caseSensitive": false } },
    { "expr": "**/*.hpp", "options": { "caseSensitive": false } },
    { "expr": "**/*.html", "options": { "caseSensitive": false } },
    { "expr": "**/*.css", "options": { "caseSensitive": false } }
  ]
}
```

> [!warning] 已有云端文件需手动删除
> 排除规则只能阻止后续上传。已在坚果云 `Vault/` 目录下的代码文件需要在 [坚果云网页端](https://www.jianguoyun.com) 手动删除。
