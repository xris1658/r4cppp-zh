# 闭包和头等函数

闭包、头等函数和高阶函数是 Rust 中的一个核心组件。C 和 C++ 带有函数指针（特别是 C++ 中的成员函数类型，我一直没理解）。然而函数指针的使用相对少见一些，而且用起来不太方便。C++11 引入了 Lambda 表达式，和 Rust 的闭包相近，特别是二者的实现方式非常类似。

进入本文内容前，我想建立一下这些东西的直觉，然后深入细节。

假设有函数 `foo`：`pub fn foo() -> u32 {42}`。假设有另一个函数 `bar` 有个函数类型的形参（函数签名之后再写）：`fn bar(f: ...) { ... }`。可以将 `foo` 传递给 `bar`，这和 C 中传递函数指针类似：`bar(foo)`。`bar` 的函数体中，可以将 `f` 视作函数并调用：`let x = f()`。

我们说 Rust 的函数是头等公民，因为和其他值一样，我们可以传递并使用函数。我们将 `bar` 称作高阶函数，因为它会将函数作为形参接收，也就是说，它是个操纵函数的函数。

Rust 中的闭包是匿名函数，语法漂亮。闭包 `|x| x + 2` 接收实参，将其加 `2` 并返回。注意，我们无需指定闭包形参的类型（因为通常可以推导出来）。我们也用不着指定返回类型。如果想让闭包函数体不只写一个表达式，可以使用花括号：`|x: i32| {let y = x + 2; y }`（译者注：类似现代 C++ 的 `[](std::int32_t x) { auto y = x + 2; return y; }`）。可以将闭包像函数一样传递：`bar(|| 42)`（译者注：类似现代 C++ 中的 `[]() { return 42; }`）。

闭包和其他函数的显著区别在于，闭包会捕获环境。这意味着我们可以在闭包内指涉闭包外的变量。例如：

```rs
let x = 42;
bar(|| x);
```

注意看 `x` 是如何进入闭包的作用域的。（译者注：现代 C++ 需要显式捕获或使用默认捕获，但 Rust 通过闭包函数体自动确定要捕获的内容。）

我们已经见过了闭包，与迭代器共用，而这正是闭包的常见用法。例如，让可扩张数组的每个元素与一个值相加：

```rs
fn baz(v: Vec<i32>) -> Vec<i32> {
    let z = 3;
    v.iter().map(|x| x + z).collect()
}
```

（译者注：上例类似 C++ 标准库的 `std::for_each`，准确地说，更像 C++20 的 `std::for_each`。）

此处的 `x` 是闭包的参数（argument，译者注：对应 C++ 中的“实参”一词，但此处似乎应使用“形参”一词才说得通），`v` 中的每个成员均作为 `x` 传入。`z` 在闭包外声明，但闭包内是可以指涉 `z` 的。也可以将函数传入 `map` 中：

```rs
fn add_two(x: i32) -> i32 {
    x + 2
}

fn baz(v: Vec<i32>) -> Vec<i32> {
    v.iter().map(add_two).collect()
}
```

注意，Rust 也允许在函数内声明函数。这种函数*不是*闭包，无法访问环境。这种函数只是方便管理作用域：

```rs
fn qux(x: i32) {
    fn quxx() -> i32 {
        x // 错误：x 不在作用域内
    }

    let a = quxx();
}
```

## 函数类型

我们来引入一个新的函数示例：

```rs
fn add_42(x: i32) -> i64 {
    x as i64 + 42
}
```

和之前一样，可以将函数存入变量：`let a = add_42`。`a` 的具体类型无法在 Rust 代码中写出（译者注：类似 C++ 中的 Lambda 表达式）。编译器会在错误信息中将其指代为 `fn(i32) -> i64 {add_42}`，各位有时会注意到。每个函数均拥有独特的匿名类型。`fn add_41(x: i32) -> i64` 拥有不同类型，尽管函数签名一致。

可以写出不那么准确的类型，例如 `let a: fn(i32) -> i64 = add_42`。所有签名相同的函数类型都可以转换为程序员可以写出的 `fn` 类型。

`a` 也被编译器表示为函数指针，然而，如果编译器知道精确类型，实际上不会用那个函数指针。形如 `a()` 的调用会根据 `a` 的类型静态派发。如果函数不知道精确类型（例如，只知道 `fn` 类型），则调用会使用值中的函数指针派发。

