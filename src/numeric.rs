#[derive(Debug)]
enum Status {
    Working(usize),
    Error(usize),
}

impl From<usize> for Status {
    fn from(code: usize) -> Self {
        if code == 0 {
            return Self::Working(code);
        }
        return Self::Error(code);
    }
}

// isize = 2^size. Starts from -2^(size/2) to +2^(size/2).
// usize -> 2^size. Starts from 0 to 2^size.
// let num = 256u8 -> this is wrong since u8 = 0..=255.
// let num = 255u8 is correct
// let num = 256u16 is correct

fn main() {
    // numeric type conversion: Type Casting
    let num = 255u8 as u16; // Convert u16 to u8
    let num2 = 256u32;
    // let sum = num + num2; // This will throw mismatched types
    let sum = num as u32 + num2; // This will compile
    println!("{:?}", sum);

    // Float to Int
    let decimal = 69.69 as u8;
    println!("{}", decimal); // This will give 69(Nice!)
    println!("{}", 269.69 as u8); // This will compile successfully but the result will be 255

    // TryFrom
    println!("{:?}", u8::try_from(255u8)); // This will be Ok(255)
    println!("{:?}", u8::try_from(25516)); // This is out of range of 0..255 from u8 so Err(TryFromIntError())

    let cvt = Status::from(random(1));
    println!("{:?}", cvt)
}

fn random(arg: usize) -> usize {
    return arg + 100;
}