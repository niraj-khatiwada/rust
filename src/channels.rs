use std::thread;

use crossbeam_channel::unbounded;

enum Task {
    Pending(String),
    Complete,
}

fn main() {
    let (sender, receiver) = unbounded();
    let receiver2 = receiver.clone();

    let handle1 = thread::spawn(move || loop {
        match receiver.recv() {
            Ok(status) => {
                match status {
                    Task::Pending(msg) => println!("{}", msg),
                    Task::Complete => break
                }
            }
            Err(e) => {
                println!("Error {:?}", e);
                break;
            }
        }
    });
    let handle2 = thread::spawn(move || loop {
        match receiver2.recv() {
            Ok(status) => {
                match status {
                    Task::Pending(msg) => println!("{}", msg),
                    Task::Complete => break
                }
            }
            Err(e) => {
                println!("Error {:?}", e);
                break;
            }
        }
    });
    println!("Sending message now");
    sender.send(Task::Pending(String::from("1: Did you receive it?"))).expect("Error sending message.");
    sender.send(Task::Pending(String::from("2: Did you receive it?"))).expect("Error sending message.");
    sender.send(Task::Complete).expect("Error sending message.");
    sender.send(Task::Pending(String::from("1: Did you receive it?"))).expect("Error sending message.");
    sender.send(Task::Pending(String::from("2: Did you receive it?"))).expect("Error sending message.");
    sender.send(Task::Complete).expect("Error sending message.");
    handle1.join().expect("Error joining thread 1.");
    handle2.join().expect("Error joining thread 2.");
}