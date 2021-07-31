# `async`/`.await` 入门

`async`/`.await` 是 Rust 的内置工具，使得你可以如同写同步代码一样编写异步程序。
`async` 将一个代码块转化为一个实现了名为 `Future` 特征的状态机。
虽然以同步的方式调用阻塞函数会阻塞整个线程，但阻塞的 `Future` 会让出线程控制权，
允许其它 `Future` 运行。

Let's add some dependencies to the `Cargo.toml` file:
让我们在 `Cargo.toml` 文件中添加一些依赖项。

```toml
{{#include ../../examples/01_04_async_await_primer/Cargo.toml:9:10}}
```

你可以使用 `async fn` 语法来创建一个异步函数：

```rust,edition2018
async fn do_something() { /* ... */ }
```

`async fn` 的返回值是一个 `Future`。需有一个执行器，`Future` 才可执行。

```rust,edition2018
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:hello_world}}
```

在 `async fn` 中，你可以使用 `.await` 等待另一个实现了 `Future` 特征的类型完成，
例如另一个 `async fn` 的返回值。与 `block_on` 不同，`.await` 不会阻塞当前进行，
而是允许其它任务继续运行，同时在异步状态等待它的完成。

例如，现在我们有三个异步函数，分别是 `learn_song`，`sing_song` 以及 `dance`：

```rust,ignore
async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }
```

一种是，学唱、唱和跳舞以阻塞的方式的执行：

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

然而，这种方式并未发挥出最好的性能——因为我们每次只做了一件事。
显然，只有在学会唱歌后才能去唱，但在我们学习或唱歌时，却可以同时跳舞的。
要实现这个，我们可以创建分别创建两个 `async fn` 来并发的执行：

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

在这个例子中，学习唱歌必须在唱歌之前，但学唱和唱歌都可与跳舞这个行为同时发生。
如果我们在 `learn_and_sing` 中使用 `block_on(learn_song())` ，
而不是 `learn_song().await`，它将在执行时阻塞主进程直至学歌完成，而无法同时跳舞。
通过 `.await`，使得在学歌这一行为发生阻塞时，让出主进程控制权，
从而使得其它任务可以并发的进行。
