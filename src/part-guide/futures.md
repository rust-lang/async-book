# Futures

We've talked a lot about futures in the preceding chapters; they're a key part of Rust's async programming story! In this chapter we're going to get into some of the details of what futures are and how they work, and some libraries for working directly with futures.

## The `Future` and `IntoFuture` traits

- Future
  - Output assoc type
  - No real detail here, polling is in the next section, reference adv sections on Pin, executors/wakers
- IntoFuture
  - Usage - general, in await, async builder pattern (pros and cons in using)
- Boxing futures, `Box<dyn Future>` and how it used to be common and necessary but mostly isn't now, except for recursion, etc.

## Polling

- what it is and who does it, Poll type
  - ready is final state
- how it connects with await
- drop = cancel
  - for futures and thus tasks
  - implications for async programming in general
  - reference to chapter on cancellation safety

### Fusing

## futures-rs crate

- History and purpose
  - see streams chapter
  - helpers for writing executors or other low-level futures stuff
    - pinning and boxing
  - executor as a partial runtime (see alternate runtimes in reference)
- TryFuture
- convenience futures: pending, ready, ok/err, etc.
- combinator functions on FutureExt
- alternative to Tokio stuff
  - functions
  - IO traits

## futures-concurrency crate

https://docs.rs/futures-concurrency/latest/futures_concurrency/


