trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for Snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("Slithering to {}, {}", x, y);
    }
}

struct GrassHopper;
impl Move for GrassHopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("Hopping to {}, {}", x, y)
    }
}

// fn should_move(thing: impl Move, x: i32, y: i32) {
//     thing.move_to(x, y);
// }

// Using generic for above function
fn should_move<T: Move>(thing: T, x: i32, y: i32) {
    thing.move_to(x, y);
}

fn main() {
    let snake = Snake;
    should_move(snake, 1, 2);

    let gh = GrassHopper;
    should_move(gh, 3, 4)
}
