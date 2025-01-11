fn main() {
    let sum = |a, b| a + b;
    let s = sum(1, 2);
    println!("Sum = {s}");
    println!("Sum = {:?}", sum_hoc(1, 2, sum));
}

fn sum_hoc<T: (Fn(i32, i32) -> i32)>(a: i32, b: i32, cb: T) -> i32 {
    cb(a, b)
}

// Another way of writing this using where clause.
fn sum_hoc<T>(a: i32, b: i32, cb: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    cb(a, b)
}
