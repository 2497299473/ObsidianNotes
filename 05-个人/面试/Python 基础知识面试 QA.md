---
title: Python 基础知识面试 QA
date: 2026-06-18
tags:
  - 面试
  - Python
  - 编程基础
aliases:
  - Python基础知识
  - Python面试题
status: ✅ 已完成
lark_doc_url: https://my.feishu.cn/docx/H29YdMVgTofAtxx0Eldcnl3Inwf
---

> [!ABSTRACT] 快速概览
> 面向 AI 视觉检测面试的 Python 基础知识梳理，覆盖可变/不可变类型、异常处理、上下文管理器、面向对象、多态、设计模式等 11 个核心问题。

---

## Q12. Python 中有哪些可变和不可变的数据类型？

| 类别 | 类型 | 说明 |
|------|------|------|
| **不可变** | `int`, `float`, `complex` | 数值类型 |
| **不可变** | `str` | 字符串 |
| **不可变** | `bytes` | 字节串 |
| **不可变** | `tuple` | 元组——元素不可修改 |
| **不可变** | `frozenset` | 冻结集合——不可增删 |
| **不可变** | `bool` | 布尔（True/False，是 int 的子类） |
| **可变** | `list` | 列表——可增删改 |
| **可变** | `dict` | 字典——键值对可修改 |
| **可变** | `set` | 集合——可增删 |
| **可变** | `bytearray` | 可变字节数组 |

**快速验证方法**：用 `id()` 检查修改前后内存地址是否变化。

```python
# 不可变：修改后是新对象
x = 1
print(id(x))  # 比如 1407...
x += 1
print(id(x))  # 变了！是新对象

# 可变：原地修改
lst = [1, 2, 3]
print(id(lst))  # 比如 2903...
lst.append(4)
print(id(lst))  # 没变！同一个对象
```

---

## Q13. 划分数据可变与不可变的依据是什么？

**核心依据**：对象创建后，能否在**不改变内存地址**的前提下修改其内部状态（值）。

```
可变：    ┌─────────────┐
         │ addr: 0x100  │  → append(4) → 还是 0x100，内容变了
         │ [1, 2, 3]    │
         └─────────────┘

不可变：  ┌─────────────┐              ┌─────────────┐
         │ addr: 0x200  │  → x += 1 →  │ addr: 0x300  │  新对象！
         │ 1            │              │ 2            │
         └─────────────┘              └─────────────┘
```

**底层原因**：
- 不可变对象的 `__hash__` 依赖其值——如果值能原地改变，hash 会变，dict key / set 的逻辑就崩溃了
- 这就是为什么 list 不能做 dict 的 key——list 是可变的，hash 不稳定

**实际影响**：

```python
# 陷阱：函数默认参数不要用可变对象
def add_item(item, target=[]):     # ❌ 默认参数在函数定义时只创建一次
    target.append(item)
    return target

add_item(1)  # [1]
add_item(2)  # [1, 2] ← 不是 [2]！target 是同一个 list 对象

# 正确做法
def add_item(item, target=None):   # ✅
    if target is None:
        target = []
    target.append(item)
    return target
```

---

## Q14. Python 中有哪些方法能捕获到异常抛出？

```python
# 基础语法
try:
    risky_operation()
except ValueError as e:
    print(f"值错误: {e}")
except (TypeError, KeyError) as e:
    print(f"类型或键错误: {e}")
except Exception as e:       # 兜底——不推荐裸 except:
    print(f"未知错误: {e}")
else:
    print("try 块没抛异常才执行")
finally:
    print("无论如何都执行——清理资源")
```

**进阶用法**：

```python
# 1. 获取完整上下文
import traceback
try:
    1 / 0
except Exception:
    tb = traceback.format_exc()    # 完整堆栈字符串
    logger.error(tb)

# 2. raise from —— 异常链
try:
    config = parse_file(path)
except FileNotFoundError as e:
    raise ConfigError("配置加载失败") from e  # 保留原始异常链

# 3. 系统级信号转异常
import signal
signal.signal(signal.SIGTERM, lambda signum, frame: raise SystemExit)
```

**不推荐的写法**：

