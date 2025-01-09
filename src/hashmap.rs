use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

fn main() {
    let mut map: HashMap<&str, String> = HashMap::new();

    map.insert("name", String::from("Niraj"));

    // Key hash
    let name = map.get("name");
    if let Some(name) = name {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        println!("Hash {:?}", hasher.finish());
    }

    // Keys
    for key in map.keys() {
        println!("Key: {}", key)
    }
    // Values
    for key in map.values() {
        println!("Value: {}", key)
    }

    // Entries
    for (key, value) in map.iter() {
        println!("{} -> {}", key, value);
    }

    map.remove("name");

    println!("Size {}", map.len());
}
