// #[derive(Debug)]
// struct GroceryItem {
//     id: i32,
//     quantity: i32,
// }

// impl GroceryItem {
//     fn create() -> Self {
//         Self {
//             id: 2,
//             quantity: 101,
//         }
//     }
//     fn print_id_quantity(&self) {
//         println!("ID: {:?}, Quantity: {:?}", self.id, self.quantity)
//     }

//     fn this_also_works(item: &GroceryItem) {
//         println!("ID: {:?}, Quantity: {:?}", item.id, item.quantity)
//     }
// }

// #[derive(PartialEq)]
// enum Status {
//     Active,
//     Inactive,
// }

// impl Status {
//     fn new() -> Self {
//         Self::Active
//     }
//     fn is_active(&self) -> bool {
//         return self == &(Status::Active);
//     }

//     fn is_inactive(&self) -> bool {
//         return self == &(Status::Inactive);
//     }
// }

// fn main() {
//     let item = GroceryItem {
//         id: 1,
//         quantity: 100,
//     };

//     println!("New Grocery Item: {:?}", GroceryItem::create());

//     item.print_id_quantity();
//     item.print_id_quantity();
//     item.print_id_quantity();

//     GroceryItem::this_also_works(&item);
//     GroceryItem::this_also_works(&item);

//     let status = Status::new();

//     println!("{:?}", status.is_active());
//     println!("{:?}", status.is_inactive())
// }
