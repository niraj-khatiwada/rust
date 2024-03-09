// #[derive(Debug)]
// enum Status {
//     Active,
//     Inactive,
// }

// struct GroceryItem {
//     id: i32,
//     quantity: i32,
// }

// fn main() {
//     let status: Status = Status::Active;
//     print_status(&status);
//     print_status(&status);
// }

// fn print_status(mut status: &Status) {
//     println!("Age is {status:?}");
//     status = &(Status::Inactive);
//     println!("Age is {status:?}");

//     let item = GroceryItem {
//         id: 100,
//         quantity: 100,
//     };
//     display_grocery_id(&item);
//     display_grocery_quantity(&item);
// }

// fn display_grocery_id(item: &GroceryItem) {
//     println!("{:?}", item.id)
// }

// fn display_grocery_quantity(item: &GroceryItem) {
//     println!("{:?}", item.quantity)
// }
