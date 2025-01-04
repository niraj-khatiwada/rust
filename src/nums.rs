use core::f64;

fn main() {
    // 1 bit = 0 or 1. Smallest entity in a memory
    // 1 Byte = 8 bits
    // 1KB = 1024 bytes = 1024 * 8 bits
    // 1MB  = 1024 KB

    // Signed
    // i8 = 2^8/2 to 2^8/2 - 1 = -128 to 127
    // u8 = 0 to 2^8 - 1 = 0 to 255
    // ...
    const PI: f64 = f64::consts::PI;
    println!("PI: {}", PI);

    let num: f64 = 100.97;
    println!("Integer from Double: {:?}", (num as u32) % 3);
    println!("Floor: {:?}", num.floor());
    println!("Ceil: {:?}", num.ceil());

    println!("Sqrt: {:?}", num.sqrt());
    println!("Power: {:?}", 10_i32.pow(2));
    println!("Truncate: {:?}", num.trunc()); // Removes the decimal part

    // Max values
    println!("{}", u64::MAX);
    println!("{}", f64::MAX);

    // usize and isize
    // The size will be assigned dynamically based on what kind of architecture does the computer run on.
    let num: usize = 100; // will be u32 on 32 bit architecture and u64 on 64 bit architecture.
    let num2: isize = -100; // same as above

    println!("{}", num);
    println!("{}", num2);

    // Format Specifiers
    // 1. Floating precision
    #[allow(clippy::approx_constant)]
    let _pi: f64 = 3.14159265;
    let precision = format!("{:.2}", _pi);
    match precision.as_str() {
        "3.14" => println!("People usually know me this far only."),
        _ => println!("My name is Jeff!"),
    }

    // Integer to Float
    let int = 100;
    let flt = int as f64;
    dbg!(flt);

    // Float to Integer
    let flt = 100.01;
    let int = flt as i32;
    dbg!(int);
}
