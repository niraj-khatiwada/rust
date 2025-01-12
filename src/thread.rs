use crossbeam_channel::{Receiver, Sender};
use std::thread;

fn main() {
    // CPU cores.
    if let Ok(cpu_len) = thread::available_parallelism() {
        println!("Number of cpus: {}", cpu_len);
    }

    // Main thread
    let current_thread = thread::current();
    println!(
        "Main thread {:?} {}",
        current_thread.id(),
        current_thread.name().unwrap()
    );

    println!("Order of execution = 1");

    let (sender, receiver): (Sender<&str>, Receiver<&str>) = crossbeam_channel::unbounded();
    let (sender_worker, receiver_worker): (Sender<&str>, Receiver<&str>) =
        crossbeam_channel::unbounded();
    // Worker Thread
    let handle: thread::JoinHandle<&str> = thread::spawn(move || {
        println!("Order of execution = 2");
        let current_thread = thread::current();
        println!(
            "Worker thread {:?} {:?}",
            current_thread.id(),
            current_thread.name() // Worker thread wont have default names
        );
        while let Ok(msg) = receiver.recv() {
            println!("[Worker]: Message received: {}", msg);
            if msg.eq("terminate") {
                sender_worker.send("terminating now...").unwrap();
                break;
            }
        }
        "Complete"
    });
    println!("Order of execution = 3");

    sender.send("Hello Worker Thread").unwrap();
    sender.send("Hello Worker Thread again.").unwrap();
    sender.send("terminate").unwrap();

    // .recv() is a blocking action
    if let Ok(msg) = receiver_worker.recv() {
        println!("[Main]: Message received: {}", msg);
    }

    let result = handle.join().unwrap();

    println!("{:?}", result);
}
