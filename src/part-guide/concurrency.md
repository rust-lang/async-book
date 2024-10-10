# Concurrent programming

The goal of this chapter is to give you a high-level idea of how async concurrency works and how it is different from concurrency with threads. I think it is important to have a good mental model of what is going on before getting in to the details, but if you're the kind of person who likes to see some real code first, you might like to read the next chapter or two and then come back to this one.

We'll start with some motivation, then cover [sequential programming](#sequential-execution), [programming with threads or processes](#processes-and-threads), then [async programming](#async-programming). The chapter finishes with a section on the distinction between [concurrency and parallelism](#concurrency-and-parallelism).

Users want their computers to do multiple things. Sometimes users want to do those things at the same time (e.g., be listening to a music app at the same time as typing in their editor). Sometimes doing multiple tasks at the same time is more efficient (e.g., getting some work done in the editor while a large file downloads). Sometimes there are multiple users wanting to use a single computer at the same time (e.g., multiple clients connected to a server).

To give a lower-level example, a music program might need to keep playing music while the user interacts with the user interface (UI). To 'keep playing music', it might need to stream music data from the server, process that data from one format to another, and send the processed data to the computer's audio system via the operating system (OS). For the user, it might need to send and receive data or commands to the server in response to the user instructions, it might need to send signals to the subsystem playing music (e.g., if the user changes track or pauses), it might need to update the graphical display (e.g., highlighting a button or changing the track name), and it must keep the mouse cursor or text inputs responsive while doing all of the above.

Doing multiple things at once (or appearing to do so) is called concurrency. Programs (in conjunction with the OS) must manage their concurrency and there are many ways to do that. We'll describe some of those ways in this chapter, but we'll start with purely sequential code, i.e., no concurrency at all.

## Sequential execution

The default mode of execution in most programming languages (including Rust) is sequential execution.

```
do_a_thing();
println!("hello!");
do_another_thing();
```

Each statement is completed before the next one starts[^obs1]. Nothing happens in between those statements[^obs2]. This might sound trivial but it is a really useful property for reasoning about our code. However, it also means we waste a lot of time. In the above example, while we're waiting for `println!("hello!")` to happen, we could have executed `do_another_thing()`. Perhaps we could even have executed all three statements at the same time.

Even where IO[^io-def] happens (printing using `println!` is IO - it is outputting text to the console via a call to the OS), the program will wait for the IO to complete[^io-complete] before executing the next statement. Waiting for IO to complete before continuing with execution is called *blocking*. Blocking IO is the easiest kind of IO to use, implement, and reason about, but it is also the least efficient - in a sequential world, the program can do nothing while it waits for the IO to complete - in other words, the program is *blocked* from making further progress.

[^obs1]: This isn't really true: modern compilers and CPUs will reorganise your code and run it any order they like. Sequential statements are likely to overlap in many different ways. However, this should never be *observable* to the program itself or its users.
[^obs2]: This isn't true either: even when one program is purely sequential, other programs might be running at the same time; more on this in the next section.
[^io-def]: IO is an acronym of input/output. It means any communication from the program to the world outside the program. That might be reading or writing to disk or the network, printing to the terminal, getting user input from a keyboard or mouse, or communicating with the OS or another program running in the system. IO is interesting in the context of concurrency because it takes several orders of magnitude longer to happen than nearly any task a program might do internally. That typically means lots of waiting and that waiting time is an opportunity to do some other work.
[^io-complete]: When IO is complete is actually rather complicated. From the program's perspective a single IO call is complete when control is returned from the OS. This usually indicates that data has been sent to some hardware or other program, but it doesn't necessarily mean that the data has actually been written to disk or displayed to the user, etc. That might require more work in the hardware or periodic flushing of caches, or for another program to read the data. Mostly we don't need to care about this, but it's good to be aware of.

## Processes and threads

Processes and threads are concepts which are provided by the operating system to provide concurrency. With the highest level view, there is one process per program, so supporting multiple processes means a computer can run multiple programs concurrently; there can be multiple threads per program, which means there can be concurrency *within* a program.

There are many small differences in the way that processes and threads are handled. The most important difference is that memory is shared between threads but not between processes[^shmem]. That means that communication between processes. From a program's perspective, the single process is their whole world; creating new processes means running new programs. Creating new threads, however, is just part of the program's regular execution.

