# IO and issues with blocking

## Blocking and non-blocking IO

- High level view
- How async IO fits with async concurrency
- Why blocking IO is bad
- forward ref to streams for streams/sinks

## Read and Write

- async Read and Write traits
  - part of the runtime
- how to use
- specific implementations
  - network vs disk
    - tcp, udp
    - file system is not really async, but io_uring (ref to that chapter)
  - practical examples
  - stdout, etc.
  - pipe, fd, etc.


## Memory management

- Issues with buffer management and async IO
- Different solutions and pros and cons
  - zero-copy approach
  - shared buffer approach
- Utility crates to help with this, Bytes, etc.

## Advanced topics on IO

- buf read/write
- Read + Write, split, join
- copy
- simplex and duplex
- cancelation

## The OS view of IO

- Different kinds of IO and mechanisms, completion IO, reference to completion IO chapter in adv section
  - different runtimes can faciliate this
  - mio for low-level interface


## Other blocking operations

- Why this is bad
- Long running CPU work
  - Using Tokio for just CPU work: https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/
- Solutions
  - spawn blocking
  - thread pool
  - etc.
- yielding to the runtime
  - not the same as Rust's yield keyword
  - await doesn't yield
  - implicit yields in Tokio
