#[derive(Debug)]
enum Gender {
    Male,
    Female,
    HowDareYou,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

fn main() {
    let person: Person = Person {
        name: String::from("niraj"),
        age: 26,
        gender: Gender::Male,
    };

    println!("{person:?}");
}
