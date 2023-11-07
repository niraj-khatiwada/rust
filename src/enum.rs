enum Status {
    Active,
    Inactive,
}

fn main() {
    let status: Status = Status::Active;
    get_me_status(status);
}


fn get_me_status(status: Status) {
    match status {
        Status::Active => println!("Row is active"),
        Status::Inactive => println!("Row is inactive")
    }
}