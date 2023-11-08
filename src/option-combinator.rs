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
    println!("{}", maybe_number_doubled.is_some()); // true when Some<> is returned
    println!("{}", maybe_number_doubled.is_none()); // true when None is returned

    // Filter combinator
    // Filter out the returned Some value based on certain condition. If returned false, it will remove the Some<> and return None instead.

    let filter = maybe_number().filter(|value| value < &50);

    println!("{}", filter.is_some()); // true when Some<> is returned
    println!("{}", filter.is_none()); // true when None is returned

    let or_else = maybe_number().or_else(|| Some(100)); // If maybe_number has Some<> return that otherwise return Some(100). Used for setting default value.
    let unwrap_or_else = maybe_number().unwrap_or_else(|| 100); // Same as or_else but or_else needs to return Option<T> type where as this needs to return T.

    dbg!(or_else);
    dbg!(unwrap_or_else);

    match maybe_number_doubled {
        Some(value) => println!("Doubled value is {:?}", value),
        _ => println!("...")
    }
}

// Start from 72




