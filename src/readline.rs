fn readline() {
    let mut buffer = String::new();

    println!("What's your name?");
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => println!("Hello {}", buffer.trim()),
        Err(_) => panic!("Something went wrong..."),
    }
}
