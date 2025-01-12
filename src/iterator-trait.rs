struct Odd {
    num: isize,
}

impl Iterator for Odd {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.num += 2;
        Some(self.num)
    }
}

fn main() {
    let mut odd = Odd { num: 3 };
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());

    for _ in 1..=10 {
        println!("{:?}", odd.next());
    }
}
