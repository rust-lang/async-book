# Async iterators (FKA streams)

- Stream as an async iterator or as many futures
- WIP
  - current status
  - futures and Tokio Stream traits
  - nightly trait
- lazy like sync iterators
- pinning and streams (forward ref to pinning chapter)
- fused streams

## Consuming an async iterator

- while let with async next
- for_each, for_each_concurrent
- collect
- into_future, buffered

## Stream combinators

- Taking a future instead of a closure
- Some example combinators
- unordered variations
- StreamGroup

## Implementing an async iterator

- Implementing the trait
- Practicalities and util functions
- async_iter stream macro

## Sinks

- https://docs.rs/futures/latest/futures/sink/index.html

## Future work

- current status
  - https://rust-lang.github.io/rfcs/2996-async-iterator.html
- async next vs poll
- async iteration syntax
- (async) generators
- lending iterators

