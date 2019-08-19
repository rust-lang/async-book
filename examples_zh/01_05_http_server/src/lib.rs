#![cfg(test)]
#![feature(async_await)]

// ANCHOR: imports
use {
    hyper::{
        // `Hyper` 中用于处理 `HTTP` 的类型.
        Body, Client, Request, Response, Server, Uri,

        // 这个函数将一个 `future` 的闭包返回转换为 `Hyper Server` trait.
        // 这是一个从普通的请求到响应的异步函数.
        service::service_fn,

        // 这个函数使用 `Hyper` 运行时运行 `future` 直到完成.
        rt::run,
    },
    futures::{
        // `futures 0.1` 的扩展 `trait`, 加上这个 `.compat()` 方法
        // 这可以使我们在 `futures 0.1` 上使用 `.await`.
        compat::Future01CompatExt,
        // 扩展的 `trait` 提供对 `future` 的额外补充方法.
        // `FutureExt` 增加了适用于所有 `future` 的方法,
        // 而 `TryFutureExt` 则向返回 `Result` 类型的 `future` 添加方法.
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,
};
// ANCHOR_END: imports

// ANCHOR: boilerplate
async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 总是返回一个包含 `hello, world!` 的成功响应.
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // 在指定的地址上创建服务器.
    let serve_future = Server::bind(&addr)
        // 使用 `async serve_req` 函数来处理请求.
        // `serve` 接受一个闭包，它将返回一个实现了 `Service` trait的类型.
        // `service_fn` 返回一个实现了 `Service` trait的值，并接受一个从
        // 请求到响应的 `future` 闭包, 要在 `Hyper` 中使用 `serve_req` 函
        // 数，我们必须将它打包好并将其放入兼容性容器中，以便从 `futures 0.3`
        // (由 `async fn`返回的那种) 转换到 `futures 0.1` （由 `Hyper` 使
        // 用的那种 ）.
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));
    
    // 等待服务器完成服务或者因错误退出.  
    // 如果发生错误，将错误输出到 `stderr`.
    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    // 设置 `socket` 地址.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 调用我们的 `run_server` 函数, 它将返回一个 `future`.
    // 和 `async fn` 一样, 要让 `run_server` 执行任何操作,
    // 都需要运行返回的 `future`. 并且我们需要将返回的 
    // `future` 从 `0.3` 转换为 `0.1`.
    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    // 最后，我们使用 `Hyper` 提供的 `run` 函数来运行未完成的 `future`.
    run(futures_01_future);
}
// ANCHOR_END: boilerplate

#[test]
fn run_main_and_query_http() -> Result<(), failure::Error> {
    std::thread::spawn(|| main());
  
    // 不好的一点是，我们没有很好的办法来检查服务器何时创建完成，所以我们
    // 将线程休眠到合适的时间点.
    std::thread::sleep(std::time::Duration::from_secs(5));
    let response = reqwest::get("http://localhost:3000")?.text()?;
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
let res = Client::new().get(url).compat().await;
// 将请求的结果直接返回给调用者.
println!("request finished-- returning response");
res
// ANCHOR_END: get_request
    }
}
