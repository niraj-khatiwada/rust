struct Person {
    name: String,
}

impl Person {
    fn print_name(&self) {
        println!("{}", self.name)
    }
}

fn main() {
    let niraj = Person { name: "Niraj".to_string() };
    niraj.print_name()
}

