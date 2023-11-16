fn main() {
    // Closure revision
    // let sum = |a: i32, b: i32| a + b;
    // println!("{}", sum(1, 2))

    // Advanced Closure
    let sum = math(1, 2, Box::new((|a, b| a + b)));
    println!("Sum is {}", sum);

    let product = math(1, 2, Box::new(|a, b| a * b));
    println!("Product is {}", product);

    // A special case
    let name = "Niraj";

    let subtract = Box::new(move |a, b| -> i32 {
        println!("Name is {}", name); // Since this name is of this scope, we need to add  move attribute in the closure to access it.
        return a - b;
    });
    println!("Subtract is {}", math(1, 2, subtract));
}

fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    return op(a, b);
}