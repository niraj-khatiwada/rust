enum Color {
    Red,
    White,
    Blue,
}

fn main() {
    let maybe = Some(100);

    // Match matches on all conditions
    match maybe {
        Some(_) => println!("Found"),
        None => println!("Not found")
    }

    // If-Let only matches on one condition and else for rest.
    // if let condition {} else {}
    if let Some(100) = maybe {
        println!("Hurray")
    } else {
        println!("Okay next time")
    }

    // We only want Red color, otherwise we don't care.
    let red = Color::Red;
    if let Color::Red = red {
        println!("Color found")
    } else {
        println!("Color not found")
    }
}





