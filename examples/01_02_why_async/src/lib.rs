#![cfg(test)]
#![feature(async_await)]

use {
    futures::{
        executor::block_on,
        join,
    },
    std::thread,
};

fn download(_url: &str) {
    // ...
}

#[test]
// ANCHOR: get_two_sites
fn get_two_sites() {
    // 生成两个线程来下载网页.
    let thread_one = thread::spawn(|| download("https:://www.foo.com"));
    let thread_two = thread::spawn(|| download("https:://www.bar.com"));

    // 等待两个线程运行下载完成.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
// ANCHOR_END: get_two_sites

async fn download_async(_url: &str) {
    // ...
}

// ANCHOR: get_two_sites_async
async fn get_two_sites_async() {
    // 创建两个不同的 "futures", 当创建完成之后将异步下载网页.
    let future_one = download_async("https:://www.foo.com");
    let future_two = download_async("https:://www.bar.com");

    // 同时运行两个 "futures" 直到完成.
    join!(future_one, future_two);
}
// ANCHOR_END: get_two_sites_async

#[test]
fn get_two_sites_async_test() {
    block_on(get_two_sites_async());
}