```python
# ❌ 裸 except —— 会吞掉 KeyboardInterrupt 和 SystemExit
try:
    do_something()
except:                    # 太宽泛
    pass                   # 静默吞异常，难以调试

# ✅ 至少捕获 Exception
try:
    do_something()
except Exception:
    logger.exception("unexpected error")
```

---

## Q15. 对文件的上下文管理器（Context Manager）了解吗？

**`with` 语句的本质**：自动管理资源获取和释放，即使发生异常也保证释放。

```python
# 不用 with 的写法（容易出 bug）
f = open("data.txt")
try:
    data = f.read()
finally:
    f.close()               # 你可能忘记写这行

# 用 with 的写法（Pythonic）
with open("data.txt") as f:
    data = f.read()
# 缩进块结束 → 自动 f.close()，即使 read() 抛异常
```

**不只是文件**——任何需要"获取→使用→释放"的资源都可以用：

```python
import threading
lock = threading.Lock()

with lock:                     # 等价于 lock.acquire() + lock.release()
    shared_counter += 1

# 数据库连接
with db.connection() as conn:
    conn.execute("...")

# 时间测量
import time
class Timer:
    def __enter__(self):
        self.start = time.time()
        return self
    def __exit__(self, *args):
        print(f"Elapsed: {time.time() - self.start:.2f}s")

with Timer():
    expensive_operation()
```

---

## Q16. 如果不用内置的 with，想自己实现一个上下文管理器，一般有什么方法？需要实现哪些必要的方法？

**方式一：实现 `__enter__` / `__exit__`（类方式）**

```python
class ManagedFile:
    def __init__(self, filename, mode="r"):
        self.filename = filename
        self.mode = mode

    def __enter__(self):
        """获取资源，返回值赋给 as 后面的变量"""
        self.file = open(self.filename, self.mode)
        return self.file

    def __exit__(self, exc_type, exc_val, exc_tb):
        """释放资源——即使发生异常也会调用"""
        self.file.close()
        # 返回 True 可以吞掉异常（不推荐）
        # return False 让异常继续传播（默认行为）

with ManagedFile("test.txt", "w") as f:
    f.write("hello")
```

**`__exit__` 的三个参数**：

| 参数 | 含义 | 正常退出时 |
|------|------|-----------|
| `exc_type` | 异常类型 | `None` |
| `exc_val` | 异常值 | `None` |
| `exc_tb` | 异常 traceback | `None` |

**方式二：`@contextmanager` 装饰器（生成器方式，更简洁）**

```python
from contextlib import contextmanager

@contextmanager
def managed_file(filename, mode="r"):
    f = open(filename, mode)    # yield 之前 = __enter__
    try:
        yield f                   # 把资源交出去
    finally:
        f.close()                 # yield 之后 = __exit__（保证执行）

with managed_file("test.txt") as f:
    f.write("hello")
```

**两种方式对比**：

| | 类方式 | 生成器方式 |
|------|--------|-----------|
| **代码量** | 较多 | 简洁 |
| **灵活性** | 高——可存储状态 | 中等 |
| **异常处理** | `__exit__` 可拦截异常 | `try/finally` 自动清理 |
| **适用场景** | 复杂资源管理（线程池、数据库连接池） | 简单资源管理（文件、锁） |

---

## Q17. 面向对象的三大特性是什么？

**封装（Encapsulation）**：隐藏内部实现，只暴露必要接口。
**继承（Inheritance）**：子类复用父类的属性和方法。
**多态（Polymorphism）**：同一接口对不同对象产生不同行为。

```python
# 封装：内部状态 + getter/setter 控制访问
class BankAccount:
    def __init__(self, owner):
        self.__balance = 0           # 私有属性
    def deposit(self, amount):
        if amount > 0:
            self.__balance += amount  # 只有通过方法才能修改
    def get_balance(self):
        return self.__balance

# 继承：复用基类
class Animal:
    def speak(self): pass

class Dog(Animal):
    def speak(self):                  # 重写
        return "Woof"

# 多态：同一接口，不同实现
def make_sound(animal: Animal):
    print(animal.speak())

make_sound(Dog())      # "Woof"
make_sound(Cat())      # "Meow"
```

