enum Status {
    Active,
    Deleted,
}


struct Person {
    name: String,
}

fn matching(status: &Status) {
    match status {
        Status::Active => println!("Active"),
        _ => println!("Inactive")
    }
}

fn main() {
    // Move Memory
    // let status: Status = Status::Active;
    // matching(status); // This is pass by value, status will be copied everytime and moved.
    // matching(status); // This second function cannot be compiled since the first matching() has moved the memory from main to matching. This called memory movement. The matching will drop the variable status at the end of the scope.

    // To prevent that, we need to borrow the memory in matching function. After borrowing, the owner of status is still main.
    // matching(&status)
    // matching(&status)


    // Borrow Memory
    let mut person: Person = Person { name: "Niraj".to_string() };
    println!("{:?}", person.name);
    change_name(&mut person);
    println!("{:?}", person.name);
}

fn change_name(person: &mut Person) {
    person.name = "Suraj".to_string();
}
