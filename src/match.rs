struct Person {
    name: String,
    age: u8,
}

fn main() {
    roast_my_age(26);

    let person = Person {
        name: String::from("Niraj"),
        age: 26,
    };

    match person {
        Person { age, .. } => println!("Wow, {} years old?", age),
    }
}

fn roast_my_age(age: u8) {
    match age {
        0..=18 => println!("I'm just a baby"),
        19..=30 => print!("Oh, someone is unemployed."),
        31..=50 => print!("I hope you earned enough money"),
        51..=65 => print!("Retire dog"),
        66..=99 => print!("I've to go to hospital."),
        _ => print!("Nah, that's too far"),
    }
}
