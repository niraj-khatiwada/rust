use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let file: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("./abc.txt")));
    let shared_file_1 = file.clone();
    let shared_file_2 = file.clone();

    let h1 = thread::spawn(move || {
        let file_path = shared_file_1.lock().unwrap();
        println!("[1] Accessing file");
        println!("[1] Reading file {}", file_path);
        thread::sleep(Duration::from_secs(3));
    });

    let h2 = thread::spawn(move || {
        let file_path = shared_file_2.lock().unwrap();
        println!("[2] Accessing file");
        println!("[2] Reading file {}", file_path);
        thread::sleep(Duration::from_secs(3));
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
