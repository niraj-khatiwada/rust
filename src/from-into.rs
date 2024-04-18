#[derive(Debug)]
enum Error {
    NetworkError(u8),
    MechanicalError,
}

impl From<u8> for Error {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::NetworkError(code),
            _ => Self::MechanicalError,
        }
    }
}

fn main() {
    let name = String::from("Niraj");

    let _name: String = "Niraj".into();

    let error = Error::from(0);
    println!("{:?}", error);
}

fn to_owned(slice: &str) -> String {
    slice.into()
}
