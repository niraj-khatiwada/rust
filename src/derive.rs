// #[derive(Debug, Copy, Clone)]
// enum Status {
//     Active,
//     Inactive,
// }

// #[derive(Debug, Copy, Clone)]
// struct Person {
//     status: Status,
//     age: u16,
// }

// fn main() {
//     println!("{:?}", Status::Active);
//     println!("{:?}", Status::Inactive);

//     let person = Person {
//         status: Status::Active,
//         age: 26,
//     };

//     print_person(person);
//     print_person(person); // Since it has Copy derive
// }

// fn print_person(person: Person) {
//     println!("Status: {:?}", person.status);
//     println!("Age: {:?}", person.age);
// }
