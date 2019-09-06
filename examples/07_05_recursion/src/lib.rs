#![cfg(test)]
#![feature(async_await)]
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
