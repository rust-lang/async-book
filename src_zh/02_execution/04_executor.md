# 应用：构建执行器

Rust的`Future`是惰性的：它们不会干任何事，除非它们被驱动执行。一个驱动future类型的
方法是在`async`函数中使用`.await`调用，但这只是将问题抛到上一层：谁来跑在顶层`async`
函数返回的future实例呢？为此，我们需要执行`Future`的执行器。

`Future`执行器会拿一组顶层`Future`去跑`poll`方法，无论这些`Future`能否进展。通常，
执行器会`poll`一个future实例来启动。当`Future`通过调用`wake()`方法来指示他们准备好继续
进展，执行器就会把它们放入队列并再一次`poll`，重复这一过程直到`Future`完成。

在这一小节，我们要写一个我们的简单执行器，能够并发地运行大量的顶层future实例。

为此，我们需要依赖`futures`库的`ArcWake`特质，这个特质提供了构造`Waker`的简易方法。

```toml
[package]
name = "xyz"
version = "0.1.0"
authors = ["XYZ Author"]
edition = "2018"

[dependencies]
futures-preview = "=0.3.0-alpha.17"
```

然后，我们在`src/main.rs`中引入以下：

```rust,no_run
{{#include ../../examples_zh/02_04_executor/src/lib.rs:imports}}
```

我们的执行器通过给通道（channel）发送任务来工作。执行器会从通道中拉取事件并执行它们。当
一个任务准备好进一步工作（被唤醒了）时，它会被放到channel的末尾，来让自己再次被调度。

在设计时，执行器自身只需要任务通道的接收端。用户会拿到发送端，那样它们就可以开辟（spawn）
新的future实例。任务自身仅仅是能够重新调度自身的future， 所以我们要把它们作为和发送端
配对的future存储。这个发送端能够让任务重新排队。

```rust,no_run
{{#include ../../examples_zh/02_04_executor/src/lib.rs:executor_decl}}
```

我们来加一个方法，让开辟器（spawner）更容易开辟新future吧。这个方法会获取一个future类型，
把它装箱并把它变成一个FutureObj对象，然后把这对象放到新的`Arc<Task>`里面。这个`Arc<Task>`
能够放到执行器的队列中。

```rust,no_run
{{#include ../../examples_zh/02_04_executor/src/lib.rs:spawn_fn}}
```

为了轮询future，我们需要创建`Waker`。正如在[任务唤醒小节]中讨论到，`Waker`负责调度任务
在`wake`函数调用时再次轮询。记住，`Waker`告诉执行器具体哪个任务已经准备好了，这使得它们
可以只轮询已经准备好的future。创建`Waker`的最简单方法是实现`ArcWake`特质，然后使用
`waker_ref`或者`.into_waker()`函数来把`Arc<impl ArcWake>`转变成`Waker`。我们来给
我们的任务实现`ArcWake`，以便它们可以变成`Waker`并且被唤醒：

```rust,no_run
{{#include ../../examples_zh/02_04_executor/src/lib.rs:arcwake_for_task}}
```

当`Waker`从`Arc<Task>`创建了之后，调用`wake()`函数会拷贝一份`Arc`，发送到任务的通道去。
我们的执行器就会拿到这个任务并轮询它。我们来实现这个吧：

```rust,no_run
{{#include ../../examples_zh/02_04_executor/src/lib.rs:executor_run}}
```

恭喜！我们现在有一个能干活的future执行器了。我们甚至能用它来运行`async/.await`代码和定制
的future，例如我们前面写的`TimeFuture`：

```rust,no_run
{{#include ../../examples_zh/02_04_executor/src/lib.rs:main}}
```

[任务唤醒小节]: ./03_wakeups.md
