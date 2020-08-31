# Running Asynchronous Code
As [the book explains](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#turning-our-single-threaded-server-into-a-multithreaded-server),
we don't want our web server to wait for each request to finish before handling the next,
as some requests could be very slow.
Instead of improving throughput by adding threads, 
we'll use asynchronous code to process requests concurrently.

Let's modify `handle_connection` to return a future by declaring it an `async fn`:
```rust,ignore
{{#include ../../examples/08_02_async_tcp_server/src/main.rs:handle_connection_async}}
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
We'll need an executor to handle scheduling and running futures to completion.
Please consult the section [Choosing an Executor](../404.md) for more information on executors.
Here, we'll use the `block_on` executor from the `async_std` crate.

It might be tempting to write something like this:
```rust
{{#include ../../examples/08_02_async_tcp_server/src/main.rs:main_func}}
```

However, just because this program uses an asynchronous connection handler
doesn't mean that it handles connections concurrently.
To illustrate this, try out the 
[simulation of a slow request](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#simulating-a-slow-request-in-the-current-server-implementation)
from the Book. You'll see that one slow request will block any other incoming requests!
This is because there are no other concurrent tasks that can make progress
while we are `await`ing the result of `handle_connection`.
We'll see how to avoid this in the next section.