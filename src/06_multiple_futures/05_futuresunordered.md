# `FuturesUnordered`

The `FuturesUnordered` struct is a set of futures which may be completed in any order.

```rust,edition2018,ignore
{{#include ../../examples/06_05_futuresunordered/src/lib.rs:simple}}
```

Because `FuturesUnordered` implements the `Stream` trait using `.next()` 
on it will return a `Future` which returns the return value from one of
the futures inside of an `Option`.
It will return `None` when all futures are completed.

# `Collect`

The `FuturesUnordered` struct can be constructed using the `new` method or by using collect.
The previous example can thus be rewritten like this.

```rust,edition2018,ignore
{{#include ../../examples/06_05_futuresunordered/src/lib.rs:collect}}
```
