fn main() {
    let search = find_row("Rust");
    if let Some(val) = search {
        println!("Found: {}", val);
    }

    let search2 = find_row_or_throw("Russt");
    match search2 {
        Ok(rs) => println!("Found: {}", rs),
        Err(err) => println!("Error: {}", err),
    }
}

/// Finds a value based on the search term. If not found, returns None.
fn find_row(name: &str) -> Option<String> {
    if name == "Rust" {
        return Some(String::from("Rust is king!"));
    }
    None
}

/// Finds a value based on the search term. If not found, throws error.
fn find_row_or_throw(name: &str) -> Result<String, String> {
    if name == "Rust" {
        return Ok(String::from("Rust is king!"));
    }
    Err(String::from("Not found."))
}

/// Demo of ? operator
fn find_row_again_or_throw(name: &str) -> Result<String, String> {
    let rs = find_row_or_throw(name)?;
    Ok(rs)
}
