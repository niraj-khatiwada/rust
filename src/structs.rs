#[derive(Debug)]
enum Status {
    Active,
    Inactive(u8), // Inactive for how many days
}

#[derive(Debug)]
struct Employee {
    name: String,
    status: Status,
}

impl Employee {
    fn greet(&self) {
        println!("Welcome {}!", self.name);
    }
}

fn main() {
    let emp1 = Employee {
        name: String::from("Niraj"),
        status: Status::Active,
    };

    println!("{:?}", emp1);

    emp1.greet();
}
