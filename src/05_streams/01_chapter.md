# The `Stream` Trait

The `Stream` trait is similar to `Future` but can yield multiple values before
completing, similar to the `Iterator` trait from the standard library:

```rust,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:stream_trait}}
```

One common example of a `Stream` is the `Receiver` for the channel type from
the `futures` crate. It will yield `Some(val)` every time a value is sent
from the `Sender` end, and will yield `None` once the `Sender` has been
dropped and all pending messages have been received:

```rust,edition2018,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:channels}}
```
