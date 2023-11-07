struct Person {
    name: String,
}

impl Person {
    fn alter_person() -> Self {
        return Self {
            name: "Suraj".to_string()
        };
    }
    fn print_name(&self) {
        println!("{:?}", self.name)
    }
}

fn main() {
    let person = Person { name: "Niraj".to_string() };
    person.print_name();
    let new_person = Person::alter_person();
    new_person.print_name()
}

