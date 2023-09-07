# Why Async?

We all love how Rust empowers us to write fast, safe software.
But how does asynchronous programming fit into this vision?

Asynchronous programming, or async for short, is a _concurrent programming model_
supported by an increasing number of programming languages.
It lets you run a large number of concurrent
tasks on a small number of OS threads, while preserving much of the
look and feel of ordinary synchronous programming, through the
`async/await` syntax.

## Async vs other concurrency models

Concurrent programming is less mature and "standardized" than
regular, sequential programming. As a result, we express concurrency
differently depending on which concurrent programming model
the language is supporting.

A brief overview of the most popular concurrency models can help
you understand how asynchronous programming fits within the broader
field of concurrent programming:

- **Processes** are the simplest way to write concurrent programs. Processes
  launched this way cannot read or write to each other's virtual memory. This
  makes it harder for processes to share state; they have to use explicit IPC
  methods, e.g. pipes, FIFOS, shared memory, and semaphores.

  In Rust, the [`std::process`](https://doc.rust-lang.org/std/process/index.html)
  provides functionality for spawning and interacting with child processes.


- **OS threads** are logical flows of execution that run in the context of a
  single process. Multiple threads of execution run concurrently in a single
  process, and are scheduled by the host operating system kernel (this is an
  instance of _preemptive multi-tasking_, i.e. control over the multi-tasking
  lies with the kernel, not the scheduled threads). Each thread has its own
  thread context, but shares the virtual address space of the parent process. 

  Threads usually don't require any changes to the programming model, which
  makes it very easy to express concurrency. However, synchronizing between
  threads can be difficult, and the performance overhead is large. Thread pools
  can mitigate some of these costs, but not enough to support massive IO-bound 
  workloads.

  In Rust, thread management primitives are supplied by the 
  [`std::thread`](https://doc.rust-lang.org/std/thread/) module.

- **Event-driven programming** is a programming model where the control flow of
  the program is determined by events. Registered events trigger matching 
  _callback functions_ when the event occurs. This model can be very performant,
  but tends to result in a complex, verbose, "non-linear" control flow. Data 
  flow and error propagation is often hard to follow.

  This programming model is not supported in the standard library, but there are
  crates that provide support for it, for example
  [cross_town crate](https://docs.rs/crosstown_bus/latest/crosstown_bus/), which
  provides an Event Bus for building event-driven systems.

- **Coroutines**, are like threads, except that they provide _cooperative multi-
  tasking_. Coroutines voluntarily _yield_ control periodically, when idle, or
  when running blocking IO to allow other coroutines to run.

  Coroutines usually don't require changes to the programming model, which makes
  them easy to use. Like async, they can also support a large number of tasks.
  However, they abstract away low-level details that are important for systems
  programming and custom runtime implementors.

- **The actor model** divides all concurrent computation into units called
  actors, which communicate through fallible message passing, much like
  in distributed systems. The actor model can be efficiently implemented, but it
  leaves many practical issues unanswered, such as flow control and retry logic.

Asynchronous programming allows highly performant implementations that are
suitable for low-level languages like Rust, while providing most of the
ergonomic benefits of threads and coroutines.

## Async in Rust vs other languages

Although asynchronous programming is supported in many languages, some details
vary across implementations.

The fundamental traits, types and functions, such as the
[`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) trait are
provided by the standard library. More utility, types, macros and functions
for async programming are provided by the
[futures crate](https://docs.rs/futures/latest/futures/).

Primitives used for async programming include:

- **Futures** are single eventual values produced by asynchronous computations.
  A future is a value that might not have finished computing yet. A thread can
  continue doing useful work while it waits for the value to become available.

- **Streams** are a series of values produced asynchronously. An object with the
  [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html)
  trait asynchronously produces a sequence of values.

- **Sinks** values into which other values can be sent asynchronously. Sinks
  include the sending side of channels, sockets, and pipes.

- **Executors** are responsible for running asynchronous tasks. Executors spawn
  futures as tasks. Most of the time they are executed on a
  [_thread pool_](https://docs.rs/futures/latest/futures/executor/struct.ThreadPool.html).
  A small set of worker threads can handle a large set of spawned tasks. It is
  also possible to run a task within a single thread via the
  [`LocalPool`](https://docs.rs/futures/latest/futures/executor/struct.LocalPool.html)
  executor, which should be used for running I/O-bound tasks that do relatively
  little work between I/O operations.

Rust's implementation of async differs from most languages in a few ways:

- **Futures are inert** in Rust and make progress only when polled. Dropping a
  future stops it from making further progress.

- **Async is zero-cost** in Rust, which means that you only pay for what you use.
  Specifically, you can use async without heap allocations and dynamic dispatch,
  which is great for performance!
  This also lets you use async in constrained environments, such as embedded systems.

- **No built-in runtime** is provided by Rust. Instead, runtimes are provided by
  community maintained crates.

- **Both single- and multithreaded** runtimes are available in Rust, which have
  different strengths and weaknesses.

## Async vs threads in Rust

The primary alternative to async in Rust is using OS threads, either directly
through [`std::thread`](https://doc.rust-lang.org/std/thread/) or indirectly
through a [thread pool](https://docs.rs/threadpool/latest/threadpool/).
Migrating from threads to async or vice versa typically requires major
refactoring work, both in terms of implementation and (if you are building a
library) any exposed public interfaces. As such, picking the model that suits
your needs early can save a lot of development time.

**OS threads** are only suitable for running a small number of tasks
concurrently, since threads come with CPU and memory overhead. Spawning and
switching between threads is quite expensive as even idle threads consume system
resources. A thread pool library can help mitigate some of these costs, but not
all. However, threads let you reuse existing synchronous code without significant
code changes—no particular programming model is required. In some operating
systems, you can also change the priority of a thread, which is useful for 
drivers and other latency sensitive applications.

**Async** provides significantly reduced CPU and memory overhead, especially for
workloads with a large amount of IO-bound tasks, such as servers and databases.
All else equal, you can have orders of magnitude more tasks than OS threads,
because an async runtime uses a small amount of (expensive) threads to handle
a large amount of (cheap) tasks. However, async Rust results in larger binary
blobs due to the state machines generated from async functions and since each
executable bundles an async runtime.

On a last note, asynchronous programming is not _better_ than threads, but
different. If you don't need async for performance reasons, threads can often be
the simpler alternative.

### Example: Concurrent downloading

In this example our goal is to download two web pages concurrently.  In a
typical threaded application we need to spawn threads to achieve concurrency:

```rust,ignore
{{#include ../../examples/01_02_why_async/src/lib.rs:get_two_sites}}
```

However, downloading a web page is a small task; creating a thread for such a
small amount of work is quite wasteful. For a larger application, it can easily
become a bottleneck. In async Rust, we can run these tasks concurrently without
extra threads:

```rust,ignore
{{#include ../../examples/01_02_why_async/src/lib.rs:get_two_sites_async}}
```

Here, no extra threads are created. Additionally, all function calls are
statically dispatched, and there are no heap allocations! However, we need to
write the code to be asynchronous in the first place, which this book will help
you achieve.

## Custom concurrency models in Rust

On a last note, Rust doesn't force you to choose between threads and async.
You can use both models within the same application, which can be useful when
you have mixed threaded and async dependencies. In fact, you can even use a
different concurrency model altogether, such as event-driven programming, as
long as you find a library that implements it.
