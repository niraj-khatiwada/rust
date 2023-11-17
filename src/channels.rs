use std::thread;

use crossbeam_channel::unbounded;

#[derive(Debug)]
enum Task {
    Pending(String),
    Complete,
}


fn main() {
    let (w_tx, w_rx) = unbounded();
    let (m_tx, m_rx) = unbounded();

    let handle = thread::spawn(move || loop {
        match w_rx.recv() {
            Ok(status) => {
                match status {
                    Task::Pending(msg) => {
                        println!("Message received on worker thread {}", msg);
                        m_tx.send(Task::Pending(msg)).expect("");
                        ()
                    }
                    Task::Complete => {
                        m_tx.send(Task::Complete).expect("");
                        break;
                    }
                }
            }
            Err(e) => {
                m_tx.send(Task::Complete).expect("");
                break;
            }
        }
    });
    println!("Sending message now");
    w_tx.send(Task::Pending(String::from("1: Did you receive it?"))).expect("Error sending message.");
    w_tx.send(Task::Pending(String::from("2: Did you really receive it?"))).expect("Error sending message.");
    w_tx.send(Task::Complete).expect("Error sending message.");

    // loop {
    //     match m_rx.recv() {
    //         Ok(task) => {
    //             match task {
    //                 Task::Pending(msg) => println!("Message received on main thread {:?}", msg),
    //                 Task::Complete => {
    //                     break;
    //                 }
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }

    while let Ok(task) = m_rx.recv() {
        match task {
            Task::Pending(msg) => println!("Message received on main thread {:?}", msg),
            Task::Complete => {
                break;
            }
        }
    }

    handle.join().expect("Error joining thread 1.");
}