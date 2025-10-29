use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Niraj"),
        age: 27,
    };
    let serialized = serde_json::to_string(&person).expect("");
    println!("{:?}", serialized);

    let deserialized: Result<Person, _> = serde_json::from_str(&serialized);
    println!("{:?}", deserialized);
}
