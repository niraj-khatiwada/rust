enum Status {
    Active,
    Inactive,
}

fn main() {
    let status: Status = Status::Active;
    // use let-else for early returns
    let Status::Inactive = status else {
        panic!("Inactive")
    };
}
