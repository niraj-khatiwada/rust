use std::{thread, time};
use std::sync::Arc;

use parking_lot::Mutex;
use rand::Rng;

fn main() {
    let shared_balance = Arc::new(Mutex::new(200));
    let balance_t1 = shared_balance.clone();
    let balance_t2 = shared_balance.clone();

    let handle1 = thread::spawn(move || {
        let mut b = balance_t1.lock();
        let amount = 100;
        if b.le(&amount) {
            println!("Cannot withdraw amount greater than balance.");
            return;
        }
        let random = rand::thread_rng().gen_range(100..=1000);
        thread::sleep(time::Duration::from_millis(random));
        *b -= amount;
        println!("{} Withdraw successful. Balance remaining = {}", amount, b);
    });
    let handle2 = thread::spawn(move || {
        let mut b = balance_t2.lock();
        let amount = 100;
        if b.le(&amount) {
            println!("Cannot withdraw amount greater than balance.");
            return;
        }
        let random = rand::thread_rng().gen_range(100..=1000);
        thread::sleep(time::Duration::from_millis(random));
        *b -= amount;
        println!("{} Withdraw successful. Balance remaining = {}", amount, b);
    });

    handle1.join().expect("Error handling thread 1");
    handle2.join().expect("Error handling thread 2");

    println!("Balance {}", &shared_balance.lock());
}


