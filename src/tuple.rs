fn main() {
    let tuple: [i32; 3] = [1, 2, 3];

    for i in tuple {
        println!("{i}")
    }

    let (one, two, three) = one_two_three();
    println!("{one}{two}{three}");

    let _tuple = one_two_three();
    println!("{}", _tuple.0);
    println!("{}", _tuple.1);
    println!("{}", _tuple.2);
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
