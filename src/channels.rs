use crossbeam_channel::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx_main, rx_main): (Sender<String>, Receiver<String>) = crossbeam_channel::unbounded();
    let (tx_worker, rx_worker) = crossbeam_channel::unbounded();

    let h = thread::spawn(move || {
        loop {
            if let Ok(msg) = rx_worker.recv() {
                println!("Message received in worker {}", msg);
                if msg == String::from("Start") {
                    println!("Starting work now...");
                    for i in 0..10 {
                        tx_main.send(format!("Message {} from worker", i)).unwrap();
                    }
                    break;
                } else {
                    println!("Waiting for start message...")
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(3));

    tx_worker.send(String::from("On your mark")).unwrap();
    tx_worker.send(String::from("Get set")).unwrap();
    tx_worker.send(String::from("Start")).unwrap();

    for msg in rx_main {
        println!("Message received in main \"{}\"", msg);
    }

    h.join().unwrap();
}
