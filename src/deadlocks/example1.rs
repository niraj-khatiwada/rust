/** Deadlocks occurs when:
- Using multiple locks
- Locking the same lock twice (in recursion)  -> This example shows this
**/
use parking_lot::{Mutex, ReentrantMutex};
use std::sync::Arc;

// type ArcNum = Arc<Mutex<u32>>; // This will deadlock
type ArcNum = Arc<ReentrantMutex<u32>>; // Will work. 
fn main() {
    let n = 5;
    let data = Arc::new(ReentrantMutex::new(n));
    println!("{}", factorial(data, n));
}

// Example showing how to lock same mutex multiple times using ReentrantMutex
fn factorial(data: ArcNum, num: u32) -> u32 {
    let _u = data.lock(); // use ReentrantMutex instead of normal Mutex, if we need to lock the same mutex multiple times.
    match num {
        1 => 1,
        _ => (num - 1) * factorial(data.clone(), num - 1),
    }
}
