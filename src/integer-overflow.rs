// make sure to run this in release mode. `cargo build --release` and run the binary from `./target/release/<bin>`
fn main() {
    let a = 8_u8;
    let b = u8::MAX;
    #[allow(arithmetic_overflow)]
    let val: u8 = a + b; // this will overflow and causes panic in debug mode. But in release mode, this will wrap around.
                         // Since for u8: 0 to 255 is the range, 255+8  will wrap and return 7. 255 + 1 = 0, 255 + 2 = 1, ... 255 + 8 = 7
    println!("{}", val); // will be 7 in release mode.

    // Non panic overflow handling
    // .checked_<operation like add , subtract, ...>: Only returns Some(value) if no overflow occurs.
    if let Some(sum) = a.checked_add(b) {
        println!("[checked_add] Sum = {}", sum);
    } else {
        println!("checked_add] Overflow occurred.")
    }

    // .overflowing_<operation like add , subtract, ...>: Same as .checked_ but returns tuple instead of option
    let (sum, did_overflow) = a.overflowing_add(b);
    if !did_overflow {
        println!("[overflowing_add] Sum = {}", sum);
    } else {
        println!("overflowing_add] Overflow occurred.")
    }

    // Limits the sum to max value only. For eg: u8::MAX will be 255 so sum will be 255 here.
    let sum = a.saturating_add(b);
    println!("[saturating_add] Sum = {}", sum);

    // This is same as normal a+b but won't panic in debug mode as well.
    let sum = a.wrapping_add(b);
    println!("[wrapping_add] Sum = {}", sum);
}
