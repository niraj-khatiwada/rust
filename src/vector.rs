fn main() {
    // let must list = Vec::<String>::new()
    // let must list: Vec<String> = Vec::new()
    let mut list: Vec<String> = vec![
        String::from("Rust"),
        String::from("Golang"),
        String::from("Cobol"),
        String::from("C"),
    ];
    list.push(String::from("Zig")); // Always inserts at last
    list.insert(2, String::from("C++")); // Inserts at given index and shifts all the values in the right by 1 index.

    let val = &list.pop(); // pop returns Option
    println!("{:?} is boring.", val);

    println!("Get value: {:?}, {:?}", list.get(1), list.get(100)); // .get() will return an Option. This is better than list[100] which will panic since it's out of bound.

    list.remove(2);

    for itm in &list {
        println!("{}", itm);
    }

    // Map: Map means we need to map to new owned value. So, the closure returns &String
    let mapped: Vec<String> = list
        .iter()
        .map(|lan: &String| {
            if lan.eq("Rust") {
                return String::from("Rust is King");
            }
            String::from("Boring")
        })
        .collect();
    dbg!(mapped);

    // Filter: Filter means to filter out items based on condition. Since, the original vector is Vec<String> it can only return reference items as Vec<&String>. That's why the closure returns &&String
    let filter: Vec<&String> = list.iter().filter(|lan| (*lan).eq("Rust")).collect();
    dbg!(filter);

    // Find: Will return Option<> type
    let found = list.iter().find(|lan| (*lan).eq(&String::from("Rust")));
    if let Some(lan) = found {
        println!("Language found {}", lan);
    }

    // Take: Kind of like slice but it will directly create new owned vector
    let take3: Vec<_> = list.iter().take(3).collect();
    dbg!(take3);

    // Same scenario as .take() but using slice.
    let slice3: Vec<_> = list[..3].iter().collect();
    dbg!(slice3);

    // Just like Java, sorting happens on the original vector
    list.sort();
    println!("{:?}", list);
}

// Vector Capacity
fn main() {
    let mut list = Vec::<u8>::with_capacity(2);
    list.push(1);
    list.push(2);
    println!("{:?} {} {}", list, list.len(), list.capacity());

    // When Rust finds out that the capacity has been filled, it will automatically expand the capacity to a new heap
    list.push(3);
    println!("{:?} {} {}", list, list.len(), list.capacity());
}
