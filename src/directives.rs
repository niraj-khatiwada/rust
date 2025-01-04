// #![allow(unused_variables)] // Apply to whole file

type KiloMeter = u32;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    #[allow(unused_variables)]
    let distance: KiloMeter = 100;

    let me = Person {
        name: String::from("Niraj"),
        age: 27,
    };
    println!("{:?}", me);
}

#[allow(unused_variables)]
fn print() {
    let distance: KiloMeter = 100;
    let distance2: KiloMeter = 100;
}
