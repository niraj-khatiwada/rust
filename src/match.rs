fn main() {
    // Match is Rust is always exhaustive i.e. need to handle all possible cases, including fallback.

    let num = 100;
    // Range match
    match num {
        0..=10 => println!("Zero to 10"),
        11..=100 => println!("Eleven to 100"),
        _ => println!("Don't care"),
    }

    let age = 10;
    // OR match
    match age {
        5 | 10 | 15 => {
            println!("kids")
        }
        _ => println!("Adults"),
    }

    // Computation via match
    match age {
        value if age % 3 == 1 => {
            println!("{value}")
        }
        _ => println!("Adults"),
    }
}
