struct Person {
    name: String,
    age: u8,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Self {
            name: format!("Cloned: {}", &self.name),
            age: self.age,
        }
    }
}

fn main() {
    let person = Person {
        name: String::from("Rust"),
        age: 15,
    };
    println!("{} {}", person.name, person.age);

    let person_clone = person.clone();
    println!("{} {}", person_clone.name, person_clone.age);
}
