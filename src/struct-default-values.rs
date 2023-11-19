#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

impl Default for Person {
    fn default() -> Self {
        Self { name: String::from("Niraj"), age: 26, gender: Gender::Male }
    }
}

fn main() {
    let person = Person::default();

    let person2 = Person { name: String::from("Suraj"), ..Person::default() }; // I needed to only change the name and keep the default values. Kind of like spread operator but it always needs to go last

    println!("{:?}", person);
    println!("{:?}", person2);
}