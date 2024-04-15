fn main() {
    let my_num = 100;
    let is_even = if my_num % 2 == 0 { true } else { false };

    println!("Is Even? {is_even}");

    let is_odd = match my_num % 2 {
        0 => false,
        _ => true,
    };
    println!("Is Odd? {is_odd}");
}
