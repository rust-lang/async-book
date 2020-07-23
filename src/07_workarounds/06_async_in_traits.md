# `async` in Traits

Currently, `async fn` cannot be used in traits. The reasons for this are
somewhat complex, but there are plans to remove this restriction in the
future.

In the meantime, however, this can be worked around using the
[async-trait crate from crates.io](https://github.com/dtolnay/async-trait).

Note that using these trait methods will result in a heap allocation
per-function-call. This is not a significant cost for the vast majority
of applications, but should be considered when deciding whether to use
this functionality in the public API of a low-level function that is expected
to be called millions of times a second.
