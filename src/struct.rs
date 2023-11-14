struct Single(i32);

impl Single {
    fn get_age(&self) -> i32 {
        return self.0;
    }
}

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
    niraj.print_name();

    let single = Single(26);
    println!("{:?}", single.get_age());
}

