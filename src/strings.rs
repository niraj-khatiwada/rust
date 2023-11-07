#[derive(Debug)] // Also has Copy and Clone derives (Debug, Clone, Copy). If you enable Copy and Clone, the ownership is not moved, it is copied everytime.
struct Person {
    name: String,
}

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
}

fn main() {
    let string = "a string".to_owned(); // Or String::from("a string")
    println!("{:?}", string);
    print_str(&string);

    let person = Person { name: String::from("Niraj") };
    println!("{:?}", person.name);

    println!("{:?}", Status::Active);
}


fn print_str(string: &str) {
    println!("{:?}", string);
}

