fn main() {
    let search = find_row("Rust");
    if let Some(val) = &search {
        println!("Found: {}", val);
    }

    // Map combinator to transform option value
    let transform = search.map(|val| format!("{}!!!", val));
    if let Some(val) = &transform {
        println!("Transform: {}", val);
    }
}

/// Finds a value based on the search term. If not found, returns None.
fn find_row(name: &str) -> Option<String> {
    if name == "Rust" {
        return Some(String::from("Rust is king"));
    }
    None
}
