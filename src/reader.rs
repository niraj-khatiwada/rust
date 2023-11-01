fn main() {
    reader();
}

fn reader() {
    println!("What is your name?");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Oops!");
    println!("Hello {}", name.trim());
}
