pub mod math {
    use std::collections::HashMap; // Each module must import libs inside their own scope
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    pub fn init_hashmap() {
        let map: HashMap<String, String> = HashMap::new();
    }
}

pub mod greet {
    pub fn hello(name: &str) {
        println!("Hello {}", name)
    }
}

fn main() {
    let sum = self::math::add(1, 2);
    println!("Sum {}", sum);

    self::greet::hello("Niraj");
}
