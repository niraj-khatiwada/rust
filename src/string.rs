// struct Employee {
//     name: String,
// }

// struct Person {
//     name: String,
//     age: u16,
//     fav_color: String,
// }

// fn main() {
//     print_string("Niraj Khatiwada");
//     println!("Owned String: {:?}", "Niraj Khatiwada".to_owned());
//     let owned_string = String::from("Niraj Khatiwada");
//     println!("Borrowed String: {:?}", &owned_string);
//     print_string(&owned_string);

//     let employee_name = "Niraj Khatiwada";
//     let employee = Employee {
//         name: String::from(employee_name),
//     };
//     println!("Employee: {:?}", employee.name);
//     print_string(&employee.name);

//     //
//     let people: Vec<Person> = vec![
//         Person {
//             name: String::from("Niraj"),
//             age: 26,
//             fav_color: String::from("Grey"),
//         },
//         Person {
//             name: String::from("Suraj"),
//             age: 9,
//             fav_color: String::from("Black"),
//         },
//     ];

//     for person in people {
//         if person.age <= 10 {
//             println!("Name: {:?}", person.name);
//             println!("Age: {:?}", person.age);
//             println!("Favorite Color: {:?}", person.fav_color);
//         }
//     }
// }

// fn print_string(string: &str) {
//     println!("String Slice: {:?}", string)
// }
