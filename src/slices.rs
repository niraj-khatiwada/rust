fn main() {
    let list: [i32; 4] = [1, 2, 3, 4];
    println!("List is {:?}", list);

    // Slice
    let mut slice: &[i32] = &list[1..=2];
    println!("Slice is {:?}", slice);

    // Slice won't change the original array. It's just a view of original array.
    slice = &[100, 200];


    // SubSlice: Slices can be created from another slice.
    let subslice = &slice[0..1];
    println!("Subslice {:?}", subslice);

    println!("Sum is {}", sum(&list));
    println!("Sum of slice is {}", sum(&slice));


    // Slice Patterns
    let chars = ['a', 'b', 'c', 'd', 'e'];
    // let chars = [];

    // We can also do chars.as_slice() instead of &chars[..]
    let first = match &chars[..] {
        [first, ..] => first,
        _ => &' '
    };
    println!("First element is {}", first);

    let last = match &chars[..] {
        [.., last] => last,
        _ => &' '
    };
    println!("Last element is {}", last);

    // Advanced patterns
    match &chars[..] {
        [first @ 'a'..='b', rest @ ..] => println!("First element is {}. Rest is {:?}", first, rest),
        _ => ()
    }

    let mut all_chars = String::from("");
    for e in 'a'..='z' {
        all_chars += &*e.to_string()
    }
    println!("All characters {}", all_chars)
}


// If you don't know thw size of args beforehand, you can ignore the size parameter in array here. But usually, we will use vector for dynamic array. Only use [] for fixed size.
fn sum(args: &[i32]) -> i32 {
    let mut s = 0;
    for a in args {
        s += a;
    }
    return s;
}

