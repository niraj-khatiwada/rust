struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        return Self { weight };
    }
}

impl Default for Package {
    fn default() -> Self {
        return Self { weight: 100.00 };
    }
}

fn main() {
    let package = Package::default();
}





