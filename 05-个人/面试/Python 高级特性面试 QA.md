---
title: Python 高级特性面试 QA
date: 2026-06-18
tags:
  - 面试
  - Python
  - 编程基础
aliases:
  - Python高级特性
  - 描述符协议
  - 元类
status: ✅ 已完成
lark_doc_url: https://my.feishu.cn/docx/UY4XdUfKuo3oYsxtSWsc72hXnKc
---

> [!ABSTRACT] 快速概览
> 面向面试的 Python 高级特性速查，覆盖描述符协议、`__init_subclass__`、元类、属性查找链、迭代器/生成器底层协议、`__slots__` 内存优化。每道题都有"面试官想听到什么"。

---

## 一、描述符协议（Descriptor Protocol）

> 面试官问这个，可能在探你对 `@property`、`@classmethod`、`@staticmethod` 底层实现的理解。

### 什么是描述符？

**定义**：实现了 `__get__` / `__set__` / `__delete__` 中任意一个方法的类，它的实例就是描述符。

```python
class Descriptor:
    def __get__(self, instance, owner):
        """读取属性时调用"""
        ...

    def __set__(self, instance, value):
        """设置属性时调用"""
        ...

    def __delete__(self, instance):
        """删除属性时调用"""
        ...
```

### 描述符是怎么工作的？

Python 属性查找时，如果发现类属性是描述符，会**拦截**点号访问，走描述符协议而非直接从 `__dict__` 取值。

```
obj.attr  # 发生了什么？

① 找 type(obj).__dict__["attr"]   → 找到了，是一个描述符对象
② 调 descriptor.__get__(obj, type(obj))
③ __get__ 返回什么，obj.attr 就是什么
```

### 三种描述符类型

| 类型 | 实现了 | 优先级 | 例子 |
|------|--------|:---:|------|
| **数据描述符** | `__get__` + `__set__`（或 `__delete__`） | **最高**——覆盖实例 `__dict__` | `property` |
| **非数据描述符** | 只有 `__get__` | 低于实例 `__dict__` | `classmethod`、`staticmethod`、普通函数 |
| **无描述符** | 啥都没实现 | 最低 | 普通类属性 |

### 实战示例

```python
# 一个类型校验的描述符
class Typed:
    """数据描述符：强制属性为指定类型"""
    def __init__(self, name, expected_type):
        self.name = name          # 存储属性名
        self.expected_type = expected_type

    def __get__(self, instance, owner):
        if instance is None:
            return self           # 类级别访问返回描述符本身
        return instance.__dict__.get(self.name)

    def __set__(self, instance, value):
        if not isinstance(value, self.expected_type):
            raise TypeError(f"{self.name} 必须是 {self.expected_type}")
        instance.__dict__[self.name] = value  # 存在实例的 __dict__ 里

    def __delete__(self, instance):
        del instance.__dict__[self.name]


class Person:
    name = Typed("name", str)     # 类属性是描述符
    age = Typed("age", int)

    def __init__(self, name, age):
        self.name = name          # 触发 Typed.__set__
        self.age = age            # 触发 Typed.__set__


p = Person("张三", 30)            # ✅
# p.age = "三十"                  # ❌ TypeError: age 必须是 int
```

**为什么数据存在 `instance.__dict__` 里而不是描述符里？**

> 描述符是**类属性**——所有实例共享同一个描述符对象。如果数据存描述符里，所有实例会互相覆盖。正确做法：描述符负责**校验和拦截**，真正的数据存实例自己的 `__dict__`。

### 内置描述符——`property` 的底层实现

```python
# 你写的
class C:
    @property
    def x(self): return self._x

    @x.setter
    def x(self, val): self._x = val

# 等价于描述符的语法糖：
class Property:
    """简化版 property 实现"""
    def __init__(self, fget=None, fset=None, fdel=None):
        self.fget = fget
        self.fset = fset
        self.fdel = fdel

    def __get__(self, instance, owner):
        if instance is None: return self
        return self.fget(instance)

    def __set__(self, instance, value):
        if self.fset is None:
            raise AttributeError("can't set attribute")
        self.fset(instance, value)

    def __delete__(self, instance):
        if self.fdel is None:
            raise AttributeError("can't delete attribute")
        self.fdel(instance)

    def setter(self, fset):
        return Property(self.fget, fset, self.fdel)

    def deleter(self, fdel):
        return Property(self.fget, self.fset, fdel)
```

> 面试话术：`@property` 本质是一个**数据描述符**——同时实现了 `__get__` 和 `__set__`。这也是为什么实例 `__dict__` 中的同名属性会被 `property` 覆盖——数据描述符的查找优先级最高。

---

## 二、`__init_subclass__` —— 子类注册钩子

> 面试官问这个，可能在探你是否深入理解类的创建过程。

### 是什么？

`__init_subclass__` 是 Python 3.6 引入的**类钩子**——当一个类被继承时，**父类的** `__init_subclass__` 会被自动调用。

