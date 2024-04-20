use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {
    let person1 = Person {
        name: String::from("Niraj"),
    };

    let serialized_json = serde_json::to_string(&person1).unwrap();
    println!("Serialized = {:?}", serialized_json);

    let deserialized_json: Person = serde_json::from_str(&serialized_json).unwrap();
    println!("Serialized = {:?}", deserialized_json);
}
