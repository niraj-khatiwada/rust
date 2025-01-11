struct Person {
    name: String,
}

struct Vehicle {
    name: String,
    manufactured_date: u16,
}

enum Status {
    Active(u32),
    Inactive,
    Incharge(Person),
}

fn main() {
    let person = Person {
        name: String::from("Niraj"),
    };
    let activity_status = Status::Incharge(person);

    if let Status::Active(val) = activity_status {
        println!("{}", val)
    }

    let vehicle = Vehicle {
        name: String::from("Bugatti"),
        manufactured_date: 2025,
    };

    match vehicle {
        Vehicle {
            manufactured_date: 2026, // exactly equal to 2026
            ..
        } => println!("1. Manufactured date: {}", vehicle.manufactured_date),
        Vehicle {
            manufactured_date: s @ 1..2000, // between 1-2000
            ..
        } => println!("2. Manufactured date: {}", s),
        Vehicle {
            manufactured_date: s @ 2000.., // beyond 2000
            ..
        } => println!("3. Manufactured date: {}", s),
        Vehicle { name, .. } => println!("Vehicle Name: {}", name),
        _ => (),
    }
}
