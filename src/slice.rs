fn main() {
    let list = [1, 2, 3];

    func(&list);

    let slice: &[u8] = &list[1..=2];
    // OR another syntax
    // let slice: &[u8] = list.as_slice();

    func(&slice);

    let subslice = &slice[0..1];

    func(&subslice);
}

// Is borrowing a slice
fn func(array: &[u8]) {
    for i in array.into_iter() {
        println!("{i}")
    }
}
