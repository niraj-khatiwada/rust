#[derive(Debug)]
struct Person {
    age: u8,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age > other.age // some custom logic
    }
}

fn main() {
    let person = Person { age: 15 };
    let person2 = Person { age: 15 };

    println!("{}", person == person2);
}
