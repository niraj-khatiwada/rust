fn main() {
    let mut num = 1e3 as i32;
    // Infinite Loop
    loop {
        println!("{}", num);
        num -= 1;
        if num == 0 {
            break;
        }
    }

    // While loops
    while num > 0 {
        println!("{}", num);
        num -= 1;
        if num == 0 {
            break;
        }
    }

    // For loops
    for n in 0..num {
        println!("{}", n);
    }

    // Labelled Block Expressions
    'outer: for n in 0..num {
        println!("{}", n);
        'inner: for n in 0..num {
            println!("{}", n);
            if n == 2 {
                break 'inner;
            }
        }
        if n == 100 {
            break 'outer;
        }
    }
}
