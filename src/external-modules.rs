mod orders;

fn main() {
    let orders = orders::Order {
        id: String::from("Hello"),
    };
    dbg!(orders);

    let shipping = orders::shipping::Shipping {
        id: String::from("Hello"),
    };
    dbg!(shipping);
}
