#[derive(Debug)]
struct Person {
    name: String,
}

impl From<&str> for Person {
    fn from(value: &str) -> Self {
        Self { name: value.into() } // &str.into() will automatically convert to String
    }
}

impl From<Person> for String {
    fn from(value: Person) -> Self {
        value.name
    }
}

fn main() {
    let name = "Niraj";
    let person: Person = name.into();
    dbg!(person);

    let person = Person::from("Niraj");
    let name: String = person.into();
    dbg!(name);
}
