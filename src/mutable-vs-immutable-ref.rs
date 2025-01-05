fn main() {
    // We cannot have (mutable reference and immutable) or multiple mutable reference of a same value in the same lifetime. This is called mutable reference restrictions.

    let mut car = String::from("Red");
    let ref1 = &mut car;
    let ref2 = &car; // this will throw error
    println!("{ref1} {ref2}");

    // They can be in a different lifetimes though. For example:
    let car2 = String::from("Red");
    {
        let ref3 = &car2; // This mutable reference only exists for this block.
        println!("{ref3}");
    }
    let ref4 = &car2; // This still works because the above mutable reference will be deallocated before running this line.
    println!("{ref4}");

    // Immutable reference uses Copy trait but mutable reference does not implement Copy trait
    let _coffee = String::from("Americano");
    let _a = &_coffee;
    let _b = _a; // This is equivalent to b = &coffee since immutable ref will be copied using Copy trait
    println!("{_a} {_b}");

    let mut __coffee = String::from("Americano");
    let __a = &mut __coffee;
    let __b = __a; // This is invalid because you __a will be moved here. Mutable reference won't be copied, instead will move. This is equivalent to __b = &mut __coffee. And, Rust does not allow multiple mutable reference of same value in the same lifetime. One way to avoid this is to clone the value.
    println!("{__a} {__b}");
}
