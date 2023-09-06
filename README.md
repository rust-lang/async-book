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
To create a finished book, run `mdbook build` to generate it under the `book/` directory.
```
mdbook build
```

## Development
While writing it can be handy to see your changes, `mdbook serve` will launch a local web
server to serve the book.
```
mdbook serve
```

# What?

This is a fork of the [async-book](https://github.com/rust-lang/async-book), with a few
additional notes and random other stuff that I add as I fill in any gaps I need to.
