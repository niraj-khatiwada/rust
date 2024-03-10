// fn main() {
//     let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

//     // Map
//     let new_numbers: Vec<i32> = numbers.iter().map(|&num| &num + 1).collect();
//     println!("{:?}", new_numbers);

//     // Filter
//     let filtered_numbers: Vec<_> = numbers.iter().filter(|&num| num % &2 == 0).collect();
//     println!("{:?}", filtered_numbers);

//     // Find
//     let find_number: Option<_> = numbers.iter().find(|&num| num == &3);
//     println!("{:?}", find_number);

//     // Count
//     println!("Count = {:?}", numbers.iter().count());

//     // Last Element
//     println!("Last element = {:?}", numbers.iter().last());

//     // Min
//     println!("Min = {:?}", numbers.iter().min());

//     // Max
//     println!("Max = {:?}", numbers.iter().max());

//     // Slice(Take)
//     let slice: Vec<_> = numbers.iter().take(3).collect();
//     println!("Slice first 3 = {:?}", slice);

//     // Reverse
//     let reversed: Vec<_> = numbers.iter().rev().collect();
//     println!("Reversed = {:?}", reversed);

//     // Chain
//     let trippled: Vec<_> = numbers
//         .iter()
//         .map(|&num| num * 3)
//         .filter(|&num| num >= 10)
//         .collect();
//     dbg!(trippled);
// }
