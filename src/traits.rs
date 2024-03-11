use rand::Rng;

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("Woof, Woof!")
    }
    fn is_silent(&self) -> bool {
        let random = get_random_value();
        return random == 1;
    }
}
struct Cat;
impl Noise for Cat {
    fn make_noise(&self) {
        println!("Meow, Meow!")
    }
    fn is_silent(&self) -> bool {
        let random = get_random_value();
        return random == 1;
    }
}

trait Noise {
    fn make_noise(&self);
    fn is_silent(&self) -> bool;
}

fn main() {
    let dog: Dog = Dog {};
    dog.make_noise();

    let cat: Cat = Cat {};
    cat.make_noise();

    make_noise(&dog);
    make_noise(&cat);

    is_silent(&dog);
    is_silent(&cat);
}

// animal that implements Noise trait
fn make_noise(animal: &impl Noise) {
    animal.make_noise();
}

fn is_silent(animal: &impl Noise) {
    println!("Is silent? {:?}", animal.is_silent())
}

fn get_random_value() -> u32 {
    let mut thread_range = rand::thread_rng();
    thread_range.gen_range(0..=1)
}
