struct Person {
    age: u16,
}

impl Person {
    fn new(age: u16) -> Self {
        Self { age }
    }
}
impl Default for Person {
    fn default() -> Self {
        Self { age: 0 }
    }
}

fn main() {
    let person = Person::new(100);
    println!("Person 1 age: {:?}", person.age);

    let person2 = Person::default();
    println!("Person 2 age: {:?}", person2.age);
}
