#[derive(Debug)]
struct Person {
    name: String,
}


fn main() {
    let mut num = vec![1, 2, 3]; // initialize with existing values


    println!("{:?}", num.len());
    num.insert(0, 100); // inserts new value on index 0 and moves elements after that to the right
    println!("{:?}", num);
    num.push(11); // pushes at the end of the list
    println!("{:?}", num);
    num.pop(); // removes last item
    println!("{:?}", num);
    println!("index 0 {:?}", num[0]);

    let mut nums: Vec<i32> = Vec::new(); // Empty Vec
    nums.push(1);
    println!("{:?}", nums);

    let mut persons: Vec<&Person> = Vec::new();

    let person = Person { name: "Niraj".to_string() };
    persons.push(&person); // When we push the item, the item will be moved. That's why we need to borrow the item from main to person vec.
    persons.push(&person);

    println!("{:?}", persons);

    let another_person_list = vec![Person { name: "Niraj".to_string() }, Person { name: "Suraj".to_string() }];
    for person in &another_person_list {
        match person.name.as_str() {
            "Niraj" => println!("N"),
            _ => println!("S")
        }
    }
    println!("Length {:?}", another_person_list.len());
}




