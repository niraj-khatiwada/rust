trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);

impl Sale for FullSale {
    fn amount(&self) -> f64 {
        return self.0;
    }
}

struct AbsoluteCouponSale(f64, f64);

impl Sale for AbsoluteCouponSale {
    fn amount(&self) -> f64 {
        return self.0 - self.1;
    }
}

struct PercentageCouponSale(f64, f64);

impl Sale for PercentageCouponSale {
    fn amount(&self) -> f64 {
        return self.0 - ((self.1 * self.0) / 100.0);
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

fn main() {
    let amount = 20.0;
    let full_sale = Box::new(FullSale(amount));
    let absolute_sale = Box::new(AbsoluteCouponSale(amount, 10.0));
    let percentage_sale = Box::new(PercentageCouponSale(amount, 10.0));

    let list: Vec<Box<dyn Sale>> = vec![full_sale, absolute_sale, percentage_sale];

    println!("{:?}", calculate_revenue(&list));
}






