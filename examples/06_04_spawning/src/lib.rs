#![cfg(test)]
#![allow(dead_code)]

// ANCHOR: example
use async_std::{task, net::TcpListener, net::TcpStream};
use futures::AsyncWriteExt;

async fn process_request(stream: &mut TcpStream) -> Result<(), std::io::Error>{
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    stream.write_all(b"Hello World").await?;
    Ok(())
}

async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        // Accept a new connection
        let (mut stream, _) = listener.accept().await.unwrap();
        // Now process this request without blocking the main loop
        task::spawn(async move {process_request(&mut stream).await});
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn run_example() {
        main().await
    }
}