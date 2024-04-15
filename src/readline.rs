use std::io;

fn main() {
    let result = readline("What's your name?");

    if let Ok(output) = result {
        println!("{:?}", format!("Hello {}", output));
    } else {
        panic!("Something went wrong...")
    }
}

fn readline(prompt: &str) -> Result<String, String> {
    println!("{}", prompt);
    let mut buffer = String::new();
    let output = io::stdin().read_line(&mut buffer);

    if let Err(err) = output {
        return Err(err.to_string());
    } else {
        return Ok(String::from(buffer.trim()));
    }
}
