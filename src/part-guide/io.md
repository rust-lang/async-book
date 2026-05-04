# IO and issues with blocking

Efficiently handling IO (input/output) is one of the primary motivators for async programming and most async programs do lots of IO. At it's root, the issue with IO is that it takes orders of magnitude more time than computation, therefore just waiting for IO to complete rather than getting on with other work is incredibly inefficient. Ideally, async programming lets a program get on with other work while waiting for IO.

This chapter is an introduction to IO in the async context. We'll cover the important difference between blocking and non-blocking IO, and why blocking IO and async programming don't mix (at least not without a bit of thought and effort). We'll cover how to use non-blocking IO, then look at some of the issues which can crop up with IO and async programming. We'll also look at how the OS handles IO and have a sneak peak at some alternative IO methods like io_uring.

We'll finish by covering some other ways of blocking an async task (which is bad) and how to properly mix async programming with blocking IO or long-running, CPU-intensive code.


## Blocking and non-blocking IO

IO is implemented by the operating system; the work of IO takes place in separate processes and/or in dedicated hardware, in either case outside of the program's process. IO can be either synchronous or asynchronous (aka blocking and non-blocking, respectively). Synchronous IO means that the program (or at least the thread) waits (aka blocks) while the IO takes place and doesn't start processing until the IO is complete and the result is received from the OS. Asynchronous IO means that the program can continue to make progress while the IO takes place and can pick up the result later. There are many different OS APIs for both kinds of IO, though more variety in the asynchronous space.

Asynchronous IO and asynchronous programming are not intrinsically linked. However, async programming facilitates ergonomic and performant async IO, and that is a major motivation for async programming. Blocking due to synchronous IO is a major source of performance issues with async programming, and we must be careful to avoid it (more on this below).

