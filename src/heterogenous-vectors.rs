trait Clicky {
    fn click(&self) -> String;
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) -> String {
        String::from("Click Click")
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) -> String {
        String::from("Click")
    }
}

// Heterogenous Vectors: vectors with different data type.
// Review "Trait Objects" i.e. `trait-objects.rs` first
fn main() {
    let keyboard: Box<dyn Clicky> = Box::new(Keyboard); // borrow
    let mouse: Box<dyn Clicky> = Box::new(Mouse);

    let clickers: Vec<Box<dyn Clicky>> = vec![keyboard, mouse];
    click_all_clickers(clickers);
}

fn click_all_clickers(clickers: Vec<Box<dyn Clicky>>) {
    for clicker in clickers {
        click_sound(clicker);
    }
}

fn click_sound(obj: Box<dyn Clicky>) {
    println!("{:?}", obj.click());
}
