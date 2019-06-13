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
fn get_two_sites() {
    // Spawn two threads to do work.
    let thread_one = thread::spawn(|| download("https:://www.foo.com"));
    let thread_two = thread::spawn(|| download("https:://www.bar.com"));

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

async fn download_async(_url: &str) {
    // ...
}

async fn get_two_sites_async() {
    // Create a two different "futures" which, when run to completion,
    // will asynchronously download the webpages.
    let future_one = download_async("https:://www.foo.com");
    let future_two = download_async("https:://www.bar.com");

    // Run both futures to completion at the same time.
    join!(future_one, future_two);
}

#[test]
fn get_two_sites_async_test() {
    block_on(get_two_sites_async());
}
