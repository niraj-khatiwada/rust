use std::convert::TryFrom;

#[derive(Debug)]
struct Animal {
    age: u8,
    is_old: bool,
}

impl From<u8> for Animal {
    fn from(age: u8) -> Self {
        match age {
            a @ 0..50 => Self {
                age: a,
                is_old: false,
            },
            o => Self {
                age: o,
                is_old: true,
            },
        }
    }
}

#[derive(Debug)]
struct Animal2 {
    age: i8,
    is_old: bool,
}

// We cannot use From and TryFrom for same struct
impl TryFrom<i8> for Animal2 {
    type Error = String;

    fn try_from(age: i8) -> Result<Self, Self::Error> {
        if age < 0 {
            Err(String::from("Invalid age"))
        } else if age >= 0 && age < 50 {
            Ok(Self {
                age: age,
                is_old: false,
            })
        } else {
            Ok(Self {
                age: age,
                is_old: true,
            })
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    // Simple From/Into in string
    let owned_string = String::from("Niraj");
    let owned_string: String = "Niraj".into();

    println!("{} {}", to_owned("Niraj"), type_of(to_owned("Niraj")));

    // Simple From/Into
    let animal = Animal::from(100);
    println!("{:?}", animal);

    let animal: Animal = 8.into();
    println!("{:?}", animal);

    // Problem with above approach is it will return Animal even when the age is negative. So, it is better to return a result on such scenario. That's where TryFrom/TryInto comes into play
    let animal2 = Animal2::try_from(-1);
    println!("{:?}", animal2);

    let animal2: Result<Animal2, _> = 100i8.try_into();
    println!("{:?}", animal2);
}

// Remember the &str.to_owned() function? It works like this under the hood:
fn to_owned(slice: &str) -> String {
    slice.into()
}
