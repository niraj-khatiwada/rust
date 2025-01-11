fn main() {
    // Array:
    let mut array: [u32; 5] = [1, 2, 3, 4, 5]; //Fixed size, same data type(homogenous)
    array[1] = 100; // can mutate
    dbg!(array[1]); // access

    // Array destructuring
    let [zero, one, two, ..] = array;
    println!(
        "Array destructuring: 1st={0}, 2nd={1}, 3rd={2}",
        zero, one, two
    );

    for i in array {
        println!("Array = {}", i);
    }

    // Tuple -> Fixed size, cannot change
    let mut tuple = (1, 2.0, 'a', true); // Fixed size, different data type allowed(heterogenous)
    tuple.0 = 2; // can mutate
    dbg!(tuple.0);
    dbg!(tuple.1);

    // Tuple destructuring
    let (zero, one, two, ..) = tuple;
    println!(
        "Tuple Destructuring: 1st={0}, 2nd={1}, 3rd={2}",
        zero, one, two
    );
}
