[阅读英文原版](https://github.com/nrc/r4cppp)

# 系统程序员的 Rust 教程

为有经验的 C 和 C++ 程序员编写的 Rust 教程。

转到[目录](#目录)。
转到[贡献内容](#贡献内容)。

本教程面向了解指针和引用工作方式，且习惯了系统编程中诸如整数位宽和内存管理等概念的程序员。我们主要涵盖 Rust 和 C++ 中的不同，使得读者无需阅读大量已知内容，就可以快速编写 Rust 程序。

对于 C++ 程序员来说，Rust 是一门很直观的语言。多数语法都很相似。就我的经验而言，Rust 的重要区别在于，把系统编程做好，涉及一些有时很模糊的概念，但这些概念会由编译器严格实行。这一点起初会让你很不爽，因为有些事你想做，但编译器不会让你做（至少安全代码中是这样），有时想做的事确实安全，但编译器不听你的话。然而，你很快会形成一套“能干什么”的直觉。和编译器交流你自己关于内存安全的想法，需要一些新的，有时复杂的类型标注。但如果对对象的生存期有着深刻认识，且有泛型编程经验，学起来应该不会太费劲。

本教程起初是[一系列博客文章](http://featherweightmusings.blogspot.co.nz/search/label/rust-for-c)。一部分原因是为了帮自己（@nrc）学习 Rust（要了解自己是否学会某件事，最好的方式就是试着解释给别人听），一部分原因则是发现已有的 Rust 教学资源不合我意，因为花了太长时间讲我已经了解的基础知识，而且使用了高级的直觉（intuition）去描述一些概念，而对我而言用低级直觉描述更好。从落笔到现在，Rust 文档质量好了*很多*，然而我还是觉得，Rust 的一大受众是 C++ 程序员，但是这部分人并未被重点对待。

## 目录

1. [引言 - hello world!](hello-world.md)
1. [控制流](control-flow.md)
1. [原始类型和运算符](primitives.md)
1. [独占指针](unique.md)
1. [借用指针](borrowed.md)
1. [引用计数和原始指针](rc-raw.md)
1. [数据类型](data-types.md)
1. [展开操作（一）](destructuring.md)
1. [展开操作（二）](destructuring-2.md)
1. 数组和动态数组
1. 图和 arena allocation
1. 闭包和一等函数

## 其他资源

* 《Rust 程序设计语言》（[英文原版](https://doc.rust-lang.org/book/) / [简体中文翻译](https://kaisery.github.io/trpl-zh-cn/)）：总的来说，学习的最佳去处。与此处内容冲突或此处内容缺失时，多半也是最佳选择。
* [Rust API 文档](http://doc.rust-lang.org/std/index.html)：Rust 程序库的详细文档。
* [Rust 参考手册](https://doc.rust-lang.org/reference/)：某些内容过期，不过内容完整，适合查细节。
* [论坛](http://users.rust-lang.org)：使用和学习 Rust 讨论和提问的通用论坛。
* [StackOverflow 上的 Rust 问题](https://stackoverflow.com/questions/tagged/rust)：Rust 相关的高级问题和针对很多新手的回答。阅读时需注意，Rust 近年来改动*很多*，有些回答可能早就过期了。
* Rust 消火栓（A Firehose of Rust）（[YouTube](https://www.youtube.com/watch?v=IPmRDS0OSxM) / [bilibili](https://www.bilibili.com/video/BV1h44y167H7)）：一段录课视频，向 C++ 程序员介绍了 Rust 中生存期、可变别名和移动语义的工作方式。

## 贡献内容

（译者注：此处翻译原文，并使用原文链接。）

当然可以！

如果发现了拼写或内容错误，别不好意思，交个 PR 就行！如果想做大改，或是想看什么新内容，欢迎[新建 issue](https://github.com/nrc/r4cppp/issues/new)。如果你觉得内容可以通过重新组织已有内容和添加示例的方式优化内容，我也乐意看到。

如果想加段落或章节，欢迎贡献内容！如果不知道要涵盖什么内容，翻一翻 [issue 列表](https://github.com/nrc/r4cppp/issues)，尤其是标记为 [new material](https://github.com/nrc/r4cppp/labels/new%20material) 的内容。有什么不确定的，请在这里（@nrc）或者 irc（#rust 或 #rust-internals 上的 nrc）上艾特原作者。

### 行文风格

显而易见，此教程主要面向 C++ 程序员。此教程会专挑有经验的 C++ 程序员不了解的东西讲，而非面向一般读者（不过，我并不假设读者熟悉较新版本的 C++）。我会避免写太多基础内容，且一定会避免和其他资源交叉，尤其是 《Rust 程序设计语言》一书。

欢迎不常见的用例（例如在 Cargo 上使用不同的构建系统，编写语法扩展，使用不稳定 API）。

我会避免把用例写成将 C++ 代码转换为 Rust 代码的“配方”式示例，不过简短的示例没问题。

我不打算加习题以及小项目的建议，不过如果你感兴趣，请告诉我。

我会使行文风格偏学术腔，但不会写得特别精 GitHub Markdown 格式。要了解写作风格、语法和标点符号等内容的建议，参见牛津风格指南或[《经济学人》风格指南](http://www.economist.com/styleguide/introduction)。请将行宽限制为 80 列。我喜欢用牛津逗号（译者注：在并列句最后的并列词前加逗号）。

不要觉得提交内容必须完美无暇，我乐意修改内容，并且确信别人以后也乐意如此。