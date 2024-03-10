// use rand::prelude::*;

// fn main() {
//     let transaction: Result<bool, String> = Ok(true);
//     let transaction2: Result<bool, String> = Err(String::from("Something went worng..."));

//     match transaction {
//         Ok(was_successful) => println!(
//             "Transaction 1 complete. Was it successful? {}",
//             was_successful
//         ),
//         Err(message) => println!("Error: {}", message),
//     }

//     match transaction2 {
//         Ok(was_successful) => println!(
//             "Transaction 2 complete. Was it successful? {}",
//             was_successful
//         ),
//         Err(message) => println!("Error: {}", message),
//     }

//     println!("{:?}", run_transaction())
// }

// fn run_transaction() -> Result<f64, String> {
//     let mut thread_rng = rand::thread_rng();
//     let random: f64 = thread_rng.gen();
//     connect_database()?;
//     Ok(random * 10000.00)
// }

// fn connect_database() -> Result<f64, String> {
//     return Err(String::from("Database host incorrect."));
//     // return Ok(100.00);
// }
