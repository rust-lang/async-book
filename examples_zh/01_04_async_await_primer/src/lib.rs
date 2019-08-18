#![cfg(test)]
#![feature(async_await)]

use futures::executor::block_on;

mod first {
// ANCHOR: hello_world
// `block_on` 将阻塞当前的线程，直到 `future` 运行完成.
// 其他执行器提供了更复杂的特性，比如将多个 `future` 安排到同一个线程上面.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 没有输出.
    block_on(future); // `future` 运行，输出 "hello, world!".
}
// ANCHOR_END: hello_world

#[test]
fn run_main() { main() }
}

struct Song;
async fn learn_song() -> Song { Song }
async fn sing_song(_: Song) {}
async fn dance() {}

mod second {
use super::*;
// ANCHOR: block_on_each
fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
// ANCHOR_END: block_on_each

#[test]
fn run_main() { main() }
}

mod third {
use super::*;
// ANCHOR: block_on_main
async fn learn_and_sing() {
    // 要唱歌必须得先学会歌曲.
    // 我们这里使用 `.await` 而不是 `block_on` 来
    // 防止线程阻塞, 这样也可以同时跳舞.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` 类似 `.await`，但是可以同时等待多个 `future` 执行完成.
    // 如果我们 `learn_and_sing` 这个 `future` 被阻塞, 那么 `dance`
    // 这个 `future` 将接管当前的线程. 如果 `dance` 被阻塞, 那么 `learn_and_sing`
    // 就可以重新开始. 如果这个两个 `future` 都被阻塞, 那么 `async_main`
    // 也将被阻塞并让位给执行程序.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
// ANCHOR_END: block_on_main

#[test]
fn run_main() { main() }
}
