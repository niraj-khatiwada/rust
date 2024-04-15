use rand::Rng;

fn main() {
    println!("{:?}", get_name());
}

fn get_name() -> Result<String, String> {
    let mut thread_rng = rand::thread_rng();
    let random: u32 = thread_rng.gen();

    if random % 2 == 0 {
        Ok(String::from("Niraj"))
    } else {
        Err(String::from("Name is missing."))
    }
}
