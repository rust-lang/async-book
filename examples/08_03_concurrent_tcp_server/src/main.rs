use std::net::TcpListener;
use std::net::TcpStream;

// ANCHOR: main_func
use async_std::task::{block_on, spawn};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    block_on(async {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            spawn(async {handle_connection(stream).await} );
        }
    })
}
// ANCHOR_END: main_func

async fn handle_connection(mut stream: TcpStream) {
    //<-- snip -->
}
