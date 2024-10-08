// Define an async function.
async fn say_hello() {
    println!("hello, world!");
}

#[tokio::main] // Boilerplate which lets us write `async fn main`, we'll explain it later.
async fn main() {
    // Call an async function and await its result.
    say_hello().await;
}
