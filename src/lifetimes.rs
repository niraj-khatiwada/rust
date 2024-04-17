#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}
#[derive(Debug)]
enum Part {
    Bolt,
    Hammer,
    Rope,
}

#[derive(Debug)]
struct MainHouse {
    part: Part,
}

// Here, the ware house need to share the same parts
#[derive(Debug)]
struct WareHouse<'a> {
    part: &'a Part,
}
// if the strict has lifetimes, it's impl should have as well
impl<'a> WareHouse<'a> {
    fn print_part(&self) {
        println!("Warehouse has {:?} part", self.part);
    }
}

fn main() {
    let mut person = Person { name: "Niraj" };
    borrow_person(&mut person);
    println!("Name = {:?}", person);

    // Automobiles

    // Let's say a part was borrowed by main house first
    let mainhouse = MainHouse { part: Part::Bolt };

    {
        let warehouse = WareHouse {
            part: &mainhouse.part,
        };
        warehouse.print_part();
    }
    // At this point, the warehouse does not exist and is already destroyed. However, the borrowed Part::Bolt will still be there.
    println!("Mainhouse has {:?} part", mainhouse.part);
    borrow_part(&mainhouse.part);

    println!("{:?}", longest_string("halo", "world"));
}

fn borrow_person(person: &mut Person) {
    *person = Person { name: "King" }
}

fn borrow_part<'a>(part: &'a Part) {
    println!("Borrowed part by function {:?}", part)
}

fn longest_string<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() > two.len() {
        one
    } else {
        two
    }
}