### 追问：用普通函数也能实现这三大特性，为什么一定要用类？

这个问题问得很好——**在 Python 中，你确实可以用闭包 + 函数实现封装、继承、多态**。但类提供的是**规模化编程的能力**，而不只是"能不能做到"。

#### 一、用函数模拟三大特性——能做，但很吃力

```python
# ======= 封装：闭包可以隐藏状态 =======
def make_bank_account(owner):
    balance = 0                          # 闭包捕获的"私有变量"

    def deposit(amount):
        nonlocal balance
        if amount > 0:
            balance += amount

    def get_balance():
        return balance

    return deposit, get_balance          # 返回"方法"

deposit, get_balance = make_bank_account("张三")
deposit(100)
print(get_balance())                     # 100 —— 封装达成！

# ======= 继承：手动委托 + 原型链模拟 =======
def make_animal():
    def speak(self):
        raise NotImplementedError
    return {"speak": speak}

def make_dog():
    proto = make_animal()                # "继承"Animal
    def speak(self):
        return "Woof"
    proto["speak"] = speak               # "重写"
    return proto

dog = make_dog()
dog["speak"](dog)                        # "Woof" —— 但调用方式丑陋

# ======= 多态：鸭子类型本来就不依赖类 =======
def make_it_speak(obj):
    print(obj["speak"](obj))             # 只要 dict 里有 speak 就行

make_it_speak(make_dog())                # 函数方式也能多态
```

**结论**：技术上可行，但代码可读性、可维护性、IDE 支持全面崩塌。

#### 二、类到底带来了什么——函数做不到的事

| 需求 | 函数/闭包方式 | 类方式 |
|------|-------------|--------|
| **创建多个同类实例** | 每次调用工厂函数，但返回的是函数/字典，没有"同类"概念 | `Dog()` 创建实例，`isinstance(d, Dog)` 天然成立 |
| **类型检查** | 无法判断"这个字典是不是 dog" | `isinstance(obj, Dog)` 一行搞定 |
| **方法调用语法** | `dog["speak"](dog)` 或 `speak(dog)`——要手动传 self | `dog.speak()` —— Python 自动传 self |
| **继承链 + super()** | 手动管理原型链，极易出错 | `super().method()` + MRO 自动处理菱形继承 |
| **IDE 补全 / 类型提示** | 闭包返回的变量类型难以推断 | `class Dog(Animal):` → IDE 自动列出属性和方法 |
| **序列化 / pickle** | 闭包无法 pickle | 类的实例可以 pickle（只要类定义在模块顶层） |
| **`__dunder__` 协议** | 无法实现 `__len__`、`__iter__`、`__getitem__` 等 | `len(obj)`、`for x in obj`、`obj[key]` 全靠 dunder |
| **多人协作** | 每个人的闭包写法不同，团队难以统一 | 类提供统一的组织范式，新人看代码一眼能定位 |

#### 三、最关键的差异：数据 + 行为的绑定

```python
# 函数方式：数据和行为是分离的
account_data = {"owner": "张三", "balance": 0}

def deposit(data, amount):               # 数据和行为是两件事
    if amount > 0:
        data["balance"] += amount        # 谁都可以绕过 deposit 直接改 data

account_data["balance"] = -999           # 💥 封装被绕过

# 类方式：数据和行为绑定在一起
class BankAccount:
    def __init__(self, owner):
        self.__balance = 0               # 数据和操作它的方法在同一个"盒子"里

    def deposit(self, amount):
        if amount > 0:                   # 校验逻辑天然嵌入
            self.__balance += amount

# account.__balance = -999               # ❌ AttributeError —— 真正的封装
```

> 类把**数据**和**操作数据的行为**捆绑在一起。函数可以模拟，但它是一种"约定"——你承诺不绕过函数直接改数据。类是"约束"——`__balance` 从语言层面阻止外部直接访问。

#### 四、面试时的回答框架

