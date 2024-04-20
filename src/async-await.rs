use std::{thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    withdraw(100_f64).await;
    println!("This will run after 1s");
}

async fn withdraw(amount: f64) {
    println!("Amount {} withdrawn.", amount);
    sleep(Duration::from_millis(1000))
}
