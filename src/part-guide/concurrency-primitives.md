# Concurrency primitives

- concurrent composition of futures
  - c.f., sequential composition with await, composition of tasks with spawn
  - concurrent/task behaviour
  - behaviour on error
- streams as alternative, forward ref
- different versions in different runtimes/other crates
  - focus on the Tokio versions

From [comment](https://github.com/rust-lang/async-book/pull/230#discussion_r1829351497): A framing I've started using is that tasks are not the async/await form of threads; it's more accurate to think of them as parallelizable futures. This framing does not match Tokio and async-std's current task design; but both also have trouble propagating cancellation. See parallel_future and tasks are the wrong abstraction for more.


## Join

- Tokio/futures-rs join macro
- c.f., joining tasks
- join in futures-concurrency
- FuturesUnordered
  - like a dynamic version of join
  - forward ref to stream

## Race/select

- Tokio select macro
- cancellation issues
- different behaviour of futures-rs version
- race in futures-concurrency
