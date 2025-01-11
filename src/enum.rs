#[derive(PartialEq)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug)]
enum DiscountType {
    Full,
    Percentage(u8), // Tuple variant
    Absolute(f64),
    Custom { price: f64 }, // struct variant
}

impl Status {
    fn is_active(&self) -> bool {
        self.eq(&Self::Active)
    }
}

fn main() {
    let status = Status::Active;
    println!("Is active? {}", status.is_active());

    let discount = DiscountType::Percentage(99);
    println!("{:?}", discount);
    let discount = DiscountType::Absolute(200.0);
    println!("{:?}", discount);
    let discount = DiscountType::Full;
    println!("{:?}", discount);
    let discount = DiscountType::Custom { price: 100.0 };
    println!("{:?}", discount);
}
