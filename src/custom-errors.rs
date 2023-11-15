// A custom error requires 3 things:
// - Debug Trait
// - Display Trait
// Error Trait from std::error

use std::error::Error;
use std::fmt;

// Debug Trait
#[derive(Debug)]
enum LoginError {
    NetworkError(u32),
    AuthenticationError,
    DatabaseError,
}

// Error Trait
impl Error for LoginError {}

// Display Trait
impl fmt::Display for LoginError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NetworkError(code) => write!(formatter, "Network error occurred! Status Code: {}", code),
            Self::AuthenticationError => write!(formatter, "Authentication error occurred!"),
            Self::DatabaseError => write!(formatter, "Database error occurred!")
        }
    }
}

fn main() {
    println!("{:?}", login());
    println!("{} \n{} \n{}", LoginError::DatabaseError, LoginError::AuthenticationError, LoginError::NetworkError(301))
}


fn login() -> Result<(), LoginError> {
    return Err(LoginError::NetworkError(302));
}