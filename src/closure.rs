fn main() {
    let sum = |a, b| a + b;
    let s = sum(1, 2);
    println!("Sum = {s}");
    println!("Sum = {:?}", sum_hoc(1, 2, sum));
}

fn sum_hoc<F>(a: i32, b: i32, cb: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    cb(a, b)
}
