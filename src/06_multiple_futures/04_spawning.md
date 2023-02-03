# `Spawning`

Spawning allows you to run a new asynchronous task in the background. This allows us to continue executing other code 
while it is running.

Say we have a web server that wants to accept connections without blocking the main thread. 
To achieve this, we can use the `async_std::task::spawn` function to create and run a new task that handles the 
connections. This function takes a future and returns a `JoinHandle`, which can be used to wait for the result of the 
task once it's completed.

```rust,edition2018
{{#include ../../examples/06_04_spawning/src/lib.rs:example}}
```

To communicate between the main task and the spawned task, we can use channels
provided by the used async runtime.