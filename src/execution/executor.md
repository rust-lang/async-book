# Applied: Build an Executor

`Future`s are lazy and must be actively driven to completion in order to do
anything. A common way to drive a future to completion is to `await!` it inside
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

For this one, we're going to have to include the `futures` crate in order to
get the `FutureObj` type, which is a dynamically-dispatched `Future`, similar
to `Box<dyn Future<Output = T>>`. `Cargo.toml` should look something like this:

```toml
[package]
name = "xyz"
version = "0.1.0"
authors = ["XYZ Author"]
edition = "2018"

[dependencies]
futures-preview = "0.3.0-alpha.9"
```

Next, we need the following imports at the top of `src/main.rs`:

```rust
#![feature(arbitrary_self_types, async_await, await_macro, futures_api, pin)]

use {
    futures::future::FutureObj,
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{
            local_waker_from_nonlocal,
            Poll, Wake,
        },
    },
};
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

```rust
/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled using a channel.
struct Task {
    // In-progress future that should be pushed to completion
    //
    // The `Mutex` is not necessary for correctness, since we only have
    // one thread executing tasks at once. However, `rustc` isn't smart
    // enough to know that `future` is only mutated from one thread,
    // so we use it in order to provide safety. A production executor would
    // not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<FutureObj<'static, ()>>>,

    // Handle to spawn tasks onto the task queue
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender})
}
```

Let's also add a method to spawner to make it easy to spawn new futures.
This method will take a future type, box it and put it in a FutureObj,
and create a new `Arc<Task>` with it inside which can be enqueued onto the
executor.

```rust
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future_obj = FutureObj::new(Box::new(future));
        let task = Arc::new(Task {
            future: Mutex::new(Some(future_obj)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
```

In order poll futures, we'll also need to create a `LocalWaker` to provide to
poll. As discussed in the [task wakeups section], `LocalWaker`s are responsible
for scheduling a task to be polled again once `wake` is called. Remember that
`LocalWaker`s tell the executor exactly which task has become ready, allowing
them to poll just the futures that are ready to make progress. The easiest way
to create a new `LocalWaker` is by implementing the `Wake` trait and then using
the `local_waker_from_nonlocal` or `local_waker` functions to turn a `Arc<T: Wake>`
into a `LocalWaker`. Let's implement `Wake` for our tasks to allow them to be
turned into `LocalWaker`s and awoken:

```rust
impl Wake for Task {
    fn wake(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}
```

When a `LocalWaker` is created from an `Arc<Task>`, calling `wake()` on it will
cause a copy of the `Arc` to be sent onto the task channel. Our executor then
needs to pick up the task and poll it. Let's implement that:

```rust
impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let lw = local_waker_from_nonlocal(task.clone());
                if let Poll::Pending = Pin::new(&mut future).poll(&lw) {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}
```

Congratulations! We now have a working futures executor. We can even use it
to run `async/await!` code and custom futures, such as the `TimerFuture` we
wrote earlier:

```rust
fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        await!(TimerFuture::new(Duration::new(2, 0)));
        println!("done!");
    });
    executor.run();
}
```

[task wakeups section]: TODO
