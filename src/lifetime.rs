const PI: f64 = 3.1;
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    // In case of self(instance), Rust will by default assume the return value reference's lifetime is the actual instance(self) lifetime. So, no explicit lifetime annotation is required.
    fn get_name(&self, b: &String) -> &str {
        &self.name
    }
}

fn main() {
    println!("{}", longest_str("Rust", "Go"));
    println!("{}", should_live_long());
}

// Lifetime elision
// Rust understands that since there's only one parameter, the reference seen in return type is of that parameter. So no explicity lifetime is required.
fn return_slice(list: &Vec<i32>) -> &[i32] {
    &list
}

// Explicit lifetimes required.
// But here, we need to explicitly supply the lifetime because Rust cannot figure out the lifetime from parameter since there are multiple parameters.
fn return_slice_2<'a, 'b>(list: &'a Vec<i32>, list2: &'b Vec<i32>) -> &'a [i32] {
    &list
}

// Question: Return the longest string
// This is called shared lifetimes.
fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }
    b
}

// This basically means that the return type &str must live through the whole app lifecycle.
fn should_live_long() -> &'static str {
    "Rust is king"
}

// We know that constant PI will exist through the whole app lifecycle.
fn get_pi() -> &'static f64 {
    &PI
}
