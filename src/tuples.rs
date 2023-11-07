#[derive(Debug)]
enum Status {
    Active,
    Deleted,
}

fn main() {
    let numbers = (1, 2, 3);
    println!("{}->{}->{}", numbers.0, numbers.1, numbers.2);

    let (a, b, c) = (1, 2, 3);
 
    println!("{},{},{}", a, b, c);

    let (sum, multiplication) = sum_and_multiplication(1, 2);
    println!("{}, {}", sum, multiplication);

    let (username, status) = ("Niraj", Status::Active);
    println!("{:?}, {:?}", username, status);
}

fn sum_and_multiplication(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a * b);
}

