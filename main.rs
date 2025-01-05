use lib::_impl::{Gender, Person};

fn main() {
    let gender = Gender::Male;
    println!("Is male? {}", gender.is_male());
    let me = Person::new(String::from("Niraj"), 27, gender);
    println!(
        "Name: {:?} Age: {:?} Gender: {:?}",
        me.get_name(),
        me.get_age(),
        me.get_gender()
    );
}
