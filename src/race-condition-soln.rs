use parking_lot::Mutex;
use std::sync::Arc;
use std::{thread, time::Duration};

use rand::Rng;

fn main() {
    let balance: Arc<Mutex<f64>> = Arc::new(Mutex::new(100_f64));

    let balance1 = balance.clone();
    let balance2 = balance.clone();

    let handle1 = thread::spawn(move || {
        withdraw_balance(100.0, balance1);
    });

    let handle2 = thread::spawn(move || {
        withdraw_balance(50.0, balance2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final balance is {:?}", balance.to_owned());
}

fn withdraw_balance(amount: f64, balance: Arc<Mutex<f64>>) {
    let mut balance_lock = balance.lock();
    let current_balance = balance_lock.to_owned();
    println!("Current Balance {:?}", current_balance);
    if amount > current_balance {
        println!("Not enough balance.")
    } else {
        let random = rand::thread_rng().gen_range(0..100);
        thread::sleep(Duration::from_millis(random));
        *balance_lock -= amount;
        let remaining_balance = balance_lock.to_owned();
        println!(
            "Withdraw successful. Amount withdrawn={}. Remaining amount={}",
            amount, remaining_balance
        );
    }
}
