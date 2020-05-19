#![cfg(test)]

// ANCHOR: imports
use {
    hyper::{
        // Following functions are used by Hyper to handle a `Request`
        // and returning a `Response` in an asynchronous manner by using a Future
        service::{make_service_fn, service_fn},
        // Miscellaneous types from Hyper for working with HTTP.
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::net::SocketAddr,
};
// ANCHOR_END: imports

// ANCHOR: boilerplate
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
        // `serve` takes a type which implements the `MakeService` trait.
        // `make_service_fn` converts a closure into a type which
        // implements the `MakeService` trait. That closure must return a
        // type that implements the `Service` trait, and `service_fn`
        // converts a request-response function into a type that implements
        // the `Service` trait.
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(serve_req))
        }));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
  // Set the address to run our socket on.
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  // Call our `run_server` function, which returns a future.
  // As with every `async fn`, for `run_server` to do anything,
  // the returned future needs to be run using `await`;
  run_server(addr).await;
}
// ANCHOR_END: boilerplate

#[test]
fn run_main_and_query_http() -> Result<(), anyhow::Error> {
    std::thread::spawn(main);
    // Unfortunately, there's no good way for us to detect when the server
    // has come up, so we sleep for an amount that should hopefully be
    // sufficient :(
    std::thread::sleep(std::time::Duration::from_secs(5));
    let response = reqwest::blocking::get("http://localhost:3000")?.text()?;
    assert_eq!(response, "hello, world!");
    Ok(())
}

mod proxy {
    use super::*;
    #[allow(unused)]
    async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        // ANCHOR: parse_url
        let url_str = "http://www.rust-lang.org/en-US/";
        let url = url_str.parse::<Uri>().expect("failed to parse URL");
        // ANCHOR_END: parse_url

        // ANCHOR: get_request
        let res = Client::new().get(url).await?;
        // Return the result of the request directly to the user
        println!("request finished-- returning response");
        Ok(res)
        // ANCHOR_END: get_request
    }
}
