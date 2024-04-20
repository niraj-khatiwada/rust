#[derive(Debug)]
struct Person {
    name: String,
}

fn main() {
    let person = Person {
        name: String::from("Niraj"),
    };
    println!("{:?}", person);
    dbg!("{:?}", &person); // Shows stack trace. Debug actually moves data

    println!("Environment Variable= {:?}", env!("PNPM_HOME")); // panics if varibale is not defined

    // todo!("BRB"); // panics

    assert!(1 == 2); // panics
    debug_assert!(1 == 2); // Same as assert!() but does not run in build
}
