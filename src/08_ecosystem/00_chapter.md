# The Async Ecosystem
Rust currently provides only the bare essentials for writing async code.
Importantly, executors, tasks, reactors, combinators, and low-level I/O futures and traits
are not yet provided in the standard library. In the meantime,
community-provided async ecosystems fill in these gaps.

The Async Foundations Team is interested in extending examples in the Async Book to cover multiple runtimes.
If you're interested in contributing to this project, please reach out to us on
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/201246-wg-async-foundations.2Fbook).

## Async Runtimes
Async runtimes are libraries used for executing async applications.
Runtimes usually bundle together a *reactor* with one or more *executors*.
Reactors provide subscription mechanisms for external events, like async I/O, interprocess communication, and timers.
In an async runtime, subscribers are typically futures representing low-level I/O operations.
Executors handle the scheduling and execution of tasks.
They keep track of running and suspended tasks, poll futures to completion, and wake tasks when they can make progress.
The word "executor" is frequently used interchangeably with "runtime".
Here, we use the word "ecosystem" to describe a runtime bundled with compatible traits and features.

## Community-Provided Async Crates

### The Futures Crate
The [`futures` crate](https://docs.rs/futures/) contains traits and functions useful for writing async code.
This includes the `Stream`, `Sink`, `AsyncRead`, and `AsyncWrite` traits, and utilities such as combinators.
These utilities and traits may eventually become part of the standard library.

`futures` has its own executor, but not its own reactor, so it does not support execution of async I/O or timer futures.
For this reason, it's not considered a full runtime.
A common choice is to use utilities from `futures` with an executor from another crate.

### Popular Async Runtimes
There is no asynchronous runtime in the standard library, and none are officially recommended.
The following crates provide popular runtimes.
- [Tokio](https://docs.rs/tokio/): A popular async ecosystem with HTTP, gRPC, and tracing frameworks.
- [async-std](https://docs.rs/async-std/): A crate that provides asynchronous counterparts to standard library components.
- [smol](https://docs.rs/smol/): A small, simplified async runtime.
Provides the `Async` trait that can be used to wrap structs like `UnixStream` or `TcpListener`.
- [fuchsia-async](https://fuchsia.googlesource.com/fuchsia/+/master/src/lib/fuchsia-async/):
An executor for use in the Fuchsia OS.

## Determining Ecosystem Compatibility
Not all async applications, frameworks, and libraries are compatible with each other, or with every OS or platform.
Most async code can be used with any ecosystem, but some frameworks and libraries require the use of a specific ecosystem.
Ecosystem constraints are not always documented, but there are several rules of thumb to determine
whether a library, trait, or function depends on a specific ecosystem.

Any async code that interacts with async I/O, timers, interprocess communication, or tasks
generally depends on a specific async executor or reactor.
All other async code, such as async expressions, combinators, synchronization types, and streams
are usually ecosystem independent, provided that any nested futures are also ecosystem independent.
Before beginning a project, it's recommended to research relevant async frameworks and libraries to ensure
compatibility with your chosen runtime and with each other.

Notably, `Tokio` uses the `mio` reactor and defines its own versions of async I/O traits,
including `AsyncRead` and `AsyncWrite`.
On its own, it's not compatible with `async-std` and `smol`,
which rely on the [`async-executor` crate](https://docs.rs/async-executor), and the `AsyncRead` and `AsyncWrite`
traits defined in `futures`.

Conflicting runtime requirements can sometimes be resolved by compatibility layers
that allow you to call code written for one runtime within another.
For example, the [`async_compat` crate](https://docs.rs/async_compat) provides a compatibility layer between
`Tokio` and other runtimes.

Libraries exposing async APIs should not depend on a specific executor or reactor,
unless they need to spawn tasks or define their own async I/O or timer futures.
Ideally, only binaries should be responsible for scheduling and running tasks.

## Single Threaded vs Multi-Threaded Executors
Async executors can be single-threaded or multi-threaded.
For example, the `async-executor` crate has both a single-threaded `LocalExecutor` and a multi-threaded `Executor`.

A multi-threaded executor makes progress on several tasks simultaneously.
It can speed up the execution greatly for workloads with many tasks,
but synchronizing data between tasks is usually more expensive.
It is recommended to measure performance for your application
when you are choosing between a single- and a multi-threaded runtime.

Tasks can either be run on the thread that created them or on a separate thread.
Async runtimes often provide functionality for spawning tasks onto separate threads.
Even if tasks are executed on separate threads, they should still be non-blocking.
In order to schedule tasks on a multi-threaded executor, they must also be `Send`.
Some runtimes provide functions for spawning non-`Send` tasks,
which ensures every task is executed on the thread that spawned it.
They may also provide functions for spawning blocking tasks onto dedicated threads,
which is useful for running blocking synchronous code from other libraries.
