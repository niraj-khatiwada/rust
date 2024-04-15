use rand::{self, Rng};

fn main() {
    let odd = maybe_num().map(|num| num + 1); // map only applies when there's some value. The mapping will still return an optional value though.
    if let Some(odd_number) = odd {
        print!("Odd number is {}", odd_number);
    }
}

fn maybe_num() -> Option<i32> {
    let mut thread_rng = rand::thread_rng();
    let random: i32 = thread_rng.gen();
    if random % 2 == 0 {
        Some(random)
    } else {
        None
    }
}
