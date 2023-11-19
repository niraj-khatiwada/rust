// Used for break or continue to specify which ones to break or continue
fn main() {
    let list = vec! {1, 2, 3};
    let matrix = vec! {vec! {1, 2, 3}, vec! {4, 5, 6}};

    'rows: for x in matrix.iter() {
        'columns: for y in x {
            if y.eq(&5) {
                break 'rows;
            }
            if y.eq(&2) {
                break 'columns;
            }
            println!("{:?}", y);
        }
    }
}