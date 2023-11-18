struct Odd {
    number: i32,
    max: i32,
}

impl Iterator for Odd {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.number += 2;
        if self.number > self.max {
            None
        } else {
            Some(self.number)
        }
    }
}

fn main() {
    let mut odd = Odd { number: 1, max: 5 };

    // Gives Option
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());
    println!("{:?}", odd.next());

    let mut odd2 = Odd { number: 1, max: 5 };
    for o in odd2 {
        // Gives direct Some value
        println!("ODD: {:?}", o)
    }

    let mut odd3 = Odd { number: 1, max: 5 };
    for e in odd3.map(|n| n + 1) {
        println!("EVEN: {:?}", e)
    }
}