// mod greet {
//     use std::collections::HashMap; // We cannot use global Hashmap inside modules so need to be imported here.

//     pub fn hello() {
//         println!("Hello!");
//         self::goodbye();
//         let mut _map = HashMap::new();
//         _map.insert("hello", "World");

//         for (key, value) in _map.iter() {
//             println!("{:?}: {:?}", key, value);
//         }
//     }

//     fn goodbye() {
//         println!("Good Bye!")
//     }
// }

// mod math {
//     pub fn sum(a: i32, b: i32) {
//         println!("Sum = {}", a + b);
//         self::multiply(a, b)
//     }

//     fn multiply(a: i32, b: i32) {
//         println!("Product = {}", a * b)
//     }
// }

// fn main() {
//     greet::hello();
//     math::sum(1, 2)
// }
