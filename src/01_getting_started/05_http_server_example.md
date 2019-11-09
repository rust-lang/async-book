# Applied: Simple HTTP Server

Let's use `async`/`.await` to build an echo server!

To start, run `rustup update stable` to make sure you've got stable Rust 1.39 or newer. Once you've done that, run
`cargo new async-await-echo` to create a new project, and open up
the resulting `async-await-echo` folder.

Let's add some dependencies to the `Cargo.toml` file:

```toml
{{#include ../../examples/01_05_http_server/Cargo.toml:9:18}}
```

Now that we've got our dependencies out of the way, let's start writing some
code. We have some imports to add:

```rust
{{#include ../../examples/01_05_http_server/src/lib.rs:imports}}
```

Once the imports are out of the way, we can start putting together the
boilerplate to allow us to serve requests:

```rust
{{#include ../../examples/01_05_http_server/src/lib.rs:boilerplate}}
```

If you `cargo run` now, you should see the message "Listening on
http://127.0.0.1:3000" printed on your terminal. If you open that URL in your
browser of choice, you'll see "hello, world!" appear in your browser.
Congratulations! You just wrote your first asynchronous webserver in Rust.

You can also inspect the request itself, which contains information such as
the request URI, HTTP version, headers, and other metadata. For example, we
can print out the URI of the request like this:

```rust
println!("Got request at {:?}", req.uri());
```

You may have noticed that we're not yet doing
anything asynchronous when handling the request-- we just respond immediately,
so we're not taking advantage of the flexibility that `async fn` gives us.
Rather than just returning a static message, let's try proxying the user's
request to another website using Hyper's HTTP client.

We start by parsing out the URL we want to request:

```rust
{{#include ../../examples/01_05_http_server/src/lib.rs:parse_url}}
```

Then we can create a new `hyper::Client` and use it to make a `GET` request,
returning the response to the user:

```rust
{{#include ../../examples/01_05_http_server/src/lib.rs:get_request}}
```

`Client::get` returns a `hyper::client::FutureResponse`, which implements
`Future<Output = Result<Response, Error>>`
(or `Future<Item = Response, Error = Error>` in futures 0.1 terms).
When we `.await` that future, an HTTP request is sent out, the current task
is suspended, and the task is queued to be continued once a response has
become available.

Now, if you `cargo run` and open `http://127.0.0.1:3000/foo` in your browser,
you'll see the Rust homepage, and the following terminal output:

```
Listening on http://127.0.0.1:3000
Got request at /foo
making request to http://www.rust-lang.org/en-US/
request finished-- returning response
```

Congratulations! You just proxied an HTTP request.
