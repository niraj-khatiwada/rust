use std::{process, thread, time};

fn main() {
    println!("Process ID {}", process::id());
    println!("Main thread {:?}", thread::current().id());
    println!("Available threads {:?}", thread::available_parallelism());


    let start = time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    //
    let handle1 = thread::spawn(|| {
        for e in 0..=100_00_00 {
            println!("1 -> {e}")
        }
    });
    let handle2 = thread::spawn(|| {
        for e in 0..=100_00_00 {
            println!("2 -> {e}")
        }
    });
    handle1.join().expect("Error joining thread 1.");
    handle2.join().expect("Error joining thread 2.");
    //
    // synchronous();
    //
    let end = time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    println!("Time takes to complete {}", end - start)
}

fn synchronous() {
    for e in 0..=100_00_00 {
        println!("{e}");
    }
    for e in 0..=100_00_00 {
        println!("{e}");
    }
}

