trait Noise {
    fn noise(&self);
}

struct Dog {
    name: String,
}

impl Noise for Dog {
    fn noise(&self) {
        println!("Bhau Bhau {:?}", self.name);
    }
}

struct Cat {
    name: String,
}

impl Noise for Cat {
    fn noise(&self) {
        println!("Meow Meow {:?}", self.name);
    }
}


trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    size: i32,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        return 4 * self.size;
    }
}

struct Rectangle {
    length: i32,
    breadth: i32,
}

impl Perimeter for Rectangle {
    fn calculate_perimeter(&self) -> i32 {
        return 2 * (self.length + self.breadth);
    }
}

fn main() {
    let dog = Dog { name: String::from("Hachiko") };
    dog.noise();

    let cat = Cat { name: String::from("Daisy") };
    cat.noise();

    greet(Dog { name: String::from("Nala") });
    greet(Cat { name: String::from("Oliver") });

    println!("{:?}", perimeter(Square { size: 2 }));
    println!("{:?}", perimeter(Rectangle { length: 2, breadth: 3 }))
}


// animal cam be anything that has Trait Noise implemented
fn greet(animal: impl Noise) {
    animal.noise()
}


// one way to implement
// fn perimeter(shape: impl Perimeter) -> i32 {
//     return shape.calculate_perimeter();
// }

// Generics way: Recommended
fn perimeter<T: Perimeter>(shape: T) -> i32 {
    return shape.calculate_perimeter();
}
