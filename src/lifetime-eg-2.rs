#[derive(Debug, PartialEq)]
enum City {
    London,
    California,
    Kathmandu,
}

#[derive(Debug)]
struct IDCard {
    name: String,
    age: u8,
    city: City,
}

impl IDCard {
    fn new(name: &str, age: u8, city: City) -> Self {
        return Self {
            name: String::from(name),
            age,
            city,
        };
    }
}

#[derive(Debug)]
struct Goats<'a>(Vec<&'a IDCard>);

impl<'a> Goats<'a> {
    fn living_in_london(&self) -> Self {
        Self { 0: self.0.iter().filter(|id| id.city == City::London).map(|id| *id).collect() }
    }
}

#[derive(Debug)]
struct Ages(Vec<u8>);

fn main() {
    let ids = vec![IDCard::new("Michael Scott", 46, City::London), IDCard::new("Dwight Schrute", 45, City::California), IDCard::new("Niraj Khatiwada", 26, City::Kathmandu)];

    let goats = Goats(ids.iter().filter(|id| id.age > 40).collect());

    println!("\nGoats");
    for goat in goats.0.iter() {
        println!("{:?}", goat)
    }

    println!("\nLiving in London");
    for goat_london in goats.living_in_london().0.iter() {
        println!("{:?}", goat_london);
    }

    // This is a copy which is not working on original ids.
    let ages = Ages(ids.iter().map(|id| id.age).collect());
    println!("{:?}", ages);

    let f = "I'm longest";
    let s = "No I'm longest";
    println!("{:?}", longest_string(f, s));
}


// Lifetime in functions
fn longest_string<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        return first;
    }
    return second;
}