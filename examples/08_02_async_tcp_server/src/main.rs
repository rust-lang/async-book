use std::net::TcpListener;
use std::net::TcpStream;

// ANCHOR: main_func
#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Warning: This is not concurrent!
        handle_connection(stream).await;
    }
}
// ANCHOR_END: main_func

// ANCHOR: handle_connection_async
async fn handle_connection(mut stream: TcpStream) {
    //<-- snip -->
}
// ANCHOR_END: handle_connection_async
