use rand::Rng;

fn main() {
    println!("{:?}", get_name());
}

fn get_name() -> Option<String> {
    let mut thread_rng = rand::thread_rng();
    let random: i32 = thread_rng.gen();
    println!("{random}");
    if random % 2 == 0 {
        Some(String::from("Niraj"))
    } else {
        None
    }
}
