use rand::prelude::*;

fn main() {
    let num: u8 = rand::random();
    println!("{}", num);

    let boolean: bool = rand::random();
    println!("{}", boolean);


    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..=100);
    println!("{}", random);

    let letters = ['a', 'b', 'c'];
    let letter = letters.iter().choose(&mut rng);

    // Random letter
    println!("{:?}", letter);

    // Shuffle the array
    let mut alphabets = ['a', 'b', 'c', 'd', 'e'];
    alphabets.shuffle(&mut rng);
    println!("{:?}", alphabets);
}