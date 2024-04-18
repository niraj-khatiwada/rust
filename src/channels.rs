use crossbeam_channel::{self, Receiver, Sender};
use std::{thread, time::Duration};

fn main() {
    let (sender, receiver): (Sender<String>, Receiver<String>) = crossbeam_channel::unbounded();

    let receiver2 = receiver.clone();

    let thread1 = thread::spawn(move || match receiver.recv() {
        Ok(message) => println!("Thread 1: Message received {}", message),
        Err(e) => println!("Thread 1: Error occurred {}", e),
    });

    let thread2 = thread::spawn(move || match receiver2.recv() {
        Ok(message) => println!("Thread 2: Message received {}", message),
        Err(e) => println!("Thread 2: Error occurred {}", e),
    });

    thread::sleep(Duration::new(3, 0));

    sender.send(String::from("Hello from main thread")).unwrap();
    sender.send(String::from("Hello from main thread")).unwrap();

    thread1.join().unwrap();
    thread2.join().unwrap();
}
