use std::convert::TryFrom;

#[derive(Debug)]
enum Status {
    Working,
    Broken(i32),
}

#[derive(Debug)]
enum StatusCopy {
    Working,
    Broken(i32),
}


impl From<i32> for Status {
    fn from(code: i32) -> Self {
        match code {
            0 => Status::Working,
            c => Status::Broken(c)
        }
    }
}

// We cannot have From and TryFrom trait for a single struct
impl TryFrom<i32> for StatusCopy {
    type Error = String;

    fn try_from(code: i32) -> Result<Self, Self::Error> {
        if code == 0 {
            return Ok(StatusCopy::Working);
        }
        // return Err(Status::Broken(code));
        return Err(String::from("Invalid value."));
    }
}

fn main() {
    let s = String::from("Niraj");
    println!("{}", to_uppercase(&s)); // &s makes a slice with type &str


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
    let status2: Status = legacy_interface().into(); // When this into() is called, it will convert the type to the one written beside the variable(:Status). That's why we need to specify the type beforehand.
    println!("{:?}", status2);

    // TryFrom
    match StatusCopy::try_from(legacy_interface()) {
        Ok(status) => println!("Working Hurray! {:?}", status),
        Err(msg) => println!("Oops! {:?}", msg)
    }
}


fn legacy_interface() -> i32 {
    return 5;
}

fn to_uppercase(s: &str) -> String {
    return s.to_uppercase();
}