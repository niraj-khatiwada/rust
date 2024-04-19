use rand::Rng;
use std::thread::{self, sleep};
use std::time::Duration;

static mut balance: f64 = 200_f64;

fn main() {
    let thread1 = thread::spawn(move || unsafe {
        if let Err(message) = withdraw_balance(100_f64) {
            println!("{}", message);
        }
    });
    let thread2 = thread::spawn(move || unsafe {
        if let Err(message) = withdraw_balance(150_f64) {
            println!("{}", message);
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    unsafe {
        println!("Final balance is {}", balance);
    }
}

unsafe fn withdraw_balance(amount: f64) -> Result<bool, String> {
    if amount > balance {
        return Err(String::from(
            "Cannot withdraw amount greater than current balance",
        ));
    }
    let mut thread_rng = rand::thread_rng();
    let random = thread_rng.gen_range(1..=10);

    sleep(Duration::from_millis(random));
    balance -= amount;

    println!(
        "Withdraw of {} amount successful. New balance is {}",
        amount, balance
    );

    Ok(true)
}
