use rand::prelude::*;

fn main() {
    let random_int: u8 = random();
    println!("{:?}", random_int);

    let random_f32: f32 = random();
    println!("{:?}", random_f32);

    let random_bool: bool = random();
    println!("{:?}", random_bool);

    let random_char: char = random();
    println!("{:?}", random_char); // This will show characters like symbols as well

    // Pick from my range of values
    let mut list = ['a', 'b', 'c', 'd', 'e'];
    let mut thread_rng = rand::thread_rng(); // Returns thread-local cryptographically secure randomly generated seed.
    if let Some(value) = list.choose(&mut thread_rng) {
        println!("{}", value);
    }

    // Shuffle my list
    list.shuffle(&mut thread_rng);
    println!("{:?}", list);

    // From range of integer
    let random_int = rand::thread_rng().gen_range(1..=10);
    println!("{:?}", random_int);
}