> "技术上，闭包能实现封装，函数字典能模拟继承，鸭子类型本身就是多态——Python 的多态甚至不需要类。
>
> 但类解决的是**规模化问题**：当系统有 50 个 entity、每个 entity 有 10 个方法时，闭包方式会让代码变成意大利面条。类提供了类型系统（`isinstance`）、标准协议（`__dunder__`）、继承链（`super()` + MRO）、以及团队统一的组织范式。
>
> 换句话说：**函数能让你跑到终点，类给你的是高速公路**。"

---

## Q18. 封装一般要实现哪些方法？最基础的封装会做哪两件事？

**最基础的两件事**：

1. **属性私有化**——用 `_` 或 `__` 标记内部属性，防止外部随意修改
2. **提供 getter/setter**——通过方法控制属性访问，加入校验逻辑

```python
class Temperature:
    def __init__(self, celsius=0):
        self._celsius = celsius     # ① 私有化（约定）

    # ② 提供访问接口
    @property
    def celsius(self):
        return self._celsius

    @celsius.setter
    def celsius(self, value):
        if value < -273.15:         # 校验
            raise ValueError("温度不能低于绝对零度")
        self._celsius = value

    @property
    def fahrenheit(self):
        return self._celsius * 9/5 + 32  # 派生属性
```

**进阶封装方法**：

| 方法 | Python 语法 | 作用 |
|------|-----------|------|
| **私有属性** | `self.__attr` | 名称改编（name mangling），防意外覆盖 |
| **受保护属性** | `self._attr` | 约定：内部使用，不要直接改 |
| **属性访问器** | `@property` | 像属性一样访问，但可以加逻辑 |
| **只读属性** | `@property`（无 setter） | 只能读不能写 |
| **`__repr__`** | `def __repr__` | 调试时的字符串表示 |
| **`__str__`** | `def __str__` | 用户友好的字符串表示 |

**`_` 和 `__` 的区别**：

```python
class MyClass:
    def __init__(self):
        self._protected = 1   # 约定：别碰
        self.__private = 2    # 实际被重命名为 _MyClass__private

obj = MyClass()
print(obj._protected)         # 1 —— 可以访问，但不应该
# print(obj.__private)        # AttributeError!
print(obj._MyClass__private)  # 2 —— 可以绕过，但不应该
```

> 面试话术：Python 的封装靠约定而非强制——`_` 和 `__` 更多是团队约定和防止命名冲突，不像 Java 的 `private` 有编译器强制。理解了这一点，就知道 Python 封装的重点是**设计清晰的公开 API** 而非技术上的锁死。

---

## Q19. 了解菱形继承吗？能简单讲一下对继承的理解吗？

**菱形继承**：一个类同时继承两个类，这两个类又继承自同一个基类。

```
       A
      / \
     B   C
      \ /
       D

class A: pass
class B(A): pass
class C(A): pass
class D(B, C): pass  # ← 菱形
```

**核心问题**：D 在调用 A 的方法时，是通过 B 走还是通过 C 走？每个父类的方法应该被调用一次还是多次？

**Python 的解决方案**：**MRO（Method Resolution Order，方法解析顺序）+ C3 线性化算法**

### 什么是 MRO？

**MRO = Method Resolution Order（方法解析顺序）**。当你写 `obj.method()` 时，Python 需要决定去**哪个类**找 `method`。对于单继承这很简单——一路往上找父类。但多重继承时，一个类有多个父类，搜索顺序就变得关键。MRO 就是这个**搜索顺序的列表**。

```python
# MRO 就是一张"找人顺序表"
class D(B, C): ...

print(D.__mro__)
# (<class 'D'>, <class 'B'>, <class 'C'>, <class 'A'>, <class 'object'>)
#  ↓              ↓              ↓              ↓              ↓
#  先搜自己  →  再搜 B  →  再搜 C  →  再搜 A  →  最后搜 object
```

**一个最直观的理解**：MRO 就是把继承图（可能有分支、有菱形）**拍平成一条线**，保证每个类出现一次且顺序合理。

```
继承图（二维）                     MRO 链（一维）
     A                              D → B → C → A → object
    / \                                 ↑
   B   C             拍平 →         这就是 super() 的调用路径
    \ /
     D
```

