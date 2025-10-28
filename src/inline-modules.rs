// Inline modules
mod greet {
    pub fn hello() {
        println!("Hello")
    }

    pub fn bye() {
        println!("Goodbye")
    }
}

mod math {
    pub fn add(a: u8, b: u8) -> u8 {
        return a + b;
    }
}

fn main() {
    greet::hello();
    greet::bye();

    println!("{}", math::add(1, 2));
}
