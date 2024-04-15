fn main() {
    // Unsigned Integer
    let integer_unsigned: u8 = 255;
    println!("{}", integer_unsigned);

    // Signed Integer
    let integer_signed: i16 = -255;
    println!("{}", integer_signed);

    // Decimal
    let decimal: f64 = 3.01;
    println!("{}", decimal);

    // Character
    let character: char = 'a';
    println!("{}", character);

    // Boolean
    let boolean: bool = true;
    println!("{}", boolean);

    // String
    let text: String = String::from("Hello World");
    println!("{}", text);

    let your_half = String::from("YOURS");
    let mut my_half = your_half;
    my_half = String::from("MINE");
    println!("{my_half}")
}
