[阅读英文原版](https://github.com/nrc/r4cppp/blob/master/hello-world.md)

# 引言 - hello world!

读者如果在使用 C 或 C++，可能是因为非用不可：需要访问系统底层，或是需要榨干性能，或许二者兼具。Rust 的目标在于提供同样的内存抽象层次，同样的性能，但又更加安全，提高用户效率。

具体而言，比起 C++，读者或许想用其他语言，如 Java，Scala，Haskell，Python 等等，又因抽象层次太高（无法直接访问内存、被迫使用垃圾回收等）或是性能问题（或是无法预测性能，或是单纯不够快）而无法使用。Rust 不会强迫读者使用垃圾回收，而且和 C++ 一样可以用指向内存的原始指针。Rust 秉承了 C++“pay for what you use”的哲学。如果某个功能用不到，便不用为这一功能的存在付出性能开销。此外，Rust 中所有语言特性的开销均可预测，且通常较低。

这些约束使得 Rust 成为了（为数不多的） C++ 的替代品，同时也有自身的优势：Rust 是内存安全的——Rust 的类型系统确保读者不会犯 C++ 中常出现的内存错误，例如，访问未初始化的内存，悬垂指针等行为在 Rust 中不可能出现。不仅如此，只要允许其他限制，Rust 力求避免其他的安全问题，例如，所有数组下表均带有边界检查（当然，想避免这一开销，Rust 可以牺牲安全性，在不安全的块中做到这一点以及很多不安全的事项。关键在于，Rust 保证不安全的块中的不安全性只存在于块中，不会影响程序的其他部分）。最后，Rust 从现代编程语言中采纳了众多概念，并将其引入了系统语言空间中。但愿这一点可以让使用 Rust 编程更具生产力、更高效、更愉悦。

在本篇剩下的部分中，我们要下载并安装 Rust，创建一个精简的 Cargo 项目，并实现 Hello World。

## 获取 Rust

可以从 [http://www.rust-lang.org/install.html](http://www.rust-lang.org/install.html) 获取 Rust。此下载链接包含了 Rust 编译器、标准库、以及包管理工具兼构建工具 Cargo。

Rust 共有三个发布通道（channel）可用：稳定版、Beta 版和 Nightly 版。Rust 的开发工作基于快速的发行节奏，计划于每六周发行新版本。发行日到来时，Nightly 版成为 Beta 版，Beta 版成为稳定版。

Nightly 于每日晚上更新，适合试验最新特性并保证自己的库可用于未来版本的 Rust 的用户。

稳定版是适合绝大多数用户的选择。Rust 的稳定性保证仅对稳定版有效。

Beta 版多用于让用户的持续集成（CI）工具检查用户代码是否按预期工作。

读者或许想使用稳定版。若使用 Linux 和 macOS，最简单的获取方式是运行
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

若使用 Windows，相似的最简单方式是运行
```
choco install rust
```

要使用其他安装方式，参阅 [http://www.rust-lang.org/install.html](http://www.rust-lang.org/install.html)。

可以在 [github.com/rust-lang/rust](github.com/rust-lang/rust) 获取源代码。要构建编译器，运行 `./configure && make rustc`。若需要更详细的步骤，请参阅 [building-from-source](https://github.com/rust-lang/rust#building-from-source)。

## Hello World!
构建 Rust 程序最简单的方式是使用 Cargo。要使用 Cargo 新建项目，运行 `cargo new --bin hello`。此命令会新建一个名为 `hello` 的目录，内含文件 `Cargo.toml`，以及目录 `src`，内含一个文件 `main.rs`。

`Cargo.toml` 定义了项目的依赖项，以及其他的元数据。我们之后详细描述。

所有源码均位于 `src` 目录中。`main.rs` 中已经带有了 Hello World 程序，代码如下：
```rs
fn main() {
    println!("Hello world!");
}
```

要构建程序，运行 `cargo build`。要构建并运行程序，运行 `cargo run`。若运行了运行命令，控制台便会向你问好了。搞定！

Cargo 会创建 `target` 目录，并存放可执行程序。

想直接使用编译器，可以运行 `rustc src/main.rs`，此命令会创建一个名为 `main` 的可执行程序。要参阅众多选项，运行 `rustc --help`。

好，我们回头看下代码，发现几个有趣之处——使用 `fn` 定义函数或方法；`main()` 是程序的默认入口点（程序实参之后再谈）；不像 C++，此处没有分离的声明和头文件；`println!` 是 Rust 中的 `printf`，`!` 代表此函数是一个宏（macro）；标准库中的一个子集（称作 prelude）无需明确导入和包含操作即可使用，`println!` 宏也包含在这一子集中。

我们简单修改下代码：
```rs
fn main() {
    let world  = "world";
    println!("Hello {}!", world);
}
```
`let` 用于引入变量，名称为 `world`，是字符串（准确而言，其类型是 `&'static str`，后面详细描述）。我们不用指明类型，编译器会推导的。

在 `println` 中使用 `{}` 和在 `printf` 中使用 `%s` 一样。其实 `{}` 比 `%s` 更通用，因为 Rust 会尝试将不会字符串的变量转换为字符串[^1]</sup>（如同 C++ 中的 `operator<<()`）。读者可以用简单用一下——试试写多个字符串，或者换用数字（整型和浮点字面量都能用）。

如果想的话，可以明确标注 `world` 的类型：
```rs
let world: &'static str = "world";
```
在 C++ 中，要声明 `T` 类型变量 `x`，我们会写 `T x`。在 Rust 中，无论是 `let` 语句（statement）处还是函数签名处等等，我们都会写 `x: T`。我们一般会在 `let` 语句中省去明确标注的类型，但函数形参中需要标注类型。我们加个函数，看看是怎么用的：
```rs
fn foo(_x: &'static str) -> &'static str {
    "world"
}

fn main() {
    println!("Hello {}!", foo("bar"));
}
```
函数 `foo` 接受一个形参，即字符串字面量 `_x`，我们在 `main` 中传入了 `"bar"`。[^2]</sup>

函数的返回类型在 `->` 后指出。如果函数什么都不返回（如同 C++ 的 `void` 函数），我们用不着指出返回类型（`main` 函数便是如此）。如果读者仍想显式指出，则写 `-> ()`，其中 `()` 是 Rust 的 `void` 类型。

Rust 中不需要 `return` 关键字，只要函数体（或者其他的代码块，更多例子以后补充）的最后一个表达式（expression）不以分号结尾，返回值就是它了。因此 `foo` 会返回 `world`。`return` 关键字仍然存在，因此我们可以提前返回。可以使用 `return "world";` 替换 `"world"`，效果相同。

## 为什么？

我想重点提一下上述的部分语言特性。

局部类型推导既方便又有用，且不会损失安全性和性能（现代 C++ 同样引入了此特性）。  
Rust 中的语言实体（language item）均使用关键字（`fn`，`let` 等）标注，使得人工和程序阅读代码更简单。一般来说，Rust 的语法比 C++ 的更加简单，且更为一致。  
`println!` 宏比 `printf` 更安全，因为会检查实参数量和格式字符串的占位符数量是否匹配，此外还检查实参类型。这意味着读者不会出现按照不同类型打印内存的错误，以及遍历实参堆栈不当的错误。（译者注：“按照不同类型打印内存”指格式指定符与实参类型不匹配；译者不清楚“遍历实参堆栈不当”的含义~~指遍历实参时由于前一错误导致的错误弹栈及其引发的不当内存访问。详情参阅 C 语言的变长参数。~~）

这些是比较小的细节，不过我希望这些内容可以体现 Rust 设计背后的哲学。

##### 1
这是由程序员指定的转换，其使用了 `Display` 特征（trait），运作方式有点类似 Java 中的 `toString`。也可以使用 `{:?}` 打印编译器生成的信息，有时用作调试。对于 `printf`，其选项众多。

##### 2
我们在 `foo` 中没有使用实参。通常 Rust 会警告此事。我们通过在实参名前加 `_` 前缀的方式排除了这一警告。其实我们用不着为实参起名，只用 `_` 就行。