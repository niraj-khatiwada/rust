use std::collections::HashMap;

struct Bill {
    id: u32,
    name: String,
    amount: f64,
}

type Bills = HashMap<u32, Bill>;

fn bill_management_system() {
    let allowed_options = vec!["l", "a", "v", "e", "d"];
    let initial_query = "Welcome to Bill Management System

What would you like to perform?

L = List all bills
A = Add new bill
V = View a bill
E = Edit a bill
D = Delete a bill
";

    let mut bills: Bills = HashMap::new();

    loop {
        let choice = readline(initial_query).to_lowercase();
        if allowed_options.contains(&choice.as_str()) {
            match choice.as_str() {
                "l" => {
                    println!("----- ALL BILLS ------");
                    list_bills(&bills);
                    println!("---------------------");
                }
                "a" => {
                    println!("----- NEW BILL ------");
                    let (bill_name, bill_amount) = add_bill_query();
                    add_bill(bill_name.as_str(), bill_amount, &mut bills);
                    println!("\nNew bill added successfully!");
                }
                "v" => {
                    println!("----- VIEW BILL ------");
                    let bill_id = get_bill_query();
                    if let Some(bill) = find_bill(bill_id, &bills) {
                        print_bill(bill);
                    } else {
                        println!("Bill not found\n");
                    }
                }
                "e" => {
                    println!("----- EDIT BILL ------");
                    let bill_id = get_bill_query();
                    if let Some(bill) = find_bill(bill_id, &bills) {
                        let (new_name, new_amount) = edit_bill_query(bill);
                        edit_bill(bill_id, new_name.as_str(), new_amount, &mut bills);
                        println!("\nBill edited successfully!");
                    } else {
                        println!("Bill not found\n");
                    }
                }
                "d" => {
                    println!("----- DELETE BILL ------");
                    let bill_id = get_bill_query();
                    if let Some(_) = find_bill(bill_id, &bills) {
                        delete_bill(bill_id, &mut bills);
                        println!("Bill deleted successfully!\n");
                    } else {
                        println!("Bill not found\n");
                    }
                }
                _ => {}
            }
        } else {
            println!("Invalid choice!");
            continue;
        }
    }
}
fn list_bills(bills: &Bills) {
    for bill in bills.values() {
        print_bill(bill);
    }
}

fn add_bill(name: &str, amount: f64, bills: &mut Bills) {
    let id = rand::random_range(1..=1000000);
    let new_bill = Bill {
        id: id,
        name: String::from(name),
        amount: amount,
    };
    bills.insert(id, new_bill);
}

fn find_bill(id: u32, bills: &Bills) -> Option<&Bill> {
    bills.get(&id)
}

fn delete_bill(id: u32, bills: &mut Bills) {
    bills.remove(&id);
}

fn edit_bill(id: u32, new_name: &str, new_amount: f64, bills: &mut Bills) {
    if bills.get(&id).is_some() {
        bills.insert(
            id,
            Bill {
                id: id,
                name: String::from(new_name),
                amount: new_amount,
            },
        );
    }
}

fn add_bill_query() -> (String, f64) {
    let bill_name = loop {
        let name = readline("What is the name of the bill?");
        if !name.is_empty() {
            break name;
        }
        println!("Please provide a valid name.");
    };

    let bill_amount = loop {
        let amount = readline("What is the amount of the bill?");
        if !amount.is_empty() {
            match amount.parse::<f64>() {
                Ok(amt) => {
                    if amt >= 0f64 {
                        break amt;
                    }
                }
                Err(_) => {}
            };
        }
        println!("Please provide a valid amount.");
    };
    (bill_name, bill_amount)
}

fn get_bill_query() -> u32 {
    loop {
        let bill_id_str = readline("Enter id of the bill.");
        let bill_id: u32 = match bill_id_str.parse::<u32>() {
            Ok(bill_id) => bill_id,
            Err(_) => {
                println!("Invalid bill id. Please enter a valid bill id.");
                continue;
            }
        };
        return bill_id;
    }
}

fn edit_bill_query(bill: &Bill) -> (String, f64) {
    println!("Current Name: {}", bill.name);
    let new_name = loop {
        let name = readline("Enter new name of the bill:");
        if !name.is_empty() {
            break name;
        }
        println!("Please provide a valid name.");
    };

    println!("Current Amount: {}", bill.amount);
    let new_amount = loop {
        let amount = readline("Enter new amount of the bill:");
        if !amount.is_empty() {
            match amount.parse::<f64>() {
                Ok(amt) => {
                    if amt >= 0f64 {
                        break amt;
                    }
                }
                Err(_) => {}
            };
        }
        println!("Please provide a valid amount.");
    };
    (new_name, new_amount)
}

fn print_bill(bill: &Bill) {
    println!(
        "Bill id = {} | name = {} | amount = {}\n",
        bill.id, bill.name, bill.amount
    );
}

fn readline(question: &str) -> String {
    println!("{}", question);

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read input.");

    String::from(buffer.trim())
}
