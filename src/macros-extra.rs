#[derive(Debug)]
enum Status {
    Active,
    Deleted,
}

fn main() {
    // Assert Macro
    // let a = 1;
    // let b = 2;
    //
    // assert!(a == b, "{} != {}", a, b);
    //
    // // In dev mode only
    // debug_assert!(a == b, "{} != {}", a, b);

    // This will be printed on build as well
    dbg!(Status::Active);

    // Format macro
    let str = format!("Hello {}", "World");
    println!("{}", str);

    // Env Macro
    // Only works for system env
    // Note: This won't compile if the variable is not set
    let user = env!("USER");
    println!("{}", user);

    // Include Str Macro
    let env = include_str!("../.env");
    println!("{}", env);

    // Todo & Unimplemented Macro
    // Program will panic
    todo!("Will ship this feature on Christmas");
    unimplemented!("Nobody wants this feature");

    // Unreachable Macro
    // Only panics at runtime
    unreachable!("Never run the code after this.")
}