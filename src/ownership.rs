fn main() {
    let has_errors = [true, false, true]; // Array of scalar types. has_errors owns the scalar types.
    let first_has_error = has_errors[0]; // Valid since scalar types are copied. So  has_errors[0] value will be copied to first_has_error variable.

    let mut languages = [String::from("Rust"), String::from("Golang")]; // Array of compound types. languages owns the owners within the array as well.

    // let _first_language = languages[0]; // Invalid since compound types cannot be copied(does not have Copy trait). You are trying to move the first value out of languages which is not allowed since the elements inside an the array is owned by languages variable. You can only reference it.
    let first_language = &languages[0]; // Valid

    println!("{:?}", languages);

    // Mutations: You can only mutate the original owner.
    let mut_first_language = &mut languages[0];
    (*mut_first_language).push_str(" is king");

    println!("{:?}", languages);

    // Same goes for Tuple as well
    let languages_tuple = (String::from("Rust"), String::from("Golang"));
    let first_language_tuple = &languages_tuple.0;
    println!("{:?} {:?}", languages_tuple, first_language_tuple);
}
