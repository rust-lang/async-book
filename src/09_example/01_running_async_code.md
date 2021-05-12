# Running Asynchronous Code
An HTTP server should be able to serve multiple clients concurrently;
that is, it should not wait for previous requests to complete before handling the current request.
The book
[solves this problem](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#turning-our-single-threaded-server-into-a-multithreaded-server)
by creating a thread pool where each connection is handled on its own thread.
Here, instead of improving throughput by adding threads, we'll achieve the same effect using asynchronous code.

Let's modify `handle_connection` to return a future by declaring it an `async fn`:
```rust,ignore
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:handle_connection_async}}
```

Adding `async` to the function declaration changes its return type
from the unit type `()` to a type that implements `Future<Output=()>`.

If we try to compile this, the compiler warns us that it will not work:
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

Because we haven't `await`ed or `poll`ed the result of `handle_connection`,
it'll never run. If you run the server and visit `127.0.0.1:7878` in a browser,
you'll see that the connection is refused; our server is not handling requests.

We can't `await` or `poll` futures within synchronous code by itself.
We'll need an asynchronous runtime to handle scheduling and running futures to completion.
Please consult the [section on choosing a runtime](../08_ecosystem/00_chapter.md)
for more information on asynchronous runtimes, executors, and reactors.
Any of the runtimes listed will work for this project, but for these examples,
we've chosen to use the `async-std` crate.

## Adding an Async Runtime
The following example will demonstrate refactoring synchronous code to use an async runtime; here, `async-std`.
The `#[async_std::main]` attribute from `async-std` allows us to write an asynchronous main function.
To use it, enable the `attributes` feature of `async-std` in `Cargo.toml`:
```toml
[dependencies.async-std]
version = "1.6"
features = ["attributes"]
```

As a first step, we'll switch to an asynchronous main function,
and `await` the future returned by the async version of `handle_connection`.
Then, we'll test how the server responds.
Here's what that would look like:
```rust
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:main_func}}
```
Now, let's test to see if our server can handle connections concurrently.
Simply making `handle_connection` asynchronous doesn't mean that the server
can handle multiple connections at the same time, and we'll soon see why.

To illustrate this, let's simulate a slow request.
When a client makes a request to `127.0.0.1:7878/sleep`,
our server will sleep for 5 seconds:

```rust,ignore
{{#include ../../examples/09_03_slow_request/src/main.rs:handle_connection}}
```
This is very similar to the 
[simulation of a slow request](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#simulating-a-slow-request-in-the-current-server-implementation)
from the Book, but with one important difference:
we're using the non-blocking function `async_std::task::sleep` instead of the blocking function `std::thread::sleep`.
It's important to remember that even if a piece of code is run within an `async fn` and `await`ed, it may still block.
To test whether our server handles connections concurrently, we'll need to ensure that `handle_connection` is non-blocking.

If you run the server, you'll see that a request to `127.0.0.1:7878/sleep`
will block any other incoming requests for 5 seconds!
This is because there are no other concurrent tasks that can make progress
while we are `await`ing the result of `handle_connection`.
In the next section, we'll see how to use async code to handle connections concurrently.
