fn main() {
    // if-let runs once.
    let val = Some(1);
    if let Some(val) = val {
        println!("Got some val {}", val);
    }

    // while-let runs until the condition is satisfied
    let list = [Some(1), Some(2), None];
    let mut it = list.iter();
    while let Some(val) = it.next() {
        if let Some(v) = val {
            println!("Has some data {:?}", v)
        }
    }
}
