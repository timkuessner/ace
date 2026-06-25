pub struct Person {
    pub id: u32,
    pub age: u32,
}

impl Person {
    pub fn new(id: u32, age: u32) -> Self {
        Person { id, age }
    }

    pub fn step(&mut self) {
        self.age += 1;
    }

    pub fn report(&self) {
        println!("Person Report - ID: {}, Age: {}", self.id, self.age);
    }
}