Because of these distinctions between processes and threads, they feel very different to a programmer. But from the OS's perspective they are very similar and we'll discuss their properties as if they were a single concept. We'll talk about threads, but unless we note otherwise, you should understand that to mean 'threads or processes'.

The OS is responsible for *scheduling* threads, which means it decides when threads run and for how long. Most modern computers have multiple cores, so they can literally run multiple threads at the same time. However, it is common to have many more threads than cores and so the OS will run each thread for a small amount of time and then pause it and run a different thread for some time[^sched]. When multiple threads are run on a single core in this fashion, it is called *interleaving* their executions or *time-slicing*. Since the OS chooses when to pause a thread's execution, it is called *pre-emptive multitasking* (multitasking here just means running multiple threads at the same time); the OS *pre-empts* execution of a thread (or more verbosely, the OS pre-emptively pauses execution. It is pre-emptive because the OS is pausing the thread to make time for another thread before the first thread would otherwise pause to ensure that the second thread can execute before it becomes a problem that it can't).

Let's look at IO again. What happens when a thread blocks waiting for IO? In a system with threads, then the OS will pause the thread (it's just waiting in any case) and wake it up again when the IO is complete[^busywait]. Depending on the scheduling algorithm it might be some time after the IO completes since the OS might wait for other threads to get some work done before waking up the thread waiting for IO. So now things are much more efficient: while one thread waits for IO, another thread (or more likely, many threads due to multitasking) can make progress. But, from the perspective of the thread doing IO, things are still pretty sequential - it waits for the IO to finish before starting the next operation.

A thread can also choose to pause itself by calling a `sleep` function, usually with a timeout. In this case the OS pauses the thread at the threads own request. Similar to pausing due to pre-emption or IO, the OS will wake the thread up again later (after the timeout) to continue execution.

When an OS pauses one thread and starts another (for any reason), it is called *context switching*. The context being switched includes the registers, operating system records, and the contents of many caches. That's a non-trivial amount of work and together with the transfer of control to the OS and back to a thread, and the costs of working with stale caches, context switching is an expensive operation.

Finally, note that some hardware or OSs do not support processes or threads, this is especially likely in the embedded world.

[^shmem]: Some OSs do support sharing memory between processes, but using it requires special treatment and most memory is not shared.
[^sched]: Exactly how the OS chooses which thread to run and for how long (and on which core), is a key part of scheduling. There are many options, both high-level strategies and options to configure those strategies. Making good choices here is crucial for good performance, but it is super-complicated and we won't dig into it here.
[^busywait]: There's another option which is that the thread can *busy wait* by just spinning in a loop until the IO is finished. This is not very efficient since other threads won't get to run and is uncommon in most modern systems. You may come across it in the implementations of locks or in very simple embedded systems.


## Async programming

Async programming is a kind of concurrency with the same high-level goals as concurrency with threads (do many things at the same time), but a different implementation. The two big differences between async concurrency and concurrency with threads, is that async concurrency is managed entirely within the program with no help from the OS[^threads], and that multitasking is cooperative rather than pre-emptive[^other] (we'll explain that in a minute). There are many different models of async concurrency, we'll compare them later on in the guide, but for now we'll focus only on Rust's model.

To distinguish them from threads, we'll call a sequence of executions in async concurrency a task (they're also called *green threads*, but this sometimes has connotations of pre-emptive scheduling). The way a task is executed, scheduled, and represented in memory is very different to a thread, but for a high-level intuition, it can be useful to think of tasks as just like threads, but managed entirely within the program, rather than by the OS.

