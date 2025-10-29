use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
    name: String,
}

struct Door {
    vehicle: Rc<Vehicle>, // Notice, Rc of Vehicle not &Vehicle, that means the Door will be the new owner of the vehicle. The case is we want to identify which vehicle did this door belong to even when vehicle is already destroyed.
}
fn main() {
    let car = Rc::new(Vehicle {
        name: String::from("Mustang"),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car), // Notice, clone will create a new copy of the original vehicle.
    };

    let right_door = Door {
        vehicle: Rc::clone(&car),
    };

    drop(car); // Since left_door and right_door are now owners of the original vehicle pointer, dropping the original vehicle won't cause error.

    println!("{:?} {:?}", left_door.vehicle, right_door.vehicle);
    // when the program ends, the RC pointer of the original vehicle will also be destroyed.
}
