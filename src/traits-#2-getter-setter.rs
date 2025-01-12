// Shared method logic in Traits
trait Tax {
    const TAX_RATE: f64 = 0.25;

    // Getter
    // Here, amount() method is called a getter method since it's trying to get a data that cut_tax() method logic utilizes
    // We do not need to enforce amount property to exist in struct or enum. This method abstracts the logic to apply the tax rate to any amount that the original data type needs to define by themselves
    fn amount(&self) -> f64;

    // Setter
    fn set_amount(&mut self, new_amount: f64) -> f64;

    // default getter logic for method .cut_tax()
    fn cut_tax(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }

    // default setter logic for doubling amount
    fn double_amount(&mut self) -> f64 {
        self.set_amount(self.amount() * 2.0)
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
    println!("Income after tax {}", income.amount() - income.cut_tax());

    // Salary doubled
    income.double_amount();

    println!("Income before tax {} after salary hike", income.amount());
    println!(
        "Income after tax {} after salary hike",
        income.amount() - income.cut_tax()
    );

    let mut land = Land {
        price: 100000.0,
        vat: 100.0,
    };
    println!("Land before tax {}", land.amount());
    println!("Land after tax {}", land.amount() - land.cut_tax());

    land.set_amount(50000.0);
    println!("Land before tax {} after market down", land.amount());
    println!(
        "Land after tax {} after market down",
        land.amount() - land.cut_tax()
    );
}
