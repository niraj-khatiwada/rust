use crossbeam_channel::{self, Receiver, Sender};
use std::{thread, time::Duration};

fn main() {
    let (worker_sender, worker_receiver): (Sender<String>, Receiver<String>) =
        crossbeam_channel::unbounded();
    let (main_sender, main_receiver): (Sender<String>, Receiver<String>) =
        crossbeam_channel::unbounded();

    let thread = thread::spawn(move || loop {
        match worker_receiver.recv() {
            Ok(message) => {
                println!("Message received from main = {}", message);
                main_sender
                    .send(String::from("Thread 1 acknowledged."))
                    .unwrap();
                main_sender.send(String::from("Disconnect")).unwrap();
            }
            Err(e) => {
                println!("Thread 1: Error occurred {}", e);
                break;
            }
        }
    });

    thread::sleep(Duration::new(3, 0));

    worker_sender
        .send(String::from("Hello from main thread"))
        .unwrap();

    while let Ok(message) = main_receiver.recv() {
        println!("Disconnecting now...");
        match message.as_str() {
            "Disconnect" => {
                drop(worker_sender);
                break;
            }
            _ => println!("Message received from worker: {}", message),
        }
    }

    thread.join().unwrap();
}
