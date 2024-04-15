fn main() {
    let a: Option<i32> = Some(100);
    // let a: Option<i32> = None;

    println!("Has Some? {}", a.is_some());
    println!("Is None? {}", a.is_none());

    println!("Mapped {:?}", a.map(|num| num + 1));

    println!("Filtered {:?}", a.filter(|num| num == &100)); // Filter will have borrowed args

    println!("Or else {:?}", a.or_else(|| Some(200)));

    println!("Unwrap or else {:?}", a.unwrap_or_else(|| 200)); // Gives unwrapped value
}
