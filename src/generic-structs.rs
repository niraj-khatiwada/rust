trait Animal {
    fn make_sound(&self);
}

struct Person<T: Animal> {
    name: String,
    pet: T,
}

struct Dog {
    name: String,
}
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Bhow Bhow");
    }
}

struct Cat {
    name: String,
}
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow Meow");
    }
}

fn main() {
    let dog1 = Dog {
        name: String::from("Oreo"),
    };
    let cat1 = Cat {
        name: String::from("Sigrid"),
    };
    let p1 = Person {
        name: String::from("Niraj"),
        pet: cat1,
    };
    p1.pet.make_sound();
    make_pet_sound(&p1);
}

fn make_pet_sound<T: Animal>(person: &Person<T>) {
    person.pet.make_sound()
}
