use std::error::Error;
use std::fmt::Display;
use thiserror::Error as ThisError;

// Custom Error Manual Way

#[derive(Debug)]
enum CustomLockError {
    MechanicalError(u16),
    NetworkError,
    NotAuthorizedError,
}

impl Error for CustomLockError {}

impl Display for CustomLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomLockError::MechanicalError(code) => write!(f, "Mechanical Error @{}", code),
            CustomLockError::NetworkError => write!(f, "Network Error"),
            CustomLockError::NotAuthorizedError => write!(f, "Unauthorized Error"),
        }
    }
}

// Automatic Way
#[derive(Debug, ThisError)]
enum LockError {
    #[error("Mechanical Error @{0}")]
    MechanicalError(u16),
    #[error("Network Error")]
    NetworkError,
    #[error("Unauthorized Error")]
    NotAuthorizedError,
}
fn main() {
    // let err = CustomLockError::MechanicalError(2002);
    let err = LockError::MechanicalError(2002);

    println!("{}", err); // Result will be same for both err.
}
