trait LivingThing {
    fn calculate_age(&self) -> u16;
}
struct Person {
    name: String,
    born: u16,
}
impl LivingThing for Person {
    fn calculate_age(&self) -> u16 {
        2025 - self.born
    }
}

struct Animal {
    name: String,
    born: u16,
}
impl LivingThing for Animal {
    fn calculate_age(&self) -> u16 {
        2025 - self.born
    }
}

fn traits() {
    let p1 = Person {
        name: String::from("Niraj"),
        born: 1998,
    };
    println!("{}", get_age(&p1));

    let a1 = Animal {
        name: String::from("Dog"),
        born: 2022,
    };
    println!("{}", get_age(&a1));
}

// fn get_age<T: LivingThing>(entity: &T) -> u16 {
//     entity.calculate_age()
// }

// fn get_age<T>(entity: &T) -> u16
// where
//     T: LivingThing,
// {
//     entity.calculate_age()
// }

fn get_age(entity: &impl LivingThing) -> u16 {
    entity.calculate_age()
}
