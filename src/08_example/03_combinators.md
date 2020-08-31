# Multiple Concurrent Actions Per Request
So far, the only way we've been able to run tasks concurrently has been to run them on separate threads.
Asynchronous code wouldn't be very useful if we could only achieve concurrency through multithreading.
Let's see how we can run multiple asynchronous tasks on a single thread.

Imagine we wanted to perform some more tasks with each incoming TCP connection.
For example, we might want to write information about the request to a database,
or put some data from the request onto a queue for processing.
Both of these actions can block, meaning that running them asynchronously will likely improve performance.

Let's simulate a slow request to a database or a blocking request to a queue:
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:slow_functions}}
```

A common mistake is to use `std::thread::sleep`, a blocking function, to simulate slow requests in examples like this one.
It's important to remember that even if a piece of code is run within an `async fn` and `await`ed, it may still block.
To make this example work, we'll need to replace `std::thread::sleep` with the non-blocking variant `async_std::task::sleep`.

Now, let's run `write_to_database` and `add_to_queue` within `handle_connection`:
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:serial_execution}}
```

If you run this code and visit `127.0.0.1:7878` in a browser, you'll see
"Write to database + add to queue took 5 seconds" printed to the console.
The request took 5 seconds because the program can only add to the queue once writing to the database has completed.

To run these two asynchronous functions concurrently, we can use the `join` combinator from the `futures` crate:
```rust,ignore
{{#include ../../examples/08_05_final_tcp_server/src/main.rs:parallel_execution}}
```
Handling a request will now take only 3 seconds. We've successfully run two tasks concurrently on one thread!
Please see the [section on combinators](../06_multiple_futures/01_chapter.md) for more information and examples.
