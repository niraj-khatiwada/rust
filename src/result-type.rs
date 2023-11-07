// #[derive(Debug)]
// struct Transaction {
//     id: i32,
//     result: Result<i32, String>,
// }
//
// fn main() {
//     let transaction1 = Transaction { id: 1, result: Ok(100) };
//     let transaction2 = Transaction { id: 1, result: Err(String::from("Something went wrong...")) };
//
//     println!("{:?}", transaction1);
//     println!("{:?}", transaction2);
//
//     println!("{:?}", process_transaction(1));
//     println!("{:?}", process_transaction(2));
// }
//
// fn process_transaction(id: i32) -> Result<i32, String> {
//     let t = Transaction { id, result: Ok(100) };
//     return match t.result {
//         Ok(status_code) => {
//             println!("Successful with status code {:?} ", status_code);
//             Ok(status_code)
//         }
//         err => {
//             println!("Oops");
//             err
//         }
//     };
// }


// #[derive(Debug)]
// struct Transaction {
//     id: i32,
//     uuid: String,
//     result: Result<i32, String>,
// }
//
// impl Transaction {
//     fn new(id: i32, uuid: &str) -> Result<Self, &str> {
//         if id > 0 {
//             return Ok(Self {
//                 id,
//                 uuid: String::from(uuid),
//                 result: Ok(id),
//             });
//         }
//         return Err("id must be greater than 0.");
//     }
// }
//
// fn main() {
//     let transaction1 = Transaction::new(1, "primeagen");
//     let transaction2 = Transaction::new(-1, "blazing_fast");
//
//     println!("{:?}", transaction1);
//     println!("{:?}", transaction2);
// }
//


use rand::Rng;

#[derive(Debug)]
struct Transaction {
    id: i32,
    uuid: String,
    result: Result<i32, String>,
}

#[derive(Debug)]
enum AuthStatus {
    Allow,
    Deny,
}

impl Transaction {
    fn new(id: i32, uuid: &str) -> Result<Self, &str> {
        if id > 0 {
            return Ok(Self {
                id,
                uuid: String::from(uuid),
                result: Ok(id),
            });
        }
        return Err("id must be greater than 0.");
    }
}


fn main() {
    // let transaction1 = Transaction::new(1, "primeagen");
    // let transaction2 = Transaction::new(-1, "blazing_fast");
    //
    // println!("{:?}", transaction1);
    // println!("{:?}", transaction2);

    println!("{:?}", authorize());
}


fn connect_db() -> Result<String, String> {
    let random = rand::thread_rng().gen_range(0..=100);

    if random % 2 == 0 {
        return Ok(String::from("Connected!"));
    }
    return Err(String::from("Cannot connect to database"));
}

fn authorize() -> Result<AuthStatus, String> {
    let db = connect_db()?; // This will only work and proceed when result is Ok()
    println!("{:?}", db); // This won't be printed when db throws error. Instead this function will automatically return the Err() from db in above line when error occurs in connect_db.
    return Ok(AuthStatus::Allow);
}
