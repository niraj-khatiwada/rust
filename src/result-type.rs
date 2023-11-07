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


#[derive(Debug)]
struct Transaction {
    id: i32,
    uuid: String,
    result: Result<i32, String>,
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
    let transaction1 = Transaction::new(1, "primeagen");
    let transaction2 = Transaction::new(-1, "blazing_fast");

    println!("{:?}", transaction1);
    println!("{:?}", transaction2);
}

