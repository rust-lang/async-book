# Destruction and clean-up

- Object destruction and recap of Drop
- General clean up requirements in software
- Async issues
  - Might want to do stuff async during clean up, e.g., send a final message
  - Might need to clean up stuff which is still being used async-ly
  - Might want to clean up when an async task completes or cancels and there is no way to catch that
  - State of the runtime during clean-up phase (esp if we're panicking or whatever)
  - No async Drop
    - WIP
  - forward ref to completion io topic

## Cancellation

- How it happens (recap of more-async-await.md)
  - drop a future
  - cancellation token
  - abort functions
- What we can do about 'catching' cancellation
  - logging or monitoring cancellation
- How cancellation affects other futures tasks (forward ref to cancellation safety chapter, this should just be a heads-up)

## Panicking and async

- Propagation of panics across tasks (spawn result)
- Panics leaving data inconsistent (tokio mutexes)
- Calling async code when panicking (make sure you don't)

## Patterns for clean-up

- Avoid needing clean up (abort/restart)
- Don't use async for cleanup and don't worry too much
- async clean up method + dtor bomb (i.e., separate clean-up from destruction)
- centralise/out-source clean-up in a separate task or thread or supervisor object/process
- https://tokio.rs/tokio/topics/shutdown

## Why no async Drop (yet)

- Note this is advanced section and not necessary to read
- Why async Drop is hard
- Possible solutions and there issues
- Current status
