#![cfg(test)]
#![feature(async_await)]

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
        // Extension trait for futures 0.1 futures, adding the `.compat()` method
        // which allows us to use `.await` on 0.1 futures.
        compat::Future01CompatExt,
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
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
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.compat().await {
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
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    // Finally, we can run the future to completion using the `run` function
    // provided by Hyper.
    run(futures_01_future);
}

#[test]
fn run_main_and_query_http() -> Result<(), failure::Error> {
    std::thread::spawn(|| main());
    // Unfortunately, there's no good way for us to detect when the server
    // has come up, so we sleep for an amount that should hopefully be
    // sufficient :(
    std::thread::sleep(std::time::Duration::from_secs(5));
    let response = reqwest::get("http://localhost:3000")?.text()?;
    assert_eq!(response, "hello, world!");
    Ok(())
}

mod proxy {
    use super::*;
    #[allow(unused)]
    async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let url_str = "http://www.rust-lang.org/en-US/";
        let url = url_str.parse::<Uri>().expect("failed to parse URL");

        let res = Client::new().get(url).compat().await;
        // Return the result of the request directly to the user
        println!("request finished-- returning response");
        res
    }
}
