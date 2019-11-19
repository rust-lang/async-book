# `select!`

The `futures::select` macro runs multiple futures simultaneously, allowing
the user to respond as soon as any future completes.

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:example}}
```

The function above will run both `t1` and `t2` concurrently. When either
`t1` or `t2` finishes, the corresponding handler will call `println!`, and
the function will end without completing the remaining task.

The basic syntax for `select` is `<pattern> = <expression> => <code>,`,
repeated for as many futures as you would like to `select` over.

## `default => ...` and `complete => ...`

`select` also supports `default` and `complete` branches.

A `default` branch will run if none of the futures being `select`ed
over are yet complete. A `select` with a `default` branch will
therefore always return immediately, since `default` will be run
if none of the other futures are ready.

`complete` branches can be used to handle the case where all futures
being `select`ed over have completed and will no longer make progress.
This is often handy when looping over a `select!`.

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:default_and_complete}}
```

## Interaction with `Unpin` and `FusedFuture`

One thing you may have noticed in the first example above is that we
had to call `.fuse()` on the futures returned by the two `async fn`s,
as well as pinning them with `pin_mut`. Both of these calls are necessary
because the futures used in `select` must implement both the `Unpin`
trait and the `FusedFuture` trait.

`Unpin` is necessary because the futures used by `select` are not
taken by value, but by mutable reference. By not taking ownership
of the future, uncompleted futures can be used again after the
call to `select`.

Similarly, the `FusedFuture` trait is required because `select` must
not poll a future after it has completed. `FusedFuture` is implemented
by futures which track whether or not they have completed. This makes
it possible to use `select` in a loop, only polling the futures which
still have yet to complete. This can be seen in the example above,
where `a_fut` or `b_fut` will have completed the second time through
the loop. Because the future returned by `future::ready` implements
`FusedFuture`, it's able to tell `select` not to poll it again.

Note that streams have a corresponding `FusedStream` trait. Streams
which implement this trait or have been wrapped using `.fuse()`
will yield `FusedFuture` futures from their
`.next()` / `.try_next()` combinators.

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fused_stream}}
```

## Concurrent tasks in a `select` loop with `Fuse` and `FuturesUnordered`

One somewhat hard-to-discover but handy function is `Fuse::terminated()`,
which allows constructing an empty future which is already terminated,
and can later be filled in with a future that needs to be run.

This can be handy when there's a task that needs to be run during a `select`
loop but which is created inside the `select` loop itself.

Note the use of the `.select_next_some()` function. This can be
used with `select` to only run the branch for `Some(_)` values
returned from the stream, ignoring `None`s.

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fuse_terminated}}
```

When many copies of the same future need to be run simultaneously,
use the `FuturesUnordered` type. The following example is similar
to the one above, but will run each copy of `run_on_new_num_fut`
to completion, rather than aborting them when a new one is created.
It will also print out a value returned by `run_on_new_num_fut`.

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:futures_unordered}}
```
