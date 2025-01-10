fn main() {
    let search2 = find_row_or_throw("Rust");
    match &search2 {
        Ok(rs) => println!("Found: {}", rs),
        Err(err) => println!("Error: {}", err),
    }
}

/// Finds a value based on the search term. If not found, throws error.
fn find_row_or_throw(name: &str) -> Result<String, String> {
    if name == "Rust" {
        return Ok(String::from("Rust is king"));
    }
    Err(String::from("Not found."))
}

/// Demo of ? operator
fn find_row_again_or_throw(name: &str) -> Result<String, String> {
    let rs = find_row_or_throw(name)?;
    Ok(rs)
}

// Result Combinators
fn main() {
    let search = find_row("Rust");
    if let Ok(val) = &search {
        println!("Found: {}", val);
    }

    println!("Is Ok?: {}", search.is_ok());
    println!("Is Err?: {}", search.is_err());

    // Differences in & and .as_ref(). We can also use .as_mut() for mutable reference.
    let _ref = &search; // Result<String, ()> -> &Result<String, ()>
    let _ref = search.as_ref(); // Result<String, ()> -> Result<&String, ()>

    // Map: Map the Ok(val) to another value.
    let map = search.as_ref().map(|val| format!("{}!!!", val));
    if let Ok(val) = map {
        println!("Map: {}", val);
    }

    // or_else: Return the actual Some(val) or return new Ok(default) when Err is found.
    let default_val = String::from("Rust will come back.");
    let or_else: Result<&String, ()> = search.as_ref().or_else(|err| Ok(&default_val));
    if let Ok(val) = &or_else {
        println!("Or else: {}", val);
    }

    // unwrap_or_else: Same as or_else but the value will be unwrapped from Ok(..) and directly return the content.
    let or_else = search.as_ref().unwrap_or_else(|err| &default_val);

    // We can also avoid the closures by directly using or and unwrap_or.
    let or: Result<&String, ()> = search.as_ref().or(Ok(&default_val));
    let unwrap_or = search.as_ref().unwrap_or(&default_val);
}

/// Finds a value based on the search term. If not found, returns None.
fn find_row(name: &str) -> Result<String, ()> {
    if name == "Rust" {
        return Ok(String::from("Rust is king"));
    }
    Err(())
}
