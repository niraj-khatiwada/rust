// Same as From/Into but returns a Result

use std::convert::TryFrom;

#[derive(Debug)]
enum _Error {
    NetworkError(u8),
    MechanicalError,
}

impl TryFrom<u8> for _Error {
    type Error = String;
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            0 => Ok(_Error::NetworkError(code)),
            _ => Ok(_Error::MechanicalError),
        }
    }
}

fn main() {
    let err = _Error::try_from(0);

    println!("{:?}", err);

    let intoed: Result<_Error, String> = 10_u8.try_into();

    println!("{:?}", intoed);
}
