use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Main thread {:?}", thread::current().id());

    do_something().await;
    do_something().await;
    do_something().await;
}

async fn do_something() {
    println!("Current thread {:?}", thread::current().id());
    thread::sleep(Duration::from_secs(1));
}
