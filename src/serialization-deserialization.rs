use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person { name: String::from("Niraj"), age: 26 };

    // This is like JSON.stringify()
    let serialized = serde_json::to_string(&person).expect("Failed to serialize");

    println!("Serialized: {:?}", serialized);

    // This is like JSON.parse()
    let deserialized: Result<Person, _>;
    deserialized = serde_json::from_str(&serialized);
    println!("Serialized: {:?}", deserialized);
}