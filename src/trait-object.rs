trait Clicky {
    fn click(&self);
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) {
        println!("click clack")
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("click click")
    }
}

fn main() {
    // let keyboard = Keyboard;
    // let mouse = Mouse;
    // let clickers = vec![keyboard, mouse]; // This will cause mismatch type error

    // We need to use trait objects to allow vector to have different data types.
    // Vectors uses stack under the hood and stack must have same data type since same data type will always have same offset value to jump to
    // But for different data types, we cannot use stack. That's why we need to convert the type to heap using Box. Heap points the data in different space unlike stack.
    let keyboard: Box<dyn Clicky> = Box::new(Keyboard);
    let mouse: Box<dyn Clicky> = Box::new(Mouse);

    click(&Keyboard);
    click(&Mouse);
    _click(&keyboard);
    _click(&mouse);

    let clickers: Vec<Box<dyn Clicky>> = vec![keyboard, mouse];

    for clicker in clickers {
        clicker.click();
    }
}

fn click(clicker: &dyn Clicky) {
    clicker.click();
}

fn _click(clicker: &Box<dyn Clicky>) {
    clicker.click();
}
