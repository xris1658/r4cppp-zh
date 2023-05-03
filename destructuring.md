[阅读英文原版](https://github.com/nrc/r4cppp/blob/master/destructuring-2.md)

# 展开操作

上篇文章中，我们关注了 Rust 的数据类型。结构中有了数据，你就会想着取出数据。对于元组、元组结构体和枚举，必须使用展开（destructure）的方式（库中有一些便利函数，但函数内部用的还是展开）。在 C++ 中，展开数据结构的行为从 C++17 开始才出现，因此 （Rust 的展开操作）和 Python 以及很多函数式编程语言类似。思路在于，与通过使用若干个局部变量填入结构体字段的方式来初始化结构体一样，可以将数据结构中的数据填入局部变量。从此，展开操作成为了 Rust 中最强大的特性之一。换句话说，展开操作结合了模式匹配和局部变量的赋值操作。

展开操作主要通过 `let` 和 `match` 语句进行。`match` 语句用于被展开的对象可能有不同变体的情况（如枚举）。`let` 表达式则将变量从对象拉到当前的作用域中，而匹配引入新的作用域。比较二者：

```rs
fn foo(pair: (int, int)) {
    let (x, y) = pair;
    // 至此，foo 中任何地方都能使用 x 和 y

    match pair {
        (x, y) => {
            // x 和 y 只能在这一作用域使用
        }
    }
}
```

两种情况下，模式（上例中 `let` 后 `=>` 前使用）的语法是（大体）相同的。也可以在函数声明中将模式用于参数处：

```rs
fn foo((x, y): (int, int)) {
}
```

（相较元组而言，这种方式对于结构体和元组结构体更有用。）

多数初始化表达式可以写作展开模式，可以相当复杂。展开模式可以包含引用和原始字面量，以及数据结构。例如：

```rs
struct St {
    f1: int,
    f2: f32
}

enum En {
    Var1,
    Var2,
    Var3(int),
    Var4(int, St, int)
}

fn foo(x: &En) {
    match x {
        &Var1 => println!("first variant"),
        &Var3(5) => println!("third variant with number 5"),
        &Var3(x) => println!("third variant with number {} (not 5)", x),
        &Var4(3, St { f1: 3, f2: x }, 45) => {
            println!("destructuring an embedded struct, found {} in f2", x)
        }
        &Var4(_, ref x, _) => {
            println!("Some other Var4 with {} in f1 and {} in f2", x.f1, x.f2)
        }
        _ => println!("other (Var2)")
    }
}
```

注意我们在模式中使用 `&` 展开引用的方式，以及混用字面量（`5`、`3`、`St { ... }`），通配符（`_`）和变量（`x`）的方式。

在模式中，要忽略某一项目，可在项目对应出现处使用 `_`，因此，我们如果不关心 `Var3` 中的整型字段，便可以写 `&Var3(_)`。第一个 `Var4` 分支中，我们展开了内嵌的结构体（嵌套的模式），第二个 `Var4` 分支中，我们将整个结构体绑定到了一个变量。也可以使用 `..` 匹配元组和结构体中的全部字段。如果想对枚举的变体进行操作，但不关心变体内容，则可写成：

```rs
fn foo(x: En) {
    match x {
        Var1 => println!("first variant"),
        Var2 => println!("second variant"),
        Var3(..) => println!("third variant"),
        Var4(..) => println!("fourth variant")
    }
}
```

展开结构体时，字段无需按顺序出现，且可以使用 `..` 省去剩余字段。例如：

```rs
struct Big {
    field1: int,
    field2: int,
    field3: int,
    field4: int,
    field5: int,
    field6: int,
    field7: int,
    field8: int,
    field9: int,
}

fn foo(b: Big) {
    let Big { field6: x, field3: y, ..} = b;
    println!("pulled out {} and {}", x, y);
}
```

作为结构体的缩写，可以只用字段名称，以创建同名的局部变量。上例的 `let` 语句创建了两个新的局部变量 `x` 和 `y`。也可以写成

```rs
fn foo(b: Big) {
    let Big { field6, field3, .. } = b;
    println!("pulled out {} and {}", field3, field6);
}
```

这样便创建了两个与字段名相同的局部变量，即上例中的 `field3` 和 `field6`。

Rust 的展开操作存在一些技巧。比如，想在模式内创建对变量的引用。这时不能用 `&`，因为这么做是匹配引用，而非创建引用（因此存在解引用对象造成的影响）。例如：

```rs
struct Foo {
    field: &'static i32
}

fn foo(x: Foo) {
    let Foo { field: &y } = x;
}
```

此处的 `y` 类型为 `i32`，是 `x` 中字段的副本。

要在模式中创建引用，使用关键字 `ref`。例如：

```rs
fn foo(b: Big) {
    let Big {field3: ref x, ref field6, ..} = b;
    println!("pulled out {} and {}", *x, *field6);
}
```

`x` 和 `field6` 类型均为 `&int`，均为 `b` 中字段的引用。

展开操作的最后一个技巧是，要展开复杂对象，你或许会想着同时为中间变量和独立字段命名。回看之前的例子中的模式 `&Var4(3, St{ f1: 3, f2: x }, 45)`。模式中给结构体的一个字段命了名，不过你或许也想给整个结构体对象命名。虽然可以写成 `&Var4(3, s, 45)`，但你可能想访问字段，亦或是想只匹配字段中的一个值，结果得使用嵌套匹配。这就不好玩了。Rust 允许使用 `@` 语法为模式的一部分命名。例如 `&Var4(3, s @ St{ f1: 3, f2: x }, 45)` 允许同时命名字段（`f2` 命名为 `x`）以及整个结构体（`s`）。

Rust 模式匹配的内容差不多讲完了。有些特性没有讲，例如匹配动态数组，不过希望你能明白如何使用 `match` 和 `let`，且意识到可以完成的一些强大操作。下篇文章涵盖匹配和借用的交互，我学 Rust 时栽了不少跟头。