fn main() {
    let sum = math(1, 2, Box::new(|a, b| a + b));
    println!("Sum = {}", sum);

    let product = math(1, 2, Box::new(|a, b| a * b));
    println!("Product = {}", product)
}

fn math(a: i32, b: i32, operation: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    operation(a, b)
}
