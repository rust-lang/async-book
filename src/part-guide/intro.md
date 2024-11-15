# Part 1: A guide to asynchronous programming in Rust

This part of the book is a tutorial-style guide to async Rust. It is aimed at newcomers to async programming in Rust. It should be useful whether or not you've done async programming in other languages. If you have, you might skip the first section or skim it as a refresher. You might also want to read this [comparison to async in other languages]() sooner rather than later.

## Core concepts

We'll start by discussing different models of [concurrent programming](concurrency.md), using processes, threads, or async tasks. The first chapter will cover the essential parts of Rust's async model before we get into the nitty-gritty of async programming in the [second chapter](async-await.md) where we introduce the async and await programming paradigm. We cover some more async programming concepts in the [following chapter](more-async-await.md).

One of the main motivations for async programming is more performant IO, which we cover in the [next chapter](io.md). We also cover *blocking* in detail in the same chapter. Blocking is a major hazard in async programming where a thread is blocked from making progress by an operation (often IO) which synchronously waits.

Another motivation for async programming is that it facilitates new models for [abstraction and composition of concurrent code](concurrency-primitives.md). After covering that, we move on to [synchronization](sync.md) between concurrent tasks.

There is a chapter on [tools for async programming](tools.md).

The last few chapters cover some more specialised topics, starting with [async destruction and clean-up](dtors.md) (which is a common requirement, but since there is currently not a good built-in solution, is a bit of a specialist topic).

The next two chapters in the guide go into detail on [futures](futures.md) and [runtimes](runtimes.md), two fundamental building blocks for async programming.

Finally, we cover [timers and signal handling](timers-signals.md) and [async iterators](streams.md) (aka streams). The latter are how we program with sequences of async events (c.f., individual async events which are represented using futures or async functions). This is an area where the language is being actively developed and can be a little rough around the edges.
