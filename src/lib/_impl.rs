#[derive(Clone, PartialEq, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn is_male(&self) -> bool {
        self.eq(&Gender::Male)
    }
}

pub struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

impl Person {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Person { name, age, gender }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_gender(&self) -> &Gender {
        &self.gender
    }
}
