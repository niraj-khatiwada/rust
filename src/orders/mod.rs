pub mod shipping;

pub use shipping::Shipping; // we can directly export the Shipping struct from orders if we want.

#[derive(Debug)]
pub struct Order {
    pub id: String,
}