```python
class Base:
    def __init_subclass__(cls, **kwargs):
        """每次有人继承 Base，这个函数都会执行"""
        super().__init_subclass__(**kwargs)
        print(f"{cls.__name__} 继承了 Base")
        # cls 是子类本身，不是实例

class Foo(Base):   # 打印: Foo 继承了 Base
    pass

class Bar(Base):   # 打印: Bar 继承了 Base
    pass
```

### 实际应用场景

```python
# 场景一：自动注册插件
class Plugin:
    _registry = []

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        Plugin._registry.append(cls)


class PDFExporter(Plugin):   # 自动注册
    pass

class CSVExporter(Plugin):   # 自动注册
    pass

print(Plugin._registry)  # [PDFExporter, CSVExporter]


# 场景二：强制子类实现接口 + 命名规范检查
class APIView:
    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        # 强制：子类名必须以 View 结尾
        if not cls.__name__.endswith("View"):
            raise TypeError(f"API 视图类必须以 View 结尾: {cls.__name__}")
        # 强制：子类必须定义 allowed_methods
        if not hasattr(cls, "allowed_methods"):
            raise TypeError(f"{cls.__name__} 必须定义 allowed_methods")


class UserView(APIView):           # ✅
    allowed_methods = ["GET", "POST"]

# class UserAPI(APIView):          # ❌ TypeError: 必须以 View 结尾
#     allowed_methods = ["GET"]
```

### `__init_subclass__` vs 元类 `__init__`

| | `__init_subclass__` | 元类 `__init__` |
|------|------|------|
| **定义位置** | 基类 | 元类 |
| **何时触发** | 基类被继承时 | 类对象被创建时 |
| **复杂度** | 简单，一个方法 | 需要单独定义元类 |
| **适用场景** | 注册、简单校验 | 修改类属性、注入方法、拦截类创建 |

```python
# 元类版本——更底层但更复杂
class PluginMeta(type):
    def __init__(cls, name, bases, namespace):
        super().__init__(name, bases, namespace)
        if name != "Plugin":  # 跳过基类本身
            cls._registry.append(cls)

class Plugin(metaclass=PluginMeta):
    _registry = []

# __init_subclass__ 版本——简单场景下更推荐
class Plugin:
    _registry = []
    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        cls._registry.append(cls)
```

---

## 三、元类（Metaclass）

> 面试官如果问到这个，说明在探你的 Python 深度天花板。

### 一句话：元类是类的类

```python
# 类是实例的模板，元类是类的模板
obj = MyClass()       # MyClass 创建了 obj
MyClass = type(...)   # type 创建了 MyClass ← type 就是元类

class Foo:
    pass

print(type(Foo))      # <class 'type'>  ← 默认元类是 type
print(type(1))        # <class 'int'>
print(type("hello"))  # <class 'str'>
```

### 类创建的完整流程

```python
# 这行代码：
class Foo(Bar):
    x = 1
    def method(self): pass

# Python 内部实际上执行了：
Foo = type(
    "Foo",              # ① 类名
    (Bar,),             # ② 父类元组
    {"x": 1, "method": method}  # ③ 类命名空间字典
)
```

### 自定义元类示例

```python
# 场景：给所有类的所有方法自动加日志
import functools

class LoggedMeta(type):
    def __new__(mcs, name, bases, namespace):
        """类创建时被调用"""
        for attr_name, attr_value in namespace.items():
            if callable(attr_value) and not attr_name.startswith("__"):
                # 给每个方法包一层日志
                namespace[attr_name] = mcs._wrap_with_log(attr_value, attr_name)
        return super().__new__(mcs, name, bases, namespace)

    @staticmethod
    def _wrap_with_log(func, name):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            print(f"[LOG] 调用 {name}")
            return func(*args, **kwargs)
        return wrapper


class Service(metaclass=LoggedMeta):
    def process(self, data):
        return data * 2

    def cleanup(self):
        print("清理...")


s = Service()
s.process(5)   # [LOG] 调用 process
s.cleanup()    # [LOG] 调用 cleanup
```

### 什么时候用元类 vs `__init_subclass__`？

| 需求 | 用哪个 |
|------|--------|
| 子类被创建时**注册到列表** | `__init_subclass__` |
| 子类被创建时**校验属性** | `__init_subclass__` |
| **修改**类的命名空间（如自动加装饰器） | 元类 |
| **拦截**类的创建过程 | 元类 |
| Django ORM 的 Model 基类 | 元类（`ModelBase`） |
| ABC 抽象基类 | 元类（`ABCMeta`） |

> 面试话术：元类 99% 的场景不需要手写。真正需要元类的标志是"你想在**类被定义时**自动做些什么"——比如 Django ORM 用元类扫描 `class Meta` 配置、`__module__` 等信息。简单场景优先用 `__init_subclass__`。

---

## 四、属性查找链（完整的优先级顺序）

> 面试官问「`obj.attr` 到底怎么查的？」时的完整回答。

