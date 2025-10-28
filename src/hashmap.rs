use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

fn main() {
    let mut map: HashMap<&str, u8> = HashMap::new();

    // Add
    map.insert("Niraj", 27);
    map.insert("Suraj", 27);

    println!("{:?}", map);

    println!("Size {}", map.len());

    println!("Get {:?}", map.get("Suraj"));

    // Get key hash
    let name = map.get("Niraj");
    if let Some(name) = name {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();
        println!("Hash {:?}", hash);
    }

    // Remove
    map.remove("Suraj");

    // Keys
    for key in map.keys() {
        println!("{}", key);
    }

    // Keys
    for value in map.values() {
        println!("{}", value);
    }

    // Entries
    for (k, v) in map.iter() {
        println!("{} -> {}", k, v);
    }
}
