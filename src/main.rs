mod orders;

use orders::Shipping;

fn main() {
    let orders = orders::Order {
        id: String::from("Hello"),
    };
    dbg!(orders);

    let shipping = Shipping {
        id: String::from("Hello"),
    };
    dbg!(shipping);
}
