# `async`/`.await`

在[第一章]中，我们对 `async`/`.await` 已有了一个简单的了解。
本章将更详尽的介绍 `async`/`.await`，解读它是如何工作的，
以及 `async` 代码与传统的 Rust 同步程序有合不同。

`async`/`.await` 是特殊的 Rust 语法，通过它可以在本身产生阻塞时，
让出当前线程的控制权，即在等待自身完成时，亦可允许其它代码运行。

有两种方法来使用 `async`：`async fn` 函数和 `async` 代码块。
它们都会返回一个实现了 `Future` 特征的值。

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:async_fn_and_block_examples}}
```

正如我们在第一章中所见，`async` 的代码和其它 futures 是惰性的：
除非去调用它们，否则它们不会做任何事。而最常用的运行 `Future` 的方法就是使用
`.await`。当 `Future` 调用 `.await` 时，这将尝试去运行 `Future` 直至完成它。
当 `Future` 阻塞时，它将让出线程的控制权。而当 `Future` 再次就绪时，
执行器会恢复其运行权限，使 `.await` 推动它完成。

## `async` 的生命周期

不同于传统函数，`async fn` 接收引用或其它非静态参数，
并返回一个受其参数的生命周期限制的 `Future`。

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:lifetimes_expanded}}
```

这意味着，`async fn` 返回的 future，必须在其非静态参数的生命周期内调用 `.await`！
通常在调用函数后立即对 future 执行 `.await` 时不会出现问题（比如
`foo(&x).await`）。然而，当这个 future 被存储起来或发送到其它任务或线程上时，
这可能会成为一个问题。

一种常见的解决办法是，将参数和 `async fn` 调用一并放置在一个 `async`
代码块中，这将 `async fn` 和引参转化成了一个 `'static` future。

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:static_future_with_borrow}}
```

通过将参数移动到 `async` 代码块中，我们将它的生命周期延长到同返回的 `Future`
一样久。

## `async move`

同普通的闭包一样，`async` 代码块和闭包中可使用 `move` 关键字。
`async move` 代码块将获取其引用变量的所有权，使它得到更长的生命周期，
但这样做就不能再与其它代码共享这些变量了：

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:async_move_examples}}
```

## `.await`ing on a Multithreaded Executor
## 在多线程执行器上的 `.await`

注意，当使用多线程 `Future` 执行器时，`Future` 可能会在线程间移动，
所以在 `async` 里使用的任何变量都必须能在线程之间传输，
因为任何 `.await` 都可能导致任务切换到一个新线程上。

这意味着使用 `Rc`, `&RefCell` 或其它任何未实现 `Send` 特征的类型及未实现
`Sync` 特征的类型的引用都是不安全的。

（警告：只要在调用 `.await` 期间内不对它们进行操作就可使用这些类型。）

同样，在 `.await` 中使用传统的“非 future 感知”锁也并不是一个好主意，
它可能导致线程池死锁：一个任务取得了锁，然后 `.await`，
而执行器调度另一个任务同样想获取这个锁，这就导致了死锁。在 `futures::lock` 中
使用 `Mutex` 而不是 `std::sync` 可以避免这种情况。

[第一章]: ../01_getting_started/04_async_await_primer_zh.md
