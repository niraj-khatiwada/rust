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

fn main() {
    let keyboard = Keyboard;
    let keyboard_obj: &dyn Clicky = &keyboard; // borrow
    click_sound_borrowed(keyboard_obj);

    let mouse: Box<dyn Clicky> = Box::new(Mouse);
    click_sound_owned(mouse);
}

// for borrowed
fn click_sound_borrowed(obj: &dyn Clicky) {
    println!("{:?}", obj.click());
}

// for owned
fn click_sound_owned(obj: Box<dyn Clicky>) {
    println!("{:?}", obj.click());
}
