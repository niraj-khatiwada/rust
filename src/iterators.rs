fn main() {
    let list: Vec<i32> = vec![1, 2, 3];

    let mapped: Vec<i32> = list.iter().map(|num| num + 1).collect();
    println!("Mapped {:?}", mapped);

    let filtered: Vec<&i32> = list.iter().filter(|num| *num % 2 == 0).collect();
    println!("Filtered {:?}", filtered);

    let find = list.iter().find(|num| *num == &1);
    println!("Find {:?}", find);

    println!("Len {:?}", list.iter().count());

    println!("Last {:?}", list.iter().last());

    println!("Min {:?}", list.iter().min());

    println!("Max {:?}", list.iter().max());

    // Takes n items from start
    let take: Vec<&i32> = list.iter().take(2).collect();
    println!("Take {:?}", take);
}
