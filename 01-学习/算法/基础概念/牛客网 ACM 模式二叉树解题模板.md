---
title: 牛客网 ACM 模式二叉树解题模板
created: 2026-06-09
tags:
  - 算法模板
  - 牛客网
  - ACM模式
  - 二叉树
  - 输入输出
description: 牛客网 ACM 模式下二叉树的完整解题模板，包括手动 build_tree、输入解析、多组测试用例处理。
lark_doc_url: https://my.feishu.cn/docx/OEyrdSLF4oJpuWx0QTxc2GzLnub
---
## 核心区别

| | LeetCode 核心模式 | 牛客网 ACM 模式 |
|---|---|---|
| 你写什么 | 只写核心算法 | **完整程序**：输入 → 建树 → 解题 → 输出 |
| `root` 从哪来 | 后台自动传 | **自己从字符串构建** |
| 输入处理 | 不需要 | `sys.stdin` 读原始字符串 |
| 输出 | `return` | `print()` |

---

## 完整模板

```python
import sys
from collections import deque
from typing import Optional, List


# ===== 1. 定义 TreeNode =====
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# ===== 2. build_tree =====
def build_tree(values: List[str]) -> Optional[TreeNode]:
    """
    把字符串数组转成二叉树
    默认用 '#' 表示空节点
    如果输入用 'null' 或 'None'，把下面 != '#' 改成对应的即可
    """
    if not values or values[0] in ('#', 'null', 'None'):
        return None

    root = TreeNode(int(values[0]))
    queue = deque([root])
    i = 1

    while queue and i < len(values):
        node = queue.popleft()

        # 左孩子
        if i < len(values) and values[i] not in ('#', 'null', 'None'):
            node.left = TreeNode(int(values[i]))
            queue.append(node.left)
        i += 1

        # 右孩子
        if i < len(values) and values[i] not in ('#', 'null', 'None'):
            node.right = TreeNode(int(values[i]))
            queue.append(node.right)
        i += 1

    return root


# ===== 3. 解题类（替换成你的 Solution）=====
class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if not root:
            return False
        if not root.left and not root.right:
            return targetSum == root.val
        return (self.hasPathSum(root.left, targetSum - root.val) or
                self.hasPathSum(root.right, targetSum - root.val))


# ===== 4. ACM 入口 =====
if __name__ == "__main__":
    # --- 单组测试用例 ---
    tree_data = sys.stdin.readline().strip().split(',')   # 按逗号分隔
    target = int(sys.stdin.readline().strip())
    root = build_tree(tree_data)
    result = Solution().hasPathSum(root, target)
    print("true" if result else "false")
```

---

## 输入格式变体

### 变体 1：逗号 + `#` 空节点（最常见）

```
输入：
5,4,8,11,#,13,4,7,2,#,#,#,1
22
```

```python
tree_data = sys.stdin.readline().strip().split(',')
target = int(sys.stdin.readline().strip())
root = build_tree(tree_data)
```

### 变体 2：空格 + `null` 空节点

```
输入：
5 4 8 11 null 13 4 7 2 null null null 1
22
```

```python
tree_data = sys.stdin.readline().strip().split()   # 默认按空白字符分隔
target = int(sys.stdin.readline().strip())

# 修改 build_tree：把 '#' 改成 'null'
def build_tree(values):
    if not values or values[0] == 'null':
        return None
    # ... 后续同理
```

### 变体 3：方括号 + 逗号（JSON 风格）

```
输入：
[5,4,8,11,null,13,4,7,2,null,null,null,1]
22
```

```python
import sys
line = sys.stdin.readline().strip()
line = line.strip('[]')                    # 去掉首尾中括号
tree_data = [x.strip() for x in line.split(',')]
target = int(sys.stdin.readline().strip())
root = build_tree(tree_data)
```

### 变体 4：多组测试用例

```
输入：
2                          ← 测试用例数量
5,4,8,11,#,13,4            ← 第一组树
22                         ← 第一组 target
1,2,3                      ← 第二组树
5                          ← 第二组 target
```

```python
if __name__ == "__main__":
    t = int(sys.stdin.readline().strip())
    for _ in range(t):
        tree_data = sys.stdin.readline().strip().split(',')
        target = int(sys.stdin.readline().strip())
        root = build_tree(tree_data)
        result = Solution().hasPathSum(root, target)
        print("true" if result else "false")
```

---

## 常见坑位

> [!warning] **坑 1：空节点的表示不统一**
> 有的题用 `#`，有的用 `null`，有的用 `None`。**先看题目的输入说明**，然后改 `build_tree` 里的判断。

> [!warning] **坑 2：输出格式要求**
> • 布尔值：有些题要求 `"true"/"false"`（小写），有些要求 `"True"/"False"`
> • 多个结果换行输出，不要用空格

> [!warning] **坑 3：读取时多余的空白字符**
> ```python
> # ✅ 正确：始终 .strip()
> line = sys.stdin.readline().strip()
>
> # ❌ 错误：忘了 strip，末尾有 \n
> line = sys.stdin.readline()
> ```

> [!warning] **坑 4：输入可能有多余空行**
> 某些题目的输入末尾有多余空行，用 `sys.stdin.read().splitlines()` 统一处理更稳：
> ```python
> lines = [l for l in sys.stdin.read().splitlines() if l.strip()]
> ```

---

## 解题流程速记

```
① 读输入    →  sys.stdin.readline() / split()
② 建树      →  build_tree(data)
③ 调用算法  →  Solution().xxx(root, ...)
④ 输出      →  print(结果)
```

---

## 相关笔记

- [[112. 路径总和]]
- [[二叉树基础与通用解题框架]]
