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
}
