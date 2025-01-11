use rand::Rng;

fn main() {
    let random_bool = rand::thread_rng().gen_bool(0.5);
    println!("{random_bool}");

    let random_int = rand::thread_rng().gen_range(0..100);
    println!("{random_int}");
}
