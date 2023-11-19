// Only valid for loop keyword, not for whie or for in
fn main() {
    let mut num = 0;
    let abc = 'label: loop {
        num += 1;
        if num == 3 {
            break 'label num;
        }
    };
    println!("{:?}", abc);
}