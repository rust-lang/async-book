# Executing Multiple Futures at a Time

Up until now, we've mostly executed futures by using `.await`, which blocks
the current task until a particular `Future` completes. However, real
asynchronous applications often need to execute several different
operations concurrently.

In this chapter, we'll cover some ways to execute multiple asynchronous
operations at the same time:

- `join!`: waits for futures to all complete
- `select!`: waits for one of several futures to complete
- Spawning: creates a top-level task which ambiently runs a future to completion
- `FuturesUnordered`: a group of futures which yields the result of each subfuture
