# `join!`
`futures::join`宏等待并发执行的多个不同future完成。

当进行多个异步操作时，可以简单地用`.await`串行执行：

```rust,no_run
{{#include ../../examples/06_02_join/src/lib.rs:naiive}}
```

然而，这实际上比必要的慢，因为我们不必在`get_book`完成后再`get_music`。在其它编程语言
中，future是运行至完成的，所以两个操作可以通过先调起`async fn`来启动future，然后再分别
await他们来并发操作：
However, this will be slower than necessary, since it won't start trying to
`get_music` until after `get_book` has completed. In some other languages,
futures are ambiently run to completion, so two operations can be
run concurrently by first calling the each `async fn` to start the futures,
and then awaiting them both:

```rust,no_run
{{#include ../../examples/06_02_join/src/lib.rs:other_langs}}
```

然而，Rust future不会干任何事情，除非它们已经`.await`了。这意味着上面这两段代码都会串行
执行`book_future`和`music_future`而非并发执行。为了正确地并发这两个future，使用
`futures::join!`：

```rust,no_run
{{#include ../../examples/06_02_join/src/lib.rs:join}}
```

`join!`返回值是包含每个传入future的输出的元组。

## `try_join!`

对于那些返回`Result`的future，考虑使用`try_join!`而非`join`。因为`join`只会在所有子
future都完成后才会完成，它甚至会在子future返回`Err`之后继续处理。

与`join!`不同，`try_join!`会在其中的字future返回错误后立即完成。

```rust,no_run
{{#include ../../examples/06_02_join/src/lib.rs:try_join}}
```

注意，传进`try_join!`的future必须要用相同的错误类型。考虑使用
`futures::future::TryFutureExt`库的`.map_err(|e| ...)`或`err_into()`函数来统一
错误类型：

```rust,no_run
{{#include ../../examples/06_02_join/src/lib.rs:try_join_map_err}}
```
