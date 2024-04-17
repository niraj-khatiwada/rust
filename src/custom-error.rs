// Custom Errors

// Custom Error need to implement 3 traits:
// - Debug
// - Display
// - Error

use thiserror::Error;

#[derive(Debug, Error)]
enum CustomError {
    #[error("Mechanical error occurred: {0}")]
    MechanicalFailure(i32),
    #[error("Network error occurred")]
    NetworkFailure,
}

fn main() {
    let custom_error: Result<_, CustomError> = result(CustomError::MechanicalFailure(200));

    match custom_error {
        Ok(_) => {}
        Err(msg) => println!("{:?}", msg.to_string()),
    }

    let custom_error2: Result<_, CustomError> = result(CustomError::NetworkFailure);

    match custom_error2 {
        Ok(_) => {}
        Err(msg) => println!("{:?}", msg.to_string()),
    }
}

fn result(err: CustomError) -> Result<String, CustomError> {
    Err(err)
}