**为什么需要 C3 线性化？** 不是随便拍平就行，必须满足两个约束：
1. 子类在父类之前（D 必须在 B、C、A 之前）
2. 父类的顺序保持类定义中的顺序（`D(B, C)` → B 必须在 C 之前）

C3 算法保证拍平后的顺序同时满足这两个约束——如果代码写得太乱导致无法同时满足，Python 直接报 `TypeError` 拒绝创建这个类。

```python
class A(B, C): ...    # 要求 B 在 C 前
class D(C, B): ...    # 要求 C 在 B 前
class E(A, D): ...    # 💥 TypeError! 无法同时满足 A 和 D 的父类顺序要求
```

**查看 MRO 的三种方式**：

```python
# 方式一：__mro__ 属性
print(D.__mro__)        # (<class 'D'>, <class 'B'>, <class 'C'>, ...)

# 方式二：mro() 方法
print(D.mro())          # [<class 'D'>, <class 'B'>, <class 'C'>, ...]

# 方式三：inspect 模块
import inspect
print(inspect.getmro(D))
```

```python
class A:
    def method(self):
        print("A")

class B(A):
    def method(self):
        print("B")
        super().method()      # super() 不一定调 A！取决于 MRO

class C(A):
    def method(self):
        print("C")
        super().method()

class D(B, C):
    def method(self):
        print("D")
        super().method()

D().method()
# 输出：D → B → C → A
# 每个类只调用一次！

print(D.__mro__)
# (D, B, C, A, object)
```

**MRO 的规则**：
1. 子类在父类之前
2. 父类的顺序按定义顺序（`D(B, C)` → B 在 C 之前）
3. 每个类只出现一次
4. C3 线性化保证一致性

### 追问：`super().method()` 到底是干嘛的？

**一句话**：`super()` 返回的是**MRO 链上的下一个类**，不是"父类"。

#### 最常见的误解

```python
class B(A):
    def method(self):
        super().method()      # ❌ 误解：调用 A.method()
                              # ✅ 真相：调用 MRO 链上 B 后面的那个类的 method()
```

在单继承中，B 后面确实是 A，所以表面看起来没问题。但一旦进入**多重继承**，这个区别就炸了。

#### 用 `super()` vs 不用的对比——菱形继承中的区别

```python
# ====== 不用 super()：硬编码调用父类 ======
class A:
    def method(self):
        print("A")

class B(A):
    def method(self):
        print("B")
        A.method(self)           # 硬编码：只调 A

class C(A):
    def method(self):
        print("C")
        A.method(self)           # 硬编码：只调 A

class D(B, C):
    def method(self):
        print("D")
        B.method(self)           # 手动调用 B
        C.method(self)           # 手动调用 C

D().method()
# 输出：D → B → A → C → A
#            ↑         ↑
#       A 被调了两次！而且 C 的方法没机会在 A 之前执行
```

```python
# ====== 用 super()：沿 MRO 链协作调用 ======
class A:
    def method(self):
        print("A")

class B(A):
    def method(self):
        print("B")
        super().method()         # 不指定调谁——让 MRO 决定

class C(A):
    def method(self):
        print("C")
        super().method()         # 同上

class D(B, C):
    def method(self):
        print("D")
        super().method()         # 整个链条只需要一个 super()！

D().method()
# 输出：D → B → C → A
# 每个类恰好调一次，顺序由 MRO 保证
```

#### 图解：`super()` 到底在做什么

```
D 的 MRO:  D → B → C → A → object

D.method() 中的 super()  →  指向 B
B.method() 中的 super()  →  指向 C   ← 注意！不是 A！
C.method() 中的 super()  →  指向 A
A.method() 中的 super()  →  指向 object
```

> **核心心法**：`super()` 不关心"我的父类是谁"，只关心"MRO 链上我后面是谁"。这就是为什么同一个 `B.method()` 里的 `super().method()`，在单继承时调 A，在 `D(B, C)` 的菱形中调 C——因为 B 在 MRO 中的位置变了，排在它后面的类也变了。

#### 为什么需要 `super()`？——协作式多重继承

Python 的多重继承设计基于一个前提：**每个类都愿意"合作"**——每个类在自己的 `method` 里调用 `super().method()`，把控制权交给 MRO 链上的下一个类。这样整个链条自动串联，每个类只需要关心自己的逻辑：

