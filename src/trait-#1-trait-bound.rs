// Trait Bound to allow methods to be called conditionally based on data types.

use std::fmt::Display;

struct Person<T> {
    name: T,
}

// Only allow .get_name() for data types that implement the Display trait.
impl<T: Display> Person<T> {
    fn get_name(&self) -> &T {
        &self.name
    }
}

fn main() {
    let person = Person {
        name: String::from("Rust"),
    };
    println!("Name: {}", person.get_name()); // Since String implements the Display trait, we can call .get_name() method

    let person = Person {
        name: vec![String::from("Rust")],
    };
    println!("Name: {}", person.get_name()); // But vector does not implement Display trait. So we cannot call .get_name() method
}
