#[derive(Debug)]
enum Status {
    Active,
    Inactive(u8), // Inactive for how many days
}

fn main() {
    let emp1_status = Status::Active;

    println!("{:?}", emp1_status);

    let emp2_status = Status::Inactive(3);
    println!("{:?}", emp2_status);
}
