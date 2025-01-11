use thiserror::Error;

struct Package {
    name: String,
    is_shipped: bool,
    error: Option<PackageError>,
}

#[derive(Debug, Error)]
enum PackageError {
    #[error("Package not found. Reason=\"{reason}\"")]
    NotFound { reason: String },
    #[error("Package is damaged.")]
    Damaged,
    #[error("Package is lost.")]
    Lost,
}

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
