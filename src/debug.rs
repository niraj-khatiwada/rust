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

    // Positional arguments
    let first = "1st";
    let second = "2nd";

    println!(
        "This should come {1} and this should come {0}",
        first, second
    )
}
