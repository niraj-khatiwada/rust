fn datatypes() {
    // const
    const PI: f32 = 3.14;

    // Integer
    let int32: i32 = -32;
    let int64: i64 = -64;

    let uint64: u8 = 255;

    // Float
    let float32: f32 = 1.20;
    let float64: f64 = 1.222;

    // Boolean
    let boolean: bool = true;

    // Character
    let character: char = 'a';
    println!("{} ~ {}", character, character as u32);

    // String
    // Owned string
    let mut name = String::from("Niraj");
    println!("My name is {name}");
    name = String::from("Suraj");
    println!("My name is {name}");

    // Borrowed string
    let mut bname = "Niraj";
    println!("My name is {bname}");
    bname = "Suraj";
    println!("My name is {bname}");
}
