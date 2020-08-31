# Handling Connections Concurrently

The problem with our code so far is that `listener.incoming()` is a blocking iterator;
we can't read a new request from this stream until we're done with the previous one.
One strategy to work around this is to spawn a new Task to handle each connection in the background:
```rust
{{#include ../../examples/08_03_concurrent_tcp_server/src/main.rs:main_func}}
```

This works because under the hood, the `async_std` executor runs `handle_connection` on a separate thread.
However, this doesn't completely solve our problem: `listener.incoming()` still blocks the executor.
Even if connections are handled in separate threads, futures running on the main thread
are blocked while `listener` waits on incoming connections.

In order to fix this, we can replace our blocking `std::net::TcpListener` with the non-blocking `async_std::net::TcpListener`.

This change prevents `listener.incoming()` from blocking the executor
by allowing us to `await` the next TCP connection on this port.
Now, the executor can yield to other futures running on the main thread
while there are no incoming TCP connections to be processed.
(Note that this change still does *not* allow `listener.incoming()` to emit items concurrently.
We still need to process a stream or spawn a task to handle it before moving on to the next one.)

Let's update our example to make use of the asynchronous `TcpListener`.
First, we'll need to update our code to `await` the next incoming connection,
rather than iterating over `listener.incoming()`:
```rust
{{#include ../../examples/08_04_nonblocking_tcp_server/src/main.rs:main_func}}
```

Lastly, we'll have to update our connection handler to accept an `async_std::net::TcpStream`:
```rust,ignore
{{#include ../../examples/08_04_nonblocking_tcp_server/src/main.rs:handle_connection}}
```
