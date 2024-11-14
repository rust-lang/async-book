use tokio::{spawn, time::{sleep, Duration}};

async fn say_hello() {
    // Wait for a while before printing to make it a more interesting race.
    sleep(Duration::from_millis(100)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world");
}

#[tokio::main]
async fn main() {
    let handle1 = spawn(say_hello());
    let handle2 = spawn(say_world());
    
    let _ = handle1.await;
    let _ = handle2.await;

    println!("!");
}
