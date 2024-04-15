use std::collections::HashMap;

mod my_library {

    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Person {
        pub name: String,
    }

    pub fn print_name(person: &Person) {
        let object: HashMap<String, String> = HashMap::new();
        println!("{:?}", object);

        println!("{  }", person.name);
    }
}

fn main() {
    // use my_library::*;
    let person = my_library::Person {
        name: String::from("Niraj"),
    };

    println!("{:?}", person);

    my_library::print_name(&person);

    let object: HashMap<String, String> = HashMap::new();
    println!("{:?}", object);
}
