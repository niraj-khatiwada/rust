use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
    id: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

fn main() {
    let car = Rc::new(Vehicle {
        id: String::from("Car"),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car), // when you do a clone here, it is the owner itself
    };

    let right_door = Door {
        vehicle: Rc::clone(&car),
    };

    // Even after dropping the car, it's reference is still available in left_door and right_door
    drop(car);

    println!("Right door {:?}", right_door);
    println!("Left door {:?}", left_door);
}
