fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn type_conversion() {
    // String to integer
    let integer = "100".parse::<u8>().expect("Invalid");
    println!("{} {}", integer, type_of(&integer));
    let integer = "100".parse::<i8>().expect("Invalid");
    println!("{} {}", integer, type_of(&integer));

    // String to float
    let float32 = "100".parse::<f32>().expect("Invalid");
    println!("{} {}", float32, type_of(&float32)); // Even though it just prints 100 and not 100.00, it is still f32

    let float64 = "100.123".parse::<f64>().expect("Invalid");
    println!("{} {}", float64, type_of(&float64));

    // String to boolean
    let boolean = "true".parse::<bool>().expect("Invalid");
    println!("{} {}", boolean, type_of(&boolean));

    // String to boolean
    let character = "a".parse::<char>().expect("Invalid");
    println!("{} {}", character, type_of(&character));

    // Anything to string
    let stringified = stringify!(true);
    println!("{} {}", stringified, type_of(&stringified));
}
