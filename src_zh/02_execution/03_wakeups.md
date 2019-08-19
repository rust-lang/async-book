# 用`Waker`唤醒任务

future第一次轮询时没有执行完这事很常见。此时，future需要保证会被再次轮询以进展（make 
progress），而这由`Waker`类型负责。

每次future被轮询时， 它是作为一个“任务”的一部分轮询的。任务（Task）是能提交到执行器上
的顶层future。

`Waker`提供`wake()`方法来告诉执行器哪个关联任务应该要唤醒。当`wake()`函数被调用时，
执行器知道`Waker`关联的任务已经准备好继续了，并且任务的future会被轮询一遍。

`Waker`类型还实现了`clone()`，因此可以到处拷贝储存。

我们来试试用`Waker`实现一个简单的计时器future吧。

## 应用：构建计时器

这个例子的目标是： 在创建计时器时创建新线程，休眠特定时间，然后过了时间窗口时通知（signal）
计时器future。

这是我们开始时需要的导入：

```rust,no_run
{{#include ../../examples_zh/02_03_timer/src/lib.rs:imports}}
```

我们开始定义future类型吧。 我们的future需要一个方法，让线程知道计时器倒数完了，future
应该要完成了。我们准备用`Arc<Mutex<..>>`共享值来为沟通线程和future。

```rust,no_run
{{#include ../../examples_zh/02_03_timer/src/lib.rs:timer_decl}}
```

现在，我们来实现`Future`吧！

```rust,no_run
{{#include ../../examples_zh/02_03_timer/src/lib.rs:future_for_timer}}
```

很简单，对吧？如果线程已经设置成`shared_state.completed = true`，我们就搞定了！否则，
我们从当前任务克隆`Waker`并把它传到`shared_state.waker`，这样线程就能回头再唤醒这个任务。

重要的是，每次future轮询后，我们必须更新`Waker`，这是因为这个future可能会移动到不同的
任务去，带着不同的`Waker`。这会在future轮询后在不同任务间移动时发生。

最后，我们需要API来构造计时器并启动线程：

```rust,no_run
{{#include ../../examples_zh/02_03_timer/src/lib.rs:timer_new}}
```

哇！这些就是我们构建一个简单计时器future所需的内容了。现在，只要一个执行器（Executor）
执行这个future...
