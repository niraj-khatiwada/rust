fn main() {
    let list = vec! {1, 2, 3};

    // Just like .map() for arrays in JS
    let new_list: Vec<_> = list.iter().map(|a| a + 1).collect();
    let new_list_count = new_list.iter().count();
    println!("{:?} count={:?}", new_list, new_list_count);

    // Just like .filter() for arrays in JS
    let new_filtered_list: Vec<_> = list.iter().filter(|a| (*a % 2 == 0)).collect();
    let new_filtered_list_count = new_filtered_list.iter().count();
    println!("{:?} count={:?}", new_filtered_list, new_filtered_list_count);

    // Last item
    let last_item_in_new_list = list.iter().last();
    println!("{:?}", last_item_in_new_list);

    // Min and Max
    let min = list.iter().min();
    let max = list.iter().max();
    println!("{:?}, {:?}", min, max);

    // Take = Kind of like slice
    let first_2: Vec<_> = list.iter().take(2).collect();
    println!("{:?}", first_2)
}





