# `join!`

The `futures::join` macro makes it possible to wait for multiple different
futures to complete while executing them all concurrently.

# `join!`

When performing multiple asynchronous operations, it's tempting to simply
`.await` them in a series:

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:naiive}}
```

However, this will be slower than necessary, since it won't start trying to
`get_music` until after `get_book` has completed. In some other languages,
futures are ambiently run to completion, so two operations can be
run concurrently by first calling each `async fn` to start the futures, and
then awaiting them both:

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:other_langs}}
```

However, Rust futures won't do any work until they're actively `.await`ed.
This means that the two code snippets above will both run
`book_future` and `music_future` in series rather than running them
concurrently. To correctly run the two futures concurrently, use
`futures::join!`:

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:join}}
```

The value returned by `join!` is a tuple containing the output of each
`Future` passed in.

## `try_join!`

For futures which return `Result`, consider using `try_join!` rather than
`join!`. Since `join!` only completes once all subfutures have completed,
it'll continue processing other futures even after one of its subfutures
has returned an `Err`.

Unlike `join!`, `try_join!` will complete immediately if one of the subfutures
returns an error.

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join}}
```

Note that the futures passed to `try_join!` must all have the same error type.
Consider using the `.map_err(|e| ...)` and `.err_into()` functions from
`futures::future::TryFutureExt` to consolidate the error types:

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join_map_err}}
```
