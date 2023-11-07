fn main() {
    pattern_match();
}

fn pattern_match() {
    let age: u32 = 26;
    match age {
        1..=18 => println!("Not old enough!"),
        (23 | 25) => println!("Get some job"),
        30..=u32::MAX => println!("Retire!"),
        _ => println!("RIP")
    }
}
