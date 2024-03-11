#[derive(Debug)]
struct Entry {
    id: i32,
}

fn main() {
    let data = Entry { id: 12 };
    let data_heap: Box<Entry> = Box::new(data);
    let data_stack = *data_heap;
    println!("{:?}", data_stack);
}
