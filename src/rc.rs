use std::{ops::Deref, rc::Rc};

// Imagine we can separate doors from a vehicle inside a factory. When separated, we want to know which vehicle it belongs to later when we re-attach them.
// With reference counted, we can still use the referenced value even after it is dropped.
#[derive(Debug, PartialEq)]
struct Vehicle {
    name: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

impl From<&str> for Vehicle {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_owned(),
        }
    }
}

fn main() {
    let mustang = Rc::new(Vehicle::from("Mustang"));

    let left_door = Door {
        vehicle: Rc::clone(&mustang), // new reference to mustang
    };

    let right_door = Door {
        vehicle: Rc::clone(&mustang), // new reference to mustang
    };

    drop(mustang);

    // Even after the original owned mustang is dropped, we can still reference it using RC.
    let mustang_ref_l = left_door.vehicle.deref();
    let mustang_ref_r = right_door.vehicle.deref();
    // NOTE: We cannot get the origin owned `Vehicle` since it's already destroyed. We can only get the reference which is stored by RC.

    println!("{}", mustang_ref_l == mustang_ref_r); // true

    // dbg!(mustang); // Mustang is already destroyed. So we can no longer use it.
    // dbg!(left_door); // But we can still reference the mustang within left_door.
    // dbg!(right_door); // But we can still reference the mustang within right_door.
}
