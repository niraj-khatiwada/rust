use rand::Rng;

// This is how we do ternary operation in Rust just like JS.
fn main() {
    let random_value = rand::thread_rng().gen_range(1..=100);
    let result: &str = if random_value % 2 == 0 {
        "Win"
    } else {
        "Lost"
    };
    println!("{result}");

    let resul2 = match result {
        "Win" => "Hurray",
        _ => "Sed"
    };

    println!("{:?}", resul2)
}