```python
class Logger:
    def __init__(self, **kwargs):
        super().__init__(**kwargs)   # 把 kwargs 传给下一个
        self.log = []

class DBConnection:
    def __init__(self, db_url, **kwargs):
        super().__init__(**kwargs)   # 同上
        self.db_url = db_url

class MyService(Logger, DBConnection):
    def __init__(self, db_url):
        super().__init__(db_url=db_url)  # 只调一次 super()

# MyService.__mro__ → (MyService, Logger, DBConnection, object)
# super() 链自动串联 Logger.__init__ → DBConnection.__init__ → object.__init__
```

如果不用 `super()` 而硬编码 `Logger.__init__(self, ...)` 和 `DBConnection.__init__(self, ...)`，你就得自己管理调用顺序、传参、还要防止重复调用——代码又脆又难维护。

#### 面试回答框架

> "`super()` 不是'调用父类'，而是'调用 MRO 链上的下一个类'。
>
> 在单继承中这两者恰好一致，所以很多人把它当成父类调用——这没问题，直到遇到多重继承。在菱形继承中，`super()` 配合 C3 线性化的 MRO，保证每个祖先类的方法恰好被调用一次，且顺序一致。
>
> 本质上，`super()` 是 Python 实现**协作式多重继承**的机制：每个类在自己的方法里调用 `super().method()`，把控制权交给链条上的下一个类，整个调用链由 MRO 自动编排。"

**对继承的理解**（面试时讲这三点）：

> ① 继承解决的是"代码复用"和"is-a 关系"——子类 is-a 父类。比如 `Dog is-a Animal`。
>
> ② Python 支持**多重继承**，通过 MRO（C3 线性化算法）决定方法搜索顺序，`super()` 沿 MRO 链向上调用，保证每个父类只被调用一次。
>
> ③ 但不建议滥用继承——**组合优先于继承**（GoF 设计原则）。大部分情况用组合（"has-a"）更灵活。只有在真正的"is-a"关系 + 需要多态时才用继承。

---

## Q20. 对多态的理解是什么？

**核心定义**：同一操作作用于不同对象时，产生不同的行为。调用方不需要知道对象的具体类型，只需要知道接口。

**Python 的鸭子类型（Duck Typing）**：

> "如果它走起路来像鸭子，叫起来像鸭子，那它就是鸭子。"

```python
# 不需要显式继承同一基类——只要有同名方法就可以
class Dog:
    def speak(self):
        return "Woof!"

class Cat:
    def speak(self):
        return "Meow!"

class Duck:
    def speak(self):
        return "Quack!"

def make_it_speak(animal):
    print(animal.speak())       # 不关心类型，只关心有没有 speak()

make_it_speak(Dog())   # Woof!
make_it_speak(Cat())   # Meow!
make_it_speak(Duck())  # Quack!
```

**Python 的多态 vs Java 的多态**：

| | Java 多态 | Python 多态 |
|------|----------|-----------|
| **实现方式** | 接口 `interface` / 抽象类 `abstract` | 鸭子类型——只要有方法即可 |
| **类型检查** | 编译时检查 | 运行时检查 |
| **耦合度** | 所有类必须显式声明实现同一接口 | 零耦合 |
| **安全性** | 编译期保证类型正确 | 运行时报 `AttributeError` |

**实际应用**：

```python
# 策略模式——不同算法实现同一接口
class DiscountStrategy:
    def calculate(self, price): ...

class NoDiscount(DiscountStrategy):
    def calculate(self, price):
        return price

class PercentOff(DiscountStrategy):
    def __init__(self, pct):
        self.pct = pct
    def calculate(self, price):
        return price * (1 - self.pct)

class FixedOff(DiscountStrategy):
    def __init__(self, amount):
        self.amount = amount
    def calculate(self, price):
        return max(0, price - self.amount)

# 调用方只需要知道 DiscountStrategy 接口
def checkout(price, strategy: DiscountStrategy):
    return strategy.calculate(price)
```

---

## Q21. 了解设计模式吗？（追问：双击软件图标唤醒后台应用而非新开窗口，这属于什么设计模式？）

