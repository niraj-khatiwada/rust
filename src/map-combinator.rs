// #[derive(Debug)]
// struct User {
//     id: i32,
//     name: String,
// }

// fn main() {
//     let num = maybe_num().map(|num| num + 1).map(|num| num + 10);
//     match num {
//         Some(n) => println!("Num is {:?}", n),
//         _ => {}
//     }

//     let username = "niraj";
//     let user = find_user(username).map(|user_id| User {
//         id: user_id,
//         name: String::from(username),
//     });
//     println!("{:?}", user)
// }

// fn maybe_num() -> Option<i32> {
//     Some(100)
// }

// fn find_user(name: &str) -> Option<i32> {
//     match name {
//         "niraj" => Some(1),
//         "suraj" => Some(2),
//         _ => None,
//     }
// }
