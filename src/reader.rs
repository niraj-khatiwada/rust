use std::io::BufRead;

fn main() {
    println!("Hi there, what's your name?");
    let mut buf = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    match reader.read_line(&mut buf) {
        Ok(_) => print!("Hello {}", buf),
        Err(_) => println!("Something went wrong..."),
    }
}
