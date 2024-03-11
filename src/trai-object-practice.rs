trait Cost {
    fn get_total_cost(&self) -> f64;
    fn get_cost_per_meter(&self) -> f64;
}

struct Carpet(f64);
impl Cost for Carpet {
    fn get_total_cost(&self) -> f64 {
        self.get_cost_per_meter() * self.0
    }
    fn get_cost_per_meter(&self) -> f64 {
        10.00
    }
}

struct Tile(f64);
impl Cost for Tile {
    fn get_total_cost(&self) -> f64 {
        self.get_cost_per_meter() * self.0
    }
    fn get_cost_per_meter(&self) -> f64 {
        15.00
    }
}

struct Wood(f64);
impl Cost for Wood {
    fn get_total_cost(&self) -> f64 {
        self.get_cost_per_meter() * self.0
    }
    fn get_cost_per_meter(&self) -> f64 {
        20.00
    }
}

fn main() {
    let carpet = Box::new(Carpet(12.00));
    let tile = Box::new(Tile(20.00));
    let wood = Box::new(Wood(12.00));

    let materials: Vec<Box<dyn Cost>> = vec![carpet, tile, wood];

    println!("Total cost is {:?}", get_total_costs(&materials))
}

fn get_total_costs(materials: &Vec<Box<dyn Cost>>) -> f64 {
    materials
        .iter()
        .map(|material| material.get_total_cost())
        .sum()
}
