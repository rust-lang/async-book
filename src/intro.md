NOTE: this guide is currently undergoing a rewrite after a long time without much work. It is work in progress, much is missing, and what exists is a bit rough.

# Introduction

This book is a guide to asynchronous programming in Rust. It is designed to help you take your first steps and to discover more about advanced topics. We don't assume any experience with asynchronous programming (in Rust or another language), but we do assume you're familiar with Rust already. If you want to learn about Rust, you could start with [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).

This book has two main parts: [part one](part-guide/intro.md) is a beginners guide, it is designed to be read in-order and to take you from total beginner to intermediate level. Part two is a collection of stand-alone chapters on more advanced topics. It should be useful once you've worked through part one or if you already have some experience with async Rust.

You can navigate this book in multiple ways:

* You can read it front to back, in order. This is the recommend path for newcomers to async Rust, at least for [part one](part-guide/intro.md) of the book.
* There is a summary contents on the left-hand side of the webpage.
* If you want information about a broad topic, you could start with the topic index.
* If you want to find all discussion about a specific topic, you could start with the detailed index.
* You could see if your question is answered in the FAQs.


## What is Async Programming and why would you do it?

In concurrent programming, the program does multiple things at the same time (or at least appears to). Programming with threads is one form of concurrent programming. Code within a thread is written in sequential style and the operating system executes threads concurrently. With async programming, concurrency happens entirely within your program (the operating system is not involved). An async runtime (which is just another crate in Rust) manages async tasks in conjunction with the programmer explicitly yielding control by using the `await` keyword.

Because the operating system is not involved, *context switching* in the async world is very fast. Furthermore, async tasks have much lower memory overhead than operating system threads. This makes async programming a good fit for systems which need to handle very many concurrent tasks and where those tasks spend a lot of time waiting (for example, for client responses or for IO). It also makes async programming a good fit for microcontrollers with very limited amounts of memory and no operating system that provides threads.

Async programming also offers the programmer fine-grained control over how tasks are executed (levels of parallelism and concurrency, control flow, scheduling, and so forth). This means that async programming can be expressive as well as ergonomic for many uses. In particular, async programming in Rust has a powerful concept of cancellation and supports many different flavours of concurrency (expressed using constructs including `spawn` and its variations, `join`, `select`, `for_each_concurrent`, etc.). These allow composable and reusable implementations of concepts like timeouts, pausing, and throttling.


## Hello, world!

Just to give you a taste of what async Rust looks like, here is a 'hello, world' example. There is no concurrency, and it doesn't really take advantage of being async. It does define and use an async function, and it does print "hello, world!":

```rust,edition2021
{{#include ../examples/hello-world/src/main.rs}}
```

We'll explain everything in detail later. For now, note how we define an asynchronous function using `async fn` and call it using `.await` - an async function in Rust doesn't do anything unless it is `await`ed[^blocking].

Like all examples in this book, if you want to see the full example (including `Cargo.toml`, for example) or to run it yourself locally, you can find them in the book's GitHub repo: e.g., [examples/hello-world](https://github.com/rust-lang/async-book/tree/master/examples/hello-world).


## Development of Async Rust

The async features of Rust have been in development for a while, but it is not a 'finished' part of the language. Async Rust (at least the parts available in the stable compiler and standard libraries) is reliable and performant. It is used in production in some of the most demanding situations at the largest tech companies. However, there are some missing parts and rough edges (rough in the sense of ergonomics rather than reliability). You are likely to stumble upon some of these parts during your journey with async Rust. For most missing parts, there are workarounds and these are covered in this book.

Currently, working with async iterators (also known as streams) is where most users find some rough parts. Some uses of async in traits are not yet well-supported. There is not a good solution for async destruction.

Async Rust is being actively worked on. If you want to follow development, you can check out the Async Working Group's [home page](https://rust-lang.github.io/wg-async/meetings.html) which includes their [roadmap](https://rust-lang.github.io/wg-async/vision/roadmap.html). Or you could read the async [project goal](https://github.com/rust-lang/rust-project-goals/issues/105) within the Rust Project.

Rust is an open source project. If you'd like to contribute to development of async Rust, start at the [contributing docs](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md) in the main Rust repo.


[^blocking]: This is actually a bad example because `println` is *blocking IO* and it is generally a bad idea to do blocking IO in async functions. We'll explain what blocking IO is in [chapter TODO]() and why you shouldn't do blocking IO in an async function in [chapter TODO]().
