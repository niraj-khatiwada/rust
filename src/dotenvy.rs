use dotenvy;
use std::env;

fn main() {
    dotenvy::dotenv().ok();
    println!("{:?}", dotenvy::var("CUSTOM_ENV"));
    // OR
    println!("{:?}", env::var("CUSTOM_ENV"));
}