```
obj.attr 查找顺序：

① 数据描述符（__get__ + __set__）
    └─ type(obj).__mro__ 中找数据描述符
       找到了 → 调用 __get__ → 返回
       没找到 ↓

② 实例 __dict__
    └─ obj.__dict__["attr"]
       找到了 → 返回
       没找到 ↓

③ 非数据描述符（只有 __get__）
    └─ type(obj).__mro__ 中找非数据描述符
       找到了 → 调用 __get__ → 返回
       没找到 ↓

④ 普通类属性
    └─ type(obj).__mro__ 中找普通属性
       找到了 → 返回
       没找到 ↓

⑤ __getattr__（最后的兜底）
    └─ 上述都没找到 → 调 obj.__getattr__("attr")
       还没实现 → AttributeError
```

```python
# 验证优先级
class DataDesc:
    def __get__(self, obj, owner): return "数据描述符"
    def __set__(self, obj, val): pass

class NonDataDesc:
    def __get__(self, obj, owner): return "非数据描述符"

class MyClass:
    data = DataDesc()         # 优先级 ①（最高）
    nondata = NonDataDesc()   # 优先级 ③

obj = MyClass()
obj.__dict__["data"] = "实例属性"     # 被 ① 覆盖，读不到
obj.__dict__["nondata"] = "实例属性"   # ② 覆盖 ③，读得到

print(obj.data)       # "数据描述符"（① 赢了）
print(obj.nondata)    # "实例属性"（② 赢了 ③）
```

---

## 五、迭代器与生成器协议

### 迭代器协议

```python
# 可迭代对象：实现 __iter__，返回迭代器
# 迭代器：实现 __iter__ + __next__

class CountDown:
    """迭代器：从 n 倒数到 0"""
    def __init__(self, start):
        self.current = start

    def __iter__(self):
        return self           # 迭代器返回自己

    def __next__(self):
        if self.current < 0:
            raise StopIteration
        val = self.current
        self.current -= 1
        return val


# 生成器：最简洁的迭代器写法
def countdown(n):
    while n >= 0:
        yield n
        n -= 1

# 生成器表达式
squares = (x**2 for x in range(10))  # ← 惰性求值，不占内存

# yield from：委托子生成器
def chain_generators():
    yield from range(3)       # 0, 1, 2
    yield from "AB"           # 'A', 'B'
```

> 面试话术：`for` 循环背后是迭代器协议——先调 `__iter__()` 拿迭代器，再反复调 `__next__()` 直到 `StopIteration`。生成器是迭代器的语法糖，`yield` 自动保存函数状态，省去了手写 `__iter__` / `__next__` 的 boilerplate。

---

## 六、`__slots__` 内存优化

```python
# 默认：每个实例有一个 __dict__（字典），动态存储属性 → 内存开销大
class Normal:
    def __init__(self, x, y):
        self.x = x
        self.y = y

# __slots__：预分配固定内存槽位 → 免去 __dict__ 的字典开销
class Slim:
    __slots__ = ("x", "y")   # 只允许这两个属性

    def __init__(self, x, y):
        self.x = x
        self.y = y

# Slim 实例没有 __dict__，不能动态加属性
# obj = Slim(1, 2); obj.z = 3  # ❌ AttributeError
```

| | `__dict__`（默认） | `__slots__` |
|------|------|------|
| **内存** | ~1KB+ per instance | ~56 bytes per slot |
| **动态属性** | 可随时添加 | 禁止 |
| **继承** | 子类默认也有 `__dict__` | 子类需显式定义 `__slots__` 才继承 |
| **适用场景** | 普通对象，数量少 | 大量小对象（如数据点、事件） |

> 面试话术：在需要创建数千个同类对象时（如仿真数据点），`__slots__` 可大幅减少内存占用。代价是失去动态性和一定程度的不兼容（如不能 pickle）。

---

## 七、面试速查表

| 追问 | 一句话回答 |
|------|-----------|
| **「描述符的 `__set_name__` 是什么？」** | Python 3.6+ 加的方法——描述符创建时自动调用，传入属性名和所属类，避免手动传名字 |
| **「`__new__` vs `__init__`？」** | `__new__` 创建对象（分配内存），`__init__` 初始化对象。`__new__` 先调用，返回实例后 `__init__` 才执行 |
| **「元类冲突怎么解决？」** | 多个父类有不同元类 → 定义一个同时继承所有父类元类的「联合元类」 |
| **「`yield from` 做了什么？」** | 把迭代委托给子生成器，自动传递 `send()`/`throw()`/`close()`，比自己写 for 循环更正确 |
| **「什么情况不用 `__slots__`？」** | 需要动态属性、多继承复杂、需要 pickle/weakref、团队不熟悉——默认 `__dict__` 没毛病 |

---

## 🔗 关联笔记

- [[面试/Python 基础知识面试 QA]] | Python 基础（前置知识）
- [[面试/2026-06-17 AI视觉检测面试复盘]] | 面试复盘主笔记
- [[面试/设计模式面试速查]] | 设计模式
