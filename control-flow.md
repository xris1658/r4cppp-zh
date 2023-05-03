[阅读英文原版](https://github.com/nrc/r4cppp/blob/master/control-flow.md)

# 控制流

## 条件分支

Rust 中的 `if` 语句和 C++ 中的大致相似。不同之一在于，花括号必须写，但被测表达式的圆括号可以不写。另一点在于，`if` 是表达式，因此可以像 C++ 中的 `?:` 三元运算符一样使用（回顾一下上节内容：如果块中最后一个表达式没有以分号结尾，则表达式成为块的返回值）。Rust 中没有三元运算符 `?:`。以下两个函数的作用相同：

```rs
fn foo(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}
```

```rs
fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}
```

（为何不用 `mut result`？`foo` 中的代码使得 `result` 不可更改，只是可能在两处初始化。Rust 可以发现，`return result` 时结果保证被初始化。）

第一个函数和 C++ 中的写法高度相似。第二个则是更好的 Rust 编码风格。

也可以写出 `let result = if x < 10 ...` 这样的代码。

## 循环

Rust 带有 while 循环，和 C++ 一致：

```rs
fn main() {
    let mut x = 10;
    while x > 0 {
        println!("Current value: {}", x);
        x = -1;
    }
}
```

Rust 中没有 `do...while` 循环，不过有无尽循环的 `loop` 语句：

```rs
fn main() {
    loop {
        println!("Just Looping");
    }
}
```

Rust 带有 `break` 和 `continue`，和 C++ 一致。

## for 循环

Rust 同样带有 `for` 循环，但有些微不同。假设有一个整数动态数组（vector），想将其全部打印出来（我们之后会涵盖动态和静态数组，迭代器和泛型的细节。就本文而言，我们只需了解 `Vec<T>` 是元素类型为 `T` 的序列，`iter()` 返回容器的迭代器）。简单的 `for` 循环代码如下：

```rs
fn print_all(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }
}
```

原文待补充：编写使用 `&all` 和 `all` 的代码

要使用索引值遍历 `all`（类似 C++ 中为数组编写的 for 循环），代码为

```rs
fn print_all(all: Vec<i32>) {
    for i in 0..all.len() {
        println!("{}: {}", i, all[i]);
    }
}
```

`len` 函数的功能但愿是不言而喻了。原文待补充：边界记号（notation）

使用枚举迭代器可以写出更偏向 Rust 风格的版本：

```rs
fn print_all(all: Vec<i32>) {
    for (i, a) in all.iter().enumerate() {
        println!("{}: {}", i, a);
    }
}
```

其中，`enumerate()`  链接自迭代器 `iter()`，在迭代时提供当前下标和元素。

*下例结合了 [借用指针](borrowed.md) 中涵盖的更高级的主题。*假设有一个整数的动态数组，想要调用函数，传容器的引用，并就地修改容器。此处 `for` 循环会用到可修改迭代器，这种迭代器可提供可修改的引用。`*` 解引用，C++ 程序员应该很熟悉了：

```rs
fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}
```

## Switch 和匹配表达式

Rust 带有匹配表达式，和 C++ 的 `switch` 语句类似，不过强大很多。下面的简单例子应该挺好懂的：
```rs
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        y => println!("x is something else {}", y),
    }
}
```

上例中有一些语法差异：使用 `=>` 从匹配的值指向要执行的表达式，不同匹配分支之间使用 `,` 分隔（最后一个 `,` 可以不写）。还有一些不太明显的语义差异：匹配必须写全，换言之必须覆盖匹配表达式（上例的 `x`）所有可能的值。尝试把 `y => ...` 一行删除，看看会发生什么（译者注：编译错误？）；因为我们只匹配了 0、1 和 10，明显还有很多整数没有匹配。最后一个匹配分支中，`y` 绑定为要匹配的值（上例中的 `x`）。也可以写成：

```rs
fn print_some(x: i32) {
    match x {
        x => println!("x is something else {}", x)
    }
}
```

匹配分支中的 `x` 引入了新变量，隐藏了实参 `x`，和在内层作用域中声明（同名）变量类似。

如果不想给变量起名，可以用 `_` 表示无名变量，这与使用通配符匹配类似。如果什么都不想做，可以加一个空分支：

```rs
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        _ => {}
    }
}
```

另一个语义差异在于，没有从一个分支跳到另一个分支的机制（译者注：指 C++ 中在 `case` 中使用的 `break`、`continue`和 `goto` 等）。换言之，此处的 `switch` 的运行类似 `if...else if...else`。

我们在之后的文章中会发现，匹配表达式相当强大。本文中，我再介绍两个机制：分支中为值使用的 or 运算符，以及分支中的 `if`。希望下例可以解释一切：

```rs
fn print_some_more(x: i32) {
    match x {
        0 | 1 | 10 => println!("x is one of zero, one, or ten"),
        y if y < 20 => println!("x is less than 20, but not zero, one, or ten"),
        y if y == 200 => println!("x is 200 (but this is not very stylish)"),
        _ => {}
    }
}
```

（译者注：上例类似数学中分段函数的写法，比起 C++ 中的 `switch`，这种方式胜在写法简洁且表意明确。）

和 `if` 表达式一样，`match` 语句其实也是表达式。据此我们可以将上例改为：

```rs
fn print_some_more(x: i32) {
    let msg = match x {
        0 | 1 | 10 => "one of zero, one, or ten",
        y if y < 20 => "less than 20, but not zero, one, or ten",
        y if y == 200 => "200 (but this is not very stylish)",
        _ => "something else"
    };

    println!("x is {}", msg);
}
```

注意右花括号后有个分号，这是因为，`let` 语句是个语句，书写形式必须形如 `let msg = ...;`。我们在省略号处填入匹配表达式（表达式本身无需加分号），但是 `let` 语句需要分号。我每次都在这儿栽跟头。（译者注：正如很多 C++ 初学者忘记在类定义后加分号一样，Rust 初学者可能会忘记在这种长相复杂的语句后加分号。）

动机：Rust 的匹配语句避免了 C++ 的 `switch` 语句的常见漏洞：不会出现忘记写 `break` 导致的控制流不当；如果匹配枚举项（后面介绍），编译器会确保 `match` 语句涵盖了所有枚举项。

## 方法调用

最后简单补充一下，和 C++ 一样，Rust 中带有方法（method）（译者注：“方法”与 C++ 中的“成员函数”和 Java 的“方法”是一个意思）。方法永远使用 `.` 运算符调用（不用 `->`，后面一篇文章详细说明）。我们在上文见过了一些例子（`len`，`iter`）。之后会补充定义和调用方法的内容。读者在 C++ 和 Java 中产生的多数假设或许都是正确的。