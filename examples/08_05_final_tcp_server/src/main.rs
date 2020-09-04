use std::fs;
use std::time::{Duration, Instant};

use futures::join;

use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task::spawn;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        spawn(handle_connection(stream));
    }
}

use async_std::io::{Read, Write};
use std::marker::Unpin;

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

// ANCHOR: slow_functions
use async_std::task::sleep;

async fn write_to_database() {
    // Simulate a slow request
    sleep(Duration::from_secs(2)).await;
}

async fn add_to_queue() {
    // Simulate a slow request
    sleep(Duration::from_secs(1)).await;
}
// ANCHOR_END: slow_functions

async fn foo() {
    // ANCHOR: serial_execution
    let now = Instant::now();
    write_to_database().await;
    add_to_queue().await;
    println!(
        "Write to database + add to queue took {} seconds",
        now.elapsed().as_secs()
    );
    // ANCHOR_END: serial_execution
}

async fn bar() {
    // ANCHOR: parallel_execution
    let now = Instant::now();
    join!(write_to_database(), add_to_queue());
    println!(
        "Write to database + add to queue took {} seconds",
        now.elapsed().as_secs()
    );
    // ANCHOR_END: parallel_execution
}

#[cfg(test)]

mod tests {
    // ANCHOR: mock_read
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};

    use std::cmp::min;
    use std::pin::Pin;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf.copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }
    // ANCHOR_END: mock_read

    // ANCHOR: mock_write
    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
            return Poll::Ready(Ok(buf.len()));
        }
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }
    // ANCHOR_END: mock_write

    // ANCHOR: unpin
    use std::marker::Unpin;
    impl Unpin for MockTcpStream {}
    // ANCHOR_END: unpin

    // ANCHOR: test
    use std::fs;

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let expected_contents = fs::read_to_string("hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
    // ANCHOR_END: test
}
