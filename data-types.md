[阅读英文原版](https://github.com/nrc/r4cppp/blob/master/data-types.md)

# 数据类型

本文探讨 Rust 的数据类型。这和 C++ 中的类、结构体和枚举大体相同。Rust 的一个不同在于，和 C++（以及 Java 和其他的面向对象的语言）相比，Rust 中的数据和行为是严格分开的。行为通过函数定义，函数则可定义于特征和 `impl`（实现（implementations））中定义，而特征无法包含数据，这与 Java 中的接口（interface）相似。之后有一篇文章专讲特征和实现，这篇文章只讲数据。

## 结构体

Rust 中的结构体和 C 的结构体以及 C++ 中没有成员函数（译者注：原文为 method，但 C 和 C++ 都不用“方法”来称呼成员函数）的结构体类似。结构体就是个具名字段（field）的列表。语法见下例：

```rs
struct S {
    field1: i32,
    field2: SomeOtherSturct
}
```

以上代码定义了一个名为 `S`，带有两个字段的结构体。字段之间用逗号分隔，愿意的话也可以在最后一个字段后用逗号结尾。

结构体引入了一个类型。例如，我们可将 `S` 作为类型使用。`SomeOtherStruct` 估计是另一种结构体（上例中作为类型使用），而且（和 C++ 类似）作为值包含于 `S` 中，而非指向内存中另一个结构体对象的指针（译者注：与 Java 的内存结构做区分。）。

结构体中的字段使用 `.` 运算符和自身名称访问。使用结构体的例子：

```rs
fn foo(s1: S, s2: &S) {
    let f = s1.field1;
    if f == f2.field1 {
        println!("field1 matches!");
    }
}
```

此处的 `s1` 是按值传入的结构体对象，`s2` 是按引用传入的结构体对象。就方法调用而言，访问二者的字段均使用 `.`，用不着 `->`。

结构体使用结构体字面量初始化。结构体字面量是结构体名称和每个字段的值的结合。例如：

```rs
fn foo(sos: SomeOtherStruct) {
    let x = S { field 1: 45, field2: sos }; // 使用结构体字面量初始化 x
    println!("x.field1 = {}", x.field1);
}
```

结构体不能递归，无法在定义和字段类型中引入结构体名称的循环。这是由于结构体的值语义（value semantics）。例如，`struct R { r: Option<R> }` 是非法的，会导致编译错误（`Option` 内容见下文）。如果需要这样的结构，则应使用指针类型；将指针引入循环是合法行为：

```rs
struct R {
    r: Option<Box<R>>
}
```

如果上面的结构体中没有 `Option`，就无法实例化（instantiate）结构体，Rust 会报错。

没有字段的结构体的定义和字面量都不带花括号。不过定义需要以分号结尾，大概是为了方便解析代码：

```rs
struct Empty;

fn foo() {
    let e = Empty;
}
```

（译者注：注意，与 C/C++ 不同的是，有字段的结构体不用分号结尾。）

## 元组

元组（tuple）是数据的匿名异构序列。作为类型，元组声明为括号中的值序列。由于没有名字，因此通过结构辨认。例如，类型 `(i32, i32)` 是一对整数，`(i32, f32, S)` 是三个数据。元组的值和元组声明的方式相同，但相应组件中填值，不填类型，例如 `(4, 5)`。下例：

```rs
// foo 接受一个结构体，返回一个元组
fn foo(x: SomeOtherStruct) -> (i32, f32, S) {
    (32, 45.82, S { field1: 54, field2: x })
}
```

元组可通过用 `let` 表达式展开的方式使用（译者注：类似 C++17 中的结构化绑定）。例如：

```rs
fn bar(x: (i32, i32)) {
    let (a, b) = x;
    println!("x was ({}, {})", a, b);
}
```

下次多讲讲元组的展开操作。

## 元组结构体

元组结构体是具名元组，或者说是带有无名字段的结构体。元组结构体使用 `struct` 关键字，圆括号包围的类型列表，以及一个分号声明。这种声明将其名称作为类型引入。字段必须通过展开（类似元组）的方式访问，无法通过名称访问。元组结构体没那么常见。

```rs
struct IntPoint (i32, i32);

fn foo(x: IntPoint) {
    let IntPoint(a, b) = x; // 注意，我们需要元组结构体的名称以进行展开操作
    println!("x was ({}, {})", a, b);
}
```

## 枚举

枚举是和 C++ 的枚举或联合体类似的类型，因为都是可以带有多个值的类型。最简单的枚举和 C++ 的枚举相似：

```rs
enum E1 {
    Var1,
    Var2,
    Var3
}

fn foo() {
    let x: E1 = Var2;
    match x {
        Var2 => println!("var2"),
        _ => {}
    }
}
```

然而 Rust 的枚举比 C++ 的强大很多。每个变体（variant）都能包含数据。和元组类似，枚举由一系列类型定义。这使它比起 C++ 的枚举，更类似 C++ 的联合体。Rust 枚举是带标签的联合体，不是 C++ 中不带标签的联合体。这意味着无法在运行时将一个变体当作另一个使用<sup>[1](#1)</sup>。例如：

```rs
enum Expr {
    Add(i32, i32),
    Or(bool, bool),
    Lit(i32)
}

fn foo() {
    let x = Or(true, false); // x 的类型为 Expr
}
```

面向对象多态的简单用例在 Rust 中使用枚举处理，效果更佳。

要使用枚举，通常利用匹配表达式。记得这种表达式和 C++ 的 `switch` 语句类似。下次会更深入介绍这一点，以及其他展开数据的方式。例如：

```rs
fn bar(e: Expr) {
    match e {
        Add(x, y) => println!("An `Add` variant: {} + {}", x, y),
        Or(..) => println!("An `Or` variant"),
        _ => println!("Something else (in this case, a `Lit`)"),
    }
}
```

匹配表达式的每个分支都匹配 `Expr` 的一个变体。每种变体都必须涵盖。最后一种情况（`_`）涵盖了所有其他的变体类型，不过上例只包含了 `Lit`。变体中任何数据都可绑定至变量。`Add` 分支中，我们将 `Add` 中的两个 `i32` 分别绑定至 `x` 和 `y`。如果不关心数据，可以使用 `..` 匹配任意数据，见上例的 `Or`。

## 可选值类型

Rust 中的一个常见的枚举便是 `Option`。它包含两个变体，一种是 `Some`，另一种是 `None`。`None` 不含数据，`Some` 含一个 `T` 类型字段（`Option` 是个泛型枚举，后面详细说，不过 C++ 用户应该都清楚了）。`Option` 用作表示可有可无的值。任何在 C++ 中用空指针表示未定义、未初始化的值或者假值的场合<sup>[2](#2)</sup>，在 Rust 中多半都该用 `Option`。用 `Option` 更安全，因为用前必须检查，无法进行类似解引用空指针的行为。`Option` 更加通用，可以用于值和指针。例如：

```rs
use std::rc::Rc;

struct Node {
    parent: Option<Rc<Node>>,
    value: i32
}

fn is_root(node: Node) -> bool {
    match node.parent {
        Some(_) => false,
        None => true
    }
}
```

此处的 `parent` 字段可以是 `None` 或包含 `Rc<Node>` 的 `Some`。上例中我们用不着 `Some` 包含的对象，但实际使用时通常会用到。

`Option` 中也有便利方法，可以将 `is_root` 的函数体写成 `node.parent.is_none()` 或 `!node.parent.is_some()`。

## 继承可变性，`Cell` 和 `RefCell`

Rust 中的局部变量默认不可变，可使用 `mut` 标记为可变。我们不用在结构体和枚举中将字段标注为可变，因为字段的可变性通过继承得来。这意味着，结构体对象的字段是否可变取决于对象本身是否可变。例如：

```rs
struct S1 {
    field1: i32,
    field2: S2
}
struct S2 {
    field: i32
}

fn main() {
    let s = S1 { field1: 45, field2: S2 { field: 23 } };
    // s 完全不可变，不得进行以下操作
    // s.field1 = 46;
    // s.field2.field = 24;

    let mut s = S1 { field1: 45, field2: S2 { field: 23 } };
    // s 可变，以下操作正确
    s.field1 = 46;
    s.field2.field = 24;
}
```

Rust 继承可变性对引用不生效。引用和 C++ 类似，可以通过不可变指针修改可变对象。要使引用字段可变，必须在字段类型中使用 `&mut`：

```rs
struct S1 {
    f: i32
}
struct S2<'a> {
    f: &'a mut S1 // 可变引用字段
}
struct S3<'a> {
    f: &'a S1 // 不可变引用字段
}

fn main() {
    let mut s1 = S1{f:56};
    let s2 = S2 { f: &mut s1};
    s2.f.f = 45; // 尽管 s2 本身不可变，此操作仍合法
    // s2.f = &mut s1; // 非法：s2 不可变
    let s1 = S1{f:56};
    let mut s3 = S3 { f: &s1};
    s3.f = &s1; // 合法：s3 可变
    s3.f.f = 45; // 非法：s3.f 不可变
}
```

（`S2` 和 `S3` 的形参 `'a` 是生存期形参，很快就讲。）

有时，某个对象在逻辑上不可变，但没有有一部分需要可变。想象下多种缓存和引用计数（引用计数在逻辑上不能是不可变的，因为修改引用计数的影响可以通过析构函数观测）。C++ 中可使用 `mutable` 关键字允许对象本身为 `const` 时被修改。Rust 中则有 `Cell` 和 `RefCell` 结构体。这两个设施允许不可变对象的一部分可变。尽管有用，但这也意味着你需要注意，在 Rust 中看到的不可变对象，其中一部分可能还是可变的。

`RefCell` 和 `Cell` 允许绕过 Rust 对于修改和别名的严格规定。它们仍是安全设施，因为会动态确保 Rust 的不变因素成立，即使编译器无法确保不变因素能否静态成立。`Cell` 和 `RefCell` 均为单线程对象。

对带有复制语义的类型（基本只有原始类型）使用 `Cell`。`Cell` 内含 `get` 和 `set` 方法，用于修改存储的值，以及用值初始化 `Cell` 的 `new` 方法。`Cell` 是个很简单的对象，无需进行高阶操作，因为 Rust 中带有复制语义的对象无法在别处持有引用，无法咋线程间共享，因此多数内容不会出问题。

对带有移动语义（move semantics）的类型使用 `RefCell`，Rust 中几乎所有类型都带有移动语义，常见的类型包括结构体类型等。`RefCell` 同样使用 `new` 创建，内置 `set` 方法。要获取 `RefCell` 中的值，必须使用借用方法（`borrow`，`borrow_mut`，`try_borrow`，`try_borrow_mut`）将其借来。借用方法会返回 `RefCell` 用对象的借用引用。这些方法同样遵循静态借用的规定：可变借用引用只能有一个，可变和不可变借用引用不能同时出现。然而不同在于，此处不会出现编译错误（compile error），而是运行时故障（runtime failure，翻译待办：补充 error 和 failure 的区别）。`try_` 变体方法返回 `Option` 对象，如果能借出值则返回 `Some(val)`，否则返回 `None`。如果值被借出，调用 `set` 也会出运行错误。

下例使用了指向 `RefCell` 的引用计数指针（常见用例）：

```rs
use std::rc::Rc;
use std::cell::RefCell;

struct S {
    field: i32
}

fn foo(x: Rc<RefCell<S>>) {
    {
        let s = x.borrow();
        println!("the field, twice {} {}", s.field, x.borrow().field); // 译者注：不可变借走两次
        // let s = x.borrow_mut(); // 错误：已经借出了 x 的内容
    }
    
    let mut s = x.borrow_mut(); // 正确：之前的借用引用离开了生存期
    s.field = 45;
    // println!("The field {}", x.borrow().field); // 错误：（已被可变借走，因此）无法进行可变和不可变借用
    println!("The field {}", s.field);
}

fn main() {
    let s = S{field:12};
    let x: Rc<RefCell<S>> = Rc::new(RefCell::new(s));
    foo(x.clone());

    println!("The field {}", x.borrow().field);
}
```

若使用 `Cell` 或 `RefCell`，应力求将其用于最小对象。换言之，将其优先用于结构体的少数字段，而非整个结构体。把这两个东西想象成单线程锁，更精细的锁更好，因为更容易避免撞锁（collide on a lock）。

##### 1

C++17 带有 `std::variant<T>` 类型，相较联合体而言，它和 Rust 的枚举更相似。

##### 2

C++17 起，`std::optional<T>` 最适合替代 Rust 中的 `Option`。