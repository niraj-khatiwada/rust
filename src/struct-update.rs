// Imagine this to be like a spread operator in JavaScript

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    NotAnswered,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,           // This should be Gender::NotAnswered by default
    location: Option<String>, // This should be None by default
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from(""),
            age: 0,
            gender: Gender::NotAnswered,
            location: None,
        }
    }
}

fn main() {
    let person: Person = Person {
        name: String::from("Niraj"),
        age: 26,
        ..Person::default() // Spread operator
    };

    // let person2 = Person { ..person };

    println!("{:?}", person);
}
