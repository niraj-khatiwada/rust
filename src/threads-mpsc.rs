use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("CPU count {:?}", thread::available_parallelism());

    let h = thread::spawn(|| {
        thread::sleep(Duration::from_secs(3));
        println!("{:?}", thread::current().id());
    });

    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(); // mpsc = multiple producer single consumer. That means you can clone the sender (tx) but not the receiver (rx). To solve this, we use crossbeam-channel crate

    let h1 = thread::spawn(move || {
        for i in 0..100 {
            if i == 50 {
                thread::sleep(Duration::from_secs(3));
            }
            tx.send(format!("Message {}", i)).unwrap();
        }
    });

    for received in rx {
        println!("Received {}", received);
    }

    h.join().unwrap();
    h1.join().unwrap()
}
