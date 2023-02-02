# `Spawning`

Spawning allows you to run a new asynchronous task in the background. This allows us to continue executing other code 
while it is running.

Say we have a webserver that wants to accept connections without blocking the main thread.
We can do this by using the `async_std::task::spawn` function to spawn a new task that runs
the connection handler. This function takes a future and returns a `JoinHandle` that can be
used to await the result of the spawned task.

```rust,edition2018
{{#include ../../examples/06_04_spawning/src/lib.rs:example}}
```