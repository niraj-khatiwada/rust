fn main() {
    println!("Sum = {:?}", sum_hoc(1, 2, |a, b| a + b));
    println!("Subtract = {:?}", sum_hoc(1, 2, |a, b| a - b));
    println!("Multiply = {:?}", sum_hoc(1, 2, |a, b| a * b));
    println!("Divide = {:?}", sum_hoc(1, 2, |a, b| a / b));

    // Using heap
    println!("Box Sum = {:?}", sum_hoc_box(1, 2, Box::new(|a, b| a + b)));
    println!(
        "Box Subtract = {:?}",
        sum_hoc_box(1, 2, Box::new(|a, b| a - b))
    );
    println!(
        "Box Multiply = {:?}",
        sum_hoc_box(1, 2, Box::new(|a, b| a * b))
    );
    println!(
        "Box Divide = {:?}",
        sum_hoc_box(1, 2, Box::new(|a, b| a / b))
    );
}

fn sum_hoc<T: (Fn(i32, i32) -> i32)>(a: i32, b: i32, cb: T) -> i32 {
    cb(a, b)
}

fn sum_hoc_box(a: i32, b: i32, cb: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    cb(a, b)
}

// Another way of writing this using where clause.
fn _sum_hoc<T>(a: i32, b: i32, cb: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    cb(a, b)
}
