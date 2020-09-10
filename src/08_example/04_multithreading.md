# Adding Parallelism
Our example so far has largely presented concurrency (using async code)
as an alternative to parallelism (using threads).
However, async code and threads are not mutually exclusive.
Async executors can be either single-threaded or multithreaded.
For example, the [`async-executor` crate](https://docs.rs/async-executor) used by `async-std`
has both a single-threaded `LocalExecutor` and a multi-threaded `Executor`.

Tasks do not always need to be run on the thread that created them,
and async runtimes often include functionality for spawning tasks onto separate threads.
Even if tasks are executed on separate threads, they should still be non-blocking.

Some libraries provide functions for spawning blocking tasks onto separate threads,
which is useful for running synchronous code from other libraries.
Tasks are usually required to be `Send`, so they can be moved to separate threads.
Some libraries also provide functions for spawning non-`Send` tasks onto a thread-local executor.
Both Tokio and async-std have `task::spawn_blocking` and `task::spawn_local` functions,
although the async-std versions are unstable.

In our example, `for_each_concurrent` processes each connection concurrently on the same thread as the `main` function.
Here, `handle_connection` is both `Send` and non-blocking,
so we could have instead spawned new tasks to run `handle_connection`.
We can use `async_std::task::spawn` for this purpose:
```rust
{{#include ../../examples/08_05_multithreaded_tcp_server/src/main.rs:main_func}}
```
