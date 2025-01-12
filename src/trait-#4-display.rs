use std::fmt::{Display, Formatter, Result};

struct Person {
    name: String,
    age: u8,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Rust"),
        age: 15,
    };
    println!("{}", person);
}
