use std::ops::{Range, RangeInclusive};

fn main() {
    let range: Range<u8> = 0..255;
    println!("{:?}", range);

    for i in range {
        println!("{i}");
    }

    let range_inclusive: RangeInclusive<u16> = 0..=255;
    println!("{:?}", range_inclusive);

    for i in range_inclusive {
        println!("{i}");
    }

    // a to z
    let alphabets = 'a'..='z';
    for i in alphabets {
        println!("{i}");
    }

    use std::ops::Range;

    let range: Range<i32> = 1..100;
    for i in range {
        println!("{}", i);
    }

    let range_vec: Vec<i32> = (1..=100).collect();
    dbg!(range_vec);

    for alpha in 'a'..='z' {
        println!("{}", alpha);
    }
    let alpha_vec: Vec<char> = ('a'..='z').collect();
    dbg!(alpha_vec);
}
