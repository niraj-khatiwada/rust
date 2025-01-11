use std::convert::TryFrom;

#[derive(Debug)]
struct Person {
    name: String,
}

impl TryFrom<&str> for Person {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self { name: value.into() })
    }
}

impl TryFrom<Person> for String {
    type Error = String;
    fn try_from(value: Person) -> Result<Self, Self::Error> {
        Ok(value.name)
    }
}

fn main() {
    let name = "Niraj";
    if let Ok(person) = name.try_into() as Result<Person, _> {
        dbg!(person);
    }

    let person = Person::try_from("Niraj").unwrap();
    let name: String = person.try_into().unwrap();
    dbg!(name);
}
