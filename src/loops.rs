fn loops() {
    // managed loop
    let mut a = 5;
    'a: loop {
        if a == 0 {
            break 'a;
        } else {
            println!("a is now {}", a);
            a -= 1;
        }
    }

    // while loop
    let mut b = 3;
    while b != 0 {
        println!("b is now {}", b);
        b -= 1;
    }

    // for loop
    for i in 0..=10 {
        println!("i = {}", i)
    }
}
