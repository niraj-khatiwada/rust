use std::env;

fn main() {
    let name = String::from("Rust");

    let formatted_name = format!("{} is king.", name);

    dbg!(formatted_name);

    // reads file as string and assigns to a variable
    let file = include_str!("./Cargo.toml");
    println!("{:?}", file);

    // reads file as buffer and assigns to a variable
    let file = include_bytes!("./Cargo.toml");
    println!("{:?}", file);

    // env value
    let term = env::var("TERM");
    println!("{:?}", term);

    // todo!("Will fix later"); // will panic

    // unimplemented!("Won't do it"); // will panic

    // unreachable!("no access"); // will panic
}
