#[derive(Debug)]
pub struct User {
    pub name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn print(&self) {
        println!("User is: {:?}", self);
    }
}
