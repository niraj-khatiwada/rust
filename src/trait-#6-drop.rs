use std::ops::Drop;

struct Person {
    name: String,
    age: u8,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Goodbye person!")
    }
}

fn main() {
    let person = Person {
        name: String::from("Rust"),
        age: 15,
    };
    println!("{} {}", person.name, person.age);
}