In an async system, there is still a scheduler which decides which task to run next (it's part of the program, not part of the OS). However, the scheduler cannot pre-empt a task, instead a task must voluntarily give up control and allow another task to be scheduled. Primarily, this happens when a program uses the `await` keyword.

Using cooperative rather than pre-emptive multitasking has many implications:

* between await points, you can guarantee that code will be executed sequentially - you'll never be unexpectedly paused,
* if a task takes a long time between await points (e.g., by doing blocking IO or performing long-running computation), other tasks will not be able to make progress,
* implementing a scheduler is much simpler and scheduling is a more lightweight operation.

Async concurrency is much more efficient than concurrency with threads. The memory overheads are much lower and context switching is a much cheaper operation - it doesn't require handing control to the OS and back to the program and there is much less data to switch. However, there can still be some cache effects - although the OS's caches such as the [TLB](https://en.wikipedia.org/wiki/Translation_lookaside_buffer) don't need to be changed, tasks are likely to operate on different parts of memory so data required by the newly scheduled task may not be in a memory cache.

Asynchronous IO is an alternative to blocking IO (it's sometimes called non-blocking IO). Async IO is not directly tied to async concurrency, but the two are usually used together. In async IO, a program initiates IO with one system call and then can either check or be notified when the IO completes. That means the program is free to get other work done while the IO takes place. In Rust, the mechanics of async IO are handled by the runtime (the scheduler is also part of the runtime, we'll discuss runtimes in more detail later in this book, but essentially the runtime is just a library which takes care of some fundamental async stuff).

From the perspective of the whole system, blocking IO in a concurrent system with threads and non-blocking IO in an async concurrent system are similar. In both cases, IO takes time and other work gets done while the IO is happening. With threads, the thread doing IO requests IO from the OS, the thread is paused by the OS, other threads get work done, and when the IO is done, the OS wakes up the thread so it can continue execution with the result of the IO. With async, the task doing IO requests IO from the runtime, the runtime requests IO from the OS but the OS returns control to the runtime. The runtime pauses the IO task and schedules other tasks to get work done. When the IO is done, the runtime wakes up the IO task and it so it can continue execution with the result of the IO.

The advantage of using async IO, is that the overheads are much lower so a system can support orders of magnitude more tasks than threads. That makes async concurrency particularly well-suited for tasks with lots of users which spend a lot of time waiting for IO (if they don't spend a lot of time waiting and instead do lots of CPU-bound work, then there is no so much advantage of the low-overheads because the bottleneck will be CPU and memory resources).

Threads and async are not mutually exclusive: many programs use both. Some programs have parts which are better implemented using threads and parts which are better implemented using async. For example, a database server may use async techniques to manage network communication with clients, but use OS threads for computation on data. Alternatively, a program may be written only using async concurrency, but the runtime will execute tasks on multiple threads. This is necessary for a program to make use of multiple CPU cores. We'll cover the intersection of threads and async tasks in a number of places later in the book.

[^threads]: We'll start our explanation assuming a program only has a single thread, but expand on that later. There will probably be other processes running on the system, but they don't really affect how async concurrency works.
[^other]: There are some programming languages (or even libraries) which have concurrency which is managed within the program (without the OS), but with a pre-emptive scheduler rather than relying on cooperation between threads. Go is a well-known example. These systems don't require `async` and `await` notation, but have other downsides including making interop with other languages or the OS much more difficult, and having a heavyweight runtime. Very early versions of Rust had such a system, but no traces of it remained by 1.0.

## Concurrency and Parallelism

So far we've been talking about concurrency (doing, or appearing to do, many things at the same time), and we've hinted at parallelism (the presence of multiple CPU cores which facilitates literally doing many things at the same time). In this section, we'll more precisely define these terms and the difference between them. We'll use a kitchen metaphor, which is a pretty common analogy for concurrent systems.

Imagine preparing a complicated dinner service in a commercial kitchen. There are several chefs producing multiple dishes for the diners. Preparing each dish consists of several steps, e.g., chop onions, chop veg, saute onions, steam veg, prepare sauce, cook sauce, combine sauce, onions, and veg, bake the dish, serve the dish. Some of these steps must be done before others (e.g., chop the onions before saute the onions) and others can be done at the same time (chop the onions and chop the veg). Some steps require a chef's attention (e.g., chop the onions), some do not (e.g., steam the veg).



concurrency and parallelism
  cooking metaphor
  multiple CPUs and time slicing
  context switching
  scheduling

both threads and tasks permit concurrency and parallelism
  the cooking metaphor applies to both, but
    the chefs are different resources
    how the chefs are scheduled is different
  for threads parallelism is managed by the OS
  for tasks, parallelism is managed by the scheduler
  both can be configured, the latter is more flexible - anything which can be coded, vs a provided API
  concurrency implied by the structure of the code
    spawn a thread vs spawn a task
