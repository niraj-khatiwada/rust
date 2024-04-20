use std::str::FromStr;

// Convert string slice to any type

#[derive(Debug)]
struct Person(String);

impl FromStr for Person {
    type Err = String;

    fn from_str(any_string: &str) -> Result<Self, Self::Err> {
        let random_boolean: bool = rand::random();
        if random_boolean {
            return Ok(Self(String::from(any_string)));
        } else {
            return Err(String::from("Lol"));
        }
    }
}

fn main() {
    let person = Person::from_str("Niraj");

    println!("{:?}", person)
}
