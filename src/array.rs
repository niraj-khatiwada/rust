fn main() {
    let list = [1, 2, 3];

    for i in list {
        println!("{i}")
    }

    func(&list);
}

// Array slice
fn func(array: &[u8]) {
    for i in array {
        println!("{i}")
    }
}
