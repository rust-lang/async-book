# `select!`

`futures::select` 宏会同时运行多个 futures，当其中任何一个 future
完成后，它会立即给用户返回一个响应。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:example}}
```

上面的函数将同时运行 `t1` 和 `t2`。当其中任意一个任务完成后，
就会运行与之对应的 `println!` 语句，同时结束此函数，无论是否还有未完成任务。

`select` 的基本语法是这样 `<pattern> = <expression> => <code>,`，
像这样你可以在 select 代码块里放进所有你需要的 futures。

## `default => ...` and `complete => ...`

`select` 同样支持 `default` 和 `complete` 分支。

当 `select` 中的 futures 都是未完成状态时，将运行 `default` 分支。
因此具有 `default` 分支的 `select` 都将立即返回一个结果。

在 `select` 的所有分支都是已完成状态，不会再取得任何进展时，`complete`
分支将会运行。当在循环中使用 `select!` 时，这是非常有用的！

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:default_and_complete}}
```

## 与 `Unpin` 和 `FusedFuture` 交互

在上面的第一个例子中，也许你发现了这点：对于在两个 `async fn` 返回的 futures，
我们必须对它们调用 `.fuse()` 方法，同时使用 `pin_mut` 来将它们固定。
这两个调用都是必要的，因为 `select` 中使用的 futures 必须同时实现 `Unpin` 和
`FusedFuture` 这两个特征。

`Unpin` 之所以有必要，是因为 `select` 使用中的 futures 不是其本身，
而是通过可变引用获取的。通过这种方式，`select` 不会获取 futures 的所有权，
从而使得其中未完成的 futures 可以在 `select` 后依然可用。

同样的，因为 `select` 不能轮询一个已完成的 future，所以我们也需要对 future 实现
`FusedFuture` 特征，以此来追踪其自身的完成状态。这样我们就可以在循环中使用
`select` 了，因为它只会去轮询未完成的 futures。在上面的示例中我们可以看到，
`a_fut` 及 `b_fut` 通过两次 `select` 循环后都已完成。因为 `future::ready`
返回的 future 实现了 `FusedFuture`，这样它就可以告知 `select` 
不要再去轮询它！

注意，streams 具有相应的 `FusedStream` 特征。实现此特征，或使用 `.fuse()`
包装后的 Streams，将从 `.next()` / `.try_next()` 组合子中产生 `FusedFuture`
futures。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fused_stream}}
```

## 带有 `Fuse` 和 `FuturesUnordered` 的 `select` 循环中的并发任务

一个有点儿难以发现但非常方便的函数是 `Fuse::terminated()`，
它允许创建一个已经终止的空 future，并可稍后再把一个需要运行的 future 填充进去。

当一个任务需要在 `select` 循环中运行，但它需要先在 `select` 循环内部产生时，
使用它就会变得很方便。

请注意 `.select_next_some` 函数的使用方法。它在同 `select` 一起使用时，
只运行 stream 返回值为 `Some(_)` 的分支，而忽略 `None`s。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fuse_terminated}}
```

当同一 future 的多个副本需要同时运行时，请使用 `FuturesUnordered` 类型。
下面的示例与上面的示例类似，但是会运行 `run_on_new_num_fut`
的每个副本直至全部完成，而非在创建新的副本后中止之前的任务。
它还将打印出 `run_on_new_num_fut` 的返回值。

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:futures_unordered}}
```
