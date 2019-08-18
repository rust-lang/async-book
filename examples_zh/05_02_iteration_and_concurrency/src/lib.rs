#![cfg(test)]
#![feature(async_await)]

use {
    futures::{
        executor::block_on,
        stream::{self, Stream},
    },
    std::{
        io,
        pin::Pin,
    },
};

// ANCHOR: nexts
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // 对于 `next`
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // 对于 `try_next`
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}
// ANCHOR_END: nexts

#[test]
fn run_sum_with_next() {
    let mut stream = stream::iter(vec![2, 3]);
    let pin: Pin<&mut stream::Iter<_>> = Pin::new(&mut stream);
    assert_eq!(5, block_on(sum_with_next(pin)));
}

#[test]
fn run_sum_with_try_next() {
    let mut stream = stream::iter(vec![Ok(2), Ok(3)]);
    let pin: Pin<&mut stream::Iter<_>> = Pin::new(&mut stream);
    assert_eq!(5, block_on(sum_with_try_next(pin)).unwrap());
}

#[allow(unused)]
// ANCHOR: try_for_each_concurrent
async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 对于 `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}
// ANCHOR_END: try_for_each_concurrent

async fn jump_n_times(_: u8) -> Result<(), io::Error> { Ok(()) }
async fn report_n_jumps(_: u8) -> Result<(), io::Error> { Ok(()) }
