// Imagine Iterator to be like Generator in JavaScript.

struct Odd(i32);

impl Iterator for Odd {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0 + 2;
        return Some(self.0);
    }
}

fn main() {
    let mut odd = Odd(1);

    println!("{:?}", odd.next());
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());

    // We can use for..in with iterator. This will automatically call .next()
    for o in odd {
        println!("Odd = {}", o);
        if o == 21 {
            break;
        }
    }

    let odd2 = Odd(1);

    for e in odd2.map(|num| num + 1) {
        println!("Even = {}", e);
        if e == 22 {
            break;
        }
    }
}
