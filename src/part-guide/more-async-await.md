# More async/await topics

## Unit tests

How to unit test async code? The issue is that you can only await from inside an async context, and unit tests in Rust are not async. Luckily, most runtimes provide a convenience attribute for tests similar to the one for `async main`. Using Tokio, it looks like this:

```rust,norun
#[tokio::test]
async fn test_something() {
  // Write a test here, including all the `await`s you like.
}
```

There are many ways to configure the test, see the [docs](https://docs.rs/tokio/latest/tokio/attr.test.html) for details.

There are some more advanced topics in testing async code (e.g., testing for race conditions, deadlock, etc.), and we'll cover some of those [later]() in this guide.


## Blocking and cancellation

Blocking and cancellation are important to keep in mind when programming with async Rust. These concepts are not localised to any particular feature or function, but are ubiquitous properties of the system which you must understand to write correct code.

### Blocking IO

We say a thread (note we're talking about OS threads here, not async tasks) is blocked when it can't make any progress. That's usually because it is waiting for the OS to complete a task on its behalf (usually I/O). Importantly, while a thread is blocked, the OS knows not to schedule it so that other threads can make progress. This is fine in a multithreaded program because it lets other threads make progress while the blocked thread is waiting. However, in an async program, there are other tasks which should be scheduled on the same OS thread, but the OS doesn't know about those and keeps the whole thread waiting. This means that rather than the single task waiting for its I/O to complete (which is fine), many tasks have to wait (which is not fine).

We’ll talk soon about non-blocking/async I/O. For now, just know that non-blocking I/O is I/O that the async runtime is aware of, so only the current task waits; the thread itself is not blocked. It is very important to only use non-blocking I/O from an async task, never blocking I/O (which is the only kind provided in Rust's standard library).

### Blocking computation

You can also block the thread by doing computation (this is not quite the same as blocking I/O, since the OS is not involved, but the effect is similar). If you have a long-running computation (with or without blocking I/O) without yielding control to the runtime, then that task will never give the runtime's scheduler a chance to schedule other tasks. Remember that async programming uses cooperative multitasking. Here a task is not cooperating, so other tasks won't get a chance to get work done. We'll discuss ways to mitigate this later.

There are many other ways to block a whole thread, and we'll come back to blocking several times in this guide.

### Cancellation

Cancellation means stopping a future (or task) from executing. Since in Rust (and in contrast to many other async/await systems), futures must be driven forward by an external force (like the async runtime), if a future is no longer driven forward then it will not execute any more. If a future is dropped (remember, a future is just a plain old Rust object), then it can never make any more progress and is canceled.

Cancellation can be initiated in a few ways:

- By simply dropping a future (if you own it).
- Calling [`abort`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort) on a task's 'JoinHandle' (or an `AbortHandle`).
- Via a [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) (which requires the future being canceled to notice the token and cooperatively cancel itself).
- Implicitly, by a function or macro like [`select`](https://docs.rs/tokio/latest/tokio/macro.select.html).

The middle two are specific to Tokio, though most runtimes provide similar facilities. Using a `CancellationToken` requires cooperation of the future being canceled, but the others do not. In these other cases, the canceled future will get no notification of cancellation and no opportunity to clean up (besides its destructor). Note that even if a future has a cancellation token, it can still be canceled via the other methods which won't trigger the cancellation token.

From the perspective of writing async code (in async functions, blocks, futures, etc.), the code might stop executing at any `await` (including hidden ones in macros) and never start again. In order for your code to be correct (specifically to be *cancellation safe*), it must work correctly whether it completes normally or whether it terminates at any await point[^cfThreads].

```rust,norun
async fn some_function(input: Option<Input>) {
    let Some(input) = input else {
        return;           // Might terminate here (`return`).
    };

    let x = foo(input)?;  // Might terminate here (`?`).

    let y = bar(x).await; // Might terminate here (`await`).

    // ...

    //                       Might terminate here (implicit return).
}
```

An example of how this can go wrong is if an async function reads data into an internal buffer, then awaits the next datum. If reading the data is destructive (i.e., cannot be re-read from the original source) and the async function is canceled, then the internal buffer will be dropped, and the data in it will be lost. It is important to consider how a future and any data it touches will be impacted by canceling the future, restarting the future, or starting a new future which touches the same data.

We'll be coming back to cancellation and cancellation safety a few times in this guide, and there is a whole [chapter]() on the topic in the reference section.

[^cfThreads]: It is interesting to compare cancellation in async programming with canceling threads. Canceling a thread is possible (e.g., using `pthread_cancel` in C, there is no direct way to do this in Rust), but it is almost always a very, very bad idea since the thread being canceled can terminate anywhere. In contrast, canceling an async task can only happen at an await point. As a consequence, it is very rare to cancel an OS thread without terminating the whole process and so as a programmer, you generally don't worry about this happening. In async Rust however, cancellation is definitely something which *can* happen. We'll be discussing how to deal with that as we go along.

## Async blocks

A regular block (`{ ... }`) groups code together in the source and creates a scope of encapsulation for names. At runtime, the block is executed in order and evaluates to the value of its last expression (or the unit type (`()`) if there is no trailing expression).

Similarly to async functions, an async block is a deferred version of a regular block. An async block scopes code and names together, but at runtime it is not immediately executed and evaluates to a future. To execute the block and obtain the result, it must be `await`ed. E.g.:

```rust,norun
let s1 = {
    let a = 42;
    format!("The answer is {a}")
};

let s2 = async {
    let q = question().await;
    format!("The question is {q}")
};
```

If we were to execute this snippet, `s1` would be a string which could be printed, but `s2` would be a future; `question()` would not have been called. To print `s2`, we first have to `s2.await`.

An async block is the simplest way to start an async context and create a future. It is commonly used to create small futures which are only used in one place.

Unfortunately, control flow with async blocks is a little quirky. Because an async block creates a future rather than straightforwardly executing, it behaves more like a function than a regular block with respect to control flow. `break` and `continue` cannot go 'through' an async block like they can with regular blocks; instead you have to use `return`:

```rust,norun
loop {
    {
        if ... {
            // ok
            continue;
        }
    }

    async {
        if ... {
            // not ok
            // continue;

            // ok - continues with the next execution of the `loop`, though note that if there was
            // code in the loop after the async block that would be executed.
            return;
        }
    }.await
}
```

To implement `break` you would need to test the value of the block (a common idiom is to use [`ControlFlow`](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html) for the value of the block, which also allows use of `?`).

Likewise, `?` inside an async block will terminate execution of the future in the presence of an error, causing the `await`ed block to take the value of the error, but won't exit the surrounding function (like `?` in a regular block would). You'll need another `?` after `await` for that:

```rust,norun
async {
    let x = foo()?;   // This `?` only exits the async block, not the surrounding function.
    consume(x);
    Ok(())
}.await?
```

Annoyingly, this often confuses the compiler since (unlike functions) the 'return' type of an async block is not explicitly stated. You'll probably need to add some type annotations on variables or use turbofished types to make this work, e.g., `Ok::<_, MyError>(())` instead of `Ok(())` in the above example.

A function which returns an async block is pretty similar to an async function. Writing `async fn foo() -> ... { ... }` is roughly equivalent to `fn foo() -> ... { async { ... } }`. In fact, from the caller's perspective they are equivalent, and changing from one form to the other is not a breaking change. Furthermore, you can override one with the other when implementing an async trait (see below). However, you do have to adjust the type, making the `Future` explicit in the async block version: `async fn foo() -> Foo` becomes `fn foo() -> impl Future<Output = Foo>` (you might also need to make other bounds explicit, e.g., `Send` and `'static`).

You would usually prefer the async function version since it is simpler and clearer. However, the async block version is more flexible since you can execute some code when the function is called (by writing it outside the async block) and some code when the result is awaited (the code inside the async block).

## Async closures

If a closure needs to await an async operation in its body, it has to return a future (just like any async functions). A simple way is to return an async block, like `|| async {}`. This often works if the returned future doesn't reference data that the closure captures. For example:

```rust,norun
#[tokio::main]
async fn main() {
    let mut logs: Vec<String> = vec![];

    let f = || {
        logs.push("".to_owned());
        async {}
    };
}
```

The closure saves a log message before any asynchronous work inside the async block. However, if the log line can only be produced by an async function call, that call must happen inside the async block, thus requiring `logs.push` to be placed there as well.

```rust,norun
#[tokio::main]
async fn main() {
    let mut logs: Vec<String> = vec![];

    let f = || async {
        let msg = get_message().await;
        logs.push(msg);
    };
    // error: captured variable cannot escape `FnMut` closure body
}

async fn get_message() -> String {
    todo!()
}
```

This example won't compile. The variable `logs` is captured by the closure and only available during its execution, but the returned future needs to reference this capture when it is later awaited, meaning the captured value has to outlive the closure for this to be valid.

It is also not possible to express signature of higher-ranked async functions using Higher-Ranked Trait Bounds (HRTBs). 

```rust,norun
#[tokio::main]
async fn main() {
    run(do_something);
    // error: implementation of `FnMut` is not general enough
}

async fn run<F, Fut>(f: F)
where
    F: for<'a> FnMut(&'a str) -> Fut, 
    Fut: Future<Output = ()>,
{}

async fn do_something(s: &str) {}
```

`F: for<'a> FnMut(&'a str) -> Fut` means for every lifetime `'a`, `F` is a closure that accepts a `&str` living for `'a`. In other words, `F` is higher-ranked over the lifetime of its input.

This code fails to compile because the inferred type of `F` isn't general enough. In particular, `F`'s returned type `Fut` is inferred to be the future produced by `do_something` due to its use in `main`. That future must capture the lifetime of its `s: &str` input in order to use it, but `for<'a>` needs `F` to work for any lifetime, not just the one tied to that particular input. Moreover, since `'a` in the HRTB isn't a generic parameter, it can't be named in the bound on `Fut`, so there's no way to express that `Fut` may capture this lifetime.

As we have seen, and to quote [the RFC for async closures](https://rust-lang.github.io/rfcs/3668-async-closures.html#motivation), two major limitations when using closures in async code includes:

> 1. That closures cannot return futures that borrow from the closure captures.
> 2. The inability to express higher-ranked async function signatures.

Async closure support was added to address both of these problems. An async closure is declared by prefixing a closure with the `async` keyword, like `async || {}` (as opposed to `|| async {}`). Like regular closures, they can capture variables from their environment. However, async closures also return a value of an anonymous future type, which can itself can borrow data from the async closure.

```rust,norun
let mut logs: Vec<String> = vec![];

let f = async || {
	//  ^-------
	let msg = get_message().await;
	logs.push(msg);
};
```

Any variables `f` captures live until it is dropped. Calling `f` returns a future that borrows the closure and its captures until the future is dropped. An `move` async closure owns the its captured data. Below, `f` takes ownership of `logs`, and calls to `f` gives back a future that borrow `f` and its owned data.

```rust,norun
let mut logs: Vec<String> = vec![];

let f = async move || {
	//        ^---
	let msg = get_message().await;
	logs.push(msg);
};
```

The standard library also provides [`AsyncFnOnce`](https://doc.rust-lang.org/std/ops/trait.AsyncFnOnce.html), [`AsyncFnMut`](https://doc.rust-lang.org/std/ops/trait.AsyncFnMut.html), and [`AsyncFn`](https://doc.rust-lang.org/std/ops/trait.AsyncFn.html) traits, similar to the `Fn` family of traits. You can use these traits to express trait bounds as they relate to async closures.

For instance, we can use `AsyncFnMut` to bound the generic type `F` of the `run` function. It compiles successfully because async closures can be higher-ranked over their argument lifetimes.

```rust,norun
#[tokio::main]
async fn main() {
    run(do_something);
}

async fn run<F>(f: F)
where
    F: for<'a> AsyncFnMut(&'a str),
{}

async fn do_something(s: &str) {};
```

It's worth understanding how each trait handles the closure's captures differently. Calling an `AsyncFn` or `AsyncFnMut` closure only needs a reference (shared or exclusive, respectively) to the closure itself, so the returned future can still borrow from the closure's data. Invoking an `AsyncFnOnce` closure, however, consumes it, so the closure value no longer exists for the resulting future to borrow from. As a result, Rust generates a new future type, and the captures are moved into this future instead. This new future behaves the same way it would if it were called by reference.

To ensure compatibility with other callable types, `AsyncFn*() -> T` is automatically implemented for any type that implements `Fn*() -> Fut`, where `Fut: Future<Output = T>`.

```rust,norun
#[tokio::main]
async fn main() {
    accept_async_fn(async || {});
    accept_async_fn(|| async {});
    accept_async_fn(foo);
    accept_async_fn(|| Box::pin(async {}));
    accept_async_fn(Box::new(|| Box::pin(async {})));
}

async fn foo() {}

fn accept_async_fn(f: impl AsyncFn()) {}
```

 

## Lifetimes and borrowing

- Mentioned the static lifetime above
- Lifetime bounds on futures (`Future + '_`, etc.)
- Borrowing across await points
- I don't know, I'm sure there are more lifetime issues with async functions ...


## `Send + 'static` bounds on futures

- Why they're there, multi-threaded runtimes
- spawn local to avoid them
- What makes an async fn `Send + 'static` and how to fix bugs with it


## Async traits

- syntax
  - The `Send + 'static` issue and working around it
    - trait_variant
    - explicit future
    - return type notation (https://blog.rust-lang.org/inside-rust/2024/09/26/rtn-call-for-testing.html)
- overriding
  - future vs async notation for methods
- object safety
- capture rules (https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html)
- history and async-trait crate


## Recursion

- Allowed (relatively new), but requires some explicit boxing
  - forward reference to futures, pinning
  - https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
  - https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#support-for-recursion-in-async-fn
  - async-recursion macro (https://docs.rs/async-recursion/latest/async_recursion/)

