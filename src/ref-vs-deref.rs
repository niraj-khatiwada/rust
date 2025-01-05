fn main() {
    let mut name = String::from("Niraj");
    change_name(&mut name);
    println!("{name}"); // Name will be changed
    let name_pass = name; // name can still be used since it was never moved. only reference was passed in above `change_name` function.
}

fn change_name(name: &mut String) {
    // name is reference to the original owned String, also called borrower
    *name = String::from("Khatiwada"); // Changing the borrower value will not change the owner value. So, to change the original value, we must point to the owner which can be done by dereferencing the borrower.
}
