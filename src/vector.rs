fn main() {
    let list: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", list);

    let mut list2: Vec<i32> = Vec::new();
    list2.push(1);
    list2.push(2);
    list2.push(3);

    println!("{:?}", list2);
    println!("Length is: {:?}", list2.len());

    println!("First item is: {:?}", list2[0]);

    list2.remove(0);

    println!("{:?}", list2);
    list2.pop();
    println!("{:?}", list2);

    for i in list {
        println!("{i}");
    }
}
