# Channels, locking, and synchronization

note on runtime specificness of sync primitves

Why we need async primitives rather than use the sync ones

## Channels

- basically same as the std ones, but await
  - communicate between tasks (same thread or different)
- one shot
- mpsc
- other channels
- bounded and unbounded channels

## Locks

- async Mutex
  - c.f., std::Mutex - can be held across await points (borrowing the mutex in the guard, guard is Send, scheduler-aware? or just because lock is async?), lock is async (will not block the thread waiting for lock to be available)
    - even a clippy lint for holding the guard across await (https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_lock)
  - more expensive because it can be held across await
    - use std::Mutex if you can
      - can use try_lock or mutex is expected to not be under contention
  - lock is not magically dropped when yield (that's kind of the point of a lock!)
  - deadlock by holding mutex over await
    - tasks deadlocked, but other tasks can make progress so might not look like a deadlock in process stats/tools/OS
    - usual advice - limit scope, minimise locks, order locks, prefer alternatives
  - no mutex poisoning
  - lock_owned
  - blocking_lock
    - cannot use in async
  - applies to other locks (should the above be moved before discussion of mutex specifically? Probably yes)
- RWLock
- Semaphore
- yielding

## Other synchronization primitives

- notify, barrier
- OnceCell
- atomics
