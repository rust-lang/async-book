# The State of Asynchronous Rust

Parts of async Rust are supported with the same stability guarantees as
synchronous Rust. Other parts are still maturing and will change
over time. With async Rust, you can expect:

- Outstanding runtime performance for typical concurrent workloads.
- More frequent interaction with advanced language features, such as lifetimes
  and [pinning](https://doc.rust-lang.org/std/pin/).
- Some compatibility constraints, both between sync and async code, and between
  different async runtimes.
- Higher maintenance burden, due to the ongoing evolution of async runtimes
  and language support.

In short, async Rust is more difficult to use and can result in a higher
maintenance burden than synchronous Rust, but gives you best-in-class
performance in return. All areas of async Rust are constantly improving, so the
impact of these issues will wear off over time.

## Language and library support

While asynchronous programming is supported by Rust itself, most async
applications depend on functionality provided by community crates. As such, you
need to rely on a mixture of language features and library support:

- The most fundamental traits, types and functions, such as the
  [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) trait
  are provided by the standard library.
- The `async/await` syntax is supported directly by the Rust compiler.
- Many utility types, macros and functions are provided by the
  [`futures`](https://docs.rs/futures/) crate. They can be used in any async
  Rust application.
- Execution of async code, IO and task spawning are provided by "async
  runtimes", such as Tokio and async-std. Most async applications, and some
  async crates, depend on a specific runtime. See
  ["The Async Ecosystem"](../08_ecosystem/00_chapter.md) section for more
  details.

Some language features you may be used to from synchronous Rust are not yet
available in async Rust. Notably, Rust does not let you declare async
functions in traits. Instead, you need to use workarounds to achieve the same
result, which can be more verbose.

## Compiling and debugging

For the most part, compiler- and runtime errors in async Rust work
the same way as they have always done in Rust. There are a few
noteworthy differences:

### Compilation errors

Compilation errors in async Rust conform to the same high standards as
synchronous Rust, but since async Rust often depends on more complex language
features, such as lifetimes and pinning, you may encounter these types of
errors more frequently.

### Runtime errors

Whenever the compiler encounters an async function, it generates a state
machine under the hood. Stack traces in async Rust typically contain details
from these state machines, as well as function calls from
the runtime. As such, interpreting stack traces can be a bit more involved than
it would be in synchronous Rust.

### New failure modes

A few novel failure modes are possible in async Rust, for instance
if you call a blocking function from an async context or if you implement
the `Future` trait incorrectly. Such errors can silently pass both the
compiler and sometimes even unit tests. Having a firm understanding
of the underlying concepts, which this book aims to give you, can help you
avoid these pitfalls.

## Compatibility considerations

Asynchronous and synchronous code cannot always be combined freely.
For instance, you can't directly call an async function from a sync function.
Sync and async code also tend to promote different design patterns, which can
make it difficult to compose code intended for the different environments.

Even async code cannot always be combined freely. Some crates depend on a
specific async runtime to function. If so, it is usually specified in the
crate's dependency list.

These compatibility issues can limit your options, so make sure to
research which async runtime and what crates you may need early.
Once you have settled in with a runtime, you won't have to worry
much about compatibility.

## Performance characteristics

The performance of async Rust depends on the implementation of the
async runtime you're using.
Even though the runtimes that power async Rust applications are relatively new,
they perform exceptionally well for most practical workloads.

That said, most of the async ecosystem assumes a _multi-threaded_ runtime.
This makes it difficult to enjoy the theoretical performance benefits
of single-threaded async applications, namely cheaper synchronization.
Another overlooked use-case is _latency sensitive tasks_, which are
important for drivers, GUI applications and so on. Such tasks depend
on runtime and/or OS support in order to be scheduled appropriately.
You can expect better library support for these use cases in the future.
