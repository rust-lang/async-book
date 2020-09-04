#![cfg(test)]

use futures::executor::block_on;

mod first {
// ANCHOR: hello_world
// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
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
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
// ANCHOR_END: block_on_main

#[test]
fn run_main() { main() }
}
