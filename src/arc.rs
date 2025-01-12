use std::sync::Arc;
use std::{ops::Deref, thread};

// Imagine we can separate doors from a vehicle inside a factory. When separated, we want to know which vehicle it belongs to later when we re-attach them.
// With reference counted, we can still use the referenced value even after it is dropped.
#[derive(Debug, PartialEq)]
struct Vehicle {
    name: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Arc<Vehicle>,
}

impl From<&str> for Vehicle {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_owned(),
        }
    }
}
// View Reference Counted `rc.rs` file first
fn main() {
    let mustang = Arc::new(Vehicle::from("Mustang"));

    let left_door = Door {
        // vehicle: mustang.clone(), // new reference to mustang
        vehicle: Arc::clone(&mustang), // We can also clone like this
    };

    let right_door = Door {
        vehicle: mustang.clone(), // new reference to mustang
    };

    drop(mustang);

    // Even after the original owned mustang is dropped, we can still reference it using RC.
    let mustang_ref_l = left_door.vehicle.deref();
    let mustang_ref_r = right_door.vehicle.deref();
    // NOTE: We cannot get the origin owned `Vehicle` since it's already destroyed. We can only get the reference which is stored by RC.

    println!("{}", mustang_ref_l == mustang_ref_r); // true

    // We need to use Arc for thread safety
    let handle = thread::spawn(move || {
        // dbg!(mustang); // Mustang is already destroyed. So we can no longer use it.
        dbg!(left_door); // But we can still reference the mustang within left_door.
        dbg!(right_door); // But we can still reference the mustang within right_door.
    });

    handle.join().unwrap();
}
