# `async`/`await!` Primer

`async`/`await!` is Rust's built-in tool for writing asynchronous functions
that look like synchronous code. `async` transforms a block of code into a
state machine that implements a trait called `Future`. Whereas calling a
blocking function in a synchronous method would block the whole thread,
blocked `Future`s will yield control of the thread, allowing other
`Future`s to run.

To create an asynchronous function, you can use the `async fn` syntax:

```rust
async fn do_something() { ... }
```

The value returned by `async fn` is a `Future` that needs to be run on
an executor in order for anything to happen:

```rust
// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheudling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
```

Inside an `async fn`, you can use `await!` to wait for the completion of
another type that implements the `Future` trait, such as the output of
another `async fn`. Unlike `block_on`, `await!` doesn't block the current
thread, but instead asynchronously waits for the future to complete, allowing
other tasks to run if the future is currently unable to make progress.

For example, imagine that we have three `async fn`: `learn_song`, `sing_song`,
and `dance`:

```rust
async fn learn_song() -> Song { ... }
async fn sing_song(song: Song) { ... }
async fn dance() { ... }
```

One way to do learn, sing, and dance would be to block on each of these
individually:

```rust
fn main() {
  let song = block_on(learn_song());
  block_on(sing_song(song));
  block_on(dance);
}
```

However, we're not giving the best performance possible this way-- we're
only ever doing one thing at once! Clearly we have to learn the song before
we can sing it, but it's possible to dance at the same time as learning and
singing the song. To do this, we can create two separate `async fn` which
can be run concurrently:

```rust
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `await!` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = await!(learn_song());
    await!(sing_song(song));
}

async fn async_main() {
    let f1 = learn_and_sing(); 
    let f2 = dance();

    // `join!` is like `await!` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    join!(f1, f2) 
}

fn main() {
    block_on(async_main());
}
```

In this example, learning the song must happen before singing the song, but
both learning and singing can happen at the same time as dancing. If we used
`block_on(learn_song())` rather than `await!(learn_song())` in `learn_and_sing`,
the thread wouldn't be able to do anything else while `learn_song` was running.
This would make it impossible to dance at the same time. By `await!`ing
the `learn_song` future, we allow other tasks to take over the current thread
if `learn_song` is blocked. This makes it possible to run multiple futures
to completion concurrently on the same thread.

Now that you've learned the basics of `async`/`await!`, let's try out an
example.
