fn main() {
    let num = 100;

    // Match is Rust is always exhaustive i.e. need to handle all possible cases, including fallback.
    match num {
        0..=10 => println!("Zero to 10"),
        11..=100 => println!("Eleven to 100"),
        _ => println!("Don't care"),
    }
}
