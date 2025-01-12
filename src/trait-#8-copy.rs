#[derive(Debug, PartialEq)]
struct Person {
    age: u8,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Self { age: self.age }
    }
}

impl Copy for Person {}

fn main() {
    let person = Person { age: 15 };

    let copied_person = copy_person(person); // person will be copied instead of moving

    println!("{:?}", person);
    println!("{}", copied_person == person) // true since == will check the content not memory address
}

fn copy_person(person: Person) -> Person {
    person
}
