// enum Discount {
//     Flat(i32),
//     Percentage(u16),
// }

// struct Event {
//     name: String,
//     price: u32,
// }

// fn main() {
//     let num = 100;

//     match num {
//         0..=99 => println!("Less than 99"),
//         _ => println!("Greater than or equal to 100"),
//     }

//     let discount = Discount::Flat(50);

//     match discount {
//         Discount::Flat(50) => println!("Discount value is exactly 50"),
//         Discount::Flat(value) => println!("Discount value is {}", value),
//         Discount::Percentage(percent) => println!("Percentage value is {}", percent),
//     }

//     let event = Event {
//         name: String::from("Taylor Swift's Concert"),
//         price: 1000,
//     };

//     // match event {
//     //     Event { name, price } => println!("Concert name is {} and price is {}", name, price),
//     // }

//     match event {
//         Event { price: 1000, .. } => {
//             println!("Don't care about concert name but price is 1000")
//         }
//         Event { .. } => print!("Unknown concert"),
//     }
// }

// Problem
// enum Ticket {
//     Backstage(String, u32),
//     VIP(String, u32),
//     Standard(u32),
// }

// fn main() {
//     let tickets: Vec<Ticket> = vec![
//         Ticket::Backstage(String::from("Niraj Khatiwada"), 10000),
//         Ticket::VIP(String::from("Chandrakala Khatiwada"), 1000),
//         Ticket::Standard(100),
//     ];

//     for ticket in tickets {
//         match ticket {
//             Ticket::Backstage(name, price) => {
//                 println!("BackStage Ticket for {} with price {}", name, price)
//             }
//             Ticket::VIP(name, price) => {
//                 println!("VIP ticket for {} with price {}", name, price)
//             }
//             Ticket::Standard(price) => {
//                 println!("Standard ticket with price {}", price)
//             }
//         }
//     }
// }
