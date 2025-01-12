// Shared method logic in Traits

// Super Trait
trait TaxActions {
    // Getter
    // Here, amount() method is called a getter method since it's trying to get a data that tax_amount() method logic utilizes
    // We do not need to enforce amount property to exist in struct or enum. This method abstracts the logic to apply the tax rate to any amount that the original data type needs to define by themselves
    fn amount(&self) -> f64;

    // Setter
    fn set_amount(&mut self, new_amount: f64) -> f64;

    // default setter logic for doubling amount
    fn double_amount(&mut self) -> f64 {
        self.set_amount(self.amount() * 2.0)
    }
}

trait Tax: TaxActions {
    const TAX_RATE: f64 = 0.25;

    fn tax_amount(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

struct Income {
    salary: f64,
    bonus: f64,
}

impl Tax for Income {
    const TAX_RATE: f64 = 0.50;
}

impl TaxActions for Income {
    fn amount(&self) -> f64 {
        self.salary + self.bonus
    }

    fn set_amount(&mut self, new_amount: f64) -> f64 {
        self.salary = new_amount;
        self.amount()
    }
}

struct Land {
    price: f64,
    vat: f64,
}

impl Tax for Land {
    const TAX_RATE: f64 = 0.10;
}

impl TaxActions for Land {
    fn amount(&self) -> f64 {
        self.price + self.vat
    }

    fn set_amount(&mut self, new_amount: f64) -> f64 {
        self.price = new_amount;
        self.amount()
    }
}

fn main() {
    let mut income = Income {
        salary: 1900.0,
        bonus: 100.0,
    };
    println!("Income before tax {}", income.amount());
    println!("Income after tax {}", income.amount() - income.tax_amount());

    // Salary doubled
    income.double_amount();

    println!("Income before tax {} after salary hike", income.amount());
    println!(
        "Income after tax {} after salary hike",
        income.amount() - income.tax_amount()
    );

    let mut land = Land {
        price: 100000.0,
        vat: 100.0,
    };
    println!("Land before tax {}", land.amount());
    println!("Land after tax {}", land.amount() - land.tax_amount());

    land.set_amount(50000.0);
    println!("Land before tax {} after market down", land.amount());
    println!(
        "Land after tax {} after market down",
        land.amount() - land.tax_amount()
    );
}
