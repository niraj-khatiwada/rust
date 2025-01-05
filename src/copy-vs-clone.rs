#[derive(Clone)] // We cannot use Copy trait for struct since there's no way to copy a struct
struct Person {
    name: String,
}

fn main() {
    // Scalar types like integers, float, boolean, characters, etc. have Copy trait by default
    let num1 = 100;
    let num2 = num1; // The value of num1 will be copied to num2

    // String has Clone trait by default
    let me = String::from("Niraj");
    let mut me_clone = me.clone(); // To use .clone() method, it must have implemented a Clone trait
    let other = me;
    me_clone.push_str(" Khatiwada");

    drop(me_clone);

    // me_clone.push_str("lolol"); // cannot use since it's already dropped above

    // Other compound types like struct do not have Copy, Clone, etc. traits by default. You have to assign them manually.
    let person1 = Person {
        name: String::from("Niraj"),
    };
    let person2 = person1.clone(); // Can only be cloned after adding Clone trait.
}
