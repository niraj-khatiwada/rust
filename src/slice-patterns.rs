fn main() {
    let list = [1, 2, 3];

    let [first, ..] = &list;

    let [.., last] = &list;

    println!("First element is {}", first);
    println!("Last element is {}", last);

    match list {
        [first, .., last] => println!("First element is {}. Last element is {}", first, last),
    }
}
