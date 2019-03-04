#![feature(async_await, await_macro, futures_api)]

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


async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

	println!("Got request at {:?}", req.uri());

    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
}


async fn serve_proxy(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

	println!("Got request at {:?}", req.uri());

    let url_str = "http://www.columbia.edu/~fdc/sample.html";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");

    let res = await!(Client::new().get(url));

    dbg!( &res );
    // Return the result of the request directly to the user
    println!("request finished --returning response");

    res
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
            serve_proxy(req).boxed().compat()
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
