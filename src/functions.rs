fn main() {
    greet()
}

fn greet() {
    println!("Hello World!");

    println!("Sum is {}", sum(1, 2));
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
