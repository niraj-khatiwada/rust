use std::collections::HashMap;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Bill {
    id: u64,
    name: String,
    amount: f64,
}

type Database = HashMap<u64, Bill>;

fn main() {
    let mut database: Database = HashMap::new();
    loop {
        let selected_number = get_selection().unwrap_or_else(|_| 0);

        match selected_number {
            1 => {
                let new_bill = add_prompt();
                add_bill(&new_bill, &mut database);
            }
            2 => {
                let id = view_prompt();
                let bill = view_bill(id, &database);
                match bill {
                    Some(b) => {
                        print_bill(b);
                    }
                    None => println!("Bill with that id not found."),
                }
            }
            3 => {
                let bill = update_prompt();
                update_bill(&bill, &mut database);
            }
            4 => {
                let id = delete_prompt();
                delete_bill(id, &mut database);
            }
            5 => {
                list_all_bills(&database);
            }
            _ => panic!(
                "Invalid selection {:?}. Select within 1-6",
                &selected_number
            ),
        }
    }
}

fn get_selection() -> io::Result<u8> {
    println!(
        r"
== Manage Bills==
1. Add Bill
2. View Bill
3. Update Bill
4. Delete Bill
5. List All Bills

Enter your selection: "
    );

    let user_input = take_input();
    let selection: Result<u8, _> = user_input.to_string().parse();
    let Ok(selected_number) = selection else {
        panic!("Invalid selection {:?}", user_input)
    };
    Ok(selected_number)
}

fn add_bill(bill: &Bill, database: &mut Database) {
    let Ok(time_since_epoch) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        panic!("Something went wrong while adding bill.")
    };
    database.insert(time_since_epoch.as_secs(), bill.clone());
    println!("New bill added successfully!")
}

fn add_prompt() -> Bill {
    let name: String;
    let amount: f64;

    loop {
        println!("Enter bill name: ");
        name = take_input();
        break;
    }

    loop {
        println!("Enter expense amount: ");
        let expense_amount_raw = take_input();
        let expense_amount: Result<f64, _> = expense_amount_raw.parse();
        let Ok(expense) = expense_amount else {
            print!("Invalid amount. Enter a valid value for amount.");
            continue;
        };
        amount = expense;
        break;
    }
    Bill {
        id: generate_id(),
        name,
        amount,
    }
}

fn list_all_bills(database: &Database) {
    println!("-------");
    for bill in database.values() {
        print_bill(bill);
    }
    println!("-------")
}

fn view_bill(id: u64, database: &Database) -> Option<&Bill> {
    database.get(&id)
}

fn view_prompt() -> u64 {
    loop {
        println!("Enter bill id: ");
        let bill_id_raw = take_input();
        let Ok(bill_id_parsed): Result<u64, _> = bill_id_raw.parse() else {
            println!("Invalid Id. Please enter a valid bill ID.");
            continue;
        };
        return bill_id_parsed;
    }
}

fn update_bill(bill: &Bill, database: &mut Database) {
    let _bill: Option<&Bill> = view_bill(bill.id, database);
    let Some(_) = _bill else {
        println!("Bill with that id not found.");
        return;
    };

    // One way is to mutate by match using get_mut
    match database.get_mut(&bill.id) {
        Some(matched_bill) => {
            matched_bill.name = bill.name.to_owned();
            matched_bill.amount = bill.amount;
        }
        None => (),
    }
    // Another way is to just enter the new value. Downside of this is, if you want to just mutate one entry, you have to create whole new struct. So prefer above.
    // database.insert(bill.id, bill.clone());
    print!("Data updated successfully!")
}

fn update_prompt() -> Bill {
    let name: String;
    let amount: f64;
    let bill_id: u64;

    loop {
        println!("Enter bill id: ");
        let bill_id_raw = take_input();
        let Ok(bill_id_parsed): Result<u64, _> = bill_id_raw.parse() else {
            println!("Invalid Id. Please enter a valid bill ID.");
            continue;
        };
        bill_id = bill_id_parsed;
        break;
    }

    loop {
        println!("Enter new bill name: ");
        let bill_name = take_input();
        name = bill_name;
        break;
    }

    loop {
        println!("Enter new expense amount: ");
        let expense_amount_raw = take_input();
        let expense_amount: Result<f64, _> = expense_amount_raw.parse();
        let Ok(expense) = expense_amount else {
            print!("Invalid amount. Enter a valid value for amount.");
            continue;
        };
        amount = expense;
        break;
    }
    Bill {
        id: bill_id,
        name,
        amount,
    }
}

fn delete_bill(id: u64, database: &mut Database) {
    match database.remove(&id) {
        Some(_) => println!("Bill deleted successfully!"),
        None => print!("Bill with that id not found"),
    }
}
fn delete_prompt() -> u64 {
    loop {
        println!("Enter bill id: ");
        let bill_id_raw = take_input();
        let Ok(bill_id_parsed): Result<u64, _> = bill_id_raw.parse() else {
            println!("Invalid Id. Please enter a valid bill ID.");
            continue;
        };
        return bill_id_parsed;
    }
}

fn take_input() -> String {
    let mut buffer = String::new();
    let readline = io::stdin().read_line(&mut buffer);
    let Ok(_) = readline else {
        panic!("Something went wrong while taking user input");
    };
    buffer.trim().to_owned()
}

fn generate_id() -> u64 {
    let Ok(time_since_epoch) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        panic!("Something went wrong while generating id.")
    };
    return time_since_epoch.as_secs();
}

fn print_bill(bill: &Bill) {
    println!(
        "Id= {:?} | Name = {:?} | Amount = {:?}",
        bill.id, bill.name, bill.amount
    )
}
