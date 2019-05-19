# The `Future` Trait

The `Future` trait is at the center of asynchronous programming in Rust.
A `Future` is an asynchronous computation that can produce a value
(although that value may be empty, e.g. `()`). A *simplified* version of
the future trait might look something like this:

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

Futures can be advanced by calling the `poll` function, which will drive the
future as far towards completion as possible. If the future completes, it
returns `Poll::Ready(result)`. If the future is not able to complete yet, it
returns `Poll::Pending` and arranges for the `wake()` function to be called
when the `Future` is ready to make more progress. When `wake()` is called, the
executor driving the `Future` will call `poll` again so that the `Future` can
make more progress.

Without `wake()`, the executor would have no way of knowing when a particular
future could make progress, and would have to be constantly polling every
future. With `wake()`, the executor knows exactly which futures are ready to
be `poll`ed.

For example, consider the case where we want to read from a socket that may
or may not have data available already. If there is data, we can read it
in and return `Poll::Ready(data)`, but if no data is ready, our future is
blocked and can no longer make progress. When no data is available, we
must register `wake` to be called when data becomes ready on the socket,
which will tell the executor that our future is ready to make progress.
A simple `SocketRead` future might look something like this:

```rust
struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // The socket has data-- read it into a buffer and return it.
            Poll::Ready(self.socket.read_buf())
        } else {
            // The socket does not yet have data.
            //
            // Arrange for `wake` to be called once data is available.
            // When data becomes available, `wake` will be called, and the
            // user of this `Future` will know to call `poll` again and
            // receive data.
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
```

This model of `Future`s allows for composing together multiple asynchronous
operations without needing intermediate allocations. Running multiple futures
at once or chaining futures together can be implemented via allocation-free
state machines, like this:

```rust
/// A SimpleFuture that runs two other futures to completion concurrently.
///
/// Concurrency is achieved via the fact that calls to `poll` each future
/// may be interleaved, allowing each future to advance itself at its own pace.
struct Join2 {
    // Each field may contain a future that should be run to completion.
    // If the future has already completed, the field is set to `None`.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl SimpleFuture for Join2 {
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // Attempt to complete future `a`.
        let finished_a = match &mut self.a {
            Some(a) => {
                match a.poll(wake) {
                    Poll::Ready(()) => true,
                    Poll::Pending => false,
                }
            }
            None => true,
        };
        if finished_a { self.a.take() }

        // Attempt to complete future `b`.
        let finished_b = match &mut self.b {
            Some(b) => {
                match b.poll(wake) {
                    Poll::Ready(()) => true,
                    Poll::Pending => false,
                }
            }
            None => true,
        };
        if finished_b { self.b.take() }

        if finished_a && finished_b {
            // Both futures have completed-- we can return successfully
            Poll::Ready(())
        } else {
            // One or both futures still have work to do, and will call
            // `wake()` when progress can be made.
            Poll::Pending
        }
    }
}
```

This shows how multiple futures can be run simultaneously without needing
separate allocations, allowing for more efficient asynchronous programs.
Similarly, multiple sequential futures can be run one after another, like this:

```rust
/// A SimpleFuture that runs two futures to completion, one after another.
//
// Note: for the purposes of this simple example, `AndThenFut` assumes both
// the first and second futures are available at creation-time. The real
// `AndThen` combinator allows creating the second future based on the output
// of the first future, like `get_breakfast.and_then(|food| eat(food))`.
enum AndThenFut {
    first: Option<FutureA>,
    second: FutureB,
}

impl SimpleFuture for AndThenFut {
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // We've completed the first future-- remove it and start on
                // the second!
                Poll::Ready(()) => self.first.take(),
                // We couldn't yet complete the first future.
                Poll::Pending => return Poll::Pending,
            }
        }
        // Now that the first future is done, attempt to complete the second.
        second.poll(wake)
    }
}
```

These examples show how the `Future` trait can be used to express asynchronous
control flow without requiring multiple allocated objects and deeply nested
callbacks. With the basic control-flow out of the way, let's talk about the
real `Future` trait and how it is different.

```rust
trait Future {
    type Output;
    fn poll(
        // note the change from `&mut self` to `Pin<&mut Self>`
        self: Pin<&mut Self>,
        cx: &mut Context<'_>, // note the change from `wake: fn()`
    ) -> Poll<Self::Output>;
}
```

The first change you'll notice is that our `self` type is no longer `&mut self`,
but has changed to `Pin<&mut Self>`. We'll talk more about pinning in [a later
section][pinning], but for now know that it allows us to create futures that
are immovable. Immovable objects can store pointers between their fields,
e.g. `struct MyFut { a: i32, ptr_to_a: *const i32 }`. This feature is necessary
in order to enable async/await.

Secondly, `wake: fn()` has changed to `Context<'_>`. In `SimpleFuture`, we used
a call to a function pointer (`fn()`) to tell the future executor that the
future in question should be polled. However, since `fn()` is zero-sized, it
can't store any data about *which* `Future` called `wake`.
In a real-world scenario, a complex application like a web server may have
thousands of different connections whose wakeups should all be
managed separately. This is where `Context<'_>` and `Waker`
come in.

[pinning]: ../pinning/chapter.md
