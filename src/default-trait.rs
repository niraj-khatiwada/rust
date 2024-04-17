struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("King"),
            age: 26,
        }
    }
}

fn main() {
    let person: Person = Person::default();
    println!("{} {}", person.name, person.age);
}
