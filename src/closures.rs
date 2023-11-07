fn main() {
    let sum = |a: i32, b: i32| -> i32 {
        return a + b;
    };

    let sum2 = |a, b| a + b;


    println!("{:?}", sum2(1, 2))
}



