use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("name"), String::from("Niraj"));
    map.insert("age".to_string(), "26".to_string());
    map.insert("gender".to_string(), "How dare you?".to_string());


    println!("{:?}", map);
    map.remove("name");
    println!("{:?}", map.get("name")); // Return tye of get is Option<&value>

    // Image this like Object.entry() of JS
    for (key, value) in map.iter() {
        println!("{:?} => {:?}", key, value)
    }

    for key in map.keys() {
        println!("{:?}", key)
    }

    for value in map.values() {
        println!("{:?}", value)
    }
}



