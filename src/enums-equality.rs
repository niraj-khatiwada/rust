#[derive(PartialEq, PartialOrd)]
enum Status {
    // 0
    Active,
    // 1
    Deleted,
    // Archived
    Archived(i32),
}

fn main() {
    let abc = Status::Active;

    println!("{:?}", abc == Status::Active);
    println!("{:?}", Status::Active < Status::Deleted); // Order
    println!("{:?}", Status::Archived(99) < Status::Archived(100));
}