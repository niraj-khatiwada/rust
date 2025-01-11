struct Warehouse {
    packages: Vec<String>,
}

// Let's say we want to reference the package that the Warehouse owns.
struct Package<'a> {
    name: &'a String,
}

fn main() {
    let warehouse = Warehouse {
        packages: vec![String::from("Laptop"), String::from("Table")],
    };

    let laptop = Package {
        name: &warehouse.packages[0],
    };

    println!("{:?}", laptop.name);
    println!("{:?}", get_laptop(&warehouse));
}

// We can elide the `<'a>` lifetime annotation in functions since Rust automatically figures it out.
fn get_laptop<'a>(warehouse: &'a Warehouse) -> &'a String {
    &warehouse.packages[0]
}
