fn main() {
    let search2 = find_row_or_throw("Rust");
    match &search2 {
        Ok(rs) => println!("Found: {}", rs),
        Err(err) => println!("Error: {}", err),
    }
    // Map combinator
    let transform = search2.map(|rs| format!("{}!!!", rs));
    println!("{}", transform.unwrap());
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
