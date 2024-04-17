trait Noise {
    fn make_noise(&self);
}

struct Cat {
    name: String,
}

impl Noise for Cat {
    fn make_noise(&self) {
        println!("{:?}", format!("{} says  Meow", self.name));
    }
}

struct Dog {
    name: String,
}

impl Noise for Dog {
    fn make_noise(&self) {
        println!("{:?}", format!("{} says Vau", self.name))
    }
}

fn main() {
    let cat = Cat {
        name: String::from("Oreo"),
    };
    cat.make_noise();

    let dog = Dog {
        name: String::from("Tiger"),
    };
    dog.make_noise()
}