**追问回答**：**单例模式（Singleton）**——确保一个类只有一个实例，并提供全局访问点。

```python
# 方式一：模块级单例（Pythonic 做法）
# config.py
class AppConfig:
    def __init__(self):
        self.settings = {}

config = AppConfig()  # 模块级变量，整个进程只有一份

# 其他文件
from config import config  # 每次 import 都是同一个对象


# 方式二：__new__ 控制实例化
class Singleton:
    _instance = None

    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

a = Singleton()
b = Singleton()
print(a is b)  # True


# 方式三：QSharedMemory / QLockFile（PyQt5 中实际使用的）
from PyQt5.QtCore import QSharedMemory, QSystemSemaphore

class SingleInstanceApp:
    """基于共享内存的单实例检测"""
    def __init__(self, app_id: str):
        self._key = app_id
        self._shared_memory = QSharedMemory(app_id)

    def is_already_running(self) -> bool:
        """检查是否已有实例在运行"""
        if self._shared_memory.attach():
            # 已存在，通知已有实例激活窗口
            self._shared_memory.detach()
            return True
        # 创建共享内存标记
        self._shared_memory.create(1)
        return False

# MainWindow 中使用
def main():
    app = QApplication(sys.argv)

    guard = SingleInstanceApp("MyApp_Unique_ID")
    if guard.is_already_running():
        # 唤醒已有实例 → 退出当前
        QMessageBox.warning(None, "提示", "程序已在运行")
        sys.exit(0)

    window = MainWindow()
    window.show()
    app.exec_()
```

**其他常见设计模式**（面试可能延伸问到）：

| 模式 | 含义 | Python 中的应用 |
|------|------|----------------|
| **单例** | 全局唯一实例 | 日志器、配置管理、数据库连接池 |
| **工厂** | 根据参数创建不同子类对象 | `pathlib.Path()` 根据 OS 返回 `PosixPath` 或 `WindowsPath` |
| **观察者** | 一对多通知（= 信号槽的原始形态） | PyQt 信号槽就是观察者模式的实现 |
| **策略** | 可替换的算法族 | `sorted(key=fn)` —— 传入不同 key 函数改变排序策略 |
| **装饰器** | 动态增加功能 | Python `@decorator` 语法、Django 的 `@login_required` |
| **适配器** | 不兼容接口的转换层 | 对接第三方 SDK 时写 wrapper |
| **模板方法** | 基类定义骨架，子类实现细节 | `unittest.TestCase`（`setUp` / `tearDown` 钩子） |
| **状态** | 对象根据内部状态改变行为 | GUI 按钮的 enabled/disabled 状态切换 |

---

## Q22. Python 是在上学期间学的，还是毕业后自学的？有没有系统性地学过？

这道题在探你的**学习路径**和**基础深度**。

**回答建议**：
- 如实说学习路径（上学 or 自学）
- 强调"系统学习"的标志：不是只会写 CRUD，而是理解 why——从内存管理到元类
- 如果有短板（如没系统学过编译原理），坦诚 + 展示补充学习能力

**系统性学习的标志**（面试官期待你覆盖以下知识层）：

```
应用层：框架、库、API 调用
  ↑
语言层：面向对象、异常、迭代器、生成器、装饰器、上下文管理器
  ↑
解释器层：GIL、垃圾回收（引用计数 + 分代回收）、字节码
  ↑
数据模型层：__dunder__ 协议、描述符、元类、属性查找链
```

> 面试话术：虽然主要靠自学，但我不是"哪里不会查哪里"的碎片化学习——我系统性地研究过 Python 数据模型（`__init__` → `__new__` → `__call__` → 元类这条链）、内存管理机制（引用计数 + GC）、以及并发模型（GIL 对多线程的影响）。这些底层理解帮我在实际开发中避免了很多坑。

---

## 🔗 关联笔记

- [[面试/2026-06-17 AI视觉检测面试复盘]] | 完整面试复盘（43 题）
- [[面试/面试自我介绍]] | 面试开场稿
- [[面试/开立空调仿真工具 项目面试 QA]] | PyQt5 实战项目