Rust's standard library includes functions and traits for blocking IO. For non-blocking IO, you must use specialized libraries, which are often part of the async runtime, e.g., Tokio's [`io`](https://docs.rs/tokio/latest/tokio/io/index.html) module.

Let's quickly look at an example (adapted from the Tokio docs):

```rust
use tokio::{io::AsyncWriteExt, net::TcpStream};

async fn write_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"hello world!").await?;

    Ok(())
}
```

`write_all` is an async IO method which writes data to `stream`. This might complete immediately, but more likely this will take some time to complete, so `stream.write_all(...).await` will cause the current task to be paused while it waits for the OS to handle the write. The scheduler will run other tasks and when the write is complete, it will wake up the task and schedule it to continue working.

However, if we used a write function from the standard library, the async scheduler would not be involved and the OS would pause the whole thread while the IO completes, meaning that not only is the current task paused but no other task can be executed using that thread. If this happens to all threads in the runtime's thread pool (which in some circumstances can be just one thread), then the whole program stops and cannot make progress. This is called blocking the thread (or program) and is very bad for performance. It is important to never block threads in an async program, and thus you should avoid using blocking IO in an async task.

Blocking a thread can be caused by long-running tasks or tasks waiting for locks, as well as by blocking IO. We'll discuss this more at [the end of this chapter](#other-blocking-operations).

It is a common pattern to repeatedly read or write, and streams and sinks (aka async iterators) are a convenient mechanism for doing so. They're covered in a [dedicated chapter](streams.md).


## Reading and writing

TODO

- async Read and Write traits
  - part of the runtime
- how to use
- specific implementations
  - network vs disk
    - tcp, udp
    - file system is not really async, but io_uring (ref to that chapter)
  - practical examples
  - stdout, etc.
  - pipe, fd, etc.


## Memory management

When we read data we need to put it somewhere and when we write data it needs to be kept somewhere until the write completes. In either case, how that memory is mangaged is important.

TODO


- Issues with buffer management and async IO
- Different solutions and pros and cons
  - zero-copy approach
  - shared buffer approach
- Utility crates to help with this, Bytes, etc.

## Advanced topics on IO

TODO


- buf read/write
- Read + Write, split, join
- copy
- simplex and duplex
- cancelation
- what if we have to do sync IO? Spawn a thread or use spawn_blocking (see below)

## The OS view of IO

TODO

- Different kinds of IO and mechanisms, completion IO, reference to completion IO chapter in adv section
  - different runtimes can faciliate this
  - mio for low-level interface


## Other blocking operations

As mentioned at the start of the chapter, not blocking threads is crucial for the performance of async programs. Blocking IO of different kinds is a common way to block, but it is also possible to block by doing lots of computation or waiting in a way which the async scheduler isn't coordinating.

Waiting is most often caused by using non-async aware synchronisation mechanisms, for example, using `std::sync::Mutex` rather than an async mutex, or waiting for a non-async channel. We'll discuss this issue in the chapter on [Channels, locking, and synchronization](sync.md). There are other ways that you might wait in a blocking way, and in general you need to find a non-blocking or otherwise async-friendly mechanism, e.g., using an async `sleep` function rather than the std one. Waiting could also be a busy wait (effectively just looping without doing any work, aka a spin lock), you should probably just avoid that.

### CPU-intensive work

Doing long-running (i.e., cpu-intensive or cpu-bound) work will prevent the scheduler from running other tasks. This *is* a kind of blocking, but it is not as bad as blocking on IO or waiting because at least your program is making some progress. However (without care and consideration), it is likely to be sub-optimal for performance by some measure (e.g., tail latency) and perhaps a correctness issue if the tasks that can't run needed to be run at a particular time. There is a meme that you should simply not use async Rust (or general purpose async runtimes like Tokio) for CPU-intensive work, but that is an over-simplification. What is correct is that you cannot mix IO- and CPU-bound (or more precisely, long-running and latency-sensitive) tasks without some special handling and expect to have a good time.

For the rest of this section, we'll assume you have a mix of latency-sensitive tasks and long-running, CPU-intensive tasks. If you don't have anything which is latency-sensitive, then things are a bit different (mostly easier).

There are essentially three solutions for running long-running or blocking tasks: use a runtime's built-in facilities, use a separate thread, or use a separate runtime.

In Tokio, you can use [`spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html) to spawn a task which might block. This works like [`spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) for spawning a task, but runs the task in a separate thread pool which is optimized for tasks which might block (the task will likely run on it's own thread). Note that this runs regular synchronous code, not an async task. That means that the task can't be cancelled (even though its `JoinHandle` has an `abort` method). Other runtimes provide similar functionality.

This example uses `spawn_blocking` to perform blocking I/O by calling a synchronous filesystem function from the standard library. Note that `tokio::fs` also exists and provides asynchronous filesystem APIs; however, under the hood it too uses blocking operations wrapped in `spawn_blocking`. This is because on most operating systems, file operations are inherently blocking.

```rust,norun
use tokio;

#[tokio::main]
async fn main() {
    let contents = tokio::task::spawn_blocking(|| {
		std::fs::read_to_string("file.txt").unwrap()
    })
	.await
	.unwrap();

    println!("{contents}");
}
```

Because tasks spawned with `spawn_blocking` cannot be aborted, it is intended for tasks that eventually complete. Tasks that may block indefinitely, such as a server listening for incoming requests, are better suited to run on a dedicated thread, to avoid occupying thread from Tokio's blocking thread pool for an extended period. You can spawn a dedicated thread using [`std::thread::spawn`](https://doc.rust-lang.org/stable/std/thread/fn.spawn.html) (or similar functions). This is pretty straightforward.

If you need to run a lot of tasks, you'll probably need some kind of thread pool or work scheduler. If you keep spawning threads and have many more than there are cores available, you'll end up sacrificing throughput. [Rayon](https://github.com/rayon-rs/rayon) is a popular choice which makes it easy to run and manage parallel tasks. You might get better performance with something which is more specific to your workload and/or has some knowledge of the tasks being run.

Here is an example of calculating [dot product](https://en.wikipedia.org/wiki/Dot_product) between two large vectors using Rayon together with Tokio. It utilizes [`tokio::oneshot::channel`](https://docs.rs/tokio/latest/tokio/sync/oneshot/fn.channel.html) to communicate results between a task spawned by Rayon and the current task in Tokio. 

```rust,norun
#[tokio::main]
async fn main() {
    let a = (1..=1024 * 1024).collect();
    let b = (1..=1024 * 1024).collect();
    println!("{}", compute_dot_product(a, b).await);
}

async fn compute_dot_product(a: Vec<u64>, b: Vec<u64>) -> u64 {
    assert_eq!(a.len(), b.len(), "a and b must have the same length");

    let (send, recv) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon to calculate the dot product.
    rayon::spawn(move || {
        let mut result = 0;
        for (a, b) in a.iter().zip(b) {
            result += a * b;
        }
        // Send the result back to Tokio.
        let _ = send.send(result);
    });

    recv.await.expect("Panic in rayon::spawn")
}
```

You can use a separate instances of the async runtime for latency-sensitive tasks and for long-running tasks. This is suitable for CPU-bound tasks, but you still shouldn't use blocking IO, even on the runtime for long-running tasks. For CPU-bound tasks, this is a good solution in that it is the only one which supports the long-running tasks be async tasks. It is also flexible (since the runtimes can be configured to be optimal for the kind of task they're running; indeed, it is necessary to put some effort into runtime configuration to get optimal performance) and lets you benefit from using mature, well-engineered sub-systems like Tokio. You can even use two different async runtimes. In any case, the runtimes must be run on different threads.

On the other hand, you do need to do a bit more thinking: you must ensure that you are running tasks on the right runtime (which can be harder than it sounds) and communication between tasks can be complicated. We'll discuss synchronisation between sync and async contexts next, but it can be even trickier between multiple async runtimes. Each runtime is it's own little universe of tasks and the schedulers are totally independent. Tokio channels and locks *can* be used from different runtimes (even non-Tokio ones), but other runtimes' primitives may not work in this way.

Since the scheduler in each runtime is oblivious of other runtimes (and the OS is oblivious to any async schedulers), there is no coordination or shared prioritisation of scheduling and work cannot be stolen between runtimes. Therefore, scheduling of tasks can be sub-optimal (especially if the runtimes are not well-tuned to their workloads). Furthermore, since all scheduling is cooperative, long-running tasks can still be starved of resources and latency can suffer. See the [next section](#yielding) for how long-running tasks can be made to be more cooperative.

As a pure scheduler, using Tokio for CPU work is likely to have slightly higher overheads than a dedicated, synchronous worker pool. This is not surprising when one considers the extra work required to support async programming. This is unlikely to be a problem in practice for most users, but might be worth considering if your code is extremely performance sensitive.

For any of the above solutions, you will have tasks running in different contexts (sync and async, or different async runtimes). If you need to communicate between tasks, then you need to take care that you are using the correct combinations of sync and async primitives (channels, mutexes, etc.) and the correct (blocking or non-blocking) methods on those primitives. For mutexes and similar locks, you should probably use the async versions if you need to hold the lock across an await point or protect an IO resource (it should be usable from sync contexts by using a blocking lock method), or a synchronous version to protect data or where the lock does not need to be held across an await point. Tokio's async channels can be used from sync context with blocking methods, but see [these docs](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code) for some detail on when to use sync or async channels.

So, which of the above solutions should you use?

- If you're doing blocking IO, you should probably use `spawn_blocking`. You cannot use a second runtime or other thread pool (at least if you need optimal performance).
- If you have a thread that will run forever, you should use `std::thread::spawn` rather than use any kind of thread pool (since it will use up one of the pool's threads).
- If you're doing *lots* of CPU work, then you should use a thread pool, either a specialised one or a second async runtime.
- If you need to run long-running async code, then you should use a second runtime.
- You might choose to use a dedicated thread or `spawn_blocking` because it is easy and has satisfactory performance, even though a more complex solution is more optimal.


### Yielding

Long-running code is an issue because it doesn't give the scheduler an opportunity to schedule other tasks. Async concurrency is cooperative: the scheduler cannot pre-empt a task to run a different one. If a long-running task doesn't yield to the scheduler, then the scheduler cannot stop it. However, if the long-running code does yield to the scheduler, then other tasks can be scheduled and the fact that a task is long-running is not an issue. This can be used as an alternative to using another thread for CPU-intensive work or for CPU-intensive work on it's own runtime to (possibly) improve performance.

Yielding is easy, simply call the runtime's yield function. In Tokio that is [`yield_now`](https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html). Note that this is different to both the standard library's [`yield_now`](https://doc.rust-lang.org/stable/std/thread/fn.yield_now.html) and the `yield` keyword for yielding from a coroutine. Calling `yield_now` won't yield to the scheduler if the current future is being run inside a `select` or `join` (see the chapter on [composing futures concurrently](concurrency-primitives.md)); that may or may not be what you want to happen.

Knowing when you need to yield is a bit more tricky. First of all you need to know if your program is implicitly yielding. This can only happen at an `.await`, so if you're not `await`ing, then you're not yielding. But await doesn't automatically yield to the scheduler. That only happens if the leaf future being `await`ed is pending (not ready) or there is an explicit `yield` somewhere in the call stack. Tokio and most async runtimes will do this in their IO and synchronization functions, but in general you can't know whether an `await` will yield without debugging or inspecting the source code.

A good rule of thumb is that code should not run for more than 10-100 microseconds without hitting a potential yield point.

### References

- [Tokio docs on CPU-bound tasks and blocking code](https://docs.rs/tokio/latest/tokio/index.html#cpu-bound-tasks-and-blocking-code)
- [Blog post: What is Blocking?](https://ryhl.io/blog/async-what-is-blocking/)
- [Blog post: Using Rustlang’s Async Tokio Runtime for CPU-Bound Tasks](https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/)
