#![cfg(test)]

struct Book;
struct Music;
async fn get_book() -> Book { Book }
async fn get_music() -> Music { Music }

mod naiive {
use super::*;
// ANCHOR: naiive
async fn get_book_and_music() -> (Book, Music) {
    let book = get_book().await;
    let music = get_music().await;
    (book, music)
}
// ANCHOR_END: naiive
}

mod other_langs {
use super::*;
// ANCHOR: other_langs
// WRONG -- don't do this
async fn get_book_and_music() -> (Book, Music) {
    let book_future = get_book();
    let music_future = get_music();
    (book_future.await, music_future.await)
}
// ANCHOR_END: other_langs
}

mod join {
use super::*;
// ANCHOR: join
use futures::join;

async fn get_book_and_music() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    join!(book_fut, music_fut)
}
// ANCHOR_END: join
}

mod try_join {
use super::{Book, Music};
// ANCHOR: try_join
use futures::try_join;

async fn get_book() -> Result<Book, String> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}
// ANCHOR_END: try_join
}

mod mismatched_err {
use super::{Book, Music};
// ANCHOR: try_join_map_err
use futures::{
    future::TryFutureExt,
    try_join,
};

async fn get_book() -> Result<Book, ()> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}
// ANCHOR_END: try_join_map_err
}
