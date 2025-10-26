fn fizzbuzz() {
    let num_str = readline("Enter an integer:");
    let num = num_str.parse::<i32>().expect("Not a number");

    let mut result = String::new();

    if num % 3 != 0 && num % 5 != 0 {
        result = format!("{}", num);
    } else {
        if num % 3 == 0 {
            result = String::from("Fizz")
        }
        if num % 5 == 0 {
            result += "Buzz"
        }
    }
    println!("{}", result);
}

fn readline(question: &str) -> String {
    let mut buffer = String::new();
    println!("{}", question);
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_owned(),
        Err(_) => panic!("Something went wrong..."),
    }
}
