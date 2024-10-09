#![cfg(test)]
#![allow(dead_code)]

// ANCHOR: example
use futures::future::{BoxFuture, FutureExt};

fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }.boxed()
}
// ANCHOR_END: example

// ANCHOR: example_pinned
async fn recursive_pinned() {
    Box::pin(recursive_pinned()).await;
    Box::pin(recursive_pinned()).await;
}
// ANCHOR_END: example_pinned
