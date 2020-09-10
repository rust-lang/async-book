# Multiple Concurrent Actions Per Request
Imagine we wanted to perform some more tasks with each incoming TCP connection.
For example, we might want to write information about the request to a database,
or put some data from the request onto a queue for processing.
Both of these actions can block, meaning that running them asynchronously will likely improve performance.

Let's modify the [simulated slow request](../08_example/01_running_async_code.md) from earlier in the example,
breaking it into multiple slow function calls:
```rust,ignore
{{#include ../../examples/08_06_final_tcp_server/src/main.rs:slow_functions}}
```
Again, we're using the non-blocking function `async_std::task::sleep` instead of `std::thread::sleep`, which blocks.

Now, let's run `write_to_database` and `add_to_queue` within `handle_connection`:
```rust,ignore
{{#include ../../examples/08_06_final_tcp_server/src/main.rs:serial_execution}}
```

If you run this code, you'll see "Write to database + add to queue took 5 seconds" printed to the console.
The request took 5 seconds because the program can only add to the queue once writing to the database has completed.

To run these two asynchronous functions concurrently, we can use the `join` combinator from the `futures` crate:
```rust,ignore
{{#include ../../examples/08_06_final_tcp_server/src/main.rs:parallel_execution}}
```
Handling a request will now take only 3 seconds. We've successfully run two concurrent tasks!
Please see the [section on combinators](../06_multiple_futures/01_chapter.md) for more information and examples.
