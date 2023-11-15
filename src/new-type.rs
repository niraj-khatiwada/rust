struct NeverZero(i32);

impl NeverZero {
    // Usually this struct and impl will be used as a module. So, we must use new to instantiate.
    fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            return Err(String::from("Value cannot be zero."));
        }
        return Ok(Self(i));
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    return a / b.0;
}

fn main() {
    let nz = NeverZero::new(10);

    match nz {
        Ok(num) => println!("{}", divide(100, num)),
        Err(msg) => println!("{}", msg)
    }
}


