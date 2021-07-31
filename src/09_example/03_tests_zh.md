# 测试 TCP 服务
让我们继续测试我们的 `handle_connection` 函数。

首先，我们需要一个 `TcpStream`。
在端到端或集成测试中，我们需要建立一个真正的 TCP 连接去测试我们的代码。
一种策略是在 `localhost` 的 0 端口上启用监听器。
端口 0 在 UNIX 上并不是一个合法端口，但我们可以在测试中使用它。
操作系统会为我们选择一个打开的 TCP 端口。

然而，在这个示例中，我们将为连接处理器写一个单元测试，
来检查是否为对应的请求返回了正确的响应。
为了保证我们的单元测试的独立性和确定性，我们将模拟一个 `TcpStream`。

首先，我们将改变 `handle_connection` 的接受值类型签名让它更易于测试。
`handle_connection` 并不一定要接收一个 `async_std::net::TcpStream`；
它只需要接收一个实现了 `async_std::io::Read`、`async_std::io::Write` 和
`marker::Unpin` 的结构体。变更类型签名以便允许我们通过模拟进行测试。
```rust,ignore
use std::marker::Unpin;
use async_std::io::{Read, Write};

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
```

然后，让我们构建一个实现了这些特征的模拟 `TcpStream`。
首先，去实现 `Read` 特征，它只有一个函数——`poll_read`。
我们的模拟 `TcpStream` 将包括一些复制到读取缓存的数据，
然后返回 `Poll::Ready` 来提示读取已完成。
```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_read}}
```

实现 `Write` 也类似，但我们需要实现 `poll_write`、`poll_flush` 和 `poll_close`
三个函数。`poll_write` 将拷贝输入的数据到模拟 `TcpStream` 中，并完成后返回
`Poll::Ready`。模拟 `TcpStream` 不需要执行 flush 和 close，所以 `poll_flush`
和 `poll_close` 可直接返回 `Poll::Ready` 即可。
```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_write}}
```

最后，我们的模拟 `TcpStream` 需要实现 `Unpin`，来表示它在内存中可安全的移动。
细节实现可回看 [固定](../04_pinning/01_chapter_zh.md)。
```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:unpin}}
```

现在我们准备好去测试 `handle_connection` 函数了。
在我们给 `MockTcpStream` 设置一些初始化数据后，我们可以通过 `#[async_std::test]`
属性来运行 `handle_connection` 函数，它的使用方法和 `#[async_std::main]` 类似。
为了确保 `handle_connection` 是按我们预期设计工作的，我们将根据其初始数据，
来检查是否已将正确的数据写入 `MockTcpStream`。
```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:test}}
```
