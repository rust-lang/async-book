# Task Wakeups with `Waker`

It's common that futures aren't able to complete the first time they are
`poll`ed. When this happens, the future needs to ensure that it is polled
again once it is ready to make more progress. This is done with the `Waker`
type.

Each time a future is polled, it is polled as part of a "task". Tasks are
the top-level futures that have been submitted to an executor.

`Waker` provides a `wake()` method that can be used to tell the executor that
the associated task should be awoken. When `wake()` is called, the executor
knows that the task associated with the `Waker` is ready to make progress, and
its future should be polled again.

`Waker` also implements `clone()` so that it can be copied around and stored.

Let's try implementing a simple timer future using `Waker`.

## Applied: Build a Timer

For the sake of the example, we'll just spin up a new thread when the timer
is created, sleep for the required time, and then signal the timer future
when the time window has elapsed.

Here are the imports we'll need to get started:

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:imports}}
```

Let's start by defining the future type itself. Our future needs a way for the
thread to communicate that the timer has elapsed and the future should complete.
We'll use a shared `Arc<Mutex<..>>` value to communicate between the thread and
the future.

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:timer_decl}}
```

Now, let's actually write the `Future` implementation!

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:future_for_timer}}
```

Pretty simple, right? If the thread has set `shared_state.completed = true`,
we're done! Otherwise, the timer either hasn't started yet, or it is running 
but hasn't completed. To figure this out we look at the `waker` field.

If we do not have `Waker`, the timer hasn't started yet and so we spawn the 
timer thread and set a `Waker`. If we do have a `Waker` it means the timer 
was started but hasn't completed yet. We clone the `Waker` for the current 
task and pass it to `shared_state.waker`  so that the thread can wake the 
task back up.

Importantly, we have to update the `Waker` every time the future is polled
because the future may have moved to a different task with a different
`Waker`. This will happen when futures are passed around between tasks after
being polled.

Finally, we need the API to actually construct the timer and start the thread:

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:timer_new}}
```

Woot! That's all we need to build a simple timer future. Now, if only we had
an executor to run the future on...
