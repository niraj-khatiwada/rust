fn main() {
    let name_str1 = "Niraj";
    let name_str2 = "Niraj";
    println!("{}", name_str1 == name_str2); // true. == will check the content not memory address
    println!("{}", name_str1.as_ptr() == name_str2.as_ptr()); // true since they point to same memory address for string "Niraj"

    let name1 = String::from("Niraj");
    let name2 = String::from("Niraj");
    println!("{}", name1 == name2); // true. == will check the content not memory address
    println!("{}", name1.as_ptr() == name2.as_ptr()); // false // Since name1 and name2 are two separate owners and point to different memory address
}
