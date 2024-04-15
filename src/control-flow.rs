fn main() {
    fizz_buzz(3);
    fizz_buzz(5);
    fizz_buzz(15);
    fizz_buzz(16);
}

fn fizz_buzz(num: i32) {
    let mut result: String = String::from("");
    let mut is_divisible = false;

    if num % 3 == 0 {
        result = String::from("fizz");
        is_divisible = true;
    }
    if num % 5 == 0 {
        result = format!("{result}buzz");
        is_divisible = true;
    }

    if !is_divisible {
        result = num.to_string();
    }

    println!("{result}")
}
