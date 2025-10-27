fn main() {
    let tuple = (true, String::from("Niraj"));
    println!("{} {}", tuple.0, tuple.1);

    let (one, two, three) = numbers();
    println!("{} {} {}", one, two, three);

    let (one, ..) = numbers();
    println!("{}", one,);
}

fn numbers() -> (u8, u8, u8) {
    (1, 2, 3)
}
