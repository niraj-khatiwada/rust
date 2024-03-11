#[derive(Debug)]
enum Part {
    Bolt,
    Panel,
}

struct AssemblyLine {
    parts: Vec<Part>,
}

struct RobotArm<'a> {
    part: &'a Part,
}

fn main() {
    let assembly_line = AssemblyLine {
        parts: vec![Part::Bolt, Part::Panel],
    };

    {
        let robot_arm = RobotArm {
            part: &assembly_line.parts[0],
        };
        println!("{:?}", robot_arm.part)
    }
}
