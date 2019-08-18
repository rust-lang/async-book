#![cfg(test)]
#![feature(async_await)]

mod stream_trait {
use {
    futures::stream::{Stream as RealStream},
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

// ANCHOR: stream_trait
trait Stream {
    /// 由 `stream` 产生的值的类型.
    type Item;

    /// 尝试解析 `stream` 中的下一项.
    /// 如果已经准备好，就重新运行 `Poll::Pending`, 如果已经完成，就重新
    /// 运行`Poll::Ready(Some(x))`，如果已经完成，就重新运行 `Poll::Ready(None)`.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>;
}
// ANCHOR_END: stream_trait

// 为 `RealStream` 实现 `Stream` trait:
impl<I> Stream for dyn RealStream<Item = I> {
    type Item = I;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>
    {
        RealStream::poll_next(self, cx)
    }
}
}

mod channels {
use {
    futures::{
        channel::mpsc,
        prelude::*,
    },
};

// ANCHOR: channels
async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但会返回一个实现
    // 了 `Future<Output = Option<T>>` 的类型.
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}
// ANCHOR_END: channels

#[test]
fn run_send_recv() { futures::executor::block_on(send_recv()) }
}
