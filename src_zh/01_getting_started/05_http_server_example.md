# 应用：建立HTTP服务器

让我们用`async`/`.await`来构建一个回显（echo）服务器吧！

首先，运行`rustup update nightly`，确保我们用的是伟大的Rust的最新拷贝——我们要用最新潮
的特性，所以保持更新很必要。然后，运行`cargo +nightly new async-await-echo`来创建
新项目，并打开生成的`async-await-echo`文件夹。

现在给`Cargo.toml`文件添加依赖:

```toml
{{#include ../../examples_zh/01_05_http_server/Cargo.toml:9:18}}
```

现在我们搞定了依赖，让我们开始写代码。打开`src/main.rs`并在文件开头启用`async_await`
特性:

```rust,no_run
#![feature(async_await)]
```

这使我们现在就可以使用夜版专属（nightly-only）的`async`/`await`语法。这个语法很快就会
稳定的。

此外，我们还要加些导入（import）：

```rust,no_run
{{#include ../../examples_zh/01_05_http_server/src/lib.rs:imports}}
```

导入之后，我们就能开始拼模板写服务了：

```rust,no_run
{{#include ../../examples_zh/01_05_http_server/src/lib.rs:boilerplate}}
```

如果你此时执行`cargo run`，你应该能看到"Listening on http://127.0.0.1:3000"打印到
终端上。如果你用浏览器打开这URL，你会看到"Hello, world!"。可喜可贺！你刚用Rust写了
第一个异步Web服务器！

你也可以检查请求，里面包含了很多信息，像请求URI，HTTP版本，报文头和其他元数据。例如，
我们可以输出请求URI，像这样：

```rust,no_run
println!("Got request at {:?}", req.uri());
```

你可能注意到我们在处理请求时还没有做任何异步操作——我们立刻返回，所以我们并没有充分利用
`async fn`函数提供给我们的灵活优势。比起返回静态信息，我们来试着来用Hyper的HTTP客户端
把用户请求代理到另外的网站。

我们从解析我们想要发送请求的URL开始：

```rust,no_run
{{#include ../../examples_zh/01_05_http_server/src/lib.rs:parse_url}}
```

然后我们创建一个`hyper::Client`，并用它发送`Get`请求，并返回响应给用户： 

```rust,no_run
{{#include ../../examples_zh/01_05_http_server/src/lib.rs:get_request}}
```

`Client::get`返回一个`hyper::client::FutureResponse`, 它实现了
`Future<Output = Result<Response, Error>>`
（或者在futures 0.1我们叫`Future<Item = Response, Error = Error>`）。
当我们`.await`这个future时, 发送了一个HTTP request, 当前任务挂起了，然后排队等待响应
可用时继续执行。

现在，如果你执行`cargo run`并打开`http://127.0.0.1:3000/foo`，你会看到Rust主页，以及
以下命令行输出：

```
Listening on http://127.0.0.1:3000
Got request at /foo
making request to http://www.rust-lang.org/en-US/
request finished-- returning response
```

再次恭喜！你刚刚代理了一个HTTP请求！
