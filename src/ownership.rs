fn main() {
    let mut name: String = String::from("Niraj");

    print_name(&mut name);

    println!("{name}");
}

fn print_name(mut name: &str) -> &str {
    name = "suraj";
    name
}
