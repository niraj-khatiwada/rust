trait Clicky {
    fn click(&self);
}

struct Mouse {
    right_left: bool,
}
impl Clicky for Mouse {
    fn click(&self) {
        println!("Mouse clicked.")
    }
}

struct Keyboard {
    alt_shift: bool,
}
impl Clicky for Keyboard {
    fn click(&self) {
        println!("Keyboard clicked.")
    }
}

fn main() {
    let keyboard = Keyboard { alt_shift: true };
    let mouse = Mouse { right_left: true };

    // Move
    let dynamic_list: Vec<Box<dyn Clicky>> = vec![Box::new(keyboard), Box::new(mouse)]; // This kind of vector is called heterogeneous vector

    for i in dynamic_list {
        clickity_click(i);
    }

    // Borrow
    // let dynamic_list: Vec<&dyn Clicky> = vec![&keyboard, &mouse];

    // for i in dynamic_list {
    //     clickity_click(i);
    // }
}

// Move
fn clickity_click(clicky: Box<dyn Clicky>) {
    clicky.click()
}

// Borrow
// fn clickity_click(clicky: &dyn Clicky) {
//     clicky.click()
// }
