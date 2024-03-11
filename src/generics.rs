trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for Snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("Slithering to ({x}, {y})")
    }
}

struct Crocodile;
impl Move for Crocodile {
    fn move_to(&self, x: i32, y: i32) {
        println!("Gliding to ({x}, {y})")
    }
}

fn main() {
    let snake = Snake;
    move_to(&snake, 1, 2);

    let crocodile = Crocodile;
    move_to(&crocodile, 4, 5);
}

// Generic functions are syntactic sugar of using `impl <Trait>` that we have used in trait. It functions in same way as that
fn move_to<T: Move>(a: &T, x: i32, y: i32) {
    a.move_to(x, y);
}

// Using impl
/*
    fn move_to(a: &impl Move, x: i32, y: i32) {
        a.move_to(x, y);
    }
*/
