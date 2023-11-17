use std::rc::Rc;

// #[derive(Debug)]
// struct Vehicle {
//     name: String,
// }
//
// #[derive(Debug)]
// struct Door<'a> {
//     vehicle: &'a Vehicle,
// }
//
// fn main() {
//     let vehicle = Vehicle { name: String::from("Mercedes") };
//
//     let left_door = Door {
//         vehicle: &vehicle
//     };
//     let right_door = Door {
//         vehicle: &vehicle
//     };
//
//     drop(vehicle);
//
//     // Even after dropping the vehicle, the reference of vehicle will exist in left_door and right_door.
//     println!("Left door belongs to {:?}", left_door.vehicle);
//     println!("Right door belongs to {:?}", right_door.vehicle);
// }

#[derive(Debug)]
struct Vehicle {
    name: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

// Look at example above to see why dropping won't work and see this afterwards.
fn main() {
    let vehicle = Rc::new(Vehicle { name: String::from("Mercedes") });

    let left_door = Door {
        vehicle: Rc::clone(&vehicle)
    };
    let right_door = Door {
        vehicle: Rc::clone(&vehicle)
    };

    drop(vehicle);

    // Even after dropping the vehicle, the reference of vehicle will exist in left_door and right_door.
    println!("Left door belongs to {:?}", left_door.vehicle);
    println!("Right door belongs to {:?}", right_door.vehicle);
}

