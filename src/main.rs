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

