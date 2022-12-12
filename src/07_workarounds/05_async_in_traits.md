# `async` in Traits

Currently, `async fn` cannot be used in traits on the stable release of Rust.
Since the 17th November 2022, an MVP of async-fn-in-trait is available on the nightly
version of the compiler tool chain, [see here for details](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html).

In the meantime, there is a work around for the stable tool chain using the
[async-trait crate from crates.io](https://github.com/dtolnay/async-trait).

Note that using these trait methods will result in a heap allocation
per-function-call. This is not a significant cost for the vast majority
of applications, but should be considered when deciding whether to use
this functionality in the public API of a low-level function that is expected
to be called millions of times a second.
