#[derive(Debug)]
enum Status {
    Active,
    Inactive,
}

fn main() {
    let status: Status = Status::Active;

    println!("{:?}", status);

    match status {
        Status::Active => println!("Let's goooo."),
        Status::Inactive => println!("What are you doing do?"),
    }
}
