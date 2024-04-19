use std::{
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};

use parking_lot::Mutex;
use rand::Rng;

struct Account {
    balance: f64,
}

type ArcAccount = Arc<Mutex<Account>>;

fn main() {
    let account_from = Arc::new(Mutex::new(Account { balance: 200_f64 }));
    let account_to = Arc::new(Mutex::new(Account { balance: 100_f64 }));

    let account_from_a = account_from.clone();
    let account_to_a = account_to.clone();

    let account_from_b = account_from.clone();
    let account_to_b = account_to.clone();

    let thread1 = thread::spawn(move || {
        withdraw(account_from_a, account_to_a, 10_f64);
    });

    let thread2 = thread::spawn(move || {
        withdraw(account_to_b, account_from_b, 10_f64);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    thread::sleep(Duration::from_millis(1000));
    println!(
        "{}, {}",
        account_from.lock().balance,
        account_to.lock().balance
    );
}

fn withdraw(from: ArcAccount, to: ArcAccount, amount: f64) {
    sleep(Duration::from_millis(
        rand::thread_rng().gen_range(1..=1000),
    ));
    let mut from = from.lock();
    sleep(Duration::from_millis(
        rand::thread_rng().gen_range(1..=1000),
    ));
    let mut to = to.lock();

    from.balance -= amount;
    to.balance += amount;
}
