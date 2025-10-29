#[derive(Debug)]
struct Counter {
    count: u32,
    max: u32,
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        // weird part is you need to have a mutable structure
        if (self.count + 1) <= self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter { count: 0, max: 100 };
    while let Some(c) = counter.next() {
        println!("{c}");
    }
    println!("{counter:?}");

    // Use .map, .filter. etc.
    let counter = Counter { count: 0, max: 100 };
    let list: Vec<u32> = counter.map(|x| x * 100).collect();
    println!("{list:?}");
}
