use std::{thread, time::Duration};

use rand::Rng;

static mut BALANCE: f64 = 100_f64;

fn main() {
    let handle1 = thread::spawn(|| unsafe {
        withdraw_balance(100.0);
    });

    let handle2 = thread::spawn(|| unsafe {
        withdraw_balance(50.0);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final balance is {}", BALANCE);
    }
}

unsafe fn withdraw_balance(amount: f64) {
    println!("Current Balance {}", BALANCE);
    if amount > BALANCE {
        println!("Not enough balance.")
    } else {
        let random = rand::thread_rng().gen_range(0..100);
        thread::sleep(Duration::from_millis(random));
        BALANCE -= amount;
        println!(
            "Withdraw successful. Amount withdrawn={}. Remaining amount={}",
            amount, BALANCE
        );
    }
}
