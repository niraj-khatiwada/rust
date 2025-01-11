trait LivingThing {
    fn get_name(&self) -> &String;
}

struct Human {
    name: String,
}

impl LivingThing for Human {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Default for Human {
    fn default() -> Self {
        Self {
            name: String::from("Jesus"),
        }
    }
}

struct Animal {
    name: String,
}

impl LivingThing for Animal {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl Default for Animal {
    fn default() -> Self {
        Self {
            name: String::from("Dinosaur"),
        }
    }
}

enum LivingCreature {
    Human(String),
    Animal(String),
}

impl LivingThing for LivingCreature {
    fn get_name(&self) -> &String {
        match self {
            Self::Human(name) => name,
            Self::Animal(name) => name,
        }
    }
}

fn main() {
    let me: Human = Human {
        name: String::from("Niraj"),
    };
    print_living_thing_name(me);

    let animal = Animal {
        name: String::from("Dog"),
    };
    print_living_thing_name(animal);

    let human_or_animal = LivingCreature::Animal(String::from("Cat"));
    print_living_thing_name(human_or_animal);
}

// `living_thing` must be something that implements `LivingThing` trait.
fn print_living_thing_name(living_thing: impl LivingThing) {
    println!("Living thing is {:?}", { living_thing.get_name() });
}

// This is same as above but using Generics. These are called Generic Functions
fn _print_living_thing_name<T: LivingThing>(living_thing: T) {
    println!("Living thing is {:?}", { living_thing.get_name() });
}

// Another way of writing Generic Functions.
fn __print_living_thing_name<T>(living_thing: T)
where
    T: LivingThing,
{
    println!("Living thing is {:?}", { living_thing.get_name() });
}
