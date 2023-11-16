#[derive(Debug)]
enum Status {
    Working,
    Broken(u8),
}

impl From<u8> for Status {
    fn from(code: u8) -> Self {
        match code {
            0 => Status::Working,
            c => Status::Broken(c)
        }
    }
}

fn main() {
    // We know that &str is borrowed string. So to make it owned, we need String type.
    // let owned = "abc"; -> This is borrowed
    let owned = String::from("abc"); // This is owned
    println!("{owned}");

    let borrowed = "abc";
    let owned2: String = borrowed.into();
    println!("{owned2} {}", owned == owned2);


    // Custom from-into

    let status = Status::from(legacy_interface());
    println!("{:?}", status);

    // The return type of legacy_interface() is u8 and From in Status takes u8. That's why this will be compiled.
    // So to remember, if the return type of a function is the parameter of "from", you can use "into" method in that function.
    // If there is from method, into will automatically work. So, start from "from" first and then try to implement "into".
    let status2: Status = legacy_interface().into();
    println!("{:?}", status2);
}


fn legacy_interface() -> u8 {
    return 5;
}