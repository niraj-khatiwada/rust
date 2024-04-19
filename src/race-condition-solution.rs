use parking_lot::Mutex;
use rand::Rng;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let balance = Arc::new(Mutex::new(200_f64));

    let balance_t1 = balance.clone();
    let balance_t2 = balance.clone();

    let thread1 = withdraw_balance(100_f64, balance_t1);
    let thread2 = withdraw_balance(150_f64, balance_t2);

    thread1.join().unwrap();
    thread2.join().unwrap();

    thread::sleep(Duration::from_millis(1000));
    println!("Final balance is {}", balance.lock())
}

fn withdraw_balance(amount: f64, balance: Arc<Mutex<f64>>) -> JoinHandle<()> {
    let thread = thread::spawn(move || {
        let mut locked_balance = balance.lock();

        if amount > locked_balance.to_owned() {
            println!("Cannot withdraw amount greater than current balance.");
            return;
        }

        let random_ms = rand::thread_rng().gen_range(1..1000);
        thread::sleep(Duration::from_millis(random_ms));

        *locked_balance -= amount;

        println!(
            "Withdraw of amount {} successful. New balance is {}",
            amount, &locked_balance,
        )
    });

    thread
}
