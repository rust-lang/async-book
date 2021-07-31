# 并发地处理连接
目前我们的代码中的问题是 `listener.incoming()` 是一个阻塞的迭代器。
执行器无法在 `listener` 等待一个入站连接时，运行其它 futures，
这便导致了我们只有等之前的请求完成，才能处理新的连接。

我们可以通过将 `listener.incoming()` 从一个阻塞迭代器转变为一个非阻塞流。
流类似于迭代器，但它是可用于异步消费的，
详情可回看[Streams](../05_streams/01_chapter_zh.md)。

让我们使用 `async-std::net::TcpListener` 替代 `std::net::TcpListener`，
并更新我们的连接处理函数，让它接受 `async_std::net::TcpStream`：
```rust,ignore
{{#include ../../examples/09_04_concurrent_tcp_server/src/main.rs:handle_connection}}
```

这个异步版本的 `TcpListener` 为 `listener.incoming()` 实现了 `Stream` 特征，
这带来了两个好处。其一，`listener.incoming()` 不再是一个阻塞的执行器了。
在没有入站的 TCP 连接可取得进展时，它可以允许其它挂起的 futures 去执行。

第二个好处是，可以使用 `Stream` 的 `for_each_concurrent` 方法，来并发地处理来自
`Stream` 的元素。在这里，我们就是使用了这个方法来并发处理每个请求。
我们需要从 `futures` 箱中导入 `Stream` 特征，现在 Cargo.toml 看起来是这样的：
```diff
+[dependencies]
+futures = "0.3"

 [dependencies.async-std]
 version = "1.6"
 features = ["attributes"]
```

现在，我们可以通过闭包函数传入 `handle_connection` 来并发处理每个连接。
闭包函数将获得每个 `TcpStream` 的所有权，并在新的 `TcpStream` 就绪时立即执行。
因为 `handle_connection` 不再是阻塞的，一个慢请求不会阻止其它请求的完成。
```rust,ignore
{{#include ../../examples/09_04_concurrent_tcp_server/src/main.rs:main_func}}
```
# 并行处理请求
到目前为止，我们的示例在很大程度上，将并发（通过异步代码）
作为并行（使用线程）的替代方案。但是，异步和线程并非完全互斥的。
在我们的示例中，`for_each_concurrent` 并发地在同一个进程中处理每个连接。
但 `async-std` 箱也允许我们去在特定的线程上生成任务。
因为 `handle_connection` 是可 `Send` 且是非阻塞的，所以它可以安全地使用
`async_std::task::spawn`。代码是这样的：
```rust
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:main_func}}
```
现在我们可以同时使用并发和并行来同时处理多个连接！详情可查看
[多线程执行器](../08_ecosystem/00_chapter_zh.md#single-threading-vs-multithreading)。
