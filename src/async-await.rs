use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Thread {:?}", thread::current().id());

    // Execution steps occur same way as JavaScript handles the event loop.
    println!("Execution step 1");
    do_something();
    println!("Execution step 2");
    do_something().await;
    println!("Execution step 3");
}

// Futures run on same thread.
async fn do_something<'a>() -> &'a str {
    println!("Thread {:?}", thread::current().id());
    println!("Execution step x");
    thread::sleep(Duration::from_secs(1));
    "Hello Future"
}
