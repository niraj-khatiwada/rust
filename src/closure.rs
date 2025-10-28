fn closure() {
    let sum = |a: u32, b: u32| -> u32 { a + b };
    sum(1, 2);

    println!("Sum {}", my_math(1, 2, sum));
    println!("Multiply {}", my_math(1, 2, |a: u32, b: u32| a * b));
}

fn my_math<F: Fn(u32, u32) -> u32>(a: u32, b: u32, action: F) -> u32 {
    return action(a, b);
}

// Readable method using where
// fn my_math<F>(a: u32, b: u32, action: F) -> u32
// where
//     F: Fn(u32, u32) -> u32,
// {
//     return action(a, b);
// }

// // Box
// fn my_math(a: u32, b: u32, action: Box<dyn Fn(u32, u32) -> u32>) -> u32 {
//     return action(a, b);
// }
