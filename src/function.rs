fn function() {
    let mut name = String::from("Niraj");
    greet_and_change_name(&mut name);
    println!("My new name is {name}");

    let name_str = "Niraj";
    greet_and_change_name_str(name_str)
}

fn greet_and_change_name(name: &mut String) {
    println!("Hello {name}.");
    *name = "Suraj".to_owned();
}
fn greet_and_change_name_str(name: &str) {
    println!("Hello str {name}.");
}
