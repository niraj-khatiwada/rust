#[derive(Debug)]
struct Person {
    name: String,
}

fn main() {
    let person = Person {
        name: String::from("Niraj"),
    };
    let person_ptr = Box::new(person); // Stores person on the heap
    println!("{:?} {:?}", person_ptr, *person_ptr); // *person_ptr puts the value back to the stack
}
