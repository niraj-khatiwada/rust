fn main() {
    let escaped_string = "\\Hello \"\n\" World \\";
    println!("{escaped_string}");

    // Nothing will be escaped
    println!("{}", r"Hello \n World");
    println!("{}", r#"\Hello "\n" World\"#);
}