fn main() {
    // Can pass both String or Option. Return value will always be an Option
    convert_into_option(String::from("Niraj"));
    convert_into_option(Some(String::from("Niraj")));
    convert_into_option(None);

    // convert_into_result(String::from("Niraj"));
    convert_into_result(Ok(String::from("Nirak")));
    convert_into_result(Err(String::from("Error")));
    // convert_into_result(String::from("Nirak")); // This won;t work because the compiler cannot figure out if this string should be Ok(_) or Error(_)
}

fn convert_into_option<T: Into<Option<String>>>(value: T) -> Option<String> {
    let val = value.into();
    println!("{:?}", val);
    val
}

fn convert_into_result<T: Into<Result<String, String>>>(value: T) -> Result<String, String> {
    let val = value.into();
    println!("{:?}", val);
    val
}
