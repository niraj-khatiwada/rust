// use std::io;

// fn main() {
//     print!("What is your name?");
//     let input = get_input();
//     if let Ok(name) = input {
//         print!("Hello {}", name)
//     }
// }

// fn get_input() -> io::Result<String> {
//     let mut buffer = String::new();
//     println!("What is your name?");
//     io::stdin().read_line(&mut buffer)?;
//     Ok(buffer)
// }
