// Shared method logic in Traits
trait Tax {
    const TAX_RATE: f64 = 0.25;

    // Here, amount() method is called a getter method since it's trying to get a data that cut_tax() method logic utilizes
    // We do not need to enforce amount property to exist in struct or enum. This method abstracts the logic to apply the tax rate to any amount that the original data type needs to define by themselves
    fn amount(&self) -> f64;

    // default logic for method .cut_tax()
    fn cut_tax(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

struct Income {
    salary: f64,
    bonus: f64,
}

impl Tax for Income {
    const TAX_RATE: f64 = 0.50;

    fn amount(&self) -> f64 {
        self.salary + self.bonus
    }
}

struct Land {
    price: f64,
    vat: f64,
}

impl Tax for Land {
    const TAX_RATE: f64 = 0.10;

    fn amount(&self) -> f64 {
        self.price + self.vat
    }
}

fn main() {
    let income = Income {
        salary: 1900.0,
        bonus: 100.0,
    };
    println!("Income before tax {}", income.amount());
    println!("Income after tax {}", income.amount() - income.cut_tax());

    let land = Land {
        price: 100000.0,
        vat: 100.0,
    };
    println!("Land before tax {}", land.amount());
    println!("Land after tax {}", land.amount() - land.cut_tax());
}
