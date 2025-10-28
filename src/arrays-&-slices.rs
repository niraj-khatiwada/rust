fn main() {
    let arrays: [u8; 3] = [1, 2, 3];

    println!("Length = {}", arrays.len());

    for i in arrays {
        println!("{}", i);
    }

    println!("{:?}", &arrays[0..2]);

    match &arrays[0..=2] {
        [first, .., last] => println!("First = {} | Last  = {}", first, last),
        [single_item_only] => println!("Single item {}", single_item_only),
        _ => {}
    }
}
