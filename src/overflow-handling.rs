// checked_<operation> -> Option
// overflowing_<operation> -> (value, did_overflowed)
// saturating_<operation> -> Min or max limit value of that data type

fn main() {
    // let num = 0u32 - 1; // This will panic since 0 - 1 = -1 which is not unsigned.
    // println!("{}", num);

    let num = 0u32.checked_sub(1);

    match num {
        Some(val) => println!("Val {}", val),
        _ => println!("Integer Overflowed"),
    }

    let (num2, overflowed) = 0u32.overflowing_sub(1);

    if !overflowed {
        println!("value is {}", num2)
    } else {
        println!("Integer Overflowed")
    }

    let num3 = 255u8.saturating_add(1);
    println!("{}", num3); // Returns min or max limit value for that data type
}
