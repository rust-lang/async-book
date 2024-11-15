# Structured Concurrency

Authors note (TODO): we might want to discuss some parts of this chapter much earlier in the book, in particularly as design principles (first intro is in guide/intro). However, in the interests of better understanding the topic and getting something written down, I'm starting with a separate chapter. It's also still a bit rough.

(Note: the first few sections are talking about the abstract concept of structured concurrency and is not specific to Rust or async programming (c.f., synchronous concurrent programming with threads). I use 'task' to mean any thread or async task or other similar concurrency primitive).

Structured concurrency is a philosophy for designing concurrent programs. For programs to fully adhere to the principals of structured concurrency requires certain language features and libraries, but many of the benefits are available by following the philosophy without such features. Structured concurrency is independent of language and concurrency primitives (threads vs async, etc.). Many people have found the ideas from structured concurrency to be useful when programming with async Rust.

The essential idea of structured concurrency is that tasks are organised into a tree. Child tasks start after their parents and always finish before them. This allows results and errors to always be passed back to parent tasks, and requires that cancellation of parents is always propagated to child tasks. Primarily, temporal scope follows lexical scope, which means that a task should not outlive the function or block where it is created. However, this is not a requirement of structured concurrency as long as longer-lived tasks are reified in the program in some way (typically by using an object to represent the temporal scope of a child task within its parent task).

TODO diagram

