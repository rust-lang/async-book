# Applied: Build an Executor

Rust's `Future`s are lazy: they won't do anything unless actively driven to
completion. One way to drive a future to completion is to `.await` it inside
an `async` function, but that just pushes the problem one level up: who will
run the futures returned from the top-level `async` functions? The answer is
that we need a `Future` executor.

`Future` executors take a set of top-level `Future`s and run them to completion
by calling `poll` whenever the `Future` can make progress. Typically, an
executor will `poll` a future once to start off. When `Future`s indicate that
they are ready to make progress by calling `wake()`, they are placed back
onto a queue and `poll` is called again, repeating until the `Future` has
completed.

In this section, we'll write our own simple executor capable of running a large
number of top-level futures to completion concurrently.

For this example, we depend on the `futures` crate for the `ArcWake` trait,
which provides an easy way to construct a `Waker`. Edit `Cargo.toml` to add
a new dependency:

```toml
[package]
name = "timer_future"
version = "0.1.0"
authors = ["XYZ Author"]
edition = "2018"

[dependencies]
futures = "0.3"
```

Next, we need the following imports at the top of `src/main.rs`:

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:imports}}
```

Our executor will work by sending tasks to run over a channel. The executor
will pull events off of the channel and run them. When a task is ready to
do more work (is awoken), it can schedule itself to be polled again by
putting itself back onto the channel.

In this design, the executor itself just needs the receiving end of the task
channel. The user will get a sending end so that they can spawn new futures.
Tasks themselves are just futures that can reschedule themselves, so we'll
store them as a future paired with a sender that the task can use to requeue
itself.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_decl}}
```

Let's also add a method to spawner to make it easy to spawn new futures.
This method will take a future type, box it, and create a new `Arc<Task>` with
it inside which can be enqueued onto the executor.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:spawn_fn}}
```

To poll futures, we'll need to create a `Waker`.
As discussed in the [task wakeups section], `Waker`s are responsible
for scheduling a task to be polled again once `wake` is called. Remember that
`Waker`s tell the executor exactly which task has become ready, allowing
them to poll just the futures that are ready to make progress. The easiest way
to create a new `Waker` is by implementing the `ArcWake` trait and then using
the `waker_ref` or `.into_waker()` functions to turn an `Arc<impl ArcWake>`
into a `Waker`. Let's implement `ArcWake` for our tasks to allow them to be
turned into `Waker`s and awoken:

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:arcwake_for_task}}
```

When a `Waker` is created from an `Arc<Task>`, calling `wake()` on it will
cause a copy of the `Arc` to be sent onto the task channel. Our executor then
needs to pick up the task and poll it. Let's implement that:

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_run}}
```

Congratulations! We now have a working futures executor. We can even use it
to run `async/.await` code and custom futures, such as the `TimerFuture` we
wrote earlier:

```rust,edition2018,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:main}}
```

[task wakeups section]: ./03_wakeups.md
