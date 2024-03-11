struct Square {
    length: f64,
}
impl Shape for Square {
    fn calculate_perimeter(&self) -> f64 {
        return self.length * 4.0;
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}
impl Shape for Triangle {
    fn calculate_perimeter(&self) -> f64 {
        return self.a + self.b + self.c;
    }
}

trait Shape {
    fn calculate_perimeter(&self) -> f64;
}

fn main() {
    let square = Square { length: 10.00 };
    print_perimeter(&square);

    let triangle = Triangle {
        a: 1.0,
        b: 2.0,
        c: 3.0,
    };
    print_perimeter(&triangle);
}

fn print_perimeter(shape: &impl Shape) {
    println!("The perimeter is {:?}", shape.calculate_perimeter())
}
