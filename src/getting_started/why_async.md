## Why Async?

We all love how Rust allows us to write fast, safe software. But why write
asynchronous code?

Asynchonous code allows us to run multiple tasks concurrently on the same OS
thread. In a typical threaded application, if you wanted to download two
different webpages at the same time, you would spread the work across two
different threads, like this:

```rust
fn get_two_sites() {
    // Spawn two threads to do work.
    let thread_one = thread::spawn(|| download("https:://www.foo.com"));
    let thread_two = thread::spawn(|| download("https:://www.bar.com"));

    // Wait for both threads to complete.
    thread_one.join();
    thread_two.join();
}
```

This works fine for many applications-- after, all threads were designed
to do just this: run multiple different tasks at once. However, they also
come with some limitations. There's a lot of overhead involved in the
process of switching between different threads and sharing data between
threads. Even a thread which just sits and does nothing uses up valuable
system resources. These are the costs that asynchronous code is designed
to eliminate. We can rewrite the function above using Rust's
`async`/`await!` notation, which will allow us to run multiple tasks at
once without creating multiple threads:

```rust
async fn get_two_sites() {
    // Create a two different "futures" which, when run to completion,
    // will asynchronously download the webpages.
    let future_one = download_async("https:://www.foo.com");
    let future_two = download_async("https:://www.bar.com");

    // Run both futures to completion at the same time.
    join!(future_one, future_two);
}
```

Overall, asynchronous applications have the potential to be much faster and
use fewer resources than a corresponding threaded implementation. However,
there is a cost. Threads are natively supported by the operating system,
and using them doesn't require any special programming model-- any function
can create a thread, and calling a function that uses threads is usually
just as easy as calling any normal function. However, asynchronous functions
require special support from the language or libraries in order to work.
In Rust, `async fn` creates an asynchronous function which, when called,
will return a future which needs to be run to completion in order for the
body of the function to execute.

It's important to remember that traditional threaded applications can be quite
effective, and that Rust's small memory footprint and predictability mean that
you can get far without ever using `async`. The increased complexity of the
asynchronous programming model isn't always worth it, and it's important to
consider whether your application would be better served by using a simpler
threaded model.
