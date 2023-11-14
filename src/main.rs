#[derive(Debug)]
enum Part {
    Bolt,
    Panel,
}

struct RobotArm<'a> {
    part: &'a Part, // To access a borrowed value in struct, you must specify the lifetime of the ownership
}

struct AssemblyLine {
    parts: Vec<Part>,
}

// By default, this is what happens under the hood of ownership. We are just explicitly assigning a lifetime which is ellided(altered) by the compiler automatically.
fn borrow_part<'a>(arg: &'a Part) -> &'a Part {
    println!("{:?}", arg);
    return arg;
}

fn main() {
    let line = AssemblyLine { parts: vec![Part::Bolt, Part::Panel] };
    {
        let arm = RobotArm { part: &line.parts[0] };
        println!("{:?}", arm.part);
    };
    borrow_part(&line.parts[0]);
}

