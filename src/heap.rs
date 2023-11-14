#[derive(Debug)]
struct Entry {
    id: i32,
}

fn main() {
    let heap: Box<Entry> = Box::new(Entry { id: 100 }); // This is stored on heap
    let stack = *heap; // This will store the data on heap back to stack
    println!("{:?}", stack)
}






