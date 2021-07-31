# 执行器与系统 IO

在之前的 [`Future` 特征] 中，我们讨论了一个在套接字上进行异步读取的 future 示例：

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:socket_read}}
```

这个 future 将从一个套接字中读取可用数据，当里面无数据时，
它就会将自身交还于执行器，以便其再次就绪、有数据可读时被唤醒。
但是，在这个例子中并不能清楚地了解到 `Socket` 类型是如何实现的，
尤其无法明确得知 `set_readable_callback` 函数是如何工作的。
一旦套接字就绪（可读），我们如何去安排调用 `wake()`？
一种选择是创建一个线程去不停地检查 `socket` 是否已就绪，并在就绪时调用 `wake()`。
然而，这样做是十分低效的，需要为每个阻塞的 IO future 创建一个单独的线程。
这将大大降低我们的异步代码的效率。

在实际上，是用集成一个阻塞 IO 感知系统来解决这个问题，例如 Linux 上的 `epoll`，
MacOS 及 FreeBSD 上的 `kqueue` 、 Windows 上使用的 IOCP，以及 Fuchsia 中的
`port`（ Rust 中跨平台的 crate [`mio`] 已实现了这些功能）。
这些都允许一个线程在上有多个异步 IO 阻塞事件，一旦事件完成就返回。
这些 APIs 通常看起来是这样的：

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

Futures 执行器可以使用这些原生支持来产生异步 IO 对象，例如可配置套接字，
在特定的事件发生时再去运行回调。在上面的 `SocketRead` 示例中，
`Socket::set_readable_callback` 的伪代码可以写成这样：

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

现在，我们就有了一个执行器，只此一个即可完成接收和调度任何 IO 事件至合适的
`Waker` 上，以此唤醒相应的任务。它可以去推动更多的任务完成而不必去（轮询）检查
IO 事件是否已就绪。

[`Future` 特征]: ./02_future_zh.md
[`mio`]: https://github.com/tokio-rs/mio
