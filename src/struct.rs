// Tuple Structs
struct DiscountPercentile(u8);

impl DiscountPercentile {
    fn calculate_total(&self, price: f64) -> f64 {
        price - ((self.0) as f64 * price) / 100.0
    }
}

#[derive(Debug, Clone)]
struct Coffee {
    name: String,
    price: u32,
}

impl Coffee {
    // static methods
    fn new(name: String, price: u32) -> Self {
        Self { name, price }
    }
}

// Rust will merge multiple impl blocks
impl Coffee {
    // The parameter (&self) is equivalent to (self: &Self)
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_price(&self) -> u32 {
        self.price
    }

    // Builder Pattern: When we return the same instance ref after performing some operation
    fn increment_price(&mut self) -> &mut Self {
        self.price += 1;
        self
    }

    fn double_price(&mut self) -> &mut Self {
        self.price *= 2;
        self
    }

    fn triple_price(&mut self) -> &mut Self {
        self.price *= 3;
        self
    }
}

fn main() {
    let discount = DiscountPercentile(10);
    println!("Total: {}", discount.calculate_total(200.0));

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 100,
    };
    let mut mocha = Coffee::new(String::from("Mocha"), 100);

    let mocha2 = Coffee {
        name: String::from("Mocha 2"),
        ..mocha // mocha won't be moved here because `price` is u32 and it will just be copied.
    };

    let mocha3 = Coffee {
        price: 200,
        // ..mocha  -> mocha will be moved here since `name` is String and won't be copied.
        ..(mocha.clone()) // -> We need to clone here otherwise mocha will be moved.
    };

    println!("{:#?}", mocha);
    println!("{} {}", mocha.get_name(), mocha.get_price());
    // Builder Pattern
    println!(
        "New Price: {}",
        mocha
            .increment_price()
            .double_price()
            .triple_price()
            .get_price()
    );
    // In most of the cases, we will clone the struct for usages in other struct instances.
}
