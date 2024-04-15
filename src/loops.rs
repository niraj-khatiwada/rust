fn main() {
    let mut num = 0;
    'a: loop {
        println!("Hello World");
        num += 1;
        if num >= 10 {
            break 'a;
        }
    }

    let mut num2 = 0;
    while num2 < 10 {
        println!("Hello World");
        num2 += 1;
    }

    let list: Vec<i32> = vec![1, 2, 3];

    for i in list {
        println!("Hello World {}", i)
    }
}
