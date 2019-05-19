# The `Stream` Trait

The `Stream` trait is similar to `Future` but can yield multiple values before
completing, similar to the `Iterator` trait from the standard library:

```rust
trait Stream {
    /// The type of value yielded by the stream.
    type Item;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>;
}
```

One common example of a `Stream` is the `Receiver` for the channel type from
the `futures` crate. It will yield `Some(val)` every time a value is sent
from the `Sender` end, and will yield `None` once the `Sender` has been
dropped and all pending messages have been received:

```rust
use futures::channel::mpsc;
use futures::prelude::*;

let fut = async {
    let (tx, rx) = mpsc::channel(BUFFER_SIZE);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` is similar to `Iterator::next`, but returns a
    // type that implements `Future<Output = Option<T>>`.
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
};
```

## Patterns: Iteration and Concurrency

Similar to synchronous `Iterator`s, there are many different ways to iterate
over and process the values in a `Stream`. There are combinator-style methods
such as `map`, `filter`, and `fold`, and their early-exit-on-error cousins
`try_map`, `try_filter`, and `try_fold`.

Unfortunately, `for` loops are not yet usable with `Stream`s, but for
imperative-style code, `while let` and `.for_each` are available:

```rust
use futures::prelude::*;

let fut = async {
    let mut stream: impl Stream<Item = Result<i32, io::Error>> = ...;

    // processing with `try_for_each`:
    stream.try_for_each(async |item| {
        // handle `item`
        Ok(())
    }).await?;

    // processing with `while let`:
    while let Some(item) = stream.try_next().await? {
        // handle `item`
    }

    ...

    Ok(())
};
```

However, if we're just processing one element at a time, we're potentially
leaving behind opportunity for concurrency, which is, after all, why we're
writing async code in the first place. To process multiple items from a stream
concurrently, use the `for_each_concurrent` and `try_for_each_concurrent`
methods:

```rust
use futures::prelude::*;

let fut = async {
    let mut stream: impl Stream<Item = Result<i32, io::Error>> = ...;

    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, async |num| {
        jump_n_times(num).await?;
        report_jumps(num).await?;
        Ok(())
    }).await?;

    ...
    Ok(())
};
```

This approach allows up to `MAX_CONCURRENT_JUMPERS` to all be jumping at once
(or performing any operation on the items, for that matter-- the API isn't
strictly tied to jumping). If you want to allow an unlimited number of
operations at once, you can use `None` rather than `MAX_CONCURRENT_...`, but
beware that if `stream` comes from untrusted user input, this can allow
badly behaved clients to overload the system with too many simultaneous
requests.
