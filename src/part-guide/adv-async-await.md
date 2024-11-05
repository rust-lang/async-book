# More async/await topics

## Unit tests

## Blocking and cancellation

- Two important concepts to be aware of early, we'll revisit in more detail as we go along
- Cancellation
  - How to do it
    - drop a future
    - cancellation token
    - abort functions
  - Why it matters, cancellation safety (forward ref)
- Blocking
  - IO and computation can block
  - why it's bad
  - how to deal is a forward ref to io chapter

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


## Async blocks and closures

- async block syntax
  - what it means
- using an async block in a function returning a future
  - subtype of async method
- closures
  - coming soon (https://github.com/rust-lang/rust/pull/132706, https://blog.rust-lang.org/inside-rust/2024/08/09/async-closures-call-for-testing.html)
  - async blocks in closures vs async closures
- errors in async blocks
  - https://rust-lang.github.io/async-book/07_workarounds/02_err_in_async_blocks.html

## Recursion

- Allowed (relatively new), but requires some explicit boxing
  - forward reference to futures, pinning
  - https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
  - https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#support-for-recursion-in-async-fn
  - async-recursion macro (https://docs.rs/async-recursion/latest/async_recursion/)


## Lifetimes and borrowing

- Mentioned the static lifetime above
- Lifetime bounds on futures (`Future + '_`, etc.)
- Borrowing across await points
- I don't know, I'm sure there are more lifetime issues with async functions ...
