fn main() {
    let integer_u8: u8 = 255;

    // let integer_u16 = integer_u8 as u16;

    let integer_u16 = integer_u8 as f32;

    println!("{}", integer_u16);

    let interesting = 600_u16 as u8;

    println!("{}", interesting);
    // Why the value is 88?
    // 600_u16 cannot be fitted into u8 so it does 600 - 256 = 344. 344 is still > 255 so again, 344 - 256 = 88. 88 is now u8 so result will be 88.

    let abc = u8::try_from(256u16); // Since u8 is max 255, this will throw TryFromIntoError
    println!("{:?}", abc);
}
