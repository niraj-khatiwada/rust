#[derive(Debug)]
enum Status {
    Active,
    Inactive,
}

enum Discount {
    Percentage(u8),
    Absolute(f64),
}

fn main() {
    let status: Status = Status::Active;

    println!("{:?}", status);

    match status {
        Status::Active => println!("Let's goooo."),
        Status::Inactive => println!("What are you doing do?"),
    }

    let discount = Discount::Percentage(10);

    match discount {
        Discount::Percentage(value) => println!("Wow {} discount?", value),
        Discount::Absolute(value) => println!("{} discount?, What?", value),
    }
}
