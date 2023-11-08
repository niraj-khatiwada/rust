use std::io;

fn main() {
    println!("Enter your name");
    let mut name: String = String::new();
    // match io::stdin().read_line(&mut name) {
    //     Ok(size) => println!("{:?}", size),
    //     Err(err) => println!("{:?}", err)
    // }

    if let Ok(_) = io::stdin().read_line(&mut name) {
        println!("Your name is {:?}", name.trim());
    } else {
        println!("Error occurred")
    }
}








