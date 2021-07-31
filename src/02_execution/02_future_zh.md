# `Future` 特征

`Future` 特征是 Rust 异步编程的核心要义。
`Future` 是一种可以产生返回值的异步计算（尽管值可能是空，如`()`）。
`Future` 特征的简化版本可以是这个样子：

```rust
{{#include ../../examples/02_02_future_trait/src/lib.rs:simple_future}}
```

通过调用 `poll` 函数可以推进 Futures，这将驱使 Future 尽快的完成。
当 future 成为完成时，将返回 `Poll::Ready(result)`。如果 future 尚未完成，
它将返回 `Poll::Pending`，并安排在 future 在取得更多的进展时调用 `wake()` 函数。
当调用 `wake()` 时，执行器会驱使 `Future` 再次调用 `Poll`，
以便 `Future` 取得更多的进展。

如果没有 `wake()`，执行器将无法得知特定的 future 什么时候可以取得进展，
将不得不去轮询每个 future，有了 `wake()`，执行器就能准确知道哪个 future
准备好被 `poll` 了。

例如，想像一下我们需要从一个套接字中读取数据，但它里面可能有数据，也可能为空。
如果有数据，我们可以读取并返回 `Poll::Ready(data)`，但如果是空，
我们的 future 将阻塞住。所以我们必须指定一个 `wake`
以便套接字中存在数据时进行调用，它将通知执行器，
读取套接字数据这个 future 已就绪。
一个简单的 `SocketRead` future 如下：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:socket_read}}
```

这种 `Future` 模型允许将多个异步操作组合起来而无需中间分配。
一次运行多个 futures 或将其链接在一起，可通过无分配状态机实现，如下：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:join}}
```

这展示了如何在不进行单独分配的情况下同时运行多个 futures，
从而实现更高效的异步程序。同样，多个连续的 futures 也可以顺序地运行，如下：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:and_then}}
```

这些例子展示了如何使用 `trait` 特征，在无需多个分配的对象和深度嵌套的回调情况下，
来表示异步控制流程。通过基本的控制流程，让我们来谈谈真正的 `Future`
特征以及它们有什么不同。

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:real_future}}
```

首先你会看到的是，`self` 类型已不再是 `&mut Self` 而是 `Pin<&mut Self>`。
我们将在[后面的章节][pinning]中详细讨论 pinning，
但现在你只需知道它允许我们创建不可移动的 futures 即可。
不可移动的对象可以在它们的字段之间存储指针，例如
`struct MyFut { a: i32, ptr_to_a: *const i32 }`。
`Pinning` 是启用 `async/awiat` 所必需的功能。

其次，`wake: fn()` 变成了 `&mut Context<'_>`。在 `SimpleFuture` 中，
我们通过调用函数指针（`fn()`）来通知 future 执行器来对当前 future 进行 `Poll`
操作。然而，因为 `fn()` 只是一个函数指针而不包含任何数据，所以你无法得知是哪个
`Future` 在调用 `wake`。

在实际场景中，像 Web 服务器这样的复杂程序可能有成千上万个不同的连接，
而它们的唤醒工作应该分开来进行管理。`Context` 类型通过提供获取 `Waker` 
的类型值的方法解决了这个问题，该值可用于唤醒特定的任务。

[pinning]: ../04_pinning/01_chapter_zh.md