Structured concurrency is named by analogy to [structured programming](https://en.wikipedia.org/wiki/Structured_programming), which is the idea that control flow should be structured using functions, loops, etc., rather than arbitrary jumps (`goto`).

Before we consider structured concurrency, it's helpful to reflect on the sense in which common concurrent designs are unstructured. A typical pattern is that a task is started using some kind of spawning statement. That task then runs to completion concurrently with other tasks in the system (including the task which spawned it). There is no constraint on which task finishes first. The program is essentially just a bag of tasks which live independently and might terminate at any time. Any communication or synchronization of the tasks is ad hoc, and the programmer cannot assume that any other task will still be running.

The practical downsides of unstructured concurrency are that returning results from a task must happen in an extra-linguistic fashion with no language-level guarantees around when or how this happens. Errors may go uncaught because languages' error handling mechanisms cannot be applied to the unconstrained control flow of unstructured concurrency. We also have no guarantees about the relative state of tasks - any task may be running, terminated successfully or with an error, or externally cancelled, independent of the state of any others[^join]. All this makes concurrent programs difficult to understand and maintain. This lack of structure is one reason why concurrent programming is considered categorically more difficult than sequential programming.

It's worth noting that structured concurrency is a programming discipline which imposes restrictions on your program. Just like functions and loops are less flexible than goto, structured concurrency is less flexible than just spawning tasks. However, as with structured programming the costs of structured concurrency in flexibility are outweighed by the gains in predictability.


[^join]: Using join handles mitigates these downsides somewhat, but is an ad hoc mechanism with no reliable guarantees. To get the full benefits of structured concurrency you have to be meticulous about always using them, as well as handling cancellation and errors properly. This is difficult without language or library support; we'll discuss this a bit more below.


## Principles of structured concurrency

The key idea of structured concurrency is that all tasks (or threads or whatever) are organized as a tree. I.e., each task (except the main task which is the root) has a single parent and there are no cycles of parents. A child task is started by its parent[^start-parent] and must *always* finish executing before its parent. There are no constraints between siblings. The parent of a task may not change.

When reasoning about programs which implement structured concurrency, the key new fact is that if a task is live, then all of its ancestor tasks must also be live. This doesn't guarantee they are in a good state - they might be in the process of shutting down or handling an error, but they must be running in some form. This means that for any task (except the root task), there is always a live task to send results or errors to. Indeed, the ideal approach is that the language's error handling is extended so that errors are always propagated to the parent task. In Rust, this should apply to both returning `Result::Err` and to panicking.

Furthermore, the lifetime of child tasks can be represented in the parent task. In the common case, the lifetime of a task (its temporal scope) is tied to the lexical scope in which it is started. For example, all tasks started within a function should complete before the function returns. This is an extremely powerful reasoning tool. Of course, this is too restrictive for all cases, and so the temporal scope of tasks can extend beyond a lexical scope by using an object in the program (often called a 'scope' or 'nursery'). Such an object can be passed or stored, and thus have an arbitrary lifetime. We still have an important reasoning tool: the tasks tied to that object cannot outlive it (in Rust this property lets us integrate tasks with the lifetime system).

The above leads to another benefit of structured concurrency: it lets us reason about resource management across multiple tasks. Cleanup code is called when a resource will no longer be used (e.g., closing a file handle). In sequential code, the problem of when to call cleanup code is solved by ensuring destructors are called when an object goes out of scope. However, in concurrent code, an object might still be in use by another task and so when to clean up is unclear (reference counting or garbage collection are solutions in many cases, but make reasoning about the lifetimes of objects difficult which can lead to errors, and also has runtime overheads).

The principle of a parent task outliving it's children has an important implication for cancellation: if a task is cancelled, then all its child tasks must be cancelled, and their cancellation must complete before the parent's cancellation completes. That in turn has implications for how cancellation can be implemented in a structurally concurrent system.

If a task completes early due to an error (in Rust, this might mean a panic, as well as an early return), then before returning the task must wait for all its child tasks to complete. In practice, an early return must trigger cancellation of child tasks. This is analogous to panicking in Rust: panicking triggers destructors in the current scope before walking up the stack, calling destructors in each scope until the program terminates or the panic is caught. Under structural concurrency, an early return must trigger cancellation of child tasks (and thus cleanup of objects in those tasks) and walks down the tree of tasks cancelling all (transitive) children.

Some designs work very naturally under structured concurrency (e.g., worker tasks with a single job to complete), while others don't fit so well. Generally these patterns are ones where not being tied to a specific task is a feature, e.g., worker pools or background threads. Even using these patterns, the tasks usually shouldn't outlive the whole program and so there is always one task which can be the parent.


[^start-parent]: This is not actually a hard requirement for structured concurrency. If the temporal scope of a task can be represented in the program and passed between tasks, then a child task can be started by one task but have another as its parent.


### Implementing structured concurrency

The exemplar implementation of structured concurrency is the Python [Trio](https://trio.readthedocs.io/en/stable/) library. Trio is a general purpose library for async programming and IO designed around the concepts of structured concurrency. Trio programs use the `async with` construct to define a lexical scope for spawning tasks. Spawned tasks are associated with a [nursery](https://trio.readthedocs.io/en/stable/reference-core.html#nurseries-and-spawning) object (which is somewhat like a [Scope](https://doc.rust-lang.org/stable/std/thread/struct.Scope.html) in Rust). The lifetime of a task is tied to the dynamic temporal scope of its nursery, and in the common case, the lexical scope of an `async with` block. This enforces the parent/child relationship between tasks and thus the tree-invariant of structured concurrency.

Error handling uses Python exceptions which are automatically propagated to parent tasks.


### Partially structured concurrency

Like many programming techniques, the full benefits of structured concurrency come from *only* using it. If all concurrency is structured, then it makes it much easier to reason about the behaviour of the whole program. However, that has requirements on a language which are not easily met; it is easy enough to do unstructured concurrency in Rust, for example. However, even applying the principles of structured concurrency selectively, or thinking in terms of structured concurrency can be useful.

One can use structured concurrency as a design discipline. When designing a program, always consider and document the parent-child relationships between tasks and ensure that a child task terminates before it's parent. This is usually fairly easy under normal execution, but can be difficult in the face of cancellation and panics.

Another element of structured concurrency which is fairly easy to adopt is to always propagate errors to the parent task. Just like regular error handling, the best thing to do might be to ignore the error, but this should be explicit in the code of the parent task.

Another programming discipline to learn from structured concurrency is to cancel all child tasks in the event of cancelling a parent task. This makes the structural concurrency guarantees much more reliable and makes cancellation in general easier to reason about.


## Practical structured concurrency with async Rust


Concurrency in Rust (whether async or using threads) is inherently unstructured. Tasks can be arbitrarily spawned, errors and panics on other tasks can be ignored, and cancellation is usually instantaneous and does not propagate to other tasks (see below for why these issues can't be easily solved). However, there are several ways you can get some of the benefits of structured concurrency in your programs:

- Design your programs at a high level in accordance with structured concurrency.
- Stick to structured concurrency idioms where possible (and avoid unstructured idioms).
- Use crates to make structured concurrency more ergonomic and reliable.

One of the trickiest issues with using structured concurrency with Rust is propagating cancellation to child futures/tasks. If you're using futures and [composing them concurrently](../part-guide/concurrency-primitives.md), then this happens naturally if abruptly (dropping a future drops any futures it owns, cancelling them). However, when a task is dropped, there is no opportunity to send a signal to tasks it has spawned (at least not with Tokio[^join_handle]).

The implication of this is that you can only assume a weaker invariant than with 'real' structured concurrency: rather than being able to assume that a parent task is always alive, you can only assume that the parent is always alive unless it has been cancelled or it has panicked. While this is sub-optimal, it can still simplify programming because you never have to handle the case of having no parent to handle some result *under normal execution*.

TODO

- ownership/lifetimes naturally leading to sc
- reasoning about resources


[^join_handle]: The semantics of Tokio's `JoinHandle` is that if the handle is dropped, then the underlying task is 'released' (c.f., dropped), i.e., the result of the child task is not handled by any other task.


### Applying structured concurrency to the design of async programs

In terms of designing programs, applying structured concurrency has a few implications:

- Organising the concurrency of a program in a tree structure, i.e., thinking in terms of parent and child tasks.
- Temporal scope should follow lexical scope where possible, or in concrete terms a function shouldn't return (including early returns and panics) until any tasks launched in the function are complete.
- Data generally flows from child tasks to parent tasks. Of course, some data will flow from parents to children or in other ways, but primarily, tasks pass the results of their work to their parent tasks for further processing. This includes errors, so parent tasks should handle the errors of their children.

If you're writing a library and want to use structured concurrency (or you want the library to be usable in a concurrent-structured program), then it is important that encapsulation of the library component includes temporal encapsulation. I.e., it doesn't start tasks which keep running beyond the API functions returning.

Since Rust can't enforce the rules of structured concurrency, it's important to be aware of, and to document, in which ways the program (or component) is structured and where it violates the structured concurrency discipline.

One useful compromise pattern is to only allow unstructured concurrency at the highest level of abstraction, and only for tasks spawned from the outer-most functions of the main task (ideally only from the `main` function, but programs often have some setup or configuration code which means that the logical 'top level' of a program is actually a few functions deep). Under such a pattern, a bunch of tasks are spawned from `main`, usually with distinct responsibilities and limited interaction between each other. These tasks might be restarted, new tasks started by any other task, or have a limited lifetime tied to clients or similar, i.e., they are concurrent-unstructured. Within each of these tasks, structured concurrency is rigorously applied.

TODO why is this useful?

TODO would be great to have a case study here.


### Structured and unstructured idioms

This subsection covers a grab-bag of idioms which work well with a structured approach to concurrency, and a few which make structuring concurrency more difficult.

The easiest way to follow structured concurrency is to use futures and [concurrent composition](../part-guide/concurrency-primitives.md) rather than tasks and spawning. If you need tasks for parallelism, then you will need to use `JoinHandle`s or `JoinSet`s. You must take care that child tasks can clean up properly if the parent task panics or is cancelled. Handles must be checked for errors to ensure errors in child tasks are properly handled.

One way to work around the lack of cancellation propagation is to avoid abruptly cancelling (dropping) any task which may have children. Instead use a signal (e.g., a cancellation token) so that the task can cancel it's children before terminating. Unfortunately this is incompatible with `select`.

To handle shutting down a program (or component), use an explicit shutdown method rather than dropping the component, so that the shutdown function can wait for child tasks to terminate or cancel them (since `drop` cannot be async).

A few idioms do not play well with structured concurrency:

- Spawning tasks without awaiting their completion via a join handle, or dropping those join handles.
- Select or race macros/functions. These are not inherently structured, but since they abruptly cancel futures, it's a common source of unstructured cancellation.
- Worker tasks or pools. For async tasks the overheads of starting/shutting down tasks is so low that there is likely to be very little benefit of using a pool of tasks rather than a pool of 'data', e.g., a connection pool.
- Data with no clear ownership structure - this isn't necessarily in contradiction with structured concurrency, but often leads to design issues.


### Crates for structured concurrency

TODO

- crates: [moro](https://github.com/nikomatsakis/moro), [async-nursery](https://github.com/najamelan/async_nursery)
- futures-concurrency


## Related topics

This section is not necessary to know to use structured concurrency with async Rust, but is useful context included for the curious.

### Scoped threads

Structured concurrency with Rust threads works pretty well. Although you can't prevent spawning threads with unscoped lifetime, this is easy to avoid. Instead, restrict yourself to using scoped threads, see the [`scope`](https://doc.rust-lang.org/stable/std/thread/fn.scope.html) function docs for how. Using scoped threads limits child lifetimes and automatically propagates panics back to the parent thread. The parent thread must check the results of child threads to handle errors though. You can even pass around the [`Scope`](https://doc.rust-lang.org/stable/std/thread/struct.Scope.html) object like a Trio nursery. Cancellation is not usually an issue for Rust threads, but if you do make use of thread cancellation, you'll have to integrate that with scoped threads manually.

Specific to Rust, scoped threads allow child threads to borrow data from the parent thread, something not possible with concurrent-unstructured threads. This can be very useful and shows how well structured concurrency and Rust-ownership-style resource management can work together.


### Async drop and scoped tasks

In Rust, destructors (`drop`) are used to ensure resources are cleaned up when an object's lifetime ends. Since futures are just objects, their destructor would be an obvious place to ensure cancellation of child futures. However, in an async program it is very often desirable for cleanup actions to be asynchronous (not doing so can block other tasks). Unfortunately Rust does not currently support asynchronous destructors (async drop). There is ongoing work to support them, but it is difficult for a number of reasons, including that an object with an async destructor might be dropped from non-async context, and that since calling `drop` is implicit, there is nowhere to write an explicit `await`.

Given how useful scoped threads are (both in general and for structured concurrency), another good question is why there is no similar construct for async programming ('scoped tasks')? TODO answer this


### References

If you're interested, here are some good blog posts for further reading:

- [Structured Concurrency](https://www.250bpm.com/p/structured-concurrency)
- [Tree-structured concurrency](https://blog.yoshuawuyts.com/tree-structured-concurrency/)
