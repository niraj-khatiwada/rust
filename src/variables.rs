fn main() {
    // Constants
    const NAME: &str = "NIRAJ";

    // String
    let mut n = String::new(); // New empty string.
    n.push_str("Niraj");
    println!("{}", n);

    let mut name = String::from("Niraj");
    print_name(&name);
    sell_name(name);
    name = String::from("Nirj k"); // This will change for the whole context now.

    // Integer
    // unsigned: +ve
    // signed: -ve /+v
    let unsigned: u32 = 255; // 0 to 2^32 -1
    let signed: i32 = -1; // -2^32 to 2^32-1

    // Floating
    let floating: f32 = -1.20;

    // Boolean
    let is_correct = true;

    // character
    let character = 'a';
}

fn print_name(mut name: &str) {
    println!("I borrowed this name {}", name);
    name = "Niraj K"; // This will only change in this context.
}

fn sell_name(name: String) {
    println!("I sold this name: {}", name);
}
