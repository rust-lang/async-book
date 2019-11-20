# Joining Futures that May Fail

When we have a collection of futures that we want to wait on, e.g.
we've sent off a bunch of requests and want to wait for them all to
come back before continuing, we can join the futures into one with
one of the join functions[^1] and `await` the resultant future which
will return a collection of the results from each future.

If one of the futures that were joined fails, the joined future will
fail.  This behavior isn't always desired.

We can workaround this to get the results of _all_ the futures by packing
the futures' results into the `Ok` variant of another `Result`, then unwrap
the results once the joined future returns.

```rust
{{#include ../../examples/07_07_join_failing_futures/src/lib.rs:example}}
```

[^1]: Functions to join different numbers of futures from [`futures::future`](https://docs.rs/futures/0.3.1/futures/future/index.html) -
[`join`](https://docs.rs/futures/0.3.1/futures/future/fn.join.html),
[`join3`](https://docs.rs/futures/0.3.1/futures/future/fn.join3.html),
[`join4`](https://docs.rs/futures/0.3.1/futures/future/fn.join4.html),
[`join5`](https://docs.rs/futures/0.3.1/futures/future/fn.join5.html),
and [`join_all`](https://docs.rs/futures/0.3.1/futures/future/fn.join_all.html) for any number of futures.
The corresponding [`try_join`](https://docs.rs/futures/0.3.1/futures/future/fn.try_join.html)
functions will return as soon as one future returns an error.
