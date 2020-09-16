# Handling Connections Concurrently
The problem with our code so far is that `listener.incoming()` is a blocking iterator.
The executor can't run other futures while `listener` waits on incoming connections,
and we can't handle a new connection until we're done with the previous one.

In order to fix this, we'll transform `listener.incoming()` from a blocking Iterator
to a non-blocking Stream. Streams are similar to Iterators, but can be consumed asynchronously.
For more information, see the [chapter on Streams](../05_streams/01_chapter.md).

Let's replace our blocking `std::net::TcpListener` with the non-blocking `async_std::net::TcpListener`,
and update our connection handler to accept an `async_std::net::TcpStream`:
```rust,ignore
{{#include ../../examples/08_04_concurrent_tcp_server/src/main.rs:handle_connection}}
```

The asynchronous version of `TcpListener` implements the `Stream` trait for `listener.incoming()`,
a change which provides two benefits.
The first is that `listener.incoming()` no longer blocks the executor.
The executor can now yield to other pending futures 
while there are no incoming TCP connections to be processed.

The second benefit is that elements from the Stream can optionally be processed concurrently,
using a Stream's `for_each_concurrent` method.
Here, we'll take advantage of this method to handle each incoming request concurrently.
We'll need to import the `Stream` trait from the `futures` crate, so our Cargo.toml now looks like this:
```diff
+[dependencies]
+futures = "0.3"

 [dependencies.async-std]
 version = "1.6"
 features = ["attributes"]
```

Now, we can handle each connection concurrently by passing `handle_connection` in through a closure function.
The closure function takes ownership of each `TcpStream`, and is run as soon as a new `TcpStream` becomes available.
As long as `handle_connection` does not block, a slow request will no longer prevent other requests from completing.
```rust,ignore
{{#include ../../examples/08_04_concurrent_tcp_server/src/main.rs:main_func}}
```