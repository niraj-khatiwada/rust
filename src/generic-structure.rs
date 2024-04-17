// Generics in Struct = Generic Structure
struct Person<T: Tall> {
    name: String,
    age: u8,
    one_extra_field: T,
}

trait Tall {
    fn height(&self) -> u8;
}

struct TallPerson;
impl Tall for TallPerson {
    fn height(&self) -> u8 {
        return 6;
    }
}

fn main() {
    let tall_person = TallPerson;
    let person = Person {
        name: String::from("Niraj"),
        age: 26,
        one_extra_field: tall_person,
    };
    println!("The height is {}", person.one_extra_field.height());
}
