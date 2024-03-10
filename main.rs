enum Status {
    Active,
    Inactive,
}

fn main() {
    let maybe_user = Some("Niraj");

    match maybe_user {
        Some(name) => println!("{:?}", name),
        None => (),
    }

    // Alternative way if we only care about Some(_) value then
    if let Some(name) = maybe_user {
        println!("{:?}", name);
    }
    // You can still use else here but it is exactly same as match
    // else {}

    let status: Status = Status::Active;
    if let Status::Inactive = status {
        println!("Active")
    } else {
        println!("Inactive")
    }
}
