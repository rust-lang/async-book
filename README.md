# async-book
Asynchronous Programming in Rust

## Requirements
The async book is built with [`mdbook`], you can install it using cargo.

```
cargo install mdbook
cargo install mdbook-linkcheck
```

[`mdbook`]: https://github.com/rust-lang/mdBook

## Building

To create a finished book, run `mdbook build` to generate it under the `book/`
directory.

```
mdbook build
```

## Development

While writing it can be handy to see your changes, `mdbook serve` will launch a
local web server to serve the book.

```
mdbook serve
```

# Why?

This is a fork of the [async-book](https://github.com/rust-lang/async-book),
which assumes a pretty advanced level of knowledge of the underlying concepts.
I find that as I read it I need to pull knowledge about a lot of foundational
knowledge, so I'm adding this to my copy for future reference.
