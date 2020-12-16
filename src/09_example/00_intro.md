# Final Project: Building a Concurrent Web Server with Async Rust
In this chapter, we'll use asynchronous Rust to modify the Rust book's 
[single-threaded web server](https://doc.rust-lang.org/book/ch20-01-single-threaded.html) 
to serve requests concurrently.
## Recap
Here's what the code looked like at the end of the lesson.

`src/main.rs`:
```rust
{{#include ../../examples/09_01_sync_tcp_server/src/main.rs}}
```

`hello.html`:
```html
{{#include ../../examples/09_01_sync_tcp_server/hello.html}}
```

`404.html`:
```html
{{#include ../../examples/09_01_sync_tcp_server/404.html}}
```

If you run the server with `cargo run` and visit `127.0.0.1:7878` in your browser,
you'll be greeted with a friendly message from Ferris!