# Applied: Simple HTTP Server

Let's use `async`/`await!` to build an echo server!

To start, run `rustup update nightly` to make sure you've got the latest and
greatest copy of Rust-- we're working with bleeding-edge features, so it's
essential to stay up-to-date. Once you've done that, run
`cargo +nightly new async-await-echo` to create a new project, and open up
the resulting `async-await-echo` folder.

Let's add some dependencies to the `Cargo.toml` file:

```toml
[dependencies]

# The latest version of the "futures" library, which has lots of utilities
# for writing async code. Enable the "tokio-compat" feature to include the
# functions for using futures 0.3 and async/await with the Tokio library.
futures-preview = { version = "0.3.0-alpha.13", features = ["compat"] }

# Hyper is an asynchronous HTTP library. We'll use it to power our HTTP
# server and to make HTTP requests.
hyper = "0.12.25"

# Tokio is a runtime for asynchronous I/O applications. Hyper uses
# it for the default server runtime. The `tokio` crate also provides an
# an `await!` macro similar to the one in `std`, but it supports `await!`ing
# both futures 0.1 futures (the kind used by Hyper and Tokio) and
# futures 0.3 futures (the kind produced by the new `async`/`await!` language
# feature).
tokio = { version = "0.1.16", features = ["async-await-preview"] }
```

Now that we've got our dependencies out of the way, let's start writing some
code. Open up `src/main.rs` and enable the following features at the top of
the file:

```rust
#![feature(async_await, await_macro, futures_api)]
```

- `async_await` adds support for the `async fn` syntax.
- `await_macro` adds support for the `await!` macro.
- `futures_api` adds support for the nightly `std::future` and `std::task`
modules which define the core `Future` trait and dependent types.

Additionally, we have some imports to add:

```rust
use {
    hyper::{
        // Miscellaneous types from Hyper for working with HTTP.
        Body, Client, Request, Response, Server, Uri,

        // This function turns a closure which returns a future into an
        // implementation of the the Hyper `Service` trait, which is an
        // asynchronous function from a generic `Request` to a `Response`.
        service::service_fn,

        // A function which runs a future to completion using the Hyper runtime.
        rt::run,
    },
    futures::{
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,

    // This is the redefinition of the await! macro which supports both
    // futures 0.1 (used by Hyper and Tokio) and futures 0.3 (the new API
    // exposed by `std::future` and implemented by `async fn` syntax).
    tokio::await,
};
```

Once the imports are out of the way, we can start putting together the
boilerplate to allow us to serve requests:

```rust
async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    unimplemented!()
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // Create a server bound on the provided address
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async serve_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response. In order to use our `serve_req`
        // function with Hyper, we have to box it and put it in a compatability
        // wrapper to go from a futures 0.3 future (the kind returned by
        // `async fn`) to a futures 0.1 future (the kind used by Hyper).
        .serve(|| service_fn(|req|
            serve_req(req).boxed().compat()
        ));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = await!(serve_future) {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    // Set the address to run our socket on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Call our run_server function, which returns a future.
    // As with every `async fn`, we need to run that future in order for
    // `run_server` to do anything. Additionally, since `run_server` is an
    // `async fn`, we need to convert it from a futures 0.3 future into a
    // futures 0.1 future.
    let futures_03_future = run_server(addr);
    let futures_01_future =
        futures_03_future.unit_error().boxed().compat();

    // Finally, we can run the future to completion using the `run` function
    // provided by Hyper.
    run(futures_01_future);
}
```

If you `cargo run` now, you should see the message "Listening on
http://127.0.0.1:300" printed on your terminal. If you open that URL in your
browser of choice, you'll see "thread ... panicked at 'not yet implemented'."
Great! Now we just need to actually handle requests. To start, let's just
return a static message:

```rust
async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
}
```

If you `cargo run` again and refresh the page, you should see "hello, world!"
appear in your browser. Congratulations! You just wrote your first asynchronous
webserver in Rust.

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
let url_str = "http://www.rust-lang.org/en-US/";
let url = url_str.parse::<Uri>().expect("failed to parse URL");
```

Then we can create a new `hyper::Client` and use it to make a `GET` request,
returning the response to the user:

```rust
let res = await!(Client::new().get(url));
// Return the result of the request directly to the user
println!("request finished --returning response");
res
```

`Client::get` returns a `hyper::client::FutureResponse`, which implements
`Future<Output = Result<Response, Error>>`
(or `Future<Item = Response, Error = Error>` in futures 0.1 terms).
When we `await!` that future, an HTTP request is sent out, the current task
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
