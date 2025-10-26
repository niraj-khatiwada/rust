// Expression macro
macro_rules! printer {
    // $expr: Expression matcher. Eg: "Name", 1+2 or any function calls
    ($msg:expr) => {
        println!("$expr -> {}", $msg)
    };
}

// Identifier macro
macro_rules! make_fn {
    // $ident: Identifier like variable name, function name, struct name, enum name, etc.
    ($id:ident) => {
        fn $id(name: &str) {
            println!("$ident -> {} from {}", name, stringify!($id))
        }
    };
}

macro_rules! make_struct {
    ($id:ident) => {
        #[derive(Debug)]
        struct $id {
            pub age: u8,
        }

        impl $id {
            pub fn print_age(&self) {
                println!("Your age is {}", self.age)
            }
        }
    };
}

// Type macro
macro_rules! create_default_from_type {
    // $type: Receives data types like u32, f32, bool, etc.
    ($type: ty) => {
        let val: $type = Default::default();
        println!("Default value of type {} is {}", stringify!($type), val);
    };
}

// Pattern macro
macro_rules! exists {
    ($val: expr, $($p:pat), *) => {
        match $val {
            $(
                $p => println!("Exists"),
            )*
            _ => println!("Does not exist.")
        }

        // In compile time, the above will look like this:
        // match 11 {
        //     1 => println!("Exists"),
        //     2 => println!("Exists"),
        //     11 => println!("Exists"),
        //     _ => println!("Does not exist.")
        // }
        // So $(...)* is just spreading the pattern
    };
}

make_fn!(hello); // Creates a function named hello in global scope
make_struct!(Person); // Creates a struct named Person in global scope

fn macros() {
    printer!("Hello World");
    printer!(1 + 2 + 3);
    printer!(greet("Niraj"));

    hello("Niraj");

    let person = Person { age: 99 };
    println!("{person:?}");
    person.print_age();

    create_default_from_type!(i32);
    create_default_from_type!(String);
    create_default_from_type!(bool);

    exists!(11, 1, 2, 11);
}

fn greet(name: &str) -> &str {
    name
}
