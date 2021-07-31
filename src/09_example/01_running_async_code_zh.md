# 运行异步代码
一个 HTTP 服务器应该能够同时为多个客户端提供服务；也就是说，在处理当前请求时，
它不应该去等待之前的请求完成。
这本书
[solves this problem](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#turning-our-single-threaded-server-into-a-multithreaded-server)
里，通过创建一个线程池,让每个连接都生成一个线程解决了这个问题。
现在，我们将使用异步代码来实现同样的效果，而不是增加线程数来提升吞吐量。

让我们修改 `handle_connection`，通过使用 `async fn` 声明它来让它返回一个 future。
```rust,ignore
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:handle_connection_async}}
```

在函数声明时，添加 `async` 会使得它的返回值由单元类型 `()`
变为实现了 `Future<Output=()>` 的类型。

如果现在尝试去编译它，编译器会警告我们，它可能不会工作：
```console
$ cargo check
    Checking async-rust v0.1.0 (file:///projects/async-rust)
warning: unused implementer of `std::future::Future` that must be used
  --> src/main.rs:12:9
   |
12 |         handle_connection(stream);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: futures do nothing unless you `.await` or poll them
```

因为我们没有 `await` 或 `poll` `handle_connection` 的返回值，它将永远不会执行。
如果你启动这个服务并在浏览器中访问 `127.0.0.1:7878`，会看到访问被拒绝，
因为服务端不会处理任何请求。

我们不能在同步代码里去 `await` 或 `poll` futures。现在我们需要一个，
可以调度并驱动 futures 去完成的异步运行时。
有关异步运行时、执行器和反应器的更多信息，请参阅
[选择一个运行时](../08_ecosystem/00_chapter.md) 这一章节。
其中列出的运行时，每一个都可在这个项目上使用，但在下面的示例中，
我们已经选择了 `async-std` 箱。

## 添加一个异步运行时
下面的例子中，将示范同步代码的重构，让它使用异步运行时，这里我们用的是 `async-std`。
`async-std` 中的 `#[async_std::main]` 属性允许我们编写异步的主函数。
这需要在 `Cargo.toml` 中启用 `async-std` 的 `attributes` 功能：
```toml
[dependencies.async-std]
version = "1.6"
features = ["attributes"]
```

首先，我们要切换到一个异步主函数上，`await` 异步版本的 `handle_connection`
返回的 future。然后，我们将测试这个服务如何响应，它看起来是这样的：
```rust
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:main_func}}
```
现在，让我们测试下看看，这个服务是否会同时处理多个连接。简单的将
`handle_connection` 标记为异步并不意味着服务就可在同时处理多个连接，
很快你就知道为什么了。

为了说明这点，让我们模拟一个很慢的请求。当一个客户端请求
`127.0.0.1:7878/sleep` 时，服务端将 sleep 5 秒。

```rust,ignore
{{#include ../../examples/09_03_slow_request/src/main.rs:handle_connection}}
```
这与
[simulation of a slow request](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#simulating-a-slow-request-in-the-current-server-implementation)
非常像，但有一个重要的区别:
我们使用非阻塞的 `async_std::task::sleep` 来替代阻塞的 `std::thread::sleep` 方法。
请记住，`async fn` 的代码在 `await` 时，可能会导致阻塞（若是阻塞代码），这很重要。
为了测试我们的服务能否正常的处理连接，我们必须确保 `handle_connection` 是非阻塞的。

现在你启动服务，并访问 `127.0.0.1:7878/sleep` 页面时，它会在5秒内，
阻塞，不接受任何新请求！
这是因为当前，在 `await` `handle_connection` 请求时，没有其它的任务可取得进展。
在下面的章节，我们将介绍如何使用异步代码来并发地处理连接。
