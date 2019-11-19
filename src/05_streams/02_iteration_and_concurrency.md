# Iteration and Concurrency

Similar to synchronous `Iterator`s, there are many different ways to iterate
over and process the values in a `Stream`. There are combinator-style methods
such as `map`, `filter`, and `fold`, and their early-exit-on-error cousins
`try_map`, `try_filter`, and `try_fold`.

Unfortunately, `for` loops are not usable with `Stream`s, but for
imperative-style code, `while let` and the `next`/`try_next` functions can
be used:

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:nexts}}
```

However, if we're just processing one element at a time, we're potentially
leaving behind opportunity for concurrency, which is, after all, why we're
writing async code in the first place. To process multiple items from a stream
concurrently, use the `for_each_concurrent` and `try_for_each_concurrent`
methods:

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:try_for_each_concurrent}}
```
