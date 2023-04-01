#![allow(unused_variables)]
#![allow(dead_code)]

extern crate futures;
use futures::prelude::*;
use futures::stream::FuturesUnordered;

async fn download_async(_url: &&str) {
    // ...
}

// ANCHOR: simple
async fn simple() {
    let sites = [
        "https://www.foo.com",
        "https://www.bar.com",
        "https://www.foobar.com",
    ];

    // Create a empty FuturesUnordered
    let mut futures = FuturesUnordered::new();

    // Push all the futures into the FuturesUnordered
    for site in sites.iter() {
        futures.push(download_async(site));
    }

    // Poll all the futures by calling next until it returns None.
    while let Some(returnvalue) = futures.next().await {
        // Do something with the returnvalue
    }
}
// ANCHOR_END: simple

// ANCHOR: collect
async fn collect() {
    let sites = [
        "https://www.foo.com",
        "https://www.bar.com",
        "https://www.foobar.com",
    ];

    // Construct all the futures and collect them in the FuturesUnordered struct
    let mut futures: FuturesUnordered<_> =
        sites.iter().map(download_async).collect();

    // Poll all the futures by calling next until it returns None.
    while let Some(returnvalue) = futures.next().await {
        // Do something with the returnvalue
    }
}
// ANCHOR_END: collect
