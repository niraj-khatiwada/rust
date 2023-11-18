struct Friends {
    names: Vec<String>,
}

// Move
// impl IntoIterator for Friends {
//     type Item = String;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.names.into_iter()
//     }
// }

// Borrow
// impl<'a> IntoIterator for &'a Friends {
//     type Item = &'a String;
//     // Since borrow of String is slice.
//     type IntoIter = std::slice::Iter<'a, String>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.names.iter()
//     }
// }

// Mutable Borrow
impl<'a> IntoIterator for &'a mut Friends {
    type Item = &'a mut String;
    // Since borrow of String is slice.
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

fn main() {
    // let names = vec! {String::from("Niraj"), String::from("Broo")};
    // let friends = Friends { names };

    // Move usage
    // We can directly access values since into_iter is already implemented
    // for f in friends {
    //     println!("{:?}", f)
    // }
    // // This second iter cannot be done since value is already moved above.
    // for f in friends {
    //     println!("{:?}", f)
    // }

    // Borrow Usage
    // for f in &friends {
    //     println!("{:?}", f)
    // }
    // for f in &friends {
    //     println!("{:?}", f)
    // }


    // Mutable Borrow
    let names = vec! {String::from("Niraj"), String::from("Broo")};
    let mut friends = Friends { names };

    for f in &mut friends {
        *f = String::from("FizzBuzz");
        println!("{:?}", f)
    }
}