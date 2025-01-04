#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let me = Person {
        name: String::from("Niraj"),
        age: 27,
    };
    println!("{:?}", me);
}
