# 通过 `Waker` 唤醒任务

futures 在第一次被 `poll` 时是未就就绪状态是很常见的。当出现这种情况时，
futures 需要确保在其就绪后即会被再次轮询。而这是通过 `Waker` 类型实现的。

每次轮询 future 时，都会将其作为“任务”的一部分进行轮询。
任务是已交由执行器控制的顶级的 future。

`Waker` 提供了 `wake()` 方法来告知执行器相关任务需要被唤醒。当调用 `wake()` 时，
执行器就知道其关联的任务已就绪，并再次轮询那个 future。

`Waker` 还实现了 `clone()`，以便复制和存储。

现在让我们尝试去使用 `Waker` 来实现一个简单的计时器吧。

## 应用：构建一个计时器

就此示例而言，我们将在创建计时器时启动一个新线程，并让它休眠一定的时间，
然后在时间窗口结束时给计时器 future 发信号。

以下是我们在开始工作前需要导入的：

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:imports}}
```

让我们首先定义这个 future 类型。
此 future 需要一种方法去通知线程计时器已完成且自身已就绪。
我们将使用 `Arc<Mutex<..>>` 共享值来在线程和 future 之间进行通信。

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:timer_decl}}
```

那么现在，让我们开始编写代码来实现 `Future`！

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:future_for_timer}}
```

非常简单，是吧？当这个线程的设置变为 `shared_state.completed = true`，就完成了！
否则，我们将克隆当前任务的 `Waker` 并把它放置在 `shared_state.waker` 中，
以便线程可再次唤醒任务。

我们必须在每次轮询完 future 后更新 `Waker`，
因为 future 可能被转移到不同的任务的 `Waker` 中了，这点非常重要。
在 futures 被轮询后，在任务间传递时，这种情况时有发生。

最后，我们需要一个 API 来实际上构建计时器并启动线程：

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:timer_new}}
```

哈！以上便是我们构建一个简单的计时器 future 所需的全部组件。
现在，我们只需要一个执行器来运行它了...
