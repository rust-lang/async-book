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
use std::time::Duration;
async fn my_task(time: Duration) {
    println!("Hello from my_task with time {:?}", time);
    task::sleep(time).await;
    println!("Goodbye from my_task with time {:?}", time);
}
// ANCHOR: join_all
use futures::future::join_all;
async fn task_spawner(){
    let tasks = vec![
        task::spawn(my_task(Duration::from_secs(1))),
        task::spawn(my_task(Duration::from_secs(2))),
        task::spawn(my_task(Duration::from_secs(3))),
    ];
    // If we do not await these tasks and the function finishes, they will be dropped
    join_all(tasks).await;
}
// ANCHOR_END: join_all

#[test]
fn run_task_spawner() {
    futures::executor::block_on(task_spawner());
}