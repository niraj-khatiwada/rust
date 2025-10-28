#[derive(Debug)]
struct Vehicle {
    name: String,
}

#[derive(Debug)]
struct Wheel<'a> {
    name: String,
    owner: &'a Vehicle,
}

fn main() {
    let name = get_random_name();
    println!("{}", name);

    let vehicle = Vehicle {
        name: String::from("Mercedes"),
    };

    {
        let wheel = Wheel {
            name: String::from("Stainless"),
            owner: &vehicle,
        };
        println!("{:?}", wheel);
        // Wheel drops here but vehicle still remains.
    }

    println!("{:?}", vehicle);

    // name will now drop when this function ends.
    // vehicle will now drop when this function ends.
}

fn get_random_name<'a>() -> &'a str {
    "Niraj"
}
