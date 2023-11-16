use std::collections::HashMap;

type U32 = u32;
type Map = HashMap<String, U32>;
type CustomVector<T> = Vec<T>;
type CustomVectorWithLifetime<'a> = Vec<&'a String>;

fn main() {
    let list: Vec<U32> = vec! {1, 2, 3};
    println!("{:?}", list);

    let mut map: Map = HashMap::new();
    map.insert(String::from("Niraj"), 26);
    println!("{:?}", map);

    let list2: CustomVector<U32> = vec! {1, 2, 3};
    println!("{:?}", list2);
}