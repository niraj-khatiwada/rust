struct Person {
    name: String,
    age: i32,
}

fn main() {
    pattern_match();
}

fn pattern_match() {
    let age: u32 = 26;
    match age {
        1..=18 => println!("Not old enough!"),
        (23 | 25) => println!("Get some job"),
        30..=u32::MAX => println!("Retire!"),
        _ => println!("RIP")
    }

    let person = Person { name: String::from("Niraj"), age: 26 };
    match person {
        // Person { name: String::new("Niraj"), .. } => println!("Niraj is king."), // match specific entry
        Person { name, .. } => println!("{:?} is king.", name) // match all entry with key name and ignore rest
    }
}
