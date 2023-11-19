use rayon::prelude::*;

fn main() {
    let list = vec! {"   1234", "-9999", "Niraj", "12gh", "100", "-69"};

    // This is ran one by one right now.
    let ids = list.iter().map(|id| id.trim()).filter_map(|id| id.parse().ok()).filter(|id| id >= &100).collect::<Vec<usize>>();
    println!("{:?}", ids);

    // Using rayon we can run this in parallel
    let ids_parallel = list.par_iter().map(|id| id.trim()).filter_map(|id| id.parse().ok()).filter(|id| id >= &100).collect::<Vec<usize>>();
    println!("{:?}", ids_parallel);

    // Parallel sorting
    let mut ids_sort = ids;
    ids_sort.par_sort();
    println!("Sorted: {:?}", ids_sort);

    // for in cannot be used with parallel
    // We need to use for_each instead

    let list2 = vec! {"1234", "-9999", "Niraj", "12gh", "100", "-69"};

    // The order of the output won't be same as the original since we are doing parallel processing.
    list2.par_iter().for_each(|id| println!("--{}--", id));
}