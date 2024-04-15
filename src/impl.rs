struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn default() -> Self {
        Self {
            first_name: String::from("John"),
            last_name: String::from("Doe"),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let person = Person::default();

    println!("{:?}", person.full_name());
    println!("{:?}", person.full_name());
}

// Activity

// #[derive(Debug)]
// enum Color {
//     Red,
//     Green,
//     Blue,
// }

// struct ShippingBox {
//     dimensions: (i32, i32, i32),
//     weight: f64,
//     color: Color,
// }

// impl ShippingBox {
//     fn new(dimensions: (i32, i32, i32), weight: f64, color: Color) -> Self {
//         ShippingBox {
//             dimensions,
//             weight,
//             color,
//         }
//     }
//     fn print_characteristics(&self) {
//         println!("{:?}", self.dimensions);
//         println!("{:?}", self.weight);
//         println!("{:?}", self.color);
//     }
// }

// fn main() {
//     let shipping_box = ShippingBox::new((1, 2, 3), 20.0, Color::Red);
//     shipping_box.print_characteristics();
// }
