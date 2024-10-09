# Asynchronous Programming in Rust

This book aims to be a thorough guide to asynchronous programming in Rust, from beginner to advanced.

This book has been unmaintained for a long time and has not had a lot of love. We're currently working to bring it up to date and make it much better! As we're making some major changes, the content might be a bit mixed up, parts may be duplicated or missing, etc. Bear with us, it'll get better soon :-) To see what we're planning and to let us know what you think, see [issue 224](https://github.com/rust-lang/async-book/issues/224).

## Requirements

The async book is built with [`mdbook`] ([docs](https://rust-lang.github.io/mdBook/index.html)), you can install it using cargo.

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
