use std::env;

use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    println!("Env loaded");

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from("3000")
    };
    println!("{}", port)
}