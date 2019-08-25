## 为什么使用异步？
我们都喜欢Rust让我们能够编写快速且安全的软件的方式，但为什么写异步代码呢？

异步代码允许我们在单个OS线程中并发执行多个任务。在使用典型线程化（threaded）应用时，
如果你想同时下载两个不同的网络报， 那么你会将任务分给两个线程，像这样：

```rust,no_run
{{#include ../../examples_zh/01_02_why_async/src/lib.rs:get_two_sites}}
```
大多数应用都能很好地运行这份代码——毕竟，线程就是设计成这样用的：一次性运行多个不同任务。
然而，它们也有限制。线程切换过程和线程共享数据时会产生大量开销，甚至空跑线程也会占用珍贵
系统资源。这其中可以通过异步代码设计来减少多余的开销"我们可以用Rust的`async`/`.await`语法
重写上面的代码，这样我们就可以一次性运行多任务而不需创建多个线程：

```rust,no_run
{{#include ../../examples_zh/01_02_why_async/src/lib.rs:get_two_sites_async}}
```

最后，异步应用和对应线程化实现相比，有潜力快得多并占用更少资源，但这是有代价的。OS天然
支持线程，使用它们不需要特定编程模型——任意函数都能创建线程，并且调用那些使用了线程的函数
通常和调用普通函数一样容易。然而，异步函数需要语言或库的特别支持。在Rust中，`async fn`
创建一个返回`Future`类型的函数。为了执行函数体，返回`Future`实例必须运行至完成状态。

重要是记住：传统线程化应用也能很高效，而且Rust的精细内存足迹和可预测性意味着你不需要用
`async`，你也可以走很远。异步编程模型带来的好处并不总是能够超过带来的复杂度增加，所以
考虑你的应用是否能够用单线程模型来获得更好表现也是很重要的。