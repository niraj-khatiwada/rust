use rand::Rng;

fn maybe_number() -> Option<i32> {
    let random = rand::thread_rng().gen_range(1..=100);
    if random % 2 == 0 {
        return Some(random);
    }
    return None;
}


fn main() {
    // Map combinator can be used for Option and Result Type.
    // Only calls the closure function if return type is Some for Option<> and Ok for Result.
    let maybe_number_doubled = maybe_number().map(|value| -> i32 {
        println!("Value is {:?}", value);
        return value * 2;
    }); // You can chain as many map as you want

    match maybe_number_doubled {
        Some(value) => println!("Doubled value is {:?}", value),
        _ => println!("...")
    }
}





