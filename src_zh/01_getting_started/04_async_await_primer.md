# `async`/`.await`初步
`async`/`.await`是Rust内置语法，用于让异步函数编写得像同步代码。`async`将代码块转化成
实现了`Future`特质的状态机。使用同步方法调用阻塞函数会阻塞整个线程，但阻塞`Future`只会
让出（yield）线程控制权，让其他`Future`继续执行。

你可以使用`async fn`语法创建异步函数：

```rust,no_run
async fn do_something() { ... }
```

`async fn`函数返回实现了`Future`的类型。为了执行这个`Future`，我们需要执行器（executor）

```rust,no_run
{{#include ../../examples_zh/01_04_async_await_primer/src/lib.rs:hello_world}}
```

在`async fn`函数中， 你可以使用`.await`来等待其他实现了`Future`特质的类型完成，例如
另外一个`async fn`的输出。和`block_on`不同，`.await`不会阻塞当前线程，而是异步地等待
future完成，在当前future无法进行下去时，允许其他任务运行。

举个例子，想想有以下三个`async fn`: `learn_song`, `sing_song`和`dance`：

```rust,no_run
async fn learn_song() -> Song { ... }
async fn sing_song(song: Song) { ... }
async fn dance() { ... }
```

一个“学，唱，跳舞”的方法，就是分别阻塞这些函数：

```rust,no_run
{{#include ../../examples_zh/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

然而，这样性能并不是最优——我们一次只能干一件事！显然我们必须在唱歌之前学会它，但是学唱
同时也可以跳舞。为了拽黑暗，我们可以创建两个独立可并发执行的`async fn`：

```rust,no_run
{{#include ../../examples_zh/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

这个示例里，唱歌之前必须要学习唱这首歌，但是学习唱歌和唱歌都可以和跳舞同时发生。如果我们
用了`block_on(learning_song())`而不是`learn_and_sing`中的`learn_song().await`,
那么当`learn_song`在执行时线程将无法做别的事，这也使得无法同时跳舞。但是通过`.await`
执行`learn_song`的future，我们就可以在`learn_song`阻塞时让其他任务来掌控当前线程。
这样就可以做到在单线程并发执行多个future到完成状态。

现在，你已经学会了`async`/`await`基础，现在我们来试着写一个例子吧。 
