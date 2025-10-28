trait Clicky {
    fn click(&self);
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) {
        println!("Type...Type...")
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("Tap...Tap...")
    }
}

fn main() {
    // Let's create a heterogenous vector using trait objects
    let keyboard = Keyboard;
    let mouse = Mouse;

    let mut clickers: Vec<&dyn Clicky> = Vec::new();
    clickers.push(&keyboard); // trait object
    clickers.push(&mouse);

    for clicker in clickers {
        clicker.click();
    }

    // We can also use Box. Use above syntax as it is more common
    let mut clickers: Vec<Box<dyn Clicky>> = Vec::new();
    clickers.push(Box::new(keyboard)); // trait object
    clickers.push(Box::new(mouse));

    for clicker in clickers {
        clicker.click();
    }
}
