fn main() {
    println!("Hello \n World"); // Will escape

    // Raw will print exactly how it's written
    println!(r"Hello \n World"); // Won't escape

    println!(
        r"Hello
    World
    "
    );

    println!(r#"Hello "Niraj""#); // Escaping Multiple "
    println!(r##""Hello"# "Niraj""##); // Escaping Multiple "#
}
