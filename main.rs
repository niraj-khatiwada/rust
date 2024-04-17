enum Error {
    NetworkError(u16),
    MechanicalError,
}

fn main() {
    let error = Error::NetworkError(200); // or 400

    match error {
        Error::NetworkError(code @ 200 | code @ 404) => println!("Status code is {code}"),
        _ => {}
    }
}