Rust 中还有个 `Fn` 类型（注意 F 是大写的）。`Fn` 类型和特征一样是绑定项（其实 `Fn` 本身*是*特征，后面会看到）。`Fn(i32) -> i64` 是绑定到所有与其签名一致的函数型对象类型的绑定项。解引用函数指针时，我们实际上是创建了一个由胖指针表示的特征对象（参见动态大小类型）。

要将函数传到另一个函数，或者将函数存入字段，必须写清类型。我们有几种选择，要么用 `fn` 类型，要么用 `Fn` 类型。后者更好，因为包含了闭包类型（以及其他的形如函数的对象类型），而 `fn` 不包含。`Fn` 类型动态确定大小，因此无法将其用于值类型。我们要么得传递函数对象，要么得使用泛型。先来看泛型的方式。例如：

```rs
fn bar<F>(f: F) -> i64
    where F: Fn(i32) -> i64
{
    f(0)
}
```

`bar` 接收任意签名为 `Fn(i32) -> i64` 的函数，即能将类型形参 `F` 实例化为任意形如函数的类型。可以调用 `bar(add_42)` 从而将 `add_42` 传给 `bar`，而 `bar` 会将 `F` 实例化为 `add_42` 的匿名类型。也可以调用 `bar(add_41)`，同样可以正常工作。

可以将闭包传递给 `bar`，例如 `bar(|x| x as i64)`。这段代码能用，因为闭包类型也可以绑定到匹配签名的 `Fn` 绑定项（和函数一样，每个闭包都拥有自己的匿名类型）。

最后，还可以传递函数或闭包的引用：`bar(&add_42)` 或者 `bar(&|x| s as i64)`。

也可以将 `bar` 写成 `fn bar(f: &Fn(i32) -> i64) ...`。两种方式（泛型和函数 / 特征对象）的语义相当不同。使用泛型时，`bar` 会被单态处理（monomorphise），因此生成代码时，编译器知道 `f` 的确切类型，意味着可以静态派发。若使用函数对象，函数不会被单态处理。不知道 `f` 的确切类型，因此编译器必须生成虚派发。后者更慢，但前者会产生更多代码（每种类型实参生成一个单态函数）。

函数特征不只 `Fn` 一种，还有 `FnMut` 和 `FnOnce`。二者使用方式和 `Fn` 相同，例如 `FnOnce(i32) -> i64`。`FnMut` 值得是可以被调用，还可以在调用时被修改的对象。这一性质不适用于一般的函数，但对于闭包而言，这一点意味着闭包可以修改自身的环境。`FnOnce` 则是（至多）只能调用一次的函数。此性质同样只适用于闭包。

`Fn`、`FnMut` 和 `FnOnce` 构成了子特征的层次结构。`Fn` 是 `FnMut`（因为可以在调用 `Fn` 时进行修改而不造成负面影响，但反过来不行）。`Fn` 和 `FnMut` 都是 `FnOnce`（因为一般的函数只调用一次没有副作用，但反过来不行）。

因此，要确保高阶函数尽可能灵活，应使用 `FnOnce` 绑定，而非使用 `Fn` 绑定（如果必须多次调用函数的话，使用 `FnMut` 绑定）。

### 方法

方法的使用方式和函数一样：取指针，存入变量等。不能使用 `.` 语法，必须使用完全明确的命名形式（有时称作通用函数调用语法，universal function call syntax，简称为 UFCS）给方法明确命名。形参 `self` 是方法的第一个参数。例如：

```rs
struct Foo;

impl Foo {
    fn bar(&self) {}
}

trait T {
    fn baz(&self);
}

impl T for Foo {
    fn baz(&self) {}
}

fn main() {
    // 内在方法
    let x = Foo::bar;
    x(&Foo);

    // 特征方法，注意完全明确的命名形式
    let y = <Foo as T>::baz;
    y(&Foo);
}
```

### 泛型函数

不能取泛型函数的指针，且无法表示泛型函数类型。然而，可以引用所有类型形参均被实例化的函数类型。例如：

```rs
fn foo<T>(x: &T) {}

fn main() {
    let x=  &foo::<i32>;
    x(&42);
}
```

无法定义泛型闭包。如果需要创建适用于多种类型的闭包，可以使用特征对象、宏（用于生成闭包）、或传递一个返回闭包的闭包（每种返回的闭包适用于不同的类型）。