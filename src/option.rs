fn main() {
    let search = find_row("Rust");
    if let Some(val) = &search {
        println!("Found: {}", val);
    }

    println!("Is Some?: {}", search.is_some());
    println!("Is None?: {}", search.is_none());

    // Differences in & and .as_ref(). We can also use .as_mut() for mutable reference.
    let _ref = &search; // Option<String> -> &Option<String>
    let _ref = search.as_ref(); // Option<String> -> Option<&String>

    // Map: Map the Some(val) to another value.
    let map = search.as_ref().map(|val| format!("{}!!!", val));
    if let Some(val) = map {
        println!("Map: {}", val);
    }

    // Filter: Filter out if the Some(val) has some specific data.
    let filter = search.as_ref().filter(|val| val == &"Rust is king");
    if let Some(val) = &filter {
        println!("Filter: {}", val);
    }

    // or_else: Return the actual Some(val) or return new Some(default) when None is found.
    let default_val = String::from("Rust will come back.");
    let or_else = search.as_ref().or_else(|| Some(&default_val));
    if let Some(val) = &or_else {
        println!("Or else: {}", val);
    }

    // unwrap_or_else: Same as or_else but the value will be unwrapped from Some(..) and directly return the content.
    let or_else = search.as_ref().unwrap_or_else(|| &default_val);

    // We can also avoid the closures by directly using or and unwrap_or.
    let or = search.as_ref().or(Some(&default_val));
    let unwrap_or = search.as_ref().unwrap_or(&default_val);
}

/// Finds a value based on the search term. If not found, returns None.
fn find_row(name: &str) -> Option<String> {
    if name == "Rust" {
        return Some(String::from("Rust is king"));
    }
    None
}

// Creating Option enum manually
#[derive(Debug, PartialEq)]
enum OptionType<T> {
    SomeType(T),
    NoneType,
}

impl<T> OptionType<T> {
    fn unwrap(&self) -> &T {
        match self {
            OptionType::SomeType(val) => val,
            _ => panic!("None value found."),
        }
    }
}

fn main() {
    let val = OptionType::SomeType("Rust").unwrap();
    let val: &usize = OptionType::NoneType.unwrap(); // panics
}

fn get_value() -> OptionType<String> {
    OptionType::SomeType(String::from("Hello"))
}
