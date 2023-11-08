// This HashMap is only available in main function not in modules.
use std::collections::HashMap;

// These are called inline modules
pub mod greet {
    use std::collections::HashMap;

    pub fn main() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(2, 2);
        println!("{:?}", map)
    }

    pub fn hello() {
        println!("Hello World")
    }

    fn goodbye() { println!("Goodbye World") }
}

fn main() {
    greet::hello();
    // greet::goodbye() goodbye() is private so cannot be invoked.

    // only hello
    use greet::hello;
    hello();

    // all
    use greet::*;
    hello();


    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);
    println!("{:?}", map);

    greet::main()
}





