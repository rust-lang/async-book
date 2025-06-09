# Composing futures concurrently

In this chapter we're going to cover more ways in which futures can be composed. In particular, some new ways in which futures can be executed concurrently (but not in parallel). Superficially, the new functions/macros we introduce in this chapter are pretty simple. However, the underlying concepts can be pretty subtle. We'll start with a recap on futures, concurrency, and parallelism, but you might also want to revisit the earlier section comparing [concurrency with parallelism](concurrency.md#concurrency-and-parallelism).

A futures is a deferred computation. A future can be progressed by using `await`, which hands over control to the runtime, causing the current task to wait for the result of the computation. If `a` and `b` are futures, then they can be sequentially composed (that is, combined to make a future which executes `a` to completion and then `b` to completion) by `await`ing one then the other: `async { a.await; b.await}`.

We have also seen parallel composition of futures using `spawn`: `async { let a = spawn(a); let b = spawn(b); (a.await, b.await)}` runs the two futures in parallel. Note that the `await`s in the tuple are not awaiting the futures themselves, but are awaiting `JoinHandle`s to get the results of the futures when they complete.

In this chapter we introduce two ways to compose futures concurrently without parallelism: `join` and `select`/`race`. In both cases, the futures run concurrently by time-slicing; each of the composed futures takes turns to execute then the next gets a turn. This is done *without involving the async runtime* (and therefore without multiple OS threads and without any potential for parallelism). The composing construct interleaves the futures locally. You can think of these constructs being like mini-executors which execute their component futures within a single async task.

The fundamental difference between join and select/race is how they handle futures completing their work: a join finishes when all futures finish, a select/race finishes when one future finishes (all the others are cancelled). There are also variations of both for handling errors.

These constructs (or similar concepts) are often used with streams, we'll touch on this below, but we'll talk more about that in the [streams chapter](streams.md).

If you want parallelism (or you don't explicitly not want parallelism), spawning tasks is often a simpler alternative to these composition constructs. Spawning tasks is usually less error-prone, more general, and performance is more predictable. On the other hand, spawning is inherently less [structured](../part-reference/structured.md), which can make lifecycle and resource management harder to reason about.

It's worth considering the performance issue in a little more depth. The potential performance problem with concurrent composition is the fairness of time sharing. If you have 100 tasks in your program, then typically the optimal way to share resources is for each task to get 1% of the processor time (or if the tasks are all waiting, then for each to have the same chance of being woken up). If you spawn 100 tasks, then this is usually what happens (roughly). However, if you spawn two tasks and join 99 futures on one of those tasks, then the scheduler will only know about two tasks and one task will get 50% of the time and the 99 futures will each get 0.5%.

Usually the distribution of tasks is not so biased, and very often we use join/select/etc. for things like timeouts where this behaviour is actually desirable. But it is worth considering to ensure that your program has the performance characteristics you want.


## Join

Tokio's [`join` macro](https://docs.rs/tokio/latest/tokio/macro.join.html) takes a list of futures and runs them all to completion concurrently (returning all the results as a tuple). It returns when all the futures have completed. The futures are always executed on the same thread (concurrently and not in parallel).

Here's a simple example:

```rust,norun
async fn main() {
  let (result_1, result_2) = join!(do_a_thing(), do_a_thing());
  // Use `result_1` and `result_2`.
}
```

Here, the two executions of `do_a_thing` happen concurrently, and the results are ready when they are both done. Notice that we don't `await` to get the results. `join!` implicitly awaits its futures and produces a value. It does not create a future. You do still need to use it within an async context (e.g., from within an async function).

Although you can't see it in the example above, `join!` takes expressions which evaluate to futures[^into]. `join` does not create an async context in it's body and you shouldn't `await` the futures passed to `join` (otherwise they'll be evaluated before the joined futures).

Because all the futures are executed on the same thread, if any future blocks the thread, then none of them can make progress. If using a mutex or other lock, this can easily lead to deadlock if one future is waiting for a lock held by another future.

[`join`](https://docs.rs/tokio/latest/tokio/macro.join.html) does not care about the result of the futures. In particular, if a future is cancelled or returns an error, it does not affect the others - they continue to execute. If you want 'fail fast' behaviour, use [`try_join`](https://docs.rs/tokio/latest/tokio/macro.try_join.html). `try_join` works similarly to `join`, however, if any future returns an `Err`, then all the other futures are cancelled and `try_join` returns the error immediately.

Back in the earlier chapter on [async/await](async-await.md), we used the word 'join' to talk about joining spawned tasks. As the name suggests, joining futures and tasks is related: joining means we execute multiple futures concurrently and wait for the result before continuing. The syntax is different: using a `JoinHandle` vs the `join` macro, but the idea is similar. The key difference is that when joining tasks, the tasks execute concurrently and in parallel, whereas using `join!`, the futures execute concurrently but not in parallel. Furthermore, spawned tasks are scheduled on the runtime's scheduler, whereas with `join!` the futures are 'scheduled' locally (on the same task and within the temporal scope of the macro's execution). Another difference is that if a spawned task panics, the panic is caught by the runtime, but if a future in `join` panics, then the whole task panics.


### Alternatives

Running futures concurrently and collecting their results is a common requirement. You should probably use `spawn` and `JoinHandle`s unless you have a good reason not to (i.e., you explicitly do not want parallelism, and even then you might prefer to use [`spawn_local`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_local.html)). The [`JoinSet`](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html) abstraction manages such spawned tasks in a way similar to `join!`.

Most runtimes (and [futures.rs](https://docs.rs/futures/latest/futures/macro.join.html)) have an equivalent to Tokio's `join` macro and they mostly behave the same way. There are also `join` functions, which are similar to the macro but a little less flexible. E.g., futures.rs has [`join`](https://docs.rs/futures/latest/futures/future/fn.join.html) for joining two futures, [`join3`](https://docs.rs/futures/latest/futures/future/fn.join3.html), [`join4`](https://docs.rs/futures/latest/futures/future/fn.join4.html), and [`join5`](https://docs.rs/futures/latest/futures/future/fn.join5.html) for joining the obvious number of futures, and [join_all](https://docs.rs/futures/latest/futures/future/fn.join_all.html) for joining a collection of futures (as well as `try_` variations of each of these).

[Futures-concurrency](https://docs.rs/futures-concurrency/latest) also provides functionality for join (and try_join). In the futures-concurrency style, these operations are trait methods on groups of futures such as tuples, `Vec`s, or arrays. E.g., to join two futures, you would write `(fut1, fut2).join().await` (note that `await` is explicit here).

If the set of futures you wish to join together varies dynamically (e.g., new futures are created as input comes in over the network), or you want the results as they complete rather than when all the futures have completed, then you'll need to use streams and the [`FuturesUnordered`](https://docs.rs/futures/latest/futures/stream/struct.FuturesUnordered.html) or [`FuturesOrdered`](https://docs.rs/futures/latest/futures/stream/struct.FuturesOrdered.html) functionality. We'll cover these in the [streams](streams.md) chapter.


[^into]: The expressions must have a type which implements `IntoFuture`. The expression is evaluated and converted to a future by the macro. I.e., they don't actually have to evaluate to a future, but rather something which can be converted into a future, but this is a pretty minor distinction. The expressions themselves are evaluated sequentially before any of the resulting futures are executed.


## Race/select

The counterpart to joining futures is racing them (aka selecting on them). With race/select the futures are executed concurrently, but rather than waiting for all the futures to complete, we only wait for the first one to complete and then cancel the others. Although this sounds similar to joining, it is significantly more interesting (and sometimes error-prone) because now we have to reason about cancellation.

Here's an example using Tokio's [`select`](https://docs.rs/tokio/latest/tokio/macro.select.html) macro:

```rust,norun
async fn main() {
  select! {
    result = do_a_thing() => {
      println!("computation completed and returned {result});
    }
    _ = timeout() => {
      println!("computation timed-out");
    }
  }
}
```

You'll notice things are already more interesting than with the `join` macro because we handle the results of the futures within the `select` macro. It looks a bit like a `match` expression, but with `select`, all branches are run concurrently and the body of the branch which finishes first is executed with its result (the other branches are not executed and the futures are cancelled by `drop`ping). In the example, `do_a_thing` and `timeout` execute concurrently and the first to complete will have it's block executed (i.e., only one `println` will run), the other future will be cancelled. As with the `join` macro, awaiting the futures is implicit.

Tokio's `select` macro supports a bunch of features:

- pattern matching: the syntax on the left of `=` on each branch can be a pattern and the block is only executed if the result of the future matches the pattern. If the pattern does not match, then the future is no longer polled (but other futures are). This can be useful for futures which optionally return a value, e.g., `Some(x) = do_a_thing() => { ... }`.
- `if` guards: each branch may have an `if` guard. When the `select` macro runs, after evaluating each expression to produce a future, the `if` guard is evaluated and the future is only polled if the guard is true. E.g., `x = = do_a_thing() if false => { ... }` will never be polled. Note that the `if` guard is not re-evaluated during polling, only when the macro is initialized.
- `else` branch: `select` can have an `else` branch `else => { ... }`, this is executed if all the futures have stopped and none of the blocks have been executed. If this happens without an `else` branch, then `select` will panic.

The value of the `select!` macro is the value of the executed branch (just like `match`), so all branches must have the same type. E.g., if we wanted to use the result of the above example outside of the `select`, we'd write it like

```rust,norun
async fn main() {
  let result = select! {
    result = do_a_thing() => {
      Some(result)
    }
    _ = timeout() => {
      None
    }
  };

  // Use `result`
}
```

As with `join!`, `select!` does not treat `Result`s in any special way (other than the pattern matching mentioned previously) and if a branch completes with an error, then all other branches will be cancelled and the error will be used as the result of select (in the same way as if the branch has completed successfully).

The `select` macro intrinsically uses cancellation, so if you're trying to avoid cancellation in your program, you must avoid `select!`. In fact, `select` is often the primary source of cancellation in an async program. As discussed [elsewhere](../part-reference/cancellation.md), cancellation has many subtle issues which can lead to bugs. In particular, note that `select` cancels futures by simply dropping them. This will not notify the future being dropped or trigger any cancellation tokens, etc.

`select!` is often used in a loop to handle streams or other sequences of futures. This adds an extra layer of complexity and opportunities for bugs. In the simple case that we create a new, independent future on each iteration of the loop, things are not much more complicated. However, this is rarely what is needed. Generally we want to preserve some state between iterations. It is common to use `select` in a loop with streams, where each iteration of the loop handles one result from the stream. E.g.:

```rust,norun
async fn main() {
  let mut stream = ...;

  loop {
    select! {
      result = stream.next() => {
        match result {
          Some(x) => println!("received: {x}"),
          None => break,
        }
      }
      _ = timeout() => {
        println!("time out!");
        break;
      }
    }
  }
}
```

In this example, we read values from `stream` and print them until there are none left or waiting for a result times out. What happens to any remaining data in the stream in the timeout case depends on the implementation of the stream (it might be lost! Or duplicated!). This is an example of why behaviour in the face of cancellation can be important (and tricky).

We may want to reuse a future, not just a stream, across iterations. For example, we may want to race against a timeout future where the timeout applies to all iterations rather than applying a new timeout for each iteration. This is possible by creating the future outside of the loop and referencing it:

```rust,norun
async fn main() {
  let mut stream = ...;
  let mut timeout = timeout();

  loop {
    select! {
      result = stream.next() => {
        match result {
          Some(x) => println!("received: {x}"),
          None => break,
        }
      }
      // Create a reference to `timeout` rather than moving it.
      _ = &mut timeout => {
        println!("time out!");
        break;
      }
    }
  }
}
```

There are a couple of important details when using `select!` in a loop with futures or streams created outside of the `select!`. These are a fundamental consequence of how `select` works, so I'll introduce them by stepping through the details of `select`, using `timeout` in the last example as an example.

- `timeout` is created outside of the loop and initialised with some time to count down.
- On each iteration of the loop, `select` creates a reference to `timeout`, but does not change its state.
- As `select` executes, it polls `timeout` which will return `Pending` while there is time left and `Ready` when the time elapses, at which point its block is executed.

In the above example, when `timeout` is ready, we `break` out of the loop. But what if we didn't do that? In that case, `select` would simply poll `timeout` again, which the `Future` [docs](https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll) say should not happen! `select` can't help this, it doesn't have any state (between iterations) to decide if `timeout` should be polled. Depending on how `timeout` is written, this might cause a panic, a logic error, or some kind of crash.

You can prevent this kind of bug in several ways:

- Use a [fused](futures.md#fusing) [future](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.fuse) or [stream](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.fuse) so that re-polling is safe.
- Ensure that your code is structured so that futures are never re-polled, e.g., by breaking out of the loop (as in the previous example), or by using an `if` guard.

Now, lets consider the type of `&mut timeout`. Lets assume that `timeout()` returns a type which implements `Future`, which might be an anonymous type from an async function, or it might be a named type like `Timeout`. Lets assume the latter because it makes the examples easier (but the logic applies in either case). Given that `Timeout` implents `Future`, will `&mut Timeout` implement `Future`? Not necessarily! There is a [blanket `impl`](https://doc.rust-lang.org/std/future/trait.Future.html#impl-Future-for-%26mut+F) which makes this true, but only if `Timeout` implements `Unpin`. That is not the case for all futures, so often you'll get a type error writing code like the last example. Such an error is easily fixed though by using the `pin` macro, e.g., `let mut timeout = pin!(timeout());`

Cancellation with `select` in a loop is a rich source of subtle bugs. These usually happen where a future contains some state involving some data but not the data itself. When the future is dropped by cancellation, that state is lost but the underlying data is not updated. This can lead to data being lost or processed multiple times.


### Alternatives

Futures.rs has its own [`select` macro](https://docs.rs/futures/latest/futures/macro.select.html) and futures-concurrency has a [Race trait](https://docs.rs/futures-concurrency/latest/futures_concurrency/future/trait.Race.html) which are alternatives to Tokio's `select` macro. These both have the same core semantics of concurrently racing multiple futures, processing the result of the first and cancelling the others, but they have different syntax and vary in the details.

Futures.rs' `select` is superficially similar to Tokio's; to summarize the differences, in the futures.rs version:

- Futures must always be fused (enforced by type-checking).
- `select` has `default` and `complete` branches, rather than an `else` branch.
- `select` does not support `if` guards.

Futures-concurrency's `Race` has a very different syntax, similar to it's version of `join`, e.g., `(future_a, future_b).race().await` (it works on `Vec`s and arrays as well as tuples). The syntax is less flexible than the macros, but fits in nicely with most async code. Note that if you use `race` within a loop, you can still have the same issues as with `select`.

As with `join`, spawning tasks and letting them execute in parallel is often a good alternative to using `select`. However, cancelling the remaining tasks after the first completes requires some extra work. This can be done using channels or a cancellation token. In either case, cancellation requires some action by the task being cancelled which means the task can do some tidying up or other graceful shutdown.

A common use for `select` (especially inside a loop) is working with streams. There are stream combinator methods which can replace some uses of select. For example, [`merge`](https://docs.rs/futures-concurrency/latest/futures_concurrency/stream/trait.Merge.html) in futures-concurrency is a good alternative to merge multiple streams together.


## Final words

In this section we've talked about two ways to run groups of futures concurrently. Joining futures means waiting for them all to finish; selecting (aka racing) futures means waiting for the first to finish. In contrast to spawning tasks, these compositions make no use of parallelism.

Both `join` and `select` operate on sets of futures which are known in advance (often when writing the program, rather than at runtime). Sometimes, the futures to be composed are not known in advance - futures must be added to the set of composed futures as they are being executed. For this we need [streams](streams.md) which have their own composition operations.

It's worth reiterating that although these composition operators are powerful and expressive, it is often easier and more appropriate to use tasks and spawning: parallelism is often desirable, you're less likely to have bugs around cancellation or blocking, and resource allocation is usually fairer (or at least simpler) and more predictable.
