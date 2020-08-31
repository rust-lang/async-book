# Testing Async Code
Let's move on to testing our `handle_connection` function.
First, we need a `TcpStream` to work with, but we don't want to make a real TCP connection in test code.
We could work around this in a few ways.
One strategy could be to refactor the code to be more modular,
and only test that the correct responses are returned for the respective inputs.

Another strategy is to connect to `localhost` on port 0.
Port 0 isn't a valid UNIX port, but it'll work for testing.
The operating system will return a connection on any open TCP port.

Instead of those strategies, we'll change the signature of `handle_connection` to make it easier to test.
`handle_connection` doesn't actually require an `async_std::net::TcpStream`;
it requires any struct that implements `async_std::io::Read`, `async_std::io::Write`, and `marker::Unpin`.
Changing the type signature to reflect this allows us to pass a mock for testing instead of a TcpStream.
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:handle_connection}}
```

Next, let's build a mock `TcpStream` that implements these traits.
First, let's implement the `Read` trait, with one method, `poll_read`.
Our mock `TcpStream` will contain some data that is copied into the read buffer,
and we'll return `Poll::Ready` to signify that the read is complete.
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:mock_read}}
```

Our implementation of `Write` is very similar,
although we'll need to write three methods: `poll_write`, `poll_flush`, and `poll_close`.
`poll_write` will copy any input data into the mock `TcpStream`, and return `Poll::Ready` when complete.
No work needs to be done to flush or close the mock `TcpStream`, so `poll_flush` and `poll_close`
can just return `Poll::Ready`.
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:mock_write}}
```

Lastly, our mock will need to implement `Unpin`, signifying that its location in memory can safely be moved.
For more information on pinning and the `Unpin` trait, see the [section on pinning](../04_pinning/01_chapter.md).
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:unpin}}
```

Now we're ready to test the `handle_connection` function.
After setting up the `MockTcpStream` containing some initial data,
we can run `handle_connection` using `async_std::task::block_on`, exactly as we did in the main method.
To ensure that `handle_connection` works as intended, we'll check that the correct data
was written to the `MockTcpStream` based on its initial contents.
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:test}}
```