use rand::Rng;

fn main() {
    get_random_value();
}

fn get_random_value() -> u32 {
    return rand::thread_rng().gen_range(1..=100);
}
