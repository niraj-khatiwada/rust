use std::collections::HashMap;

fn main() {
    let mut object: HashMap<String, String> = HashMap::new();

    object.insert(String::from("name"), String::from("Niraj"));
    object.insert(String::from("age"), String::from("26"));

    println!("{:?}", object);
    println!("Length is = {}", object.len());
    // Get -> This will be an Option
    println!("Name= {:?}", object.get("name"));
    println!("Age= {:?}", object.get("age"));

    if let Some(name) = object.get("name") {
        println!(">> Name = {}", name);
    }

    if let Some(age) = object.get("age") {
        println!(">> Age = {}", age);
    }

    // Update
    object.insert(String::from("age"), String::from("25"));
    println!("{:?}", object);

    // Remove
    object.remove("name");
    println!("{:?}", object);
}
