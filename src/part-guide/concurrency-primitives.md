# Concurrency primitives

- concurrent composition of futures
  - c.f., sequential composition with await, composition of tasks with spawn
  - concurrent/task behaviour
  - behaviour on error
- streams as alternative, forward ref
- different versions in different runtimes/other crates
  - focus on the Tokio versions

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
