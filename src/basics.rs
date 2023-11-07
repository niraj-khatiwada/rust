macro_rules! codeforreal {
    () => {
      println!("Rust is awesome!")
    }
}

fn main() {
    // let name: &str = "Niraj";
    // let is_true: bool = false;
    // let character: char = 'a';
    // let int32: i32 = -1;
    // let uint32: u32 = 1;
    // let float32: f64 = 12.20;

    println!("{}", sum(1, 2));
    println!("{}", factorial(5));
    codeforreal!();

    let mut i = 0;
    loop {
        if i == 5 {
            break;
        }
        println!("{i}");
        i += 1;
    }
    i = 0;
    while i != 5 {
        println!("{i}");
        i += 1;
    }
}


fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn factorial(num: i32) -> i32 {
    if num == 0 {
        return 1;
    }
    return num * factorial(num - 1);
}

