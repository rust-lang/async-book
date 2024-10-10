# Recursion

Internally, `async fn` creates a state machine type containing each
sub-`Future` being `.await`ed. This makes recursive `async fn`s a little
tricky, since the resulting state machine type has to contain itself:

```rust,edition2018
# async fn step_one() { /* ... */ }
# async fn step_two() { /* ... */ }
# struct StepOne;
# struct StepTwo;
// This function:
async fn foo() {
    step_one().await;
    step_two().await;
}
// generates a type like this:
enum Foo {
    First(StepOne),
    Second(StepTwo),
}

// So this function:
async fn recursive() {
    recursive().await;
    recursive().await;
}

// generates a type like this:
enum Recursive {
    First(Recursive),
    Second(Recursive),
}
```

This won't work—we've created an infinitely-sized type!
The compiler will complain:

```
error[E0733]: recursion in an async fn requires boxing
 --> src/lib.rs:1:1
  |
1 | async fn recursive() {
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = note: a recursive `async fn` call must introduce indirection such as `Box::pin` to avoid an infinitely sized future
```

In order to allow this, we have to introduce an indirection using `Box`.

Prior to Rust 1.77, due to compiler limitations, just wrapping the calls to
`recursive()` in `Box::pin` isn't enough. To make this work, we have
to make `recursive` into a non-`async` function which returns a `.boxed()`
`async` block:

```rust,edition2018
{{#include ../../examples/07_05_recursion/src/lib.rs:example}}
```

In newer version of Rust, [that compiler limitation has been lifted].

Since Rust 1.77, support for recursion in `async fn` with allocation
indirection [becomes stable], so recursive calls are permitted so long as they
use some form of indirection to avoid an infinite size for the state of the
function.

This means that code like this now works:

```rust,edition2021
{{#include ../../examples/07_05_recursion/src/lib.rs:example_pinned}}
```

[becomes stable]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#support-for-recursion-in-async-fn
[that compiler limitation has been lifted]: https://github.com/rust-lang/rust/pull/117703/
