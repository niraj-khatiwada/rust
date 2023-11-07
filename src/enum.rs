#[derive(Debug, PartialEq)]
enum Status {
    Active,
    Inactive,
    IsDeleted(i32), // Lets say you want to make an enum for each id of deleted value
}

fn main() {
    let status: Status = Status::Active;
    get_me_status(status);

    let status = Status::IsDeleted(100);
    println!("{:?}", status == Status::IsDeleted(100)); // Is id 100 deleted?
}


fn get_me_status(status: Status) {
    match status {
        Status::Active => println!("Row is active"),
        _ => println!("Row is inactive")
    }
}