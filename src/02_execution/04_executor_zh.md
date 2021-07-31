# 应用：构建一个执行器

Rust 的 `Future` 是懒惰的：除非积极地推动它完成，不然它不会做任何事情。
一种推动 future 完成的方式是在 `async` 函数中使用 `.await`，
但这只是将问题推进了一层，还面临着：谁将运行从顶级 `async` 返回的 future？
很明显我们需要一个 `Future` 执行器。

`Future` 执行器获取一级顶级 `Future`s 并在 `Future`
取得工作进展时通过调用 `poll` 来将它们运行直至完成。
通常，执行器会调用一次 `poll` 来使 future 开始运行。
当 `Future` 通过调用 `wake()` 表示它们已就绪时，会被再次放入队列中以便 `poll`
再次调用，重复直到 `Future` 完成。

在本章中，我们将编写一个简单的，能够同时运行大量顶级 futures 的执行器。

在这个例子中，我们依赖于 `futures` 箱，它提供了 `ArcWake` 特征，
有了这个特征，我们可以很方便的构建一个 `Waker`。

```toml
[package]
name = "xyz"
version = "0.1.0"
authors = ["XYZ Author"]
edition = "2018"

[dependencies]
futures = "0.3"
```

接下来，我们需要在 `src/main.rs` 的顶部导入以下路径：

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:imports}}
```

我们将通过将任务发送到通道（channel)上，来使执行器运行它们。
执行器会从通道道中取出事件并运行它。当一个任务已就绪（awoken 状态），
它可以通过通过将自己再次放入通道以便被再次轮询到。

在这个设计中，执行器本身只需要拥有任务通道的接收端。
用户则拥有此通道的发送端，以便生成新的 futures。任务本身只是可以自我重新调度的
futures，所以我们将它和发送端绑定成一对儿，它可以此重新回到任务队列中。

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_decl}}
```

同时，让我们也给 spawner 添加一个新方法，使它可以方便地生成新的 futures。
这个方法将接收一个 future 类型，将它打包，并在其中创建一个新的 `Arc<Task>`
以便它可以添加到执行器的队列中。

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:spawn_fn}}
```

我们需要创建一个 `Waker`，来轮询 futures。之前在 [唤醒任务]
中提到过，一旦任务的 `wake` 被调用，`Waker` 就会安排再次轮询它。请记住，
`Waker` 会准确的告知执行器哪个任务已就绪，这样就会只轮询已就绪的 futures。
创建一个 `Waker` 最简单的方法，就是实现 `ArcWake` 特征，之后使用 `waker_ref`
或 `.into_waker` 方法来将一个 `Arc<impl ArcWake>` 转化成 `Waker`。
下面让我们为任务实现 `ArcWake` 以便将它们转化成可唤醒的 `Waker`。

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:arcwake_for_task}}
```

当从 `Arc<Task>` 创建 `Waker` 后，调用其 `wake()` 将拷贝一份 `Arc`
并将之发送到任务通道。之后执行器会取得这个任务并轮询它。让我们来实现它：

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_run}}
```

恭喜！现在我们就有了一个可工作的 futures 执行器。
我们甚至可以使用它去运行 `async/.await` 代码和自定义的 futures，
比如说之前完成的 `TimerFuture`。

```rust,edition2018,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:main}}
```

[唤醒任务]: ./03_wakeups_zh.md
