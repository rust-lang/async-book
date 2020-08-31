// ANCHOR: main_func
use async_std::net::{TcpListener, TcpStream};
use async_std::task::{block_on, spawn};

fn main() {
    block_on(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            spawn(handle_connection(stream));
        }
    })
}
// ANCHOR_END: main_func

// ANCHOR: handle_connection
use async_std::prelude::*;

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    //<-- snip -->
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
// ANCHOR_END: handle_connection
