#[derive(PartialEq, PartialOrd)]
enum Status {
    // 0
    Active,
    // 1
    Deleted,
    // Archived
    Archived(i32),
}

#[derive(PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let abc = Status::Active;

    println!("{:?}", abc == Status::Active);
    println!("{:?}", Status::Active < Status::Deleted); // Order
    println!("{:?}", Status::Archived(99) < Status::Archived(100));

    let person1 = Person { name: String::from("niraj"), age: 26 };
    let person2 = Person { name: String::from("niraj"), age: 27 };

    println!("{:?}", person1 == person2)
}