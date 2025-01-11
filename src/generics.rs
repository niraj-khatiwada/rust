#[derive(Debug)]
struct Vehicle<T> {
    name_or_date: T,
}

// For all types
impl<T> Vehicle<T> {
    fn print(&self) -> &T {
        &self.name_or_date
    }
}

// Explicit to String types only
impl Vehicle<String> {
    fn print_name(&self) -> &String {
        &self.name_or_date
    }
}

enum Discount<T> {
    Percentile(T),
}

fn main() {
    let str = identity::<String>(String::from("Rust"));
    let str = identity::<&str>("Rust");
    let int = identity::<u32>(900);
    let int = identity::<bool>(true);
    // ::<T> is called a Turbo Fish operator

    let v = Vehicle {
        name_or_date: String::from("Bugatti"),
    };
    println!("Name: {:?} {:?}", v, v.print_name());

    let v = Vehicle { name_or_date: 1920 };
    println!("Name or date: {:?} {:?}", v, v.print());

    let discount = Discount::Percentile(100);
    let discount = Discount::Percentile("100%");
}

fn identity<T>(value: T) -> T {
    value
}
