fn main() {
    // A slice is a reference to a portion/window of an owned data.

    let name = String::from("Niraj Khatiwada");
    let first_name_slice = &name[0..=5]; // Here [0..5] is a window. Remember, the type of slice is `&str` here instead of `&String`.
    println!("{first_name_slice}");

    let name_as_str = &name[..]; // Quick way to convert String to string slice
    println!("{name_as_str}");

    let languages = ["JavaScript", "Python", "Rust"];
    let bad_languages_slice = &languages[0..2]; // Slice must be created as a reference type.
    println!("Bad languages: {:?}", bad_languages_slice); // Here [0..2] is a window

    let emoji = "🚀";
    // let emoji_slice = &emoji[..2]; // This will panic. We need to slice the whole bytes of the emoji
    let emoji_slice = &emoji[..4]; // This will work.
    println!("{}", emoji_slice);

    let another_name = String::from("Rust");
    get_name(&another_name); // Although &another_name = &String, Rust will convert this into a string slice &str and pass it since both of them is trying to access the string reference.
    get_name(&another_name[..]); // We can pass string slice as usual but is same as shown above.

    let list = [1, 2, 3, 4, 5, 6];
    print_length(&list); // A list of exact 6 lengths must be supplied.
    print_slice_length(&list); // &[u8; 6] will be inferred as &[u8] slice by Rust
    print_slice_length(&list[..1]); // Any length can be supplied.

    let mut nums = [1, 2, 3, 4, 5];
    let num_slice = &mut nums[2..4]; // reference to [3,4] window
    num_slice[0] = 0; // Changes the value in reference which in turn changes the original array value as well.
    println!("{:?}", nums);
}

fn get_name(name: &str) {
    // name accepts both &String and &str. &String will be eventually use the full slice of the name. This is called a Deref Coercion
    println!("Hello {name}")
}

fn print_length(list: &[u8; 6]) {
    for i in list {
        println!("List: {i}")
    }
}

fn print_slice_length(list: &[u8]) {
    for i in list {
        println!("Slice: {i}")
    }
}
