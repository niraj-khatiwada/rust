use rand::Rng;

fn main() {
    println!("{:?}", get_name());
}

/// Get's name randomly
fn get_name() -> Result<String, String> {
    let name = get_get_name()?; // If get_get_name() return Err(..) it will be returned directly from get_name(). Otherwise. it will continue next.
    return Ok(name);
}

fn get_get_name() -> Result<String, String> {
    let mut thread_rng = rand::thread_rng();
    let random: u32 = thread_rng.gen();

    if random % 2 == 0 {
        Ok(String::from("King"))
    } else {
        Err(String::from("Name is missing."))
    }
}
