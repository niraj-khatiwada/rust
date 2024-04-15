fn main() {
    let a: Option<bool> = Some(false);

    let mut n = 0;
    while let Some(boolean) = a {
        if n > 10 {
            break;
        }
        println!("Hurray! {}", boolean);
        n += 1;
    }

    let list: Vec<i32> = vec![1, 2, 3];

    let mut list_iter = list.iter();

    while let Some(value) = list_iter.next() {
        println!("Value is {value}");
    }
}
