fn main() {
    let mut list: Vec<String> = vec![String::from("Rust"), String::from("Golang")];
    list.push(String::from("Zig"));

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
}
