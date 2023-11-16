use rand::Rng;

#[derive(PartialEq)]
enum Species {
    Hawk,
    Eagle,
}

struct Bird {
    age: u8,
    species: Species,
}

fn main() {
    let random = rand::thread_rng().gen_range(1..=100);

    // @ Guard. Also called binding
    match random {
        v @ ..=50 | v @ ..=10 => println!("Less than 50 @ {}", v),
        v @ _ => println!("Greater than 50 @ {}", v)
    }


    // If guard
    let random2 = rand::thread_rng().gen_range(1..=50);
    match random2 {
        s if (s >= 20) => println!("value {}", s),
        _ => println!("lLess than 20")
    }


    let bird = Bird { age: 20, species: Species::Hawk };
    // Ignore guard
    match bird {
        Bird { age: 10..=20, .. } => println!("This is a bird with age less than or equal to 20."),
        Bird { species: Species::Hawk, .. } => println!("This is a Hawk."),
        _ => println!("Could not identify the bird.")
    }

    // all of the combined
    let bird2 = Bird { age: 100, species: Species::Eagle };
    match bird2 {
        Bird { age, species } if (age > 10 && species == Species::Eagle) => println!("This is an Eagle with age less than 10."),
        Bird { species: Species::Hawk, .. } => println!("This is a Hawk."),
        _ => println!("Could not identify the bird.")
    }
}