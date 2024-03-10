// struct Person {
//     name: String,
//     gender: Option<String>,
// }

// fn main() {
//     let person = Person {
//         name: String::from("Karen"),
//         gender: Some(String::from("Male")),
//     };

//     println!("{:?}", person.name);
//     println!("{:?}", person.gender);

//     println!(
//         "{:?} {:?}",
//         person.gender.is_some(),
//         person.gender.is_none()
//     );

//     match &person.gender {
//         Some(gender) => println!("Person's gender is {}", gender),
//         None => println!("Person has no gender"),
//     }

//     // Map moves the Option value
//     // let mapped_value = &person
//     //     .gender
//     //     .map(|gender| [gender, String::from(" Gender")].concat());
//     // println!("Mapped value {:?}", mapped_value);

//     // Filter borrows the Option value
//     // let filtered_value = &person.gender.filter(|gender| gender.len() == 4);
//     // println!("Filtered value {:?}", filtered_value);

//     // Default value for Option
//     // let or_elsed = &person.gender.or_else(|| Some(String::from("Not answered")));
//     // println!("Default value {:?}", or_elsed)

//     // Return value from Option directly
//     let unwrapped = &person
//         .gender
//         .unwrap_or_else(|| String::from("Not answered"));
//     println!("Final value {:?}", unwrapped);

//     dbg!(unwrapped);
// }
