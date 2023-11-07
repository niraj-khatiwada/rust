// use crate::Option::Some; // If you want custom Option enum types, you need to import it it like this first.
// use crate::Option::None;
//
// #[derive(Debug)]
// enum Option<T> {
//     HasAge(T), // Rust has built-in Some(T) for this. See below
//     NoAge, // Rust has built-in None for this.
// }
//
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: Option<i32>,
// }
//
// fn main() {
//     let person = Person { name: String::from("Niraj"), age: HasAge(26) };
//     let person2 = Person { name: String::from("Unknown"), age: NoAge };
//
//     println!("{:?}", person);
//     println!("{:?}", person2);
//
//     has_age(&person);
//     has_age(&person2);
//
//     println!("{:?}", find_age(&person));
//     println!("{:?}", find_age(&person2));
// }
//
//
// fn has_age(person: &Person) {
//     match person.age {
//         HasAge(age) => println!("{:?} is {:?} years old.", person.name, age),
//         NoAge => println!("{:?} did not provide the age.", person.name)
//     }
// }
//
// fn find_age(person: &Person) -> Option<i32> {
//     return match person.age {
//         HasAge(age) => HasAge(age),
//         _ => NoAge
//     };
// }

#[derive(Debug)]
struct Person {
    name: String,
    age: Option<i32>,
}

// See above commented code as well to understand better
fn main() {
    let person = Person { name: String::from("Niraj"), age: Some(26) };
    let person2 = Person { name: String::from("Unknown"), age: None };

    println!("{:?}", person);
    println!("{:?}", person2);

    has_age(&person);
    has_age(&person2);

    println!("{:?}", find_age(&person));
    println!("{:?}", find_age(&person2));
}


fn has_age(person: &Person) {
    match person.age {
        Some(age) => println!("{:?} is {:?} years old.", person.name, age),
        None => println!("{:?} did not provide the age.", person.name)
    }
}

fn find_age(person: &Person) -> Option<i32> {
    return match person.age {
        Some(age) => Some(age),
        _ => None
    };
}