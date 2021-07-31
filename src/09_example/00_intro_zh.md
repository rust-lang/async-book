# 最终的项目：使用异步 Rust 构建一个并发 Web 服务器
在本章中，我们将以 Rust book 中的
[single-threaded web server](https://doc.rust-lang.org/book/ch20-01-single-threaded.html) 
为基础，改进它以便可处理并发请求。
## 总结
这会是我们代码的最终形态：
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

使用 `cargo run` 来启动服务，并在浏览器中访问 `127.0.0.1:7878`，
你将看到 Ferris 带来的友好的问候！
