use backoff::{self, ExponentialBackoff};
/** Deadlocks occurs when:
- Using multiple locks
> NOTE: Try to run this program multiple times and at certain point, it'll hang due to deadlock.
    At a moment, we'll try to get the lock of the account which has already been lock due to first transaction.
    SOLUTION: Any subsequent transaction needs to wait for first lock to be released.
**/
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type ArcAccount = Arc<Mutex<Account>>;

struct Account {
    name: String,
    balance: u32,
}

fn main() {
    let person1 = Arc::new(Mutex::new(Account {
        name: String::from("Jon"),
        balance: 100,
    }));
    let person2 = Arc::new(Mutex::new(Account {
        name: String::from("Doe"),
        balance: 200,
    }));

    let p1_clone = person1.clone();
    let p2_clone = person2.clone();

    let h1 = thread::spawn(move || {
        // transfer(person1, person2, 10);
        // safe_transfer(person1, person2, 10);
        safe_transfer_with_exponent_backoff(person1, person2, 10);
    });

    let h2 = thread::spawn(move || {
        // transfer(p2_clone, p1_clone, 20);
        // safe_transfer(p2_clone, p1_clone, 20);
        safe_transfer_with_exponent_backoff(p2_clone, p1_clone, 20);
    });
    h1.join().unwrap();
    h2.join().unwrap();
}

// Unsafe: Can cause deadlock
fn transfer(from: ArcAccount, to: ArcAccount, amount: u32) {
    let mut from = from.lock();
    let mut to = to.lock();

    from.balance -= amount;
    to.balance += amount;

    thread::sleep(Duration::from_millis(10));
}

// Safe: Will wait for previous locks to release
fn safe_transfer(from: ArcAccount, to: ArcAccount, amount: u32) {
    loop {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                break;
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn safe_transfer_with_exponent_backoff(from: ArcAccount, to: ArcAccount, amount: u32) {
    let _ = backoff::retry(ExponentialBackoff::default(), || {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                println!("{} transferred ${} to {}", from.name, amount, to.name);
                println!(
                    "Remaining balance for {}=${} and {}=${}",
                    from.name, from.balance, to.name, to.balance
                );
                return Ok(());
            }
        }
        Err(0)?
    });
}
