// Trait Objects are used to make a heterogeneous collection, a collection having different types

trait Noise {
    fn make_noise(&self);
}

struct Car;

impl Noise for Car {
    fn make_noise(&self) {
        println!("Car: Vroom Vroom")
    }
}

struct Bike;

impl Noise for Bike {
    fn make_noise(&self) {
        println!("Bike: Vroom Vroom")
    }
}

fn borrow_vehicle(obj: &dyn Noise) {
    obj.make_noise();
}

fn borrow_vehicle_box(obj: Box<dyn Noise>) {
    obj.make_noise();
}

fn main() {
    // let list = vec! {Bike, Car}; -> This won't compile because we can only have similar types in a collection

    let car: Box<dyn Noise> = Box::new(Car);
    let bike: Box<dyn Noise> = Box::new(Bike);

    let list = vec! {car, bike}; // Now this will compile
    for item in list {
        item.make_noise();
    }

    let car2: Box<dyn Noise> = Box::new(Car);
    let bike2: Box<dyn Noise> = Box::new(Bike);
    let mut list2: Vec<Box<dyn Noise>> = Vec::new();
    list2.push(car2);
    list2.push(bike2);


    // Passing parameters in function
    let c = Car;
    borrow_vehicle(&c);
    let b = Bike;
    borrow_vehicle(&b);

    // The above method is for borrowing trait objects.
    // We can also move trait objects using Box
    let car3: Box<dyn Noise> = Box::new(Car);
    let bike3: Box<dyn Noise> = Box::new(Bike);
    borrow_vehicle_box(car3);
    borrow_vehicle_box(bike3);
}






