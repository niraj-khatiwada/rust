fn main() {
    let mut array: Vec<i32> = Vec::new();

    array.push(1);
    array.push(2);
    array.push(3);
    array.push(4);
    array.push(-5);
    array.push(5);

    array.insert(0, 0); // insert at given position

    println!("{:?}", array);

    array.pop();
    array.remove(4); // Remove at specific position

    println!("{:?}", array);
    println!("{:?} {:?}", array[0], array.get(0)); // panic vs non-panic access

    array[0] = -1;
    println!("{:?}", array);
    println!("{:?} {:?}", &array[0..], &array[1..2]); // slicing

    // find index
    if let Some(idx) = array.iter().position(|&x| x == -1) {
        println!("Index is {}", idx)
    } else {
        println!("Index is -1",)
    }

    for (idx, item) in array.iter().enumerate() {
        println!("{} {}", idx, item)
    }

    for alphabet in 'a'..='z' {
        println!("Alphabet = {}", alphabet);
    }

    println!("Array {:?}", array);
    // Map
    let mapped: Vec<i32> = array.iter().map(|x| x + 1).collect(); // Map dereferences the ref value of .iter() automatically and gives you new vector on .collect()
    println!("Mapped {:?}", mapped);

    // Filter
    let filter: Vec<&i32> = array.iter().filter(|x| *x % 2 == 0).collect(); // Filter is just checking for conditions so it'll still refer to the original .iter() value upon .collect() unlike .map()
    println!("Filtered {:?}", filter);

    // Take first 3: But better to use slice &array[0..3]
    let first_3: Vec<i32> = array.iter().take(3).map(|x| *x).collect();
    println!("First 3 {:?}", first_3);

    array.sort(); //sort occurs in-place
    println!("Array {:?}", array);

    // Create vector from range
    let list1: Vec<u32> = (0..=100).collect();
    println!("{:?}", list1);
    let list2: Vec<char> = ('a'..='z').collect();
    println!("{:?}", list2);
}
