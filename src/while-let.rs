fn main() {
    let mut value = Some(100);

    // If you want to find dynamic value
    while let Some(i) = value {
        println!("Matched {:?}", i);
        value = None; // If you don't want to change value to None, you can use break
    }

    let list = vec! {1, 2, 3};
    let mut iterable = list.iter();
    while let Some(num) = iterable.next() {
        println!("Number= {:?}", num);
    }
    println!("Complete")
}





