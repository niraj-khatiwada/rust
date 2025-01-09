enum Status {
    Active(u32),
    Inactive,
}

fn main() {
    let activity_status = Status::Active(100);

    if let Status::Active(val) = activity_status {
        println!("{}", val)
    }
}
