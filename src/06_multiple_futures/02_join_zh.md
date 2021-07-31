# `join!`

`futures::join` 宏可以同时执行多个不同的 futures 并等待它们的完成。

# `join!`

当执行多个异步操作时，可以很简单地将它们组成一个序列并使用 `.await`： 

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:naiive}}
```

但是，这样做会使它变得更慢，除非 `get_book` 已经完成，否则 `get_music`
不会开始运行。在其它一些语言中，futures 是在环境中自发去运行、完成的，
所以可以通过先去调用每个 `async fn` 来启动 future，然后再等待它们完成：

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:other_langs}}
```

然而，在 Rust 中，futures 不会在被 `.await` 前做任何操作。
这就意味着上面的两个代码块都会按序来运行 `book_future` 和 `music_future`
而非并发地运行它们。我们可以使用 `futures::join!` 来正确的并发运行它们：

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:join}}
```

`join!` 返回的值是一个包含每个传入的 `Future` 的输出的元组。

## `try_join!`

对于那些返回值是 `Result` 类型的 futures，可以考虑使用 `try_join!` 而非 `join!`。
因为 `join!` 只会在所有的子 futures 完成后，才会完成，
即使其中的子 future 返回了错误，也会继续等待其它子 future 完成。

不同于 `join!`，`try_join!` 将在某个子 future 返回 error 后立即完成。

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join}}
```

要注意！所有传入 `try_join!` 的 futures 都必须有相同的错误类型。
你可以 `futures::future::TryFutureExt` 中的 `.map_err(|e| ...)` 与
`.err_into()` 来转化错误类型。

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join_map_err}}
```
