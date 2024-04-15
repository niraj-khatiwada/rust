#[derive(Debug, Clone, Copy)]
struct Person {
    age: u8,
}

fn main() {
    let person = Person { age: 26 };
    print_age(person);
    print_age(person); // This'll make copy and make a clone so won't throw error
}

fn print_age(person: Person) {
    println!("{:?}", person.age);
}
