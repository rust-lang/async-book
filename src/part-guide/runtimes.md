# Runtimes and runtime issues

## Running async code

- Explicit startup vs async main
- tokio context concept
- block_on
- runtime as reflected in the code (Runtime, Handle)
- runtime shutdown

## Threads and tasks

- default work stealing, multi-threaded
  - revisit Send + 'static bounds
- yield
- spawn-local
- spawn-blocking (recap), block-in-place
- tokio-specific stuff on yielding to other threads, local vs global queues, etc

## Configuration options

- thread pool size
- single threaded, thread per core etc.

## Alternate runtimes

- Why you'd want to use a different runtime or implement your own
- What kind of variations exist in the high-level design
- Forward ref to adv chapters
