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
error[E0733]: recursion in an `async fn` requires boxing
 --> src/lib.rs:1:22
  |
1 | async fn recursive() {
  |                      ^ an `async fn` cannot invoke itself directly
  |
  = note: a recursive `async fn` must be rewritten to return a boxed future.
```

In order to allow this, we have to introduce an indirection using `Box`.
Unfortunately, compiler limitations mean that just wrapping the calls to
`recursive()` in `Box::pin` isn't enough. To make this work, we have
to make `recursive` into a non-`async` function which returns a `.boxed()`
`async` block:

```rust,edition2018
{{#include ../../examples/07_05_recursion/src/lib.rs:example}}
```
