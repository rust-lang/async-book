use std::net::TcpListener;
use std::net::TcpStream;

// ANCHOR: main_func
use async_std::task::spawn;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        spawn(handle_connection(stream));
    }
}
// ANCHOR_END: main_func

async fn handle_connection(mut stream: TcpStream) {
    //<-- snip -->
}
