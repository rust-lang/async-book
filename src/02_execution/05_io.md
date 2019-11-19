# Executors and System IO

In the previous section on [The `Future` Trait], we discussed this example of
a future that performed an asynchronous read on a socket:

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:socket_read}}
```

This future will read available data on a socket, and if no data is available,
it will yield to the executor, requesting that its task be awoken when the
socket becomes readable again. However, it's not clear from this example how
the `Socket` type is implemented, and in particular it isn't obvious how the
`set_readable_callback` function works. How can we arrange for `wake()`
to be called once the socket becomes readable? One option would be to have
a thread that continually checks whether `socket` is readable, calling
`wake()` when appropriate. However, this would be quite inefficient, requiring
a separate thread for each blocked IO future. This would greatly reduce the
efficiency of our async code.

In practice, this problem is solved through integration with an IO-aware
system blocking primitive, such as `epoll` on Linux, `kqueue` on FreeBSD and
Mac OS, IOCP on Windows, and `port`s on Fuchsia (all of which are exposed
through the cross-platform Rust crate [`mio`]). These primitives all allow
a thread to block on multiple asynchronous IO events, returning once one of
the events completes. In practice, these APIs usually look something like
this:

```rust,ignore
struct IoBlocker {
    /* ... */
}

struct Event {
    // An ID uniquely identifying the event that occurred and was listened for.
    id: usize,

    // A set of signals to wait for, or which occurred.
    signals: Signals,
}

impl IoBlocker {
    /// Create a new collection of asynchronous IO events to block on.
    fn new() -> Self { /* ... */ }

    /// Express an interest in a particular IO event.
    fn add_io_event_interest(
        &self,

        /// The object on which the event will occur
        io_object: &IoObject,

        /// A set of signals that may appear on the `io_object` for
        /// which an event should be triggered, paired with
        /// an ID to give to events that result from this interest.
        event: Event,
    ) { /* ... */ }

    /// Block until one of the events occurs.
    fn block(&self) -> Event { /* ... */ }
}

let mut io_blocker = IoBlocker::new();
io_blocker.add_io_event_interest(
    &socket_1,
    Event { id: 1, signals: READABLE },
);
io_blocker.add_io_event_interest(
    &socket_2,
    Event { id: 2, signals: READABLE | WRITABLE },
);
let event = io_blocker.block();

// prints e.g. "Socket 1 is now READABLE" if socket one became readable.
println!("Socket {:?} is now {:?}", event.id, event.signals);
```

Futures executors can use these primitives to provide asynchronous IO objects
such as sockets that can configure callbacks to be run when a particular IO
event occurs. In the case of our `SocketRead` example above, the
`Socket::set_readable_callback` function might look like the following pseudocode:

```rust,ignore
impl Socket {
    fn set_readable_callback(&self, waker: Waker) {
        // `local_executor` is a reference to the local executor.
        // this could be provided at creation of the socket, but in practice
        // many executor implementations pass it down through thread local
        // storage for convenience.
        let local_executor = self.local_executor;

        // Unique ID for this IO object.
        let id = self.id;

        // Store the local waker in the executor's map so that it can be called
        // once the IO event arrives.
        local_executor.event_map.insert(id, waker);
        local_executor.add_io_event_interest(
            &self.socket_file_descriptor,
            Event { id, signals: READABLE },
        );
    }
}
```

We can now have just one executor thread which can receive and dispatch any
IO event to the appropriate `Waker`, which will wake up the corresponding
task, allowing the executor to drive more tasks to completion before returning
to check for more IO events (and the cycle continues...).

[The `Future` Trait]: ./02_future.md
[`mio`]: https://github.com/tokio-rs/mio
