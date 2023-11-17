use std::{thread, time};

use rand::Rng;

type Balance = i32;

static mut BALANCE: Balance = 200;

fn main() {
    let handle1 = thread::spawn(|| unsafe {
        match withdraw(100) {
            Ok(msg) => println!("{msg}"),
            Err(msg) => println!("{msg}")
        };
    });
    let handle2 = thread::spawn(|| unsafe {
        match withdraw(150) {
            Ok(msg) => println!("{msg}"),
            Err(msg) => println!("{msg}")
        };
    });

    handle1.join().expect("Error handling thread 1");
    handle2.join().expect("Error handling thread 2");
    unsafe { println!("Balance {}", BALANCE); }
}

unsafe fn withdraw(amount: i32) -> Result<String, String> {
    if amount > BALANCE {
        return Err(String::from("Cannot withdraw amount greater than balance."));
    }
    let random = rand::thread_rng().gen_range(100..=1000);
    thread::sleep(time::Duration::from_millis(random));
    BALANCE -= amount;
    return Ok(format!("{} Withdraw successful. Balance remaining = {}", amount, BALANCE));
}


// use std::{thread, time};
//
// use rand::Rng;
//
// fn main() {
//     let balance = 200;
//     let mut balance_1 = balance.clone();
//     let mut balance_2 = balance.clone();
//     let handle1 = thread::spawn(move || {
//         match withdraw(100, &mut balance_1) {
//             Ok(msg) => println!("{msg}"),
//             Err(msg) => println!("{msg}")
//         };
//     });
//     let handle2 = thread::spawn(move || unsafe {
//         match withdraw(150, &mut balance_2) {
//             Ok(msg) => println!("{msg}"),
//             Err(msg) => println!("{msg}")
//         };
//     });
//
//     handle1.join().expect("Error handling thread 1");
//     handle2.join().expect("Error handling thread 2");
//     println!("Balance {}", &balance);
// }
//
// fn withdraw(amount: i32, balance: &mut i32) -> Result<String, String> {
//     if amount > *balance {
//         return Err(String::from("Cannot withdraw amount greater than balance."));
//     }
//     let random = rand::thread_rng().gen_range(100..=1000);
//     thread::sleep(time::Duration::from_millis(random));
//     *balance -= amount;
//     return Ok(format!("{} Withdraw successful. Balance remaining = {}", amount, *balance));
// }
//



