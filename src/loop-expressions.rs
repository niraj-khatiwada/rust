fn main() {
    'a: loop {
        loop {
            break 'a;
        }
    }

    let mut num = 0;

    let result = 'a: loop {
        num += 1;
        if num == 5 {
            break 'a num; // We can return value from loop using break
        }
    };

    println!("{:?}", result);
}
