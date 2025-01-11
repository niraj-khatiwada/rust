use std::error::Error;
use std::fmt::Display;

struct Package {
    name: String,
    is_shipped: bool,
    error: Option<PackageError>,
}

#[derive(Debug)]
enum PackageError {
    NotFound { reason: String },
    Damaged,
    Lost,
}

impl Error for PackageError {}

impl Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { reason } => write!(f, "Package not found. Reason=\"{}\"", reason),
            Self::Damaged => write!(f, "Package damaged."),
            Self::Lost => write!(f, "Package lost."),
        }
    }
}

// View ./custom-error-crate for automatically managing & simplifying this using Error trait from this error package
fn main() {
    let package = Package {
        name: String::from("Laptop"),
        is_shipped: false,
        error: Some(PackageError::NotFound {
            reason: String::from("Location mismatch."),
        }),
    };

    println!("{}", package.error.unwrap()); // Package not found. Reason="Location not found"
}